pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
}
pub mod stdint_intn_h {
    pub type i32 = __int32_t;
    use super::types_h::__int32_t;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod sys_types_h {
    pub type ssize_t = isize;
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_keysym_flags = u32;
    pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;
    pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
    pub const XKB_KEYSYM_MAX: i32 = 0x1fffffff as i32;
    pub use crate::xkb::keysym_utf::xkb_utf32_to_keysym;
}
pub mod keysym_names_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct name_keysym {
        pub keysym: xkb_keysym_t,
        pub offset: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct deprecated_keysym {
        pub keysym: xkb_keysym_t,
        pub offset: uint16_t,
        pub explicit_index: uint8_t,
        pub explicit_count: uint8_t,
    }
    pub static mut keysym_names: *const i8 = b"0\x001\x002\x003\x003270_AltCursor\x003270_Attn\x003270_BackTab\x003270_ChangeScreen\x003270_Copy\x003270_CursorBlink\x003270_CursorSelect\x003270_DeleteWord\x003270_Duplicate\x003270_Enter\x003270_EraseEOF\x003270_EraseInput\x003270_ExSelect\x003270_FieldMark\x003270_Ident\x003270_Jump\x003270_KeyClick\x003270_Left2\x003270_PA1\x003270_PA2\x003270_PA3\x003270_Play\x003270_PrintScreen\x003270_Quit\x003270_Record\x003270_Reset\x003270_Right2\x003270_Rule\x003270_Setup\x003270_Test\x004\x005\x006\x007\08\09\0A\0a\0Aacute\0aacute\0Abelowdot\0abelowdot\0abovedot\0Abreve\0abreve\0Abreveacute\0abreveacute\0Abrevebelowdot\0abrevebelowdot\0Abrevegrave\0abrevegrave\0Abrevehook\0abrevehook\0Abrevetilde\0abrevetilde\0AccessX_Enable\0AccessX_Feedback_Enable\0Acircumflex\0acircumflex\0Acircumflexacute\0acircumflexacute\0Acircumflexbelowdot\0acircumflexbelowdot\0Acircumflexgrave\0acircumflexgrave\0Acircumflexhook\0acircumflexhook\0Acircumflextilde\0acircumflextilde\0acute\0Adiaeresis\0adiaeresis\0AE\0ae\0Agrave\0agrave\0Ahook\0ahook\0Alt_L\0Alt_R\0Amacron\0amacron\0ampersand\0Aogonek\0aogonek\0apostrophe\0approxeq\0approximate\0Arabic_0\0Arabic_1\0Arabic_2\0Arabic_3\0Arabic_4\0Arabic_5\0Arabic_6\0Arabic_7\0Arabic_8\0Arabic_9\0Arabic_ain\0Arabic_alef\0Arabic_alefmaksura\0Arabic_beh\0Arabic_comma\0Arabic_dad\0Arabic_dal\0Arabic_damma\0Arabic_dammatan\0Arabic_ddal\0Arabic_farsi_yeh\0Arabic_fatha\0Arabic_fathatan\0Arabic_feh\0Arabic_fullstop\0Arabic_gaf\0Arabic_ghain\0Arabic_ha\0Arabic_hah\0Arabic_hamza\0Arabic_hamza_above\0Arabic_hamza_below\0Arabic_hamzaonalef\0Arabic_hamzaonwaw\0Arabic_hamzaonyeh\0Arabic_hamzaunderalef\0Arabic_heh\0Arabic_heh_doachashmee\0Arabic_heh_goal\0Arabic_jeem\0Arabic_jeh\0Arabic_kaf\0Arabic_kasra\0Arabic_kasratan\0Arabic_keheh\0Arabic_khah\0Arabic_lam\0Arabic_madda_above\0Arabic_maddaonalef\0Arabic_meem\0Arabic_noon\0Arabic_noon_ghunna\0Arabic_peh\0Arabic_percent\0Arabic_qaf\0Arabic_question_mark\0Arabic_ra\0Arabic_rreh\0Arabic_sad\0Arabic_seen\0Arabic_semicolon\0Arabic_shadda\0Arabic_sheen\0Arabic_sukun\0Arabic_superscript_alef\0Arabic_switch\0Arabic_tah\0Arabic_tatweel\0Arabic_tcheh\0Arabic_teh\0Arabic_tehmarbuta\0Arabic_thal\0Arabic_theh\0Arabic_tteh\0Arabic_veh\0Arabic_waw\0Arabic_yeh\0Arabic_yeh_baree\0Arabic_zah\0Arabic_zain\0Aring\0aring\0Armenian_accent\0Armenian_amanak\0Armenian_apostrophe\0Armenian_AT\0Armenian_at\0Armenian_AYB\0Armenian_ayb\0Armenian_BEN\0Armenian_ben\0Armenian_but\0Armenian_CHA\0Armenian_cha\0Armenian_DA\0Armenian_da\0Armenian_DZA\0Armenian_dza\0Armenian_E\0Armenian_e\0Armenian_exclam\0Armenian_FE\0Armenian_fe\0Armenian_full_stop\0Armenian_GHAT\0Armenian_ghat\0Armenian_GIM\0Armenian_gim\0Armenian_HI\0Armenian_hi\0Armenian_HO\0Armenian_ho\0Armenian_hyphen\0Armenian_INI\0Armenian_ini\0Armenian_JE\0Armenian_je\0Armenian_KE\0Armenian_ke\0Armenian_KEN\0Armenian_ken\0Armenian_KHE\0Armenian_khe\0Armenian_ligature_ew\0Armenian_LYUN\0Armenian_lyun\0Armenian_MEN\0Armenian_men\0Armenian_NU\0Armenian_nu\0Armenian_O\0Armenian_o\0Armenian_paruyk\0Armenian_PE\0Armenian_pe\0Armenian_PYUR\0Armenian_pyur\0Armenian_question\0Armenian_RA\0Armenian_ra\0Armenian_RE\0Armenian_re\0Armenian_SE\0Armenian_se\0Armenian_separation_mark\0Armenian_SHA\0Armenian_sha\0Armenian_shesht\0Armenian_TCHE\0Armenian_tche\0Armenian_TO\0Armenian_to\0Armenian_TSA\0Armenian_tsa\0Armenian_TSO\0Armenian_tso\0Armenian_TYUN\0Armenian_tyun\0Armenian_verjaket\0Armenian_VEV\0Armenian_vev\0Armenian_VO\0Armenian_vo\0Armenian_VYUN\0Armenian_vyun\0Armenian_YECH\0Armenian_yech\0Armenian_yentamna\0Armenian_ZA\0Armenian_za\0Armenian_ZHE\0Armenian_zhe\0asciicircum\0asciitilde\0asterisk\0at\0Atilde\0atilde\0AudibleBell_Enable\0B\0b\0Babovedot\0babovedot\0backslash\0BackSpace\0BackTab\0ballotcross\0bar\0because\0Begin\0blank\0block\0botintegral\0botleftparens\0botleftsqbracket\0botleftsummation\0botrightparens\0botrightsqbracket\0botrightsummation\0bott\0botvertsummationconnector\0BounceKeys_Enable\0braceleft\0braceright\0bracketleft\0bracketright\0braille_blank\0braille_dot_1\0braille_dot_10\0braille_dot_2\0braille_dot_3\0braille_dot_4\0braille_dot_5\0braille_dot_6\0braille_dot_7\0braille_dot_8\0braille_dot_9\0braille_dots_1\0braille_dots_12\0braille_dots_123\0braille_dots_1234\0braille_dots_12345\0braille_dots_123456\0braille_dots_1234567\0braille_dots_12345678\0braille_dots_1234568\0braille_dots_123457\0braille_dots_1234578\0braille_dots_123458\0braille_dots_12346\0braille_dots_123467\0braille_dots_1234678\0braille_dots_123468\0braille_dots_12347\0braille_dots_123478\0braille_dots_12348\0braille_dots_1235\0braille_dots_12356\0braille_dots_123567\0braille_dots_1235678\0braille_dots_123568\0braille_dots_12357\0braille_dots_123578\0braille_dots_12358\0braille_dots_1236\0braille_dots_12367\0braille_dots_123678\0braille_dots_12368\0braille_dots_1237\0braille_dots_12378\0braille_dots_1238\0braille_dots_124\0braille_dots_1245\0braille_dots_12456\0braille_dots_124567\0braille_dots_1245678\0braille_dots_124568\0braille_dots_12457\0braille_dots_124578\0braille_dots_12458\0braille_dots_1246\0braille_dots_12467\0braille_dots_124678\0braille_dots_12468\0braille_dots_1247\0braille_dots_12478\0braille_dots_1248\0braille_dots_125\0braille_dots_1256\0braille_dots_12567\0braille_dots_125678\0braille_dots_12568\0braille_dots_1257\0braille_dots_12578\0braille_dots_1258\0braille_dots_126\0braille_dots_1267\0braille_dots_12678\0braille_dots_1268\0braille_dots_127\0braille_dots_1278\0braille_dots_128\0braille_dots_13\0braille_dots_134\0braille_dots_1345\0braille_dots_13456\0braille_dots_134567\0braille_dots_1345678\0braille_dots_134568\0braille_dots_13457\0braille_dots_134578\0braille_dots_13458\0braille_dots_1346\0braille_dots_13467\0braille_dots_134678\0braille_dots_13468\0braille_dots_1347\0braille_dots_13478\0braille_dots_1348\0braille_dots_135\0braille_dots_1356\0braille_dots_13567\0braille_dots_135678\0braille_dots_13568\0braille_dots_1357\0braille_dots_13578\0braille_dots_1358\0braille_dots_136\0braille_dots_1367\0braille_dots_13678\0braille_dots_1368\0braille_dots_137\0braille_dots_1378\0braille_dots_138\0braille_dots_14\0braille_dots_145\0braille_dots_1456\0braille_dots_14567\0braille_dots_145678\0braille_dots_14568\0braille_dots_1457\0braille_dots_14578\0braille_dots_1458\0braille_dots_146\0braille_dots_1467\0braille_dots_14678\0braille_dots_1468\0braille_dots_147\0braille_dots_1478\0braille_dots_148\0braille_dots_15\0braille_dots_156\0braille_dots_1567\0braille_dots_15678\0braille_dots_1568\0braille_dots_157\0braille_dots_1578\0braille_dots_158\0braille_dots_16\0braille_dots_167\0braille_dots_1678\0braille_dots_168\0braille_dots_17\0braille_dots_178\0braille_dots_18\0braille_dots_2\0braille_dots_23\0braille_dots_234\0braille_dots_2345\0braille_dots_23456\0braille_dots_234567\0braille_dots_2345678\0braille_dots_234568\0braille_dots_23457\0braille_dots_234578\0braille_dots_23458\0braille_dots_2346\0braille_dots_23467\0braille_dots_234678\0braille_dots_23468\0braille_dots_2347\0braille_dots_23478\0braille_dots_2348\0braille_dots_235\0braille_dots_2356\0braille_dots_23567\0braille_dots_235678\0braille_dots_23568\0braille_dots_2357\0braille_dots_23578\0braille_dots_2358\0braille_dots_236\0braille_dots_2367\0braille_dots_23678\0braille_dots_2368\0braille_dots_237\0braille_dots_2378\0braille_dots_238\0braille_dots_24\0braille_dots_245\0braille_dots_2456\0braille_dots_24567\0braille_dots_245678\0braille_dots_24568\0braille_dots_2457\0braille_dots_24578\0braille_dots_2458\0braille_dots_246\0braille_dots_2467\0braille_dots_24678\0braille_dots_2468\0braille_dots_247\0braille_dots_2478\0braille_dots_248\0braille_dots_25\0braille_dots_256\0braille_dots_2567\0braille_dots_25678\0braille_dots_2568\0braille_dots_257\0braille_dots_2578\0braille_dots_258\0braille_dots_26\0braille_dots_267\0braille_dots_2678\0braille_dots_268\0braille_dots_27\0braille_dots_278\0braille_dots_28\0braille_dots_3\0braille_dots_34\0braille_dots_345\0braille_dots_3456\0braille_dots_34567\0braille_dots_345678\0braille_dots_34568\0braille_dots_3457\0braille_dots_34578\0braille_dots_3458\0braille_dots_346\0braille_dots_3467\0braille_dots_34678\0braille_dots_3468\0braille_dots_347\0braille_dots_3478\0braille_dots_348\0braille_dots_35\0braille_dots_356\0braille_dots_3567\0braille_dots_35678\0braille_dots_3568\0braille_dots_357\0braille_dots_3578\0braille_dots_358\0braille_dots_36\0braille_dots_367\0braille_dots_3678\0braille_dots_368\0braille_dots_37\0braille_dots_378\0braille_dots_38\0braille_dots_4\0braille_dots_45\0braille_dots_456\0braille_dots_4567\0braille_dots_45678\0braille_dots_4568\0braille_dots_457\0braille_dots_4578\0braille_dots_458\0braille_dots_46\0braille_dots_467\0braille_dots_4678\0braille_dots_468\0braille_dots_47\0braille_dots_478\0braille_dots_48\0braille_dots_5\0braille_dots_56\0braille_dots_567\0braille_dots_5678\0braille_dots_568\0braille_dots_57\0braille_dots_578\0braille_dots_58\0braille_dots_6\0braille_dots_67\0braille_dots_678\0braille_dots_68\0braille_dots_7\0braille_dots_78\0braille_dots_8\0Break\0breve\0brokenbar\0Byelorussian_SHORTU\0Byelorussian_shortu\0C\0c\0C_H\0C_h\0c_h\0Cabovedot\0cabovedot\0Cacute\0cacute\0Cancel\0Caps_Lock\0careof\0caret\0caron\0Ccaron\0ccaron\0Ccedilla\0ccedilla\0Ccircumflex\0ccircumflex\0cedilla\0cent\0CH\0Ch\0ch\0checkerboard\0checkmark\0circle\0Clear\0ClearLine\0club\0Codeinput\0colon\0ColonSign\0combining_acute\0combining_belowdot\0combining_grave\0combining_hook\0combining_tilde\0comma\0containsas\0Control_L\0Control_R\0copyright\0cr\0crossinglines\0CruzeiroSign\0cuberoot\0currency\0cursor\0Cyrillic_A\0Cyrillic_a\0Cyrillic_BE\0Cyrillic_be\0Cyrillic_CHE\0Cyrillic_che\0Cyrillic_CHE_descender\0Cyrillic_che_descender\0Cyrillic_CHE_vertstroke\0Cyrillic_che_vertstroke\0Cyrillic_DE\0Cyrillic_de\0Cyrillic_DZHE\0Cyrillic_dzhe\0Cyrillic_E\0Cyrillic_e\0Cyrillic_EF\0Cyrillic_ef\0Cyrillic_EL\0Cyrillic_el\0Cyrillic_EM\0Cyrillic_em\0Cyrillic_EN\0Cyrillic_en\0Cyrillic_EN_descender\0Cyrillic_en_descender\0Cyrillic_ER\0Cyrillic_er\0Cyrillic_ES\0Cyrillic_es\0Cyrillic_GHE\0Cyrillic_ghe\0Cyrillic_GHE_bar\0Cyrillic_ghe_bar\0Cyrillic_HA\0Cyrillic_ha\0Cyrillic_HA_descender\0Cyrillic_ha_descender\0Cyrillic_HARDSIGN\0Cyrillic_hardsign\0Cyrillic_I\0Cyrillic_i\0Cyrillic_I_macron\0Cyrillic_i_macron\0Cyrillic_IE\0Cyrillic_ie\0Cyrillic_IO\0Cyrillic_io\0Cyrillic_JE\0Cyrillic_je\0Cyrillic_KA\0Cyrillic_ka\0Cyrillic_KA_descender\0Cyrillic_ka_descender\0Cyrillic_KA_vertstroke\0Cyrillic_ka_vertstroke\0Cyrillic_LJE\0Cyrillic_lje\0Cyrillic_NJE\0Cyrillic_nje\0Cyrillic_O\0Cyrillic_o\0Cyrillic_O_bar\0Cyrillic_o_bar\0Cyrillic_PE\0Cyrillic_pe\0Cyrillic_SCHWA\0Cyrillic_schwa\0Cyrillic_SHA\0Cyrillic_sha\0Cyrillic_SHCHA\0Cyrillic_shcha\0Cyrillic_SHHA\0Cyrillic_shha\0Cyrillic_SHORTI\0Cyrillic_shorti\0Cyrillic_SOFTSIGN\0Cyrillic_softsign\0Cyrillic_TE\0Cyrillic_te\0Cyrillic_TSE\0Cyrillic_tse\0Cyrillic_U\0Cyrillic_u\0Cyrillic_U_macron\0Cyrillic_u_macron\0Cyrillic_U_straight\0Cyrillic_u_straight\0Cyrillic_U_straight_bar\0Cyrillic_u_straight_bar\0Cyrillic_VE\0Cyrillic_ve\0Cyrillic_YA\0Cyrillic_ya\0Cyrillic_YERU\0Cyrillic_yeru\0Cyrillic_YU\0Cyrillic_yu\0Cyrillic_ZE\0Cyrillic_ze\0Cyrillic_ZHE\0Cyrillic_zhe\0Cyrillic_ZHE_descender\0Cyrillic_zhe_descender\0D\0d\0Dabovedot\0dabovedot\0Dacute_accent\0dagger\0Dcaron\0dcaron\0Dcedilla_accent\0Dcircumflex_accent\0Ddiaeresis\0dead_A\0dead_a\0dead_abovecomma\0dead_abovedot\0dead_abovereversedcomma\0dead_abovering\0dead_aboveverticalline\0dead_acute\0dead_apostrophe\0dead_belowbreve\0dead_belowcircumflex\0dead_belowcomma\0dead_belowdiaeresis\0dead_belowdot\0dead_belowmacron\0dead_belowring\0dead_belowtilde\0dead_belowverticalline\0dead_breve\0dead_capital_schwa\0dead_caron\0dead_cedilla\0dead_circumflex\0dead_currency\0dead_dasia\0dead_diaeresis\0dead_doubleacute\0dead_doublegrave\0dead_E\0dead_e\0dead_grave\0dead_greek\0dead_hamza\0dead_hook\0dead_horn\0dead_I\0dead_i\0dead_invertedbreve\0dead_iota\0dead_longsolidusoverlay\0dead_lowline\0dead_macron\0dead_O\0dead_o\0dead_ogonek\0dead_perispomeni\0dead_psili\0dead_SCHWA\0dead_schwa\0dead_semivoiced_sound\0dead_small_schwa\0dead_stroke\0dead_tilde\0dead_U\0dead_u\0dead_voiced_sound\0decimalpoint\0degree\0Delete\0DeleteChar\0DeleteLine\0Dgrave_accent\0diaeresis\0diamond\0digitspace\0dintegral\0division\0dollar\0DongSign\0doubbaselinedot\0doubleacute\0doubledagger\0doublelowquotemark\0Down\0downarrow\0downcaret\0downshoe\0downstile\0downtack\0DRemove\0Dring_accent\0Dstroke\0dstroke\0Dtilde\0E\0e\0Eabovedot\0eabovedot\0Eacute\0eacute\0Ebelowdot\0ebelowdot\0Ecaron\0ecaron\0Ecircumflex\0ecircumflex\0Ecircumflexacute\0ecircumflexacute\0Ecircumflexbelowdot\0ecircumflexbelowdot\0Ecircumflexgrave\0ecircumflexgrave\0Ecircumflexhook\0ecircumflexhook\0Ecircumflextilde\0ecircumflextilde\0EcuSign\0Ediaeresis\0ediaeresis\0Egrave\0egrave\0Ehook\0ehook\0eightsubscript\0eightsuperior\0Eisu_Shift\0Eisu_toggle\0elementof\0ellipsis\0em3space\0em4space\0Emacron\0emacron\0emdash\0emfilledcircle\0emfilledrect\0emopencircle\0emopenrectangle\0emptyset\0emspace\0End\0endash\0enfilledcircbullet\0enfilledsqbullet\0ENG\0eng\0enopencircbullet\0enopensquarebullet\0enspace\0Eogonek\0eogonek\0equal\0Escape\0ETH\0Eth\0eth\0Etilde\0etilde\0EuroSign\0exclam\0exclamdown\0Execute\0Ext16bit_L\0Ext16bit_R\0EZH\0ezh\0F\0f\0F1\0F10\0F11\0F12\0F13\0F14\0F15\0F16\0F17\0F18\0F19\0F2\0F20\0F21\0F22\0F23\0F24\0F25\0F26\0F27\0F28\0F29\0F3\0F30\0F31\0F32\0F33\0F34\0F35\0F4\0F5\0F6\0F7\0F8\0F9\0Fabovedot\0fabovedot\0Farsi_0\0Farsi_1\0Farsi_2\0Farsi_3\0Farsi_4\0Farsi_5\0Farsi_6\0Farsi_7\0Farsi_8\0Farsi_9\0Farsi_yeh\0femalesymbol\0ff\0FFrancSign\0figdash\0filledlefttribullet\0filledrectbullet\0filledrighttribullet\0filledtribulletdown\0filledtribulletup\0Find\0First_Virtual_Screen\0fiveeighths\0fivesixths\0fivesubscript\0fivesuperior\0fourfifths\0foursubscript\0foursuperior\0fourthroot\0function\0G\0g\0Gabovedot\0gabovedot\0Gbreve\0gbreve\0Gcaron\0gcaron\0Gcedilla\0gcedilla\0Gcircumflex\0gcircumflex\0Georgian_an\0Georgian_ban\0Georgian_can\0Georgian_char\0Georgian_chin\0Georgian_cil\0Georgian_don\0Georgian_en\0Georgian_fi\0Georgian_gan\0Georgian_ghan\0Georgian_hae\0Georgian_har\0Georgian_he\0Georgian_hie\0Georgian_hoe\0Georgian_in\0Georgian_jhan\0Georgian_jil\0Georgian_kan\0Georgian_khar\0Georgian_las\0Georgian_man\0Georgian_nar\0Georgian_on\0Georgian_par\0Georgian_phar\0Georgian_qar\0Georgian_rae\0Georgian_san\0Georgian_shin\0Georgian_tan\0Georgian_tar\0Georgian_un\0Georgian_vin\0Georgian_we\0Georgian_xan\0Georgian_zen\0Georgian_zhar\0grave\0greater\0greaterthanequal\0Greek_accentdieresis\0Greek_ALPHA\0Greek_alpha\0Greek_ALPHAaccent\0Greek_alphaaccent\0Greek_BETA\0Greek_beta\0Greek_CHI\0Greek_chi\0Greek_DELTA\0Greek_delta\0Greek_EPSILON\0Greek_epsilon\0Greek_EPSILONaccent\0Greek_epsilonaccent\0Greek_ETA\0Greek_eta\0Greek_ETAaccent\0Greek_etaaccent\0Greek_finalsmallsigma\0Greek_GAMMA\0Greek_gamma\0Greek_horizbar\0Greek_IOTA\0Greek_iota\0Greek_IOTAaccent\0Greek_iotaaccent\0Greek_iotaaccentdieresis\0Greek_IOTAdiaeresis\0Greek_IOTAdieresis\0Greek_iotadieresis\0Greek_KAPPA\0Greek_kappa\0Greek_LAMBDA\0Greek_lambda\0Greek_LAMDA\0Greek_lamda\0Greek_MU\0Greek_mu\0Greek_NU\0Greek_nu\0Greek_OMEGA\0Greek_omega\0Greek_OMEGAaccent\0Greek_omegaaccent\0Greek_OMICRON\0Greek_omicron\0Greek_OMICRONaccent\0Greek_omicronaccent\0Greek_PHI\0Greek_phi\0Greek_PI\0Greek_pi\0Greek_PSI\0Greek_psi\0Greek_RHO\0Greek_rho\0Greek_SIGMA\0Greek_sigma\0Greek_switch\0Greek_TAU\0Greek_tau\0Greek_THETA\0Greek_theta\0Greek_UPSILON\0Greek_upsilon\0Greek_UPSILONaccent\0Greek_upsilonaccent\0Greek_upsilonaccentdieresis\0Greek_UPSILONdieresis\0Greek_upsilondieresis\0Greek_XI\0Greek_xi\0Greek_ZETA\0Greek_zeta\0guilder\0guillemetleft\0guillemetright\0guillemotleft\0guillemotright\0H\0h\0hairspace\0Hangul\0Hangul_A\0Hangul_AE\0Hangul_AraeA\0Hangul_AraeAE\0Hangul_Banja\0Hangul_Cieuc\0Hangul_Codeinput\0Hangul_Dikeud\0Hangul_E\0Hangul_End\0Hangul_EO\0Hangul_EU\0Hangul_Hanja\0Hangul_Hieuh\0Hangul_I\0Hangul_Ieung\0Hangul_J_Cieuc\0Hangul_J_Dikeud\0Hangul_J_Hieuh\0Hangul_J_Ieung\0Hangul_J_Jieuj\0Hangul_J_Khieuq\0Hangul_J_Kiyeog\0Hangul_J_KiyeogSios\0Hangul_J_KkogjiDalrinIeung\0Hangul_J_Mieum\0Hangul_J_Nieun\0Hangul_J_NieunHieuh\0Hangul_J_NieunJieuj\0Hangul_J_PanSios\0Hangul_J_Phieuf\0Hangul_J_Pieub\0Hangul_J_PieubSios\0Hangul_J_Rieul\0Hangul_J_RieulHieuh\0Hangul_J_RieulKiyeog\0Hangul_J_RieulMieum\0Hangul_J_RieulPhieuf\0Hangul_J_RieulPieub\0Hangul_J_RieulSios\0Hangul_J_RieulTieut\0Hangul_J_Sios\0Hangul_J_SsangKiyeog\0Hangul_J_SsangSios\0Hangul_J_Tieut\0Hangul_J_YeorinHieuh\0Hangul_Jamo\0Hangul_Jeonja\0Hangul_Jieuj\0Hangul_Khieuq\0Hangul_Kiyeog\0Hangul_KiyeogSios\0Hangul_KkogjiDalrinIeung\0Hangul_Mieum\0Hangul_MultipleCandidate\0Hangul_Nieun\0Hangul_NieunHieuh\0Hangul_NieunJieuj\0Hangul_O\0Hangul_OE\0Hangul_PanSios\0Hangul_Phieuf\0Hangul_Pieub\0Hangul_PieubSios\0Hangul_PostHanja\0Hangul_PreHanja\0Hangul_PreviousCandidate\0Hangul_Rieul\0Hangul_RieulHieuh\0Hangul_RieulKiyeog\0Hangul_RieulMieum\0Hangul_RieulPhieuf\0Hangul_RieulPieub\0Hangul_RieulSios\0Hangul_RieulTieut\0Hangul_RieulYeorinHieuh\0Hangul_Romaja\0Hangul_SingleCandidate\0Hangul_Sios\0Hangul_Special\0Hangul_SsangDikeud\0Hangul_SsangJieuj\0Hangul_SsangKiyeog\0Hangul_SsangPieub\0Hangul_SsangSios\0Hangul_Start\0Hangul_SunkyeongeumMieum\0Hangul_SunkyeongeumPhieuf\0Hangul_SunkyeongeumPieub\0Hangul_switch\0Hangul_Tieut\0Hangul_U\0Hangul_WA\0Hangul_WAE\0Hangul_WE\0Hangul_WEO\0Hangul_WI\0Hangul_YA\0Hangul_YAE\0Hangul_YE\0Hangul_YEO\0Hangul_YeorinHieuh\0Hangul_YI\0Hangul_YO\0Hangul_YU\0Hankaku\0Hcircumflex\0hcircumflex\0heart\0hebrew_aleph\0hebrew_ayin\0hebrew_bet\0hebrew_beth\0hebrew_chet\0hebrew_dalet\0hebrew_daleth\0hebrew_doublelowline\0hebrew_finalkaph\0hebrew_finalmem\0hebrew_finalnun\0hebrew_finalpe\0hebrew_finalzade\0hebrew_finalzadi\0hebrew_gimel\0hebrew_gimmel\0hebrew_he\0hebrew_het\0hebrew_kaph\0hebrew_kuf\0hebrew_lamed\0hebrew_mem\0hebrew_nun\0hebrew_pe\0hebrew_qoph\0hebrew_resh\0hebrew_samech\0hebrew_samekh\0hebrew_shin\0Hebrew_switch\0hebrew_taf\0hebrew_taw\0hebrew_tet\0hebrew_teth\0hebrew_waw\0hebrew_yod\0hebrew_zade\0hebrew_zadi\0hebrew_zain\0hebrew_zayin\0Help\0Henkan\0Henkan_Mode\0hexagram\0Hiragana\0Hiragana_Katakana\0Home\0horizconnector\0horizlinescan1\0horizlinescan3\0horizlinescan5\0horizlinescan7\0horizlinescan9\0hpBackTab\0hpblock\0hpClearLine\0hpDeleteChar\0hpDeleteLine\0hpguilder\0hpInsertChar\0hpInsertLine\0hpIO\0hpKP_BackTab\0hplira\0hplongminus\0hpModelock1\0hpModelock2\0hpmute_acute\0hpmute_asciicircum\0hpmute_asciitilde\0hpmute_diaeresis\0hpmute_grave\0hpReset\0hpSystem\0hpUser\0hpYdiaeresis\0Hstroke\0hstroke\0ht\0Hyper_L\0Hyper_R\0hyphen\0I\0i\0Iabovedot\0Iacute\0iacute\0Ibelowdot\0ibelowdot\0Ibreve\0ibreve\0Icircumflex\0icircumflex\0identical\0Idiaeresis\0idiaeresis\0idotless\0ifonlyif\0Igrave\0igrave\0Ihook\0ihook\0Imacron\0imacron\0implies\0includedin\0includes\0infinity\0Insert\0InsertChar\0InsertLine\0integral\0intersection\0IO\0Iogonek\0iogonek\0ISO_Center_Object\0ISO_Continuous_Underline\0ISO_Discontinuous_Underline\0ISO_Emphasize\0ISO_Enter\0ISO_Fast_Cursor_Down\0ISO_Fast_Cursor_Left\0ISO_Fast_Cursor_Right\0ISO_Fast_Cursor_Up\0ISO_First_Group\0ISO_First_Group_Lock\0ISO_Group_Latch\0ISO_Group_Lock\0ISO_Group_Shift\0ISO_Last_Group\0ISO_Last_Group_Lock\0ISO_Left_Tab\0ISO_Level2_Latch\0ISO_Level3_Latch\0ISO_Level3_Lock\0ISO_Level3_Shift\0ISO_Level5_Latch\0ISO_Level5_Lock\0ISO_Level5_Shift\0ISO_Lock\0ISO_Move_Line_Down\0ISO_Move_Line_Up\0ISO_Next_Group\0ISO_Next_Group_Lock\0ISO_Partial_Line_Down\0ISO_Partial_Line_Up\0ISO_Partial_Space_Left\0ISO_Partial_Space_Right\0ISO_Prev_Group\0ISO_Prev_Group_Lock\0ISO_Release_Both_Margins\0ISO_Release_Margin_Left\0ISO_Release_Margin_Right\0ISO_Set_Margin_Left\0ISO_Set_Margin_Right\0Itilde\0itilde\0J\0j\0Jcircumflex\0jcircumflex\0jot\0K\0k\0kana_A\0kana_a\0kana_CHI\0kana_closingbracket\0kana_comma\0kana_conjunctive\0kana_E\0kana_e\0kana_FU\0kana_fullstop\0kana_HA\0kana_HE\0kana_HI\0kana_HO\0kana_HU\0kana_I\0kana_i\0kana_KA\0kana_KE\0kana_KI\0kana_KO\0kana_KU\0Kana_Lock\0kana_MA\0kana_ME\0kana_MI\0kana_middledot\0kana_MO\0kana_MU\0kana_N\0kana_NA\0kana_NE\0kana_NI\0kana_NO\0kana_NU\0kana_O\0kana_o\0kana_openingbracket\0kana_RA\0kana_RE\0kana_RI\0kana_RO\0kana_RU\0kana_SA\0kana_SE\0kana_SHI\0Kana_Shift\0kana_SO\0kana_SU\0kana_switch\0kana_TA\0kana_TE\0kana_TI\0kana_TO\0kana_TSU\0kana_tsu\0kana_TU\0kana_tu\0kana_U\0kana_u\0kana_WA\0kana_WO\0kana_YA\0kana_ya\0kana_YO\0kana_yo\0kana_YU\0kana_yu\0Kanji\0Kanji_Bangou\0kappa\0Katakana\0Kcedilla\0kcedilla\0Korean_Won\0KP_0\0KP_1\0KP_2\0KP_3\0KP_4\0KP_5\0KP_6\0KP_7\0KP_8\0KP_9\0KP_Add\0KP_BackTab\0KP_Begin\0KP_Decimal\0KP_Delete\0KP_Divide\0KP_Down\0KP_End\0KP_Enter\0KP_Equal\0KP_F1\0KP_F2\0KP_F3\0KP_F4\0KP_Home\0KP_Insert\0KP_Left\0KP_Multiply\0KP_Next\0KP_Page_Down\0KP_Page_Up\0KP_Prior\0KP_Right\0KP_Separator\0KP_Space\0KP_Subtract\0KP_Tab\0KP_Up\0kra\0L\0l\0L1\0L10\0L2\0L3\0L4\0L5\0L6\0L7\0L8\0L9\0Lacute\0lacute\0Last_Virtual_Screen\0latincross\0Lbelowdot\0lbelowdot\0Lcaron\0lcaron\0Lcedilla\0lcedilla\0Left\0leftanglebracket\0leftarrow\0leftcaret\0leftdoublequotemark\0leftmiddlecurlybrace\0leftopentriangle\0leftpointer\0leftradical\0leftshoe\0leftsingleanglequotemark\0leftsinglequotemark\0leftt\0lefttack\0less\0lessthanequal\0lf\0Linefeed\0lira\0LiraSign\0logicaland\0logicalor\0longminus\0lowleftcorner\0lowrightcorner\0Lstroke\0lstroke\0M\0m\0Mabovedot\0mabovedot\0Macedonia_DSE\0Macedonia_dse\0Macedonia_GJE\0Macedonia_gje\0Macedonia_KJE\0Macedonia_kje\0macron\0Mae_Koho\0malesymbol\0maltesecross\0marker\0masculine\0Massyo\0Menu\0Meta_L\0Meta_R\0MillSign\0minus\0minutes\0Mode_switch\0MouseKeys_Accel_Enable\0MouseKeys_Enable\0mu\0Muhenkan\0Multi_key\0MultipleCandidate\0multiply\0musicalflat\0musicalsharp\0mute_acute\0mute_asciicircum\0mute_asciitilde\0mute_diaeresis\0mute_grave\0N\0n\0nabla\0Nacute\0nacute\0NairaSign\0Ncaron\0ncaron\0Ncedilla\0ncedilla\0NewSheqelSign\0Next\0Next_Virtual_Screen\0ninesubscript\0ninesuperior\0nl\0nobreakspace\0NoSymbol\0notapproxeq\0notelementof\0notequal\0notidentical\0notsign\0Ntilde\0ntilde\0Num_Lock\0numbersign\0numerosign\0O\0o\0Oacute\0oacute\0Obarred\0obarred\0Obelowdot\0obelowdot\0Ocaron\0ocaron\0Ocircumflex\0ocircumflex\0Ocircumflexacute\0ocircumflexacute\0Ocircumflexbelowdot\0ocircumflexbelowdot\0Ocircumflexgrave\0ocircumflexgrave\0Ocircumflexhook\0ocircumflexhook\0Ocircumflextilde\0ocircumflextilde\0Odiaeresis\0odiaeresis\0Odoubleacute\0odoubleacute\0OE\0oe\0ogonek\0Ograve\0ograve\0Ohook\0ohook\0Ohorn\0ohorn\0Ohornacute\0ohornacute\0Ohornbelowdot\0ohornbelowdot\0Ohorngrave\0ohorngrave\0Ohornhook\0ohornhook\0Ohorntilde\0ohorntilde\0Omacron\0omacron\0oneeighth\0onefifth\0onehalf\0onequarter\0onesixth\0onesubscript\0onesuperior\0onethird\0Ooblique\0ooblique\0openrectbullet\0openstar\0opentribulletdown\0opentribulletup\0ordfeminine\0ordmasculine\0osfActivate\0osfAddMode\0osfBackSpace\0osfBackTab\0osfBeginData\0osfBeginLine\0osfCancel\0osfClear\0osfCopy\0osfCut\0osfDelete\0osfDeselectAll\0osfDown\0osfEndData\0osfEndLine\0osfEscape\0osfExtend\0osfHelp\0osfInsert\0osfLeft\0osfMenu\0osfMenuBar\0osfNextField\0osfNextMenu\0osfPageDown\0osfPageLeft\0osfPageRight\0osfPageUp\0osfPaste\0osfPrevField\0osfPrevMenu\0osfPrimaryPaste\0osfQuickPaste\0osfReselect\0osfRestore\0osfRight\0osfSelect\0osfSelectAll\0osfUndo\0osfUp\0Oslash\0oslash\0Otilde\0otilde\0overbar\0Overlay1_Enable\0Overlay2_Enable\0overline\0P\0p\0Pabovedot\0pabovedot\0Page_Down\0Page_Up\0paragraph\0parenleft\0parenright\0partdifferential\0partialderivative\0Pause\0percent\0period\0periodcentered\0permille\0PesetaSign\0phonographcopyright\0plus\0plusminus\0Pointer_Accelerate\0Pointer_Button1\0Pointer_Button2\0Pointer_Button3\0Pointer_Button4\0Pointer_Button5\0Pointer_Button_Dflt\0Pointer_DblClick1\0Pointer_DblClick2\0Pointer_DblClick3\0Pointer_DblClick4\0Pointer_DblClick5\0Pointer_DblClick_Dflt\0Pointer_DfltBtnNext\0Pointer_DfltBtnPrev\0Pointer_Down\0Pointer_DownLeft\0Pointer_DownRight\0Pointer_Drag1\0Pointer_Drag2\0Pointer_Drag3\0Pointer_Drag4\0Pointer_Drag5\0Pointer_Drag_Dflt\0Pointer_EnableKeys\0Pointer_Left\0Pointer_Right\0Pointer_Up\0Pointer_UpLeft\0Pointer_UpRight\0prescription\0Prev_Virtual_Screen\0PreviousCandidate\0Print\0Prior\0prolongedsound\0punctspace\0Q\0q\0quad\0question\0questiondown\0quotedbl\0quoteleft\0quoteright\0R\0r\0R1\0R10\0R11\0R12\0R13\0R14\0R15\0R2\0R3\0R4\0R5\0R6\0R7\0R8\0R9\0Racute\0racute\0radical\0Rcaron\0rcaron\0Rcedilla\0rcedilla\0Redo\0registered\0RepeatKeys_Enable\0Reset\0Return\0Right\0rightanglebracket\0rightarrow\0rightcaret\0rightdoublequotemark\0rightmiddlecurlybrace\0rightmiddlesummation\0rightopentriangle\0rightpointer\0rightshoe\0rightsingleanglequotemark\0rightsinglequotemark\0rightt\0righttack\0Romaji\0RupeeSign\0S\0s\0Sabovedot\0sabovedot\0Sacute\0sacute\0Scaron\0scaron\0Scedilla\0scedilla\0SCHWA\0schwa\0Scircumflex\0scircumflex\0script_switch\0Scroll_Lock\0seconds\0section\0Select\0semicolon\0semivoicedsound\0Serbian_DJE\0Serbian_dje\0Serbian_DZE\0Serbian_dze\0Serbian_JE\0Serbian_je\0Serbian_LJE\0Serbian_lje\0Serbian_NJE\0Serbian_nje\0Serbian_TSHE\0Serbian_tshe\0seveneighths\0sevensubscript\0sevensuperior\0Shift_L\0Shift_Lock\0Shift_R\0signaturemark\0signifblank\0similarequal\0SingleCandidate\0singlelowquotemark\0Sinh_a\0Sinh_aa\0Sinh_aa2\0Sinh_ae\0Sinh_ae2\0Sinh_aee\0Sinh_aee2\0Sinh_ai\0Sinh_ai2\0Sinh_al\0Sinh_au\0Sinh_au2\0Sinh_ba\0Sinh_bha\0Sinh_ca\0Sinh_cha\0Sinh_dda\0Sinh_ddha\0Sinh_dha\0Sinh_dhha\0Sinh_e\0Sinh_e2\0Sinh_ee\0Sinh_ee2\0Sinh_fa\0Sinh_ga\0Sinh_gha\0Sinh_h2\0Sinh_ha\0Sinh_i\0Sinh_i2\0Sinh_ii\0Sinh_ii2\0Sinh_ja\0Sinh_jha\0Sinh_jnya\0Sinh_ka\0Sinh_kha\0Sinh_kunddaliya\0Sinh_la\0Sinh_lla\0Sinh_lu\0Sinh_lu2\0Sinh_luu\0Sinh_luu2\0Sinh_ma\0Sinh_mba\0Sinh_na\0Sinh_ndda\0Sinh_ndha\0Sinh_ng\0Sinh_ng2\0Sinh_nga\0Sinh_nja\0Sinh_nna\0Sinh_nya\0Sinh_o\0Sinh_o2\0Sinh_oo\0Sinh_oo2\0Sinh_pa\0Sinh_pha\0Sinh_ra\0Sinh_ri\0Sinh_rii\0Sinh_ru2\0Sinh_ruu2\0Sinh_sa\0Sinh_sha\0Sinh_ssha\0Sinh_tha\0Sinh_thha\0Sinh_tta\0Sinh_ttha\0Sinh_u\0Sinh_u2\0Sinh_uu\0Sinh_uu2\0Sinh_va\0Sinh_ya\0sixsubscript\0sixsuperior\0slash\0SlowKeys_Enable\0soliddiamond\0space\0squareroot\0SSHARP\0ssharp\0sterling\0StickyKeys_Enable\0stricteq\0SunAgain\0SunAltGraph\0SunAudioLowerVolume\0SunAudioMute\0SunAudioRaiseVolume\0SunCompose\0SunCopy\0SunCut\0SunF36\0SunF37\0SunFA_Acute\0SunFA_Cedilla\0SunFA_Circum\0SunFA_Diaeresis\0SunFA_Grave\0SunFA_Tilde\0SunFind\0SunFront\0SunOpen\0SunPageDown\0SunPageUp\0SunPaste\0SunPowerSwitch\0SunPowerSwitchShift\0SunPrint_Screen\0SunProps\0SunStop\0SunSys_Req\0SunUndo\0SunVideoDegauss\0SunVideoLowerBrightness\0SunVideoRaiseBrightness\0Super_L\0Super_R\0Sys_Req\0System\0T\0t\0Tab\0Tabovedot\0tabovedot\0Tcaron\0tcaron\0Tcedilla\0tcedilla\0telephone\0telephonerecorder\0Terminate_Server\0Thai_baht\0Thai_bobaimai\0Thai_chochan\0Thai_chochang\0Thai_choching\0Thai_chochoe\0Thai_dochada\0Thai_dodek\0Thai_fofa\0Thai_fofan\0Thai_hohip\0Thai_honokhuk\0Thai_khokhai\0Thai_khokhon\0Thai_khokhuat\0Thai_khokhwai\0Thai_khorakhang\0Thai_kokai\0Thai_lakkhangyao\0Thai_lekchet\0Thai_lekha\0Thai_lekhok\0Thai_lekkao\0Thai_leknung\0Thai_lekpaet\0Thai_leksam\0Thai_leksi\0Thai_leksong\0Thai_leksun\0Thai_lochula\0Thai_loling\0Thai_lu\0Thai_maichattawa\0Thai_maiek\0Thai_maihanakat\0Thai_maihanakat_maitho\0Thai_maitaikhu\0Thai_maitho\0Thai_maitri\0Thai_maiyamok\0Thai_moma\0Thai_ngongu\0Thai_nikhahit\0Thai_nonen\0Thai_nonu\0Thai_oang\0Thai_paiyannoi\0Thai_phinthu\0Thai_phophan\0Thai_phophung\0Thai_phosamphao\0Thai_popla\0Thai_rorua\0Thai_ru\0Thai_saraa\0Thai_saraaa\0Thai_saraae\0Thai_saraaimaimalai\0Thai_saraaimaimuan\0Thai_saraam\0Thai_sarae\0Thai_sarai\0Thai_saraii\0Thai_sarao\0Thai_sarau\0Thai_saraue\0Thai_sarauee\0Thai_sarauu\0Thai_sorusi\0Thai_sosala\0Thai_soso\0Thai_sosua\0Thai_thanthakhat\0Thai_thonangmontho\0Thai_thophuthao\0Thai_thothahan\0Thai_thothan\0Thai_thothong\0Thai_thothung\0Thai_topatak\0Thai_totao\0Thai_wowaen\0Thai_yoyak\0Thai_yoying\0therefore\0thinspace\0THORN\0Thorn\0thorn\0threeeighths\0threefifths\0threequarters\0threesubscript\0threesuperior\0tintegral\0topintegral\0topleftparens\0topleftradical\0topleftsqbracket\0topleftsummation\0toprightparens\0toprightsqbracket\0toprightsummation\0topt\0topvertsummationconnector\0Touroku\0trademark\0trademarkincircle\0Tslash\0tslash\0twofifths\0twosubscript\0twosuperior\0twothirds\0U\0u\0Uacute\0uacute\0Ubelowdot\0ubelowdot\0Ubreve\0ubreve\0Ucircumflex\0ucircumflex\0Udiaeresis\0udiaeresis\0Udoubleacute\0udoubleacute\0Ugrave\0ugrave\0Uhook\0uhook\0Uhorn\0uhorn\0Uhornacute\0uhornacute\0Uhornbelowdot\0uhornbelowdot\0Uhorngrave\0uhorngrave\0Uhornhook\0uhornhook\0Uhorntilde\0uhorntilde\0Ukrainian_GHE_WITH_UPTURN\0Ukrainian_ghe_with_upturn\0Ukrainian_I\0Ukrainian_i\0Ukrainian_IE\0Ukrainian_ie\0Ukrainian_YI\0Ukrainian_yi\0Ukranian_I\0Ukranian_i\0Ukranian_JE\0Ukranian_je\0Ukranian_YI\0Ukranian_yi\0Umacron\0umacron\0underbar\0underscore\0Undo\0union\0Uogonek\0uogonek\0Up\0uparrow\0upcaret\0upleftcorner\0uprightcorner\0upshoe\0upstile\0uptack\0Uring\0uring\0User\0Utilde\0utilde\0V\0v\0variation\0vertbar\0vertconnector\0voicedsound\0VoidSymbol\0vt\0W\0w\0Wacute\0wacute\0Wcircumflex\0wcircumflex\0Wdiaeresis\0wdiaeresis\0Wgrave\0wgrave\0WonSign\0X\0x\0Xabovedot\0xabovedot\0XF8610ChannelsDown\0XF8610ChannelsUp\0XF863DMode\0XF86Accessibility\0XF86AddFavorite\0XF86Addressbook\0XF86ALSToggle\0XF86ApplicationLeft\0XF86ApplicationRight\0XF86AppSelect\0XF86AspectRatio\0XF86Assistant\0XF86AttendantOff\0XF86AttendantOn\0XF86AttendantToggle\0XF86Audio\0XF86AudioBassBoost\0XF86AudioChannelMode\0XF86AudioCycleTrack\0XF86AudioDesc\0XF86AudioForward\0XF86AudioLowerVolume\0XF86AudioMedia\0XF86AudioMicMute\0XF86AudioMute\0XF86AudioNext\0XF86AudioPause\0XF86AudioPlay\0XF86AudioPreset\0XF86AudioPrev\0XF86AudioRaiseVolume\0XF86AudioRandomPlay\0XF86AudioRecord\0XF86AudioRepeat\0XF86AudioRewind\0XF86AudioStop\0XF86AutopilotEngageToggle\0XF86Away\0XF86Back\0XF86BackForward\0XF86Battery\0XF86Blue\0XF86Bluetooth\0XF86Book\0XF86Break\0XF86BrightnessAdjust\0XF86BrightnessAuto\0XF86BrightnessMax\0XF86BrightnessMin\0XF86Buttonconfig\0XF86Calculater\0XF86Calculator\0XF86Calendar\0XF86CameraAccessDisable\0XF86CameraAccessEnable\0XF86CameraAccessToggle\0XF86CameraDown\0XF86CameraFocus\0XF86CameraLeft\0XF86CameraRight\0XF86CameraUp\0XF86CameraZoomIn\0XF86CameraZoomOut\0XF86CD\0XF86ChannelDown\0XF86ChannelUp\0XF86Clear\0XF86ClearGrab\0XF86ClearvuSonar\0XF86Close\0XF86Community\0XF86ContextMenu\0XF86ContrastAdjust\0XF86ControlPanel\0XF86Copy\0XF86Cut\0XF86CycleAngle\0XF86Data\0XF86Database\0XF86Dictate\0XF86Display\0XF86DisplayOff\0XF86DisplayToggle\0XF86Documents\0XF86DoNotDisturb\0XF86DOS\0XF86DualRangeRadar\0XF86DVD\0XF86Editor\0XF86Eject\0XF86ElectronicPrivacyScreenOff\0XF86ElectronicPrivacyScreenOn\0XF86EmojiPicker\0XF86Excel\0XF86Exit\0XF86Explorer\0XF86FastReverse\0XF86Favorites\0XF86Finance\0XF86FishingChart\0XF86Fn\0XF86Fn_1\0XF86Fn_2\0XF86Fn_B\0XF86Fn_D\0XF86Fn_E\0XF86Fn_Esc\0XF86Fn_F\0XF86Fn_F1\0XF86Fn_F10\0XF86Fn_F11\0XF86Fn_F12\0XF86Fn_F2\0XF86Fn_F3\0XF86Fn_F4\0XF86Fn_F5\0XF86Fn_F6\0XF86Fn_F7\0XF86Fn_F8\0XF86Fn_F9\0XF86Fn_S\0XF86FnRightShift\0XF86Forward\0XF86FrameBack\0XF86FrameForward\0XF86FullScreen\0XF86Game\0XF86Go\0XF86GoTo\0XF86GraphicsEditor\0XF86Green\0XF86HangupPhone\0XF86Hibernate\0XF86History\0XF86HomePage\0XF86HotLinks\0XF86Images\0XF86Info\0XF86iTouch\0XF86Journal\0XF86KbdBrightnessDown\0XF86KbdBrightnessUp\0XF86KbdInputAssistAccept\0XF86KbdInputAssistCancel\0XF86KbdInputAssistNext\0XF86KbdInputAssistNextgroup\0XF86KbdInputAssistPrev\0XF86KbdInputAssistPrevgroup\0XF86KbdLcdMenu1\0XF86KbdLcdMenu2\0XF86KbdLcdMenu3\0XF86KbdLcdMenu4\0XF86KbdLcdMenu5\0XF86KbdLightOnOff\0XF86Keyboard\0XF86Launch0\0XF86Launch1\0XF86Launch2\0XF86Launch3\0XF86Launch4\0XF86Launch5\0XF86Launch6\0XF86Launch7\0XF86Launch8\0XF86Launch9\0XF86LaunchA\0XF86LaunchB\0XF86LaunchC\0XF86LaunchD\0XF86LaunchE\0XF86LaunchF\0XF86LeftDown\0XF86LeftUp\0XF86LightBulb\0XF86LightsToggle\0XF86LinkPhone\0XF86LogGrabInfo\0XF86LogOff\0XF86LogWindowTree\0XF86Macro1\0XF86Macro10\0XF86Macro11\0XF86Macro12\0XF86Macro13\0XF86Macro14\0XF86Macro15\0XF86Macro16\0XF86Macro17\0XF86Macro18\0XF86Macro19\0XF86Macro2\0XF86Macro20\0XF86Macro21\0XF86Macro22\0XF86Macro23\0XF86Macro24\0XF86Macro25\0XF86Macro26\0XF86Macro27\0XF86Macro28\0XF86Macro29\0XF86Macro3\0XF86Macro30\0XF86Macro4\0XF86Macro5\0XF86Macro6\0XF86Macro7\0XF86Macro8\0XF86Macro9\0XF86MacroPreset1\0XF86MacroPreset2\0XF86MacroPreset3\0XF86MacroPresetCycle\0XF86MacroRecordStart\0XF86MacroRecordStop\0XF86Mail\0XF86MailForward\0XF86Market\0XF86MarkWaypoint\0XF86MediaLanguageMenu\0XF86MediaPlayer\0XF86MediaPlayPause\0XF86MediaPlaySlow\0XF86MediaRepeat\0XF86MediaSelectAuxiliary\0XF86MediaSelectCable\0XF86MediaSelectCD\0XF86MediaSelectDVD\0XF86MediaSelectHome\0XF86MediaSelectPC\0XF86MediaSelectProgramGuide\0XF86MediaSelectRadio\0XF86MediaSelectSatellite\0XF86MediaSelectTape\0XF86MediaSelectTeletext\0XF86MediaSelectTuner\0XF86MediaSelectTV\0XF86MediaSelectVCR\0XF86MediaSelectVCRPlus\0XF86MediaTitleMenu\0XF86MediaTopMenu\0XF86Meeting\0XF86Memo\0XF86MenuKB\0XF86MenuPB\0XF86Messenger\0XF86ModeLock\0XF86MonBrightnessAuto\0XF86MonBrightnessCycle\0XF86MonBrightnessDown\0XF86MonBrightnessUp\0XF86Music\0XF86MyComputer\0XF86MySites\0XF86NavChart\0XF86NavInfo\0XF86New\0XF86News\0XF86Next_VMode\0XF86NextElement\0XF86NextFavorite\0XF86NotificationCenter\0XF86NumberEntryMode\0XF86Numeric0\0XF86Numeric1\0XF86Numeric11\0XF86Numeric12\0XF86Numeric2\0XF86Numeric3\0XF86Numeric4\0XF86Numeric5\0XF86Numeric6\0XF86Numeric7\0XF86Numeric8\0XF86Numeric9\0XF86NumericA\0XF86NumericB\0XF86NumericC\0XF86NumericD\0XF86NumericPound\0XF86NumericStar\0XF86OfficeHome\0XF86OK\0XF86OnScreenKeyboard\0XF86Open\0XF86OpenURL\0XF86Option\0XF86Paste\0XF86PauseRecord\0XF86PerformanceMode\0XF86Phone\0XF86PickupPhone\0XF86Pictures\0XF86PowerDown\0XF86PowerOff\0XF86Presentation\0XF86Prev_VMode\0XF86PreviousElement\0XF86PrivacyScreenToggle\0XF86Q\0XF86RadarOverlay\0XF86Red\0XF86Refresh\0XF86RefreshRateToggle\0XF86Reload\0XF86Reply\0XF86RFKill\0XF86RightDown\0XF86RightUp\0XF86RockerDown\0XF86RockerEnter\0XF86RockerUp\0XF86RootMenu\0XF86RotateWindows\0XF86RotationKB\0XF86RotationLockToggle\0XF86RotationPB\0XF86Save\0XF86ScreenSaver\0XF86Screensaver\0XF86ScrollClick\0XF86ScrollDown\0XF86ScrollUp\0XF86Search\0XF86Select\0XF86SelectiveScreenshot\0XF86Send\0XF86Shop\0XF86SidevuSonar\0XF86SingleRangeRadar\0XF86Sleep\0XF86SlowReverse\0XF86Sos\0XF86Spell\0XF86SpellCheck\0XF86SplitScreen\0XF86Sport\0XF86Standby\0XF86Start\0XF86Stop\0XF86StopRecord\0XF86Subtitle\0XF86Support\0XF86Suspend\0XF86Switch_VT_1\0XF86Switch_VT_10\0XF86Switch_VT_11\0XF86Switch_VT_12\0XF86Switch_VT_2\0XF86Switch_VT_3\0XF86Switch_VT_4\0XF86Switch_VT_5\0XF86Switch_VT_6\0XF86Switch_VT_7\0XF86Switch_VT_8\0XF86Switch_VT_9\0XF86Taskmanager\0XF86TaskPane\0XF86Terminal\0XF86Time\0XF86ToDoList\0XF86Tools\0XF86TopMenu\0XF86TouchpadOff\0XF86TouchpadOn\0XF86TouchpadToggle\0XF86TraditionalSonar\0XF86Travel\0XF86Ungrab\0XF86Unmute\0XF86User1KB\0XF86User2KB\0XF86UserPB\0XF86UWB\0XF86VendorHome\0XF86VendorLogo\0XF86Video\0XF86VideoPhone\0XF86View\0XF86VOD\0XF86VoiceCommand\0XF86Voicemail\0XF86WakeUp\0XF86WebCam\0XF86WheelButton\0XF86WLAN\0XF86Word\0XF86WPSButton\0XF86WWAN\0XF86WWW\0XF86Xfer\0XF86Yellow\0XF86ZoomIn\0XF86ZoomOut\0XF86ZoomReset\0Y\0y\0Yacute\0yacute\0Ybelowdot\0ybelowdot\0Ycircumflex\0ycircumflex\0Ydiaeresis\0ydiaeresis\0yen\0Ygrave\0ygrave\0Yhook\0yhook\0Ytilde\0ytilde\0Z\0z\0Zabovedot\0zabovedot\0Zacute\0zacute\0Zcaron\0zcaron\0Zen_Koho\0Zenkaku\0Zenkaku_Hankaku\0zerosubscript\0zerosuperior\0Zstroke\0zstroke\0\0"
        .as_ptr() as *const i8;
    pub static mut keysym_name_G: [uint16_t; 4728] = [
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2700 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        493 as i32 as uint16_t,
        0 as i32 as uint16_t,
        177 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4536 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2918 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3763 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1499 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3907 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3178 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        517 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3994 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1572 as i32 as uint16_t,
        0 as i32 as uint16_t,
        707 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3717 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3803 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2063 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1010 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4131 as i32 as uint16_t,
        0 as i32 as uint16_t,
        221 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1616 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4288 as i32 as uint16_t,
        387 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4135 as i32 as uint16_t,
        713 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3214 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4364 as i32 as uint16_t,
        4099 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3448 as i32 as uint16_t,
        227 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1119 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3832 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        815 as i32 as uint16_t,
        0 as i32 as uint16_t,
        415 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1701 as i32 as uint16_t,
        2344 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3193 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        436 as i32 as uint16_t,
        147 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3646 as i32 as uint16_t,
        2899 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1857 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2348 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        643 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2257 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        871 as i32 as uint16_t,
        2710 as i32 as uint16_t,
        630 as i32 as uint16_t,
        2243 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1106 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1263 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3366 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3111 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1218 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3547 as i32 as uint16_t,
        0 as i32 as uint16_t,
        939 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1112 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2144 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3737 as i32 as uint16_t,
        4308 as i32 as uint16_t,
        3320 as i32 as uint16_t,
        1700 as i32 as uint16_t,
        594 as i32 as uint16_t,
        0 as i32 as uint16_t,
        799 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4057 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1908 as i32 as uint16_t,
        4331 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1361 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1942 as i32 as uint16_t,
        0 as i32 as uint16_t,
        61 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1660 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2422 as i32 as uint16_t,
        1514 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1067 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2968 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1989 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1974 as i32 as uint16_t,
        4592 as i32 as uint16_t,
        3523 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        222 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3722 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3867 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        806 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2233 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4050 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        698 as i32 as uint16_t,
        2784 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1092 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2819 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2433 as i32 as uint16_t,
        4664 as i32 as uint16_t,
        2245 as i32 as uint16_t,
        901 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2816 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4693 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4125 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2133 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1956 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1616 as i32 as uint16_t,
        1716 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        52 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        7 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4358 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4043 as i32 as uint16_t,
        2366 as i32 as uint16_t,
        1320 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2050 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1781 as i32 as uint16_t,
        3818 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4019 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2746 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2980 as i32 as uint16_t,
        200 as i32 as uint16_t,
        3598 as i32 as uint16_t,
        1551 as i32 as uint16_t,
        4176 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4318 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2592 as i32 as uint16_t,
        0 as i32 as uint16_t,
        42 as i32 as uint16_t,
        4429 as i32 as uint16_t,
        3736 as i32 as uint16_t,
        2741 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3567 as i32 as uint16_t,
        2557 as i32 as uint16_t,
        1537 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        717 as i32 as uint16_t,
        1274 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        818 as i32 as uint16_t,
        2129 as i32 as uint16_t,
        648 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        888 as i32 as uint16_t,
        510 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        208 as i32 as uint16_t,
        1522 as i32 as uint16_t,
        3794 as i32 as uint16_t,
        1374 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1550 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2843 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3657 as i32 as uint16_t,
        1248 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4497 as i32 as uint16_t,
        4142 as i32 as uint16_t,
        52 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4490 as i32 as uint16_t,
        4517 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4136 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1278 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1540 as i32 as uint16_t,
        2216 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4297 as i32 as uint16_t,
        844 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2326 as i32 as uint16_t,
        2293 as i32 as uint16_t,
        2797 as i32 as uint16_t,
        342 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3796 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3135 as i32 as uint16_t,
        0 as i32 as uint16_t,
        957 as i32 as uint16_t,
        540 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2849 as i32 as uint16_t,
        0 as i32 as uint16_t,
        854 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1747 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        178 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4641 as i32 as uint16_t,
        509 as i32 as uint16_t,
        4462 as i32 as uint16_t,
        1693 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4526 as i32 as uint16_t,
        1062 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        576 as i32 as uint16_t,
        2654 as i32 as uint16_t,
        44 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2527 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2197 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4376 as i32 as uint16_t,
        3290 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4662 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2176 as i32 as uint16_t,
        1845 as i32 as uint16_t,
        1647 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3717 as i32 as uint16_t,
        0 as i32 as uint16_t,
        404 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        997 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3022 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1965 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4691 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1568 as i32 as uint16_t,
        2224 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1731 as i32 as uint16_t,
        850 as i32 as uint16_t,
        2880 as i32 as uint16_t,
        4405 as i32 as uint16_t,
        1383 as i32 as uint16_t,
        0 as i32 as uint16_t,
        174 as i32 as uint16_t,
        2562 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2239 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1863 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4560 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1714 as i32 as uint16_t,
        3211 as i32 as uint16_t,
        0 as i32 as uint16_t,
        880 as i32 as uint16_t,
        1873 as i32 as uint16_t,
        654 as i32 as uint16_t,
        190 as i32 as uint16_t,
        91 as i32 as uint16_t,
        0 as i32 as uint16_t,
        897 as i32 as uint16_t,
        4701 as i32 as uint16_t,
        2704 as i32 as uint16_t,
        1806 as i32 as uint16_t,
        0 as i32 as uint16_t,
        623 as i32 as uint16_t,
        0 as i32 as uint16_t,
        989 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4349 as i32 as uint16_t,
        3114 as i32 as uint16_t,
        3887 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2769 as i32 as uint16_t,
        0 as i32 as uint16_t,
        448 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1191 as i32 as uint16_t,
        1865 as i32 as uint16_t,
        3089 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1567 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4350 as i32 as uint16_t,
        144 as i32 as uint16_t,
        3244 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3023 as i32 as uint16_t,
        3625 as i32 as uint16_t,
        1802 as i32 as uint16_t,
        3930 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3441 as i32 as uint16_t,
        1756 as i32 as uint16_t,
        435 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2437 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        587 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        241 as i32 as uint16_t,
        462 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2053 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3567 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3830 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4161 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2009 as i32 as uint16_t,
        1812 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2749 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2260 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1764 as i32 as uint16_t,
        0 as i32 as uint16_t,
        583 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2260 as i32 as uint16_t,
        1711 as i32 as uint16_t,
        1269 as i32 as uint16_t,
        826 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        928 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2574 as i32 as uint16_t,
        4161 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3829 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1767 as i32 as uint16_t,
        3178 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2579 as i32 as uint16_t,
        2014 as i32 as uint16_t,
        1398 as i32 as uint16_t,
        2734 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1366 as i32 as uint16_t,
        2076 as i32 as uint16_t,
        3799 as i32 as uint16_t,
        1830 as i32 as uint16_t,
        1719 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4658 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2048 as i32 as uint16_t,
        1840 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1296 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2315 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1427 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2382 as i32 as uint16_t,
        337 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4236 as i32 as uint16_t,
        4678 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4629 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4722 as i32 as uint16_t,
        3215 as i32 as uint16_t,
        673 as i32 as uint16_t,
        2242 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1918 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        588 as i32 as uint16_t,
        4379 as i32 as uint16_t,
        2428 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4553 as i32 as uint16_t,
        2933 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        836 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1226 as i32 as uint16_t,
        854 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3645 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2769 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3258 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        824 as i32 as uint16_t,
        42 as i32 as uint16_t,
        906 as i32 as uint16_t,
        1456 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3681 as i32 as uint16_t,
        3990 as i32 as uint16_t,
        944 as i32 as uint16_t,
        441 as i32 as uint16_t,
        1891 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4012 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3325 as i32 as uint16_t,
        1185 as i32 as uint16_t,
        1823 as i32 as uint16_t,
        387 as i32 as uint16_t,
        2357 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        176 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4059 as i32 as uint16_t,
        4099 as i32 as uint16_t,
        940 as i32 as uint16_t,
        0 as i32 as uint16_t,
        449 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1436 as i32 as uint16_t,
        2567 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1800 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        562 as i32 as uint16_t,
        1692 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4417 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3954 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3290 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3651 as i32 as uint16_t,
        1591 as i32 as uint16_t,
        3235 as i32 as uint16_t,
        3517 as i32 as uint16_t,
        1340 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        714 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1657 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1997 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2466 as i32 as uint16_t,
        1852 as i32 as uint16_t,
        2623 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2171 as i32 as uint16_t,
        0 as i32 as uint16_t,
        717 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3173 as i32 as uint16_t,
        2358 as i32 as uint16_t,
        864 as i32 as uint16_t,
        1525 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3822 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2009 as i32 as uint16_t,
        0 as i32 as uint16_t,
        371 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2030 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3795 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3034 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2199 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4490 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3342 as i32 as uint16_t,
        2098 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2235 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        579 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        295 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1341 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4648 as i32 as uint16_t,
        1233 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1425 as i32 as uint16_t,
        4363 as i32 as uint16_t,
        2928 as i32 as uint16_t,
        4149 as i32 as uint16_t,
        4050 as i32 as uint16_t,
        232 as i32 as uint16_t,
        3137 as i32 as uint16_t,
        501 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2762 as i32 as uint16_t,
        1217 as i32 as uint16_t,
        3238 as i32 as uint16_t,
        3318 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3825 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        392 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4376 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3720 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        875 as i32 as uint16_t,
        4366 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        47 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2137 as i32 as uint16_t,
        887 as i32 as uint16_t,
        4030 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        4341 as i32 as uint16_t,
        1333 as i32 as uint16_t,
        58 as i32 as uint16_t,
        1091 as i32 as uint16_t,
        4534 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4413 as i32 as uint16_t,
        1245 as i32 as uint16_t,
        904 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1319 as i32 as uint16_t,
        1088 as i32 as uint16_t,
        2230 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2559 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2965 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3306 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4024 as i32 as uint16_t,
        4422 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4396 as i32 as uint16_t,
        245 as i32 as uint16_t,
        1609 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2753 as i32 as uint16_t,
        945 as i32 as uint16_t,
        191 as i32 as uint16_t,
        575 as i32 as uint16_t,
        3515 as i32 as uint16_t,
        861 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3355 as i32 as uint16_t,
        901 as i32 as uint16_t,
        493 as i32 as uint16_t,
        4400 as i32 as uint16_t,
        3129 as i32 as uint16_t,
        4577 as i32 as uint16_t,
        1707 as i32 as uint16_t,
        77 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2460 as i32 as uint16_t,
        1539 as i32 as uint16_t,
        3399 as i32 as uint16_t,
        1834 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3033 as i32 as uint16_t,
        0 as i32 as uint16_t,
        905 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2185 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2231 as i32 as uint16_t,
        898 as i32 as uint16_t,
        328 as i32 as uint16_t,
        70 as i32 as uint16_t,
        2954 as i32 as uint16_t,
        2800 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2305 as i32 as uint16_t,
        2822 as i32 as uint16_t,
        1892 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2087 as i32 as uint16_t,
        2249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        220 as i32 as uint16_t,
        4504 as i32 as uint16_t,
        0 as i32 as uint16_t,
        580 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3132 as i32 as uint16_t,
        2321 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3868 as i32 as uint16_t,
        1278 as i32 as uint16_t,
        286 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2514 as i32 as uint16_t,
        1870 as i32 as uint16_t,
        3984 as i32 as uint16_t,
        0 as i32 as uint16_t,
        896 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1743 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3849 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3432 as i32 as uint16_t,
        2231 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        906 as i32 as uint16_t,
        2577 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3384 as i32 as uint16_t,
        4259 as i32 as uint16_t,
        0 as i32 as uint16_t,
        797 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2803 as i32 as uint16_t,
        916 as i32 as uint16_t,
        4226 as i32 as uint16_t,
        699 as i32 as uint16_t,
        3817 as i32 as uint16_t,
        4126 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3789 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4176 as i32 as uint16_t,
        416 as i32 as uint16_t,
        1589 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1923 as i32 as uint16_t,
        4313 as i32 as uint16_t,
        3929 as i32 as uint16_t,
        1560 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1229 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2776 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        732 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3179 as i32 as uint16_t,
        897 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1595 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1453 as i32 as uint16_t,
        2206 as i32 as uint16_t,
        318 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2069 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3411 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2304 as i32 as uint16_t,
        0 as i32 as uint16_t,
        950 as i32 as uint16_t,
        2103 as i32 as uint16_t,
        1532 as i32 as uint16_t,
        2671 as i32 as uint16_t,
        4251 as i32 as uint16_t,
        2050 as i32 as uint16_t,
        0 as i32 as uint16_t,
        917 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2648 as i32 as uint16_t,
        2755 as i32 as uint16_t,
        2028 as i32 as uint16_t,
        226 as i32 as uint16_t,
        1588 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1596 as i32 as uint16_t,
        514 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2035 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        793 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1498 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2709 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1008 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4377 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2689 as i32 as uint16_t,
        4556 as i32 as uint16_t,
        3784 as i32 as uint16_t,
        1285 as i32 as uint16_t,
        1632 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        908 as i32 as uint16_t,
        4222 as i32 as uint16_t,
        1764 as i32 as uint16_t,
        0 as i32 as uint16_t,
        519 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        883 as i32 as uint16_t,
        3725 as i32 as uint16_t,
        4430 as i32 as uint16_t,
        1754 as i32 as uint16_t,
        3890 as i32 as uint16_t,
        3872 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2180 as i32 as uint16_t,
        0 as i32 as uint16_t,
        463 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3998 as i32 as uint16_t,
        4604 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3511 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4387 as i32 as uint16_t,
        3512 as i32 as uint16_t,
        248 as i32 as uint16_t,
        31 as i32 as uint16_t,
        4625 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1009 as i32 as uint16_t,
        899 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2577 as i32 as uint16_t,
        4642 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        96 as i32 as uint16_t,
        1663 as i32 as uint16_t,
        1509 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2459 as i32 as uint16_t,
        0 as i32 as uint16_t,
        85 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1920 as i32 as uint16_t,
        303 as i32 as uint16_t,
        0 as i32 as uint16_t,
        709 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3928 as i32 as uint16_t,
        638 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3440 as i32 as uint16_t,
        3219 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1297 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1706 as i32 as uint16_t,
        217 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1471 as i32 as uint16_t,
        4178 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2847 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2857 as i32 as uint16_t,
        2440 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1014 as i32 as uint16_t,
        0 as i32 as uint16_t,
        13 as i32 as uint16_t,
        2835 as i32 as uint16_t,
        0 as i32 as uint16_t,
        593 as i32 as uint16_t,
        0 as i32 as uint16_t,
        64 as i32 as uint16_t,
        1019 as i32 as uint16_t,
        0 as i32 as uint16_t,
        842 as i32 as uint16_t,
        2947 as i32 as uint16_t,
        3902 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3582 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1326 as i32 as uint16_t,
        910 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2903 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2444 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1381 as i32 as uint16_t,
        2556 as i32 as uint16_t,
        4364 as i32 as uint16_t,
        3576 as i32 as uint16_t,
        3939 as i32 as uint16_t,
        920 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3990 as i32 as uint16_t,
        1142 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4414 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2113 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4090 as i32 as uint16_t,
        991 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3608 as i32 as uint16_t,
        1049 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2669 as i32 as uint16_t,
        1854 as i32 as uint16_t,
        3910 as i32 as uint16_t,
        0 as i32 as uint16_t,
        394 as i32 as uint16_t,
        0 as i32 as uint16_t,
        196 as i32 as uint16_t,
        3192 as i32 as uint16_t,
        4038 as i32 as uint16_t,
        2225 as i32 as uint16_t,
        4259 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        857 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3495 as i32 as uint16_t,
        0 as i32 as uint16_t,
        806 as i32 as uint16_t,
        197 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1223 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1807 as i32 as uint16_t,
        3168 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3614 as i32 as uint16_t,
        2477 as i32 as uint16_t,
        581 as i32 as uint16_t,
        1429 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        989 as i32 as uint16_t,
        2565 as i32 as uint16_t,
        3705 as i32 as uint16_t,
        1273 as i32 as uint16_t,
        1891 as i32 as uint16_t,
        721 as i32 as uint16_t,
        3885 as i32 as uint16_t,
        139 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3525 as i32 as uint16_t,
        354 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2302 as i32 as uint16_t,
        3242 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4326 as i32 as uint16_t,
        0 as i32 as uint16_t,
        607 as i32 as uint16_t,
        539 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2140 as i32 as uint16_t,
        4632 as i32 as uint16_t,
        3858 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2502 as i32 as uint16_t,
        1692 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2511 as i32 as uint16_t,
        680 as i32 as uint16_t,
        172 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3316 as i32 as uint16_t,
        1228 as i32 as uint16_t,
        4480 as i32 as uint16_t,
        1126 as i32 as uint16_t,
        3542 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2126 as i32 as uint16_t,
        3884 as i32 as uint16_t,
        1730 as i32 as uint16_t,
        1366 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2597 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        940 as i32 as uint16_t,
        55 as i32 as uint16_t,
        3928 as i32 as uint16_t,
        311 as i32 as uint16_t,
        2323 as i32 as uint16_t,
        2334 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1556 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2213 as i32 as uint16_t,
        3691 as i32 as uint16_t,
        1780 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2894 as i32 as uint16_t,
        2037 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4286 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2123 as i32 as uint16_t,
        1269 as i32 as uint16_t,
        886 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1282 as i32 as uint16_t,
        0 as i32 as uint16_t,
        480 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1245 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3407 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1223 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1078 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        442 as i32 as uint16_t,
        2479 as i32 as uint16_t,
        3294 as i32 as uint16_t,
        1338 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        338 as i32 as uint16_t,
        0 as i32 as uint16_t,
        739 as i32 as uint16_t,
        461 as i32 as uint16_t,
        3503 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2573 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3439 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4083 as i32 as uint16_t,
        110 as i32 as uint16_t,
        19 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1014 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3885 as i32 as uint16_t,
        187 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3213 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3954 as i32 as uint16_t,
        1520 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2072 as i32 as uint16_t,
        4384 as i32 as uint16_t,
        287 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1840 as i32 as uint16_t,
        807 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2469 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2178 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1683 as i32 as uint16_t,
        1175 as i32 as uint16_t,
        2249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2480 as i32 as uint16_t,
        3441 as i32 as uint16_t,
        2283 as i32 as uint16_t,
        0 as i32 as uint16_t,
        522 as i32 as uint16_t,
        3654 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2717 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1202 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3428 as i32 as uint16_t,
        748 as i32 as uint16_t,
        4552 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4612 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2499 as i32 as uint16_t,
        1034 as i32 as uint16_t,
        4667 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1005 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3225 as i32 as uint16_t,
        876 as i32 as uint16_t,
        4140 as i32 as uint16_t,
        4429 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3456 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        809 as i32 as uint16_t,
        0 as i32 as uint16_t,
        307 as i32 as uint16_t,
        2324 as i32 as uint16_t,
        4558 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        780 as i32 as uint16_t,
        1910 as i32 as uint16_t,
        1392 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        913 as i32 as uint16_t,
        375 as i32 as uint16_t,
        2343 as i32 as uint16_t,
        2481 as i32 as uint16_t,
        1661 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        280 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4516 as i32 as uint16_t,
        812 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4640 as i32 as uint16_t,
        1163 as i32 as uint16_t,
        2988 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1009 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3745 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2542 as i32 as uint16_t,
        818 as i32 as uint16_t,
        492 as i32 as uint16_t,
        2909 as i32 as uint16_t,
        214 as i32 as uint16_t,
        1052 as i32 as uint16_t,
        1626 as i32 as uint16_t,
        2580 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1867 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2480 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4147 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3 as i32 as uint16_t,
        2372 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1098 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2786 as i32 as uint16_t,
        1447 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1607 as i32 as uint16_t,
        2251 as i32 as uint16_t,
        3102 as i32 as uint16_t,
        4011 as i32 as uint16_t,
        1420 as i32 as uint16_t,
        41 as i32 as uint16_t,
        605 as i32 as uint16_t,
        2129 as i32 as uint16_t,
        3461 as i32 as uint16_t,
        4435 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2807 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1865 as i32 as uint16_t,
        2284 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        983 as i32 as uint16_t,
        1941 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        142 as i32 as uint16_t,
        523 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3981 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4020 as i32 as uint16_t,
        1247 as i32 as uint16_t,
        4109 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1630 as i32 as uint16_t,
        2720 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2093 as i32 as uint16_t,
        1923 as i32 as uint16_t,
        3645 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3088 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3183 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2253 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2422 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4064 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4102 as i32 as uint16_t,
        1454 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1762 as i32 as uint16_t,
        0 as i32 as uint16_t,
        40 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1780 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1310 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3932 as i32 as uint16_t,
        1278 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1096 as i32 as uint16_t,
        240 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2077 as i32 as uint16_t,
        2895 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3771 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2219 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1874 as i32 as uint16_t,
        436 as i32 as uint16_t,
        4139 as i32 as uint16_t,
        310 as i32 as uint16_t,
        2398 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2770 as i32 as uint16_t,
        1364 as i32 as uint16_t,
        2341 as i32 as uint16_t,
        2537 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        407 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3473 as i32 as uint16_t,
        3883 as i32 as uint16_t,
        1766 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2326 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2630 as i32 as uint16_t,
        3338 as i32 as uint16_t,
        2513 as i32 as uint16_t,
        4383 as i32 as uint16_t,
        2538 as i32 as uint16_t,
        2427 as i32 as uint16_t,
        3728 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3814 as i32 as uint16_t,
        759 as i32 as uint16_t,
        2764 as i32 as uint16_t,
        4600 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1890 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3867 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3283 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4646 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3995 as i32 as uint16_t,
        3652 as i32 as uint16_t,
        2620 as i32 as uint16_t,
        1375 as i32 as uint16_t,
        3902 as i32 as uint16_t,
        2595 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2688 as i32 as uint16_t,
        0 as i32 as uint16_t,
        168 as i32 as uint16_t,
        373 as i32 as uint16_t,
        3692 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2363 as i32 as uint16_t,
        140 as i32 as uint16_t,
        1327 as i32 as uint16_t,
        0 as i32 as uint16_t,
        185 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3078 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4609 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        856 as i32 as uint16_t,
        2244 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2138 as i32 as uint16_t,
        0 as i32 as uint16_t,
        415 as i32 as uint16_t,
        384 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1413 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1782 as i32 as uint16_t,
        0 as i32 as uint16_t,
        874 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1261 as i32 as uint16_t,
        672 as i32 as uint16_t,
        0 as i32 as uint16_t,
        27 as i32 as uint16_t,
        4296 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2826 as i32 as uint16_t,
        1668 as i32 as uint16_t,
        0 as i32 as uint16_t,
        927 as i32 as uint16_t,
        1765 as i32 as uint16_t,
        36 as i32 as uint16_t,
        0 as i32 as uint16_t,
        702 as i32 as uint16_t,
        2913 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1418 as i32 as uint16_t,
        3523 as i32 as uint16_t,
        1891 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2319 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4330 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1912 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1800 as i32 as uint16_t,
        4539 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4434 as i32 as uint16_t,
        2600 as i32 as uint16_t,
        2945 as i32 as uint16_t,
        0 as i32 as uint16_t,
        539 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2280 as i32 as uint16_t,
        4576 as i32 as uint16_t,
        1611 as i32 as uint16_t,
        1598 as i32 as uint16_t,
        2148 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2364 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2783 as i32 as uint16_t,
        3103 as i32 as uint16_t,
        0 as i32 as uint16_t,
        911 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2450 as i32 as uint16_t,
        2534 as i32 as uint16_t,
        6 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1889 as i32 as uint16_t,
        1558 as i32 as uint16_t,
        1853 as i32 as uint16_t,
        548 as i32 as uint16_t,
        2787 as i32 as uint16_t,
        4088 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2563 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3277 as i32 as uint16_t,
        381 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4638 as i32 as uint16_t,
        4474 as i32 as uint16_t,
        2135 as i32 as uint16_t,
        994 as i32 as uint16_t,
        0 as i32 as uint16_t,
        120 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        828 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4114 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1089 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3223 as i32 as uint16_t,
        521 as i32 as uint16_t,
        821 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1826 as i32 as uint16_t,
        1419 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3740 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2924 as i32 as uint16_t,
        991 as i32 as uint16_t,
        0 as i32 as uint16_t,
        503 as i32 as uint16_t,
        4165 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3686 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        297 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1491 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4123 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2139 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        108 as i32 as uint16_t,
        77 as i32 as uint16_t,
        2635 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2115 as i32 as uint16_t,
        63 as i32 as uint16_t,
        4688 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2885 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        764 as i32 as uint16_t,
        2507 as i32 as uint16_t,
        3345 as i32 as uint16_t,
        2956 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1736 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3040 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3713 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1668 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1099 as i32 as uint16_t,
        11 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1785 as i32 as uint16_t,
        2042 as i32 as uint16_t,
        2002 as i32 as uint16_t,
        3947 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1983 as i32 as uint16_t,
        1972 as i32 as uint16_t,
        1973 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1322 as i32 as uint16_t,
        511 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3060 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4657 as i32 as uint16_t,
        910 as i32 as uint16_t,
        1150 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4231 as i32 as uint16_t,
        4682 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        219 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1218 as i32 as uint16_t,
        3797 as i32 as uint16_t,
        2883 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1696 as i32 as uint16_t,
        566 as i32 as uint16_t,
        1425 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1962 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1940 as i32 as uint16_t,
        2366 as i32 as uint16_t,
        133 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2923 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1655 as i32 as uint16_t,
        1797 as i32 as uint16_t,
        1802 as i32 as uint16_t,
        1057 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1443 as i32 as uint16_t,
        0 as i32 as uint16_t,
        348 as i32 as uint16_t,
        1688 as i32 as uint16_t,
        2004 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        485 as i32 as uint16_t,
        2030 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3364 as i32 as uint16_t,
        0 as i32 as uint16_t,
        868 as i32 as uint16_t,
        3605 as i32 as uint16_t,
        4341 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        491 as i32 as uint16_t,
        1316 as i32 as uint16_t,
        2072 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4283 as i32 as uint16_t,
        4000 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4379 as i32 as uint16_t,
        1928 as i32 as uint16_t,
        3106 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4558 as i32 as uint16_t,
        118 as i32 as uint16_t,
        130 as i32 as uint16_t,
        4398 as i32 as uint16_t,
        488 as i32 as uint16_t,
        3291 as i32 as uint16_t,
        2944 as i32 as uint16_t,
        1275 as i32 as uint16_t,
        0 as i32 as uint16_t,
        906 as i32 as uint16_t,
        0 as i32 as uint16_t,
        661 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1917 as i32 as uint16_t,
        2855 as i32 as uint16_t,
        2067 as i32 as uint16_t,
        4131 as i32 as uint16_t,
        3001 as i32 as uint16_t,
        27 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1598 as i32 as uint16_t,
        1401 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4567 as i32 as uint16_t,
        1079 as i32 as uint16_t,
        94 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3113 as i32 as uint16_t,
        3045 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1310 as i32 as uint16_t,
        1641 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2367 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        29 as i32 as uint16_t,
        3620 as i32 as uint16_t,
        0 as i32 as uint16_t,
        608 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4281 as i32 as uint16_t,
        113 as i32 as uint16_t,
        2180 as i32 as uint16_t,
        998 as i32 as uint16_t,
        433 as i32 as uint16_t,
        839 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        190 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2418 as i32 as uint16_t,
        2557 as i32 as uint16_t,
        2767 as i32 as uint16_t,
        3079 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2518 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2625 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1030 as i32 as uint16_t,
        3375 as i32 as uint16_t,
        1406 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2484 as i32 as uint16_t,
        4113 as i32 as uint16_t,
        0 as i32 as uint16_t,
        230 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3523 as i32 as uint16_t,
        523 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2516 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3506 as i32 as uint16_t,
        520 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4567 as i32 as uint16_t,
        2736 as i32 as uint16_t,
        4388 as i32 as uint16_t,
        2085 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1322 as i32 as uint16_t,
        2152 as i32 as uint16_t,
        707 as i32 as uint16_t,
        1035 as i32 as uint16_t,
        4476 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3014 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1818 as i32 as uint16_t,
        2863 as i32 as uint16_t,
        4292 as i32 as uint16_t,
        4409 as i32 as uint16_t,
        3406 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2492 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2368 as i32 as uint16_t,
        3568 as i32 as uint16_t,
        1366 as i32 as uint16_t,
        3525 as i32 as uint16_t,
        1853 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4308 as i32 as uint16_t,
        0 as i32 as uint16_t,
        653 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1924 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        565 as i32 as uint16_t,
        1805 as i32 as uint16_t,
        451 as i32 as uint16_t,
        538 as i32 as uint16_t,
        102 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1143 as i32 as uint16_t,
        4618 as i32 as uint16_t,
        2885 as i32 as uint16_t,
        1699 as i32 as uint16_t,
        0 as i32 as uint16_t,
        270 as i32 as uint16_t,
        1353 as i32 as uint16_t,
        788 as i32 as uint16_t,
        3640 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1618 as i32 as uint16_t,
        4606 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        525 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        903 as i32 as uint16_t,
        2081 as i32 as uint16_t,
        1828 as i32 as uint16_t,
        1963 as i32 as uint16_t,
        4618 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2566 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2086 as i32 as uint16_t,
        3544 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4284 as i32 as uint16_t,
        3274 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1621 as i32 as uint16_t,
        0 as i32 as uint16_t,
        313 as i32 as uint16_t,
        536 as i32 as uint16_t,
        4167 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2236 as i32 as uint16_t,
        663 as i32 as uint16_t,
        317 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2486 as i32 as uint16_t,
        1371 as i32 as uint16_t,
        3855 as i32 as uint16_t,
        3209 as i32 as uint16_t,
        4160 as i32 as uint16_t,
        2540 as i32 as uint16_t,
        3172 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2191 as i32 as uint16_t,
        0 as i32 as uint16_t,
        516 as i32 as uint16_t,
        2846 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3187 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4264 as i32 as uint16_t,
        398 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2201 as i32 as uint16_t,
        1408 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4368 as i32 as uint16_t,
        1448 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3811 as i32 as uint16_t,
        2315 as i32 as uint16_t,
        637 as i32 as uint16_t,
        1023 as i32 as uint16_t,
        784 as i32 as uint16_t,
        2113 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        492 as i32 as uint16_t,
        4007 as i32 as uint16_t,
        1237 as i32 as uint16_t,
        2663 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1834 as i32 as uint16_t,
        869 as i32 as uint16_t,
        3212 as i32 as uint16_t,
        339 as i32 as uint16_t,
        1641 as i32 as uint16_t,
        3705 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        4615 as i32 as uint16_t,
        3200 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2543 as i32 as uint16_t,
        0 as i32 as uint16_t,
        128 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2050 as i32 as uint16_t,
        3956 as i32 as uint16_t,
        2978 as i32 as uint16_t,
        1473 as i32 as uint16_t,
        1253 as i32 as uint16_t,
        2303 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2106 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3565 as i32 as uint16_t,
        4646 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1082 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4298 as i32 as uint16_t,
        0 as i32 as uint16_t,
        657 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        236 as i32 as uint16_t,
        2455 as i32 as uint16_t,
        255 as i32 as uint16_t,
        2370 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2377 as i32 as uint16_t,
        1875 as i32 as uint16_t,
        2195 as i32 as uint16_t,
        858 as i32 as uint16_t,
        0 as i32 as uint16_t,
        58 as i32 as uint16_t,
        1557 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2654 as i32 as uint16_t,
        3623 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2073 as i32 as uint16_t,
        2112 as i32 as uint16_t,
        2303 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3134 as i32 as uint16_t,
        0 as i32 as uint16_t,
        157 as i32 as uint16_t,
        1111 as i32 as uint16_t,
        862 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3426 as i32 as uint16_t,
        3668 as i32 as uint16_t,
        923 as i32 as uint16_t,
        0 as i32 as uint16_t,
        696 as i32 as uint16_t,
        4110 as i32 as uint16_t,
        4586 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4210 as i32 as uint16_t,
        1894 as i32 as uint16_t,
        3126 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        524 as i32 as uint16_t,
        1801 as i32 as uint16_t,
        1875 as i32 as uint16_t,
        4637 as i32 as uint16_t,
        3424 as i32 as uint16_t,
        2033 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3119 as i32 as uint16_t,
        3169 as i32 as uint16_t,
        181 as i32 as uint16_t,
        2269 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4172 as i32 as uint16_t,
        3585 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        545 as i32 as uint16_t,
        1821 as i32 as uint16_t,
        385 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1906 as i32 as uint16_t,
        352 as i32 as uint16_t,
        2541 as i32 as uint16_t,
        182 as i32 as uint16_t,
        2585 as i32 as uint16_t,
        4105 as i32 as uint16_t,
        0 as i32 as uint16_t,
        281 as i32 as uint16_t,
        726 as i32 as uint16_t,
        3905 as i32 as uint16_t,
        2257 as i32 as uint16_t,
        1211 as i32 as uint16_t,
        0 as i32 as uint16_t,
        895 as i32 as uint16_t,
        0 as i32 as uint16_t,
        229 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1784 as i32 as uint16_t,
        3867 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2416 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2633 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3616 as i32 as uint16_t,
        558 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3087 as i32 as uint16_t,
        0 as i32 as uint16_t,
        195 as i32 as uint16_t,
        4108 as i32 as uint16_t,
        880 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        548 as i32 as uint16_t,
        363 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1541 as i32 as uint16_t,
        0 as i32 as uint16_t,
        387 as i32 as uint16_t,
        4440 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1026 as i32 as uint16_t,
        4472 as i32 as uint16_t,
        0 as i32 as uint16_t,
        73 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4552 as i32 as uint16_t,
        851 as i32 as uint16_t,
        2810 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4005 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4232 as i32 as uint16_t,
        1953 as i32 as uint16_t,
        4432 as i32 as uint16_t,
        1226 as i32 as uint16_t,
        2079 as i32 as uint16_t,
        0 as i32 as uint16_t,
        522 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1585 as i32 as uint16_t,
        1919 as i32 as uint16_t,
        1772 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        336 as i32 as uint16_t,
        0 as i32 as uint16_t,
        471 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3999 as i32 as uint16_t,
        582 as i32 as uint16_t,
        55 as i32 as uint16_t,
        3313 as i32 as uint16_t,
        4626 as i32 as uint16_t,
        447 as i32 as uint16_t,
        2467 as i32 as uint16_t,
        2864 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2414 as i32 as uint16_t,
        2750 as i32 as uint16_t,
        3824 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        389 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2173 as i32 as uint16_t,
        370 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4708 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3794 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4639 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        88 as i32 as uint16_t,
        4320 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2525 as i32 as uint16_t,
        3086 as i32 as uint16_t,
        4357 as i32 as uint16_t,
        601 as i32 as uint16_t,
        2523 as i32 as uint16_t,
        758 as i32 as uint16_t,
        1698 as i32 as uint16_t,
        3380 as i32 as uint16_t,
        1450 as i32 as uint16_t,
        231 as i32 as uint16_t,
        1648 as i32 as uint16_t,
        3628 as i32 as uint16_t,
        4356 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4273 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1210 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3813 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1990 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4059 as i32 as uint16_t,
        3926 as i32 as uint16_t,
        1570 as i32 as uint16_t,
        1927 as i32 as uint16_t,
        162 as i32 as uint16_t,
        1096 as i32 as uint16_t,
        50 as i32 as uint16_t,
        3637 as i32 as uint16_t,
        1952 as i32 as uint16_t,
        633 as i32 as uint16_t,
        457 as i32 as uint16_t,
        369 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1809 as i32 as uint16_t,
        2297 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        971 as i32 as uint16_t,
        952 as i32 as uint16_t,
        248 as i32 as uint16_t,
        1156 as i32 as uint16_t,
        0 as i32 as uint16_t,
        458 as i32 as uint16_t,
        0 as i32 as uint16_t,
        749 as i32 as uint16_t,
        3498 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        322 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2146 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4669 as i32 as uint16_t,
        1446 as i32 as uint16_t,
        1898 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2666 as i32 as uint16_t,
        910 as i32 as uint16_t,
        1685 as i32 as uint16_t,
        3396 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        149 as i32 as uint16_t,
        1740 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        124 as i32 as uint16_t,
        4214 as i32 as uint16_t,
        2840 as i32 as uint16_t,
        356 as i32 as uint16_t,
        0 as i32 as uint16_t,
        150 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4134 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1549 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1016 as i32 as uint16_t,
        553 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2707 as i32 as uint16_t,
        0 as i32 as uint16_t,
        359 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        832 as i32 as uint16_t,
        0 as i32 as uint16_t,
        810 as i32 as uint16_t,
        1504 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3541 as i32 as uint16_t,
        3515 as i32 as uint16_t,
        4352 as i32 as uint16_t,
        3460 as i32 as uint16_t,
        2363 as i32 as uint16_t,
        3899 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1024 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        582 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1408 as i32 as uint16_t,
        1654 as i32 as uint16_t,
        0 as i32 as uint16_t,
        473 as i32 as uint16_t,
        1109 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1741 as i32 as uint16_t,
        1094 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3705 as i32 as uint16_t,
        3273 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4086 as i32 as uint16_t,
        2553 as i32 as uint16_t,
        4417 as i32 as uint16_t,
        3511 as i32 as uint16_t,
        0 as i32 as uint16_t,
        533 as i32 as uint16_t,
        71 as i32 as uint16_t,
        785 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        36 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4364 as i32 as uint16_t,
        420 as i32 as uint16_t,
        552 as i32 as uint16_t,
        3311 as i32 as uint16_t,
        2425 as i32 as uint16_t,
        449 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        314 as i32 as uint16_t,
        1196 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4714 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3702 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3336 as i32 as uint16_t,
        1742 as i32 as uint16_t,
        347 as i32 as uint16_t,
        4152 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3518 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4608 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1417 as i32 as uint16_t,
        1508 as i32 as uint16_t,
        1777 as i32 as uint16_t,
        1242 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1110 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2166 as i32 as uint16_t,
        2079 as i32 as uint16_t,
        1208 as i32 as uint16_t,
        1047 as i32 as uint16_t,
        3945 as i32 as uint16_t,
        2550 as i32 as uint16_t,
        276 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2316 as i32 as uint16_t,
        142 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        266 as i32 as uint16_t,
        3376 as i32 as uint16_t,
        798 as i32 as uint16_t,
        1465 as i32 as uint16_t,
        1227 as i32 as uint16_t,
        2339 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3671 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1035 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2292 as i32 as uint16_t,
        272 as i32 as uint16_t,
        903 as i32 as uint16_t,
        2441 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1920 as i32 as uint16_t,
        229 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3992 as i32 as uint16_t,
        2326 as i32 as uint16_t,
        3732 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1507 as i32 as uint16_t,
        0 as i32 as uint16_t,
        519 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2132 as i32 as uint16_t,
        983 as i32 as uint16_t,
        2472 as i32 as uint16_t,
        1459 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1962 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3948 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1452 as i32 as uint16_t,
        318 as i32 as uint16_t,
        0 as i32 as uint16_t,
        208 as i32 as uint16_t,
        3787 as i32 as uint16_t,
        1446 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2894 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4333 as i32 as uint16_t,
        89 as i32 as uint16_t,
        1823 as i32 as uint16_t,
        1661 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2563 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        408 as i32 as uint16_t,
        2624 as i32 as uint16_t,
        513 as i32 as uint16_t,
        0 as i32 as uint16_t,
        800 as i32 as uint16_t,
        1036 as i32 as uint16_t,
        2046 as i32 as uint16_t,
        3018 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1630 as i32 as uint16_t,
        1819 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2845 as i32 as uint16_t,
        1360 as i32 as uint16_t,
        3400 as i32 as uint16_t,
        3039 as i32 as uint16_t,
        2061 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1337 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1455 as i32 as uint16_t,
        4122 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3048 as i32 as uint16_t,
        479 as i32 as uint16_t,
        742 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3767 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1289 as i32 as uint16_t,
        509 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1814 as i32 as uint16_t,
        166 as i32 as uint16_t,
        2766 as i32 as uint16_t,
        3733 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1561 as i32 as uint16_t,
        2099 as i32 as uint16_t,
        0 as i32 as uint16_t,
        607 as i32 as uint16_t,
        210 as i32 as uint16_t,
        466 as i32 as uint16_t,
        4362 as i32 as uint16_t,
        2424 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3343 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1181 as i32 as uint16_t,
        1426 as i32 as uint16_t,
        2135 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1063 as i32 as uint16_t,
        2041 as i32 as uint16_t,
        0 as i32 as uint16_t,
        894 as i32 as uint16_t,
        3038 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4229 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4001 as i32 as uint16_t,
        725 as i32 as uint16_t,
        1539 as i32 as uint16_t,
        916 as i32 as uint16_t,
        2282 as i32 as uint16_t,
        2199 as i32 as uint16_t,
        3936 as i32 as uint16_t,
        3175 as i32 as uint16_t,
        330 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3466 as i32 as uint16_t,
        238 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        160 as i32 as uint16_t,
        2619 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1518 as i32 as uint16_t,
        985 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4140 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3432 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1324 as i32 as uint16_t,
        4623 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1903 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3481 as i32 as uint16_t,
        3456 as i32 as uint16_t,
        2356 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1703 as i32 as uint16_t,
        3799 as i32 as uint16_t,
        0 as i32 as uint16_t,
        175 as i32 as uint16_t,
        1558 as i32 as uint16_t,
        2457 as i32 as uint16_t,
        1705 as i32 as uint16_t,
        2152 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2286 as i32 as uint16_t,
        10 as i32 as uint16_t,
        4290 as i32 as uint16_t,
        2121 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1030 as i32 as uint16_t,
        1862 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2626 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1282 as i32 as uint16_t,
        3960 as i32 as uint16_t,
        1203 as i32 as uint16_t,
        0 as i32 as uint16_t,
        298 as i32 as uint16_t,
        251 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        735 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1365 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2445 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3056 as i32 as uint16_t,
        656 as i32 as uint16_t,
        1638 as i32 as uint16_t,
        4358 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1417 as i32 as uint16_t,
        3323 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2633 as i32 as uint16_t,
        2460 as i32 as uint16_t,
        4680 as i32 as uint16_t,
        599 as i32 as uint16_t,
        1759 as i32 as uint16_t,
        4349 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4291 as i32 as uint16_t,
        1591 as i32 as uint16_t,
        4136 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3258 as i32 as uint16_t,
        3969 as i32 as uint16_t,
        158 as i32 as uint16_t,
        126 as i32 as uint16_t,
        2296 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1936 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2215 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1121 as i32 as uint16_t,
        332 as i32 as uint16_t,
        1010 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1318 as i32 as uint16_t,
        2982 as i32 as uint16_t,
        851 as i32 as uint16_t,
        2617 as i32 as uint16_t,
        3810 as i32 as uint16_t,
        1151 as i32 as uint16_t,
        4701 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1258 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4314 as i32 as uint16_t,
        1304 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2014 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1871 as i32 as uint16_t,
        4057 as i32 as uint16_t,
        652 as i32 as uint16_t,
        2043 as i32 as uint16_t,
        2440 as i32 as uint16_t,
        1072 as i32 as uint16_t,
        4177 as i32 as uint16_t,
        2949 as i32 as uint16_t,
        2086 as i32 as uint16_t,
        4522 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4658 as i32 as uint16_t,
        3277 as i32 as uint16_t,
        942 as i32 as uint16_t,
        144 as i32 as uint16_t,
        1430 as i32 as uint16_t,
        2565 as i32 as uint16_t,
        4329 as i32 as uint16_t,
        3142 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4085 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1682 as i32 as uint16_t,
        2852 as i32 as uint16_t,
        1419 as i32 as uint16_t,
        231 as i32 as uint16_t,
        0 as i32 as uint16_t,
        378 as i32 as uint16_t,
        757 as i32 as uint16_t,
        0 as i32 as uint16_t,
        225 as i32 as uint16_t,
        3315 as i32 as uint16_t,
        3304 as i32 as uint16_t,
        3179 as i32 as uint16_t,
        668 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3093 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4363 as i32 as uint16_t,
        2521 as i32 as uint16_t,
        3601 as i32 as uint16_t,
        4103 as i32 as uint16_t,
        1370 as i32 as uint16_t,
        3670 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4542 as i32 as uint16_t,
        1378 as i32 as uint16_t,
        304 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4699 as i32 as uint16_t,
        3784 as i32 as uint16_t,
        0 as i32 as uint16_t,
        95 as i32 as uint16_t,
        0 as i32 as uint16_t,
        785 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1969 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1244 as i32 as uint16_t,
        2044 as i32 as uint16_t,
        901 as i32 as uint16_t,
        3046 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3301 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1823 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        806 as i32 as uint16_t,
        1470 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4208 as i32 as uint16_t,
        4301 as i32 as uint16_t,
        877 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3983 as i32 as uint16_t,
        1416 as i32 as uint16_t,
        365 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3269 as i32 as uint16_t,
        2611 as i32 as uint16_t,
        0 as i32 as uint16_t,
        376 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1316 as i32 as uint16_t,
        0 as i32 as uint16_t,
        20 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        142 as i32 as uint16_t,
        3840 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        752 as i32 as uint16_t,
        2088 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4181 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3746 as i32 as uint16_t,
        2689 as i32 as uint16_t,
        434 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2373 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1435 as i32 as uint16_t,
        2060 as i32 as uint16_t,
        2335 as i32 as uint16_t,
        1242 as i32 as uint16_t,
        2341 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4100 as i32 as uint16_t,
        442 as i32 as uint16_t,
        3610 as i32 as uint16_t,
        2169 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1149 as i32 as uint16_t,
        2122 as i32 as uint16_t,
        0 as i32 as uint16_t,
        236 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1837 as i32 as uint16_t,
        2019 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        939 as i32 as uint16_t,
        4458 as i32 as uint16_t,
        322 as i32 as uint16_t,
        638 as i32 as uint16_t,
        1378 as i32 as uint16_t,
        1666 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1031 as i32 as uint16_t,
        4255 as i32 as uint16_t,
        4090 as i32 as uint16_t,
        4042 as i32 as uint16_t,
        434 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1486 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4622 as i32 as uint16_t,
        3684 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2861 as i32 as uint16_t,
        494 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2326 as i32 as uint16_t,
        2505 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1307 as i32 as uint16_t,
        3623 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2629 as i32 as uint16_t,
        0 as i32 as uint16_t,
        305 as i32 as uint16_t,
        1280 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4658 as i32 as uint16_t,
        519 as i32 as uint16_t,
        4078 as i32 as uint16_t,
        0 as i32 as uint16_t,
        568 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2363 as i32 as uint16_t,
        2244 as i32 as uint16_t,
        4710 as i32 as uint16_t,
        2364 as i32 as uint16_t,
        269 as i32 as uint16_t,
        4511 as i32 as uint16_t,
        3744 as i32 as uint16_t,
        3311 as i32 as uint16_t,
        2374 as i32 as uint16_t,
        2530 as i32 as uint16_t,
        2509 as i32 as uint16_t,
        733 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2824 as i32 as uint16_t,
        747 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3101 as i32 as uint16_t,
        3928 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        652 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1860 as i32 as uint16_t,
        854 as i32 as uint16_t,
        2328 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1151 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4397 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3895 as i32 as uint16_t,
        4667 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        705 as i32 as uint16_t,
        0 as i32 as uint16_t,
        604 as i32 as uint16_t,
        4303 as i32 as uint16_t,
        3096 as i32 as uint16_t,
        3290 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1559 as i32 as uint16_t,
        0 as i32 as uint16_t,
        436 as i32 as uint16_t,
        754 as i32 as uint16_t,
        2386 as i32 as uint16_t,
        0 as i32 as uint16_t,
        669 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2533 as i32 as uint16_t,
        3422 as i32 as uint16_t,
        2462 as i32 as uint16_t,
        4716 as i32 as uint16_t,
        862 as i32 as uint16_t,
        712 as i32 as uint16_t,
        485 as i32 as uint16_t,
        1168 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1037 as i32 as uint16_t,
        2311 as i32 as uint16_t,
        1093 as i32 as uint16_t,
        2395 as i32 as uint16_t,
        0 as i32 as uint16_t,
        526 as i32 as uint16_t,
        765 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2077 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4699 as i32 as uint16_t,
        2621 as i32 as uint16_t,
        1474 as i32 as uint16_t,
        0 as i32 as uint16_t,
        534 as i32 as uint16_t,
        229 as i32 as uint16_t,
        2294 as i32 as uint16_t,
        282 as i32 as uint16_t,
        1534 as i32 as uint16_t,
        2375 as i32 as uint16_t,
        2683 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1617 as i32 as uint16_t,
        306 as i32 as uint16_t,
        0 as i32 as uint16_t,
        183 as i32 as uint16_t,
        996 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        852 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3351 as i32 as uint16_t,
        1837 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        756 as i32 as uint16_t,
        1970 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1462 as i32 as uint16_t,
        1505 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2513 as i32 as uint16_t,
        2870 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        973 as i32 as uint16_t,
        1436 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1803 as i32 as uint16_t,
        2535 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2601 as i32 as uint16_t,
        1465 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        178 as i32 as uint16_t,
        2 as i32 as uint16_t,
        1351 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4550 as i32 as uint16_t,
        4396 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2399 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1700 as i32 as uint16_t,
        0 as i32 as uint16_t,
        500 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2891 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2730 as i32 as uint16_t,
        530 as i32 as uint16_t,
        1545 as i32 as uint16_t,
        3881 as i32 as uint16_t,
        530 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4468 as i32 as uint16_t,
        738 as i32 as uint16_t,
        0 as i32 as uint16_t,
        853 as i32 as uint16_t,
        1749 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1181 as i32 as uint16_t,
        542 as i32 as uint16_t,
        4318 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2550 as i32 as uint16_t,
        965 as i32 as uint16_t,
        2714 as i32 as uint16_t,
        1032 as i32 as uint16_t,
        211 as i32 as uint16_t,
        2423 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1153 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3711 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2360 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4024 as i32 as uint16_t,
        2453 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4056 as i32 as uint16_t,
        974 as i32 as uint16_t,
        2955 as i32 as uint16_t,
        3875 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        357 as i32 as uint16_t,
        3611 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2984 as i32 as uint16_t,
        2813 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1337 as i32 as uint16_t,
        2334 as i32 as uint16_t,
        1271 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4003 as i32 as uint16_t,
        2127 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1665 as i32 as uint16_t,
        2410 as i32 as uint16_t,
        281 as i32 as uint16_t,
        410 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3073 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3249 as i32 as uint16_t,
        1483 as i32 as uint16_t,
        300 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3667 as i32 as uint16_t,
        3322 as i32 as uint16_t,
        2153 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2390 as i32 as uint16_t,
        1060 as i32 as uint16_t,
        4324 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1143 as i32 as uint16_t,
        4589 as i32 as uint16_t,
        1340 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3960 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2097 as i32 as uint16_t,
        532 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1720 as i32 as uint16_t,
        1484 as i32 as uint16_t,
        567 as i32 as uint16_t,
        1158 as i32 as uint16_t,
        4349 as i32 as uint16_t,
        2377 as i32 as uint16_t,
        4051 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3760 as i32 as uint16_t,
        2252 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2651 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2630 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1958 as i32 as uint16_t,
        1928 as i32 as uint16_t,
        301 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1123 as i32 as uint16_t,
        516 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4572 as i32 as uint16_t,
        622 as i32 as uint16_t,
        1962 as i32 as uint16_t,
        443 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2739 as i32 as uint16_t,
        400 as i32 as uint16_t,
        794 as i32 as uint16_t,
        1194 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4598 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1079 as i32 as uint16_t,
        4441 as i32 as uint16_t,
        4574 as i32 as uint16_t,
        0 as i32 as uint16_t,
        130 as i32 as uint16_t,
        218 as i32 as uint16_t,
        1138 as i32 as uint16_t,
        2355 as i32 as uint16_t,
        2898 as i32 as uint16_t,
        4471 as i32 as uint16_t,
        31 as i32 as uint16_t,
        4641 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1938 as i32 as uint16_t,
        1604 as i32 as uint16_t,
        879 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2153 as i32 as uint16_t,
        3213 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1059 as i32 as uint16_t,
        1260 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2391 as i32 as uint16_t,
        586 as i32 as uint16_t,
        1791 as i32 as uint16_t,
        1540 as i32 as uint16_t,
        1384 as i32 as uint16_t,
        3100 as i32 as uint16_t,
        0 as i32 as uint16_t,
        845 as i32 as uint16_t,
        935 as i32 as uint16_t,
        1850 as i32 as uint16_t,
        2616 as i32 as uint16_t,
        4323 as i32 as uint16_t,
        587 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1985 as i32 as uint16_t,
        2636 as i32 as uint16_t,
        1921 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2385 as i32 as uint16_t,
        2497 as i32 as uint16_t,
        173 as i32 as uint16_t,
        3971 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2502 as i32 as uint16_t,
        1549 as i32 as uint16_t,
        0 as i32 as uint16_t,
        548 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        35 as i32 as uint16_t,
        1630 as i32 as uint16_t,
        1776 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3696 as i32 as uint16_t,
        3169 as i32 as uint16_t,
        3566 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1929 as i32 as uint16_t,
        1037 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3344 as i32 as uint16_t,
        101 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2548 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4166 as i32 as uint16_t,
        3962 as i32 as uint16_t,
        1221 as i32 as uint16_t,
        3928 as i32 as uint16_t,
        1014 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3624 as i32 as uint16_t,
        3461 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2535 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2350 as i32 as uint16_t,
        0 as i32 as uint16_t,
        942 as i32 as uint16_t,
        4574 as i32 as uint16_t,
        166 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1509 as i32 as uint16_t,
        2067 as i32 as uint16_t,
        3966 as i32 as uint16_t,
        2717 as i32 as uint16_t,
        4451 as i32 as uint16_t,
        4047 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3925 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2505 as i32 as uint16_t,
        961 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3514 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3902 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        506 as i32 as uint16_t,
        0 as i32 as uint16_t,
        100 as i32 as uint16_t,
        3824 as i32 as uint16_t,
        254 as i32 as uint16_t,
        464 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4262 as i32 as uint16_t,
        358 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4625 as i32 as uint16_t,
        3350 as i32 as uint16_t,
        4556 as i32 as uint16_t,
        1193 as i32 as uint16_t,
        1473 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2732 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2103 as i32 as uint16_t,
        794 as i32 as uint16_t,
        1165 as i32 as uint16_t,
        1612 as i32 as uint16_t,
        3616 as i32 as uint16_t,
        1988 as i32 as uint16_t,
        3737 as i32 as uint16_t,
        3924 as i32 as uint16_t,
        2342 as i32 as uint16_t,
        0 as i32 as uint16_t,
        164 as i32 as uint16_t,
        4398 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1955 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4611 as i32 as uint16_t,
        1612 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1994 as i32 as uint16_t,
        2208 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2425 as i32 as uint16_t,
        937 as i32 as uint16_t,
        4317 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3606 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2842 as i32 as uint16_t,
        671 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2641 as i32 as uint16_t,
        0 as i32 as uint16_t,
        437 as i32 as uint16_t,
        3751 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4107 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        1837 as i32 as uint16_t,
        2404 as i32 as uint16_t,
        1260 as i32 as uint16_t,
        281 as i32 as uint16_t,
        981 as i32 as uint16_t,
        2499 as i32 as uint16_t,
        693 as i32 as uint16_t,
        1802 as i32 as uint16_t,
        413 as i32 as uint16_t,
        744 as i32 as uint16_t,
        2163 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3845 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2540 as i32 as uint16_t,
        3519 as i32 as uint16_t,
        1235 as i32 as uint16_t,
        4136 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2107 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        832 as i32 as uint16_t,
        731 as i32 as uint16_t,
        2436 as i32 as uint16_t,
        79 as i32 as uint16_t,
        249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2366 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2940 as i32 as uint16_t,
        1542 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        812 as i32 as uint16_t,
        1426 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1769 as i32 as uint16_t,
        1069 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        787 as i32 as uint16_t,
        626 as i32 as uint16_t,
        4653 as i32 as uint16_t,
        2225 as i32 as uint16_t,
        1061 as i32 as uint16_t,
        873 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4481 as i32 as uint16_t,
        1170 as i32 as uint16_t,
        1662 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1223 as i32 as uint16_t,
        1181 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3861 as i32 as uint16_t,
        3039 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3556 as i32 as uint16_t,
        1492 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4597 as i32 as uint16_t,
        400 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1653 as i32 as uint16_t,
        1396 as i32 as uint16_t,
        0 as i32 as uint16_t,
        677 as i32 as uint16_t,
        3848 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3121 as i32 as uint16_t,
        2289 as i32 as uint16_t,
        1198 as i32 as uint16_t,
        2014 as i32 as uint16_t,
        2634 as i32 as uint16_t,
        2812 as i32 as uint16_t,
        2255 as i32 as uint16_t,
        1182 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        579 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        629 as i32 as uint16_t,
        4452 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1551 as i32 as uint16_t,
        2626 as i32 as uint16_t,
        2362 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3958 as i32 as uint16_t,
        1725 as i32 as uint16_t,
        1357 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1673 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2728 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2609 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2977 as i32 as uint16_t,
        1350 as i32 as uint16_t,
        3003 as i32 as uint16_t,
        943 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2261 as i32 as uint16_t,
        4303 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2359 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4449 as i32 as uint16_t,
        100 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2521 as i32 as uint16_t,
        76 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2851 as i32 as uint16_t,
        3597 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2395 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1777 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2090 as i32 as uint16_t,
        859 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1167 as i32 as uint16_t,
        4662 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4581 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1707 as i32 as uint16_t,
        2877 as i32 as uint16_t,
        587 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2381 as i32 as uint16_t,
        38 as i32 as uint16_t,
        607 as i32 as uint16_t,
        1408 as i32 as uint16_t,
        2284 as i32 as uint16_t,
        2449 as i32 as uint16_t,
        1556 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4238 as i32 as uint16_t,
        2669 as i32 as uint16_t,
        273 as i32 as uint16_t,
        1578 as i32 as uint16_t,
        1403 as i32 as uint16_t,
        527 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2859 as i32 as uint16_t,
        310 as i32 as uint16_t,
        2528 as i32 as uint16_t,
        1775 as i32 as uint16_t,
        4043 as i32 as uint16_t,
        41 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        313 as i32 as uint16_t,
        1912 as i32 as uint16_t,
        3434 as i32 as uint16_t,
        1584 as i32 as uint16_t,
        0 as i32 as uint16_t,
        180 as i32 as uint16_t,
        832 as i32 as uint16_t,
        25 as i32 as uint16_t,
        562 as i32 as uint16_t,
        1880 as i32 as uint16_t,
        1055 as i32 as uint16_t,
        1405 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2974 as i32 as uint16_t,
        1042 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1757 as i32 as uint16_t,
        1308 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1944 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1740 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2254 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2159 as i32 as uint16_t,
        2123 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1635 as i32 as uint16_t,
        1602 as i32 as uint16_t,
        0 as i32 as uint16_t,
        977 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1276 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1386 as i32 as uint16_t,
        584 as i32 as uint16_t,
        1114 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3483 as i32 as uint16_t,
        3749 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1166 as i32 as uint16_t,
        1702 as i32 as uint16_t,
        516 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3480 as i32 as uint16_t,
        1678 as i32 as uint16_t,
        1832 as i32 as uint16_t,
        3840 as i32 as uint16_t,
        217 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3749 as i32 as uint16_t,
        1025 as i32 as uint16_t,
        0 as i32 as uint16_t,
        969 as i32 as uint16_t,
        2568 as i32 as uint16_t,
        0 as i32 as uint16_t,
        526 as i32 as uint16_t,
        867 as i32 as uint16_t,
        1813 as i32 as uint16_t,
        4222 as i32 as uint16_t,
        4067 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        955 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2222 as i32 as uint16_t,
        509 as i32 as uint16_t,
        2402 as i32 as uint16_t,
        1094 as i32 as uint16_t,
        1200 as i32 as uint16_t,
        4192 as i32 as uint16_t,
        4471 as i32 as uint16_t,
        0 as i32 as uint16_t,
        129 as i32 as uint16_t,
        147 as i32 as uint16_t,
        363 as i32 as uint16_t,
        3249 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3767 as i32 as uint16_t,
        4592 as i32 as uint16_t,
        751 as i32 as uint16_t,
        0 as i32 as uint16_t,
        330 as i32 as uint16_t,
        1540 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3096 as i32 as uint16_t,
        5 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2420 as i32 as uint16_t,
        631 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1060 as i32 as uint16_t,
        733 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4132 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3809 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4001 as i32 as uint16_t,
        2593 as i32 as uint16_t,
        1891 as i32 as uint16_t,
        4277 as i32 as uint16_t,
        199 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1811 as i32 as uint16_t,
        1816 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3734 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1382 as i32 as uint16_t,
        1537 as i32 as uint16_t,
        0 as i32 as uint16_t,
        205 as i32 as uint16_t,
        0 as i32 as uint16_t,
        258 as i32 as uint16_t,
        3305 as i32 as uint16_t,
        1566 as i32 as uint16_t,
        1966 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1410 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2309 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2984 as i32 as uint16_t,
        695 as i32 as uint16_t,
        1544 as i32 as uint16_t,
        4425 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2073 as i32 as uint16_t,
        1466 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1223 as i32 as uint16_t,
        588 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1199 as i32 as uint16_t,
        4493 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1825 as i32 as uint16_t,
        717 as i32 as uint16_t,
        0 as i32 as uint16_t,
        227 as i32 as uint16_t,
        3467 as i32 as uint16_t,
        4458 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3412 as i32 as uint16_t,
        90 as i32 as uint16_t,
        1208 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4058 as i32 as uint16_t,
        4236 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1771 as i32 as uint16_t,
        4672 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        763 as i32 as uint16_t,
        2812 as i32 as uint16_t,
        601 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3605 as i32 as uint16_t,
        967 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1364 as i32 as uint16_t,
        2176 as i32 as uint16_t,
        2820 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1913 as i32 as uint16_t,
        421 as i32 as uint16_t,
        0 as i32 as uint16_t,
        111 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2262 as i32 as uint16_t,
        2052 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2150 as i32 as uint16_t,
        1857 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1893 as i32 as uint16_t,
        3413 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        829 as i32 as uint16_t,
        1520 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2132 as i32 as uint16_t,
        238 as i32 as uint16_t,
        3160 as i32 as uint16_t,
        4447 as i32 as uint16_t,
        312 as i32 as uint16_t,
        2310 as i32 as uint16_t,
        4329 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3651 as i32 as uint16_t,
        3034 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1843 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3999 as i32 as uint16_t,
        0 as i32 as uint16_t,
        127 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2233 as i32 as uint16_t,
        2913 as i32 as uint16_t,
        1415 as i32 as uint16_t,
        4129 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2437 as i32 as uint16_t,
        4696 as i32 as uint16_t,
        902 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2473 as i32 as uint16_t,
        561 as i32 as uint16_t,
        0 as i32 as uint16_t,
        827 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1971 as i32 as uint16_t,
        3875 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3285 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3777 as i32 as uint16_t,
        1901 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3203 as i32 as uint16_t,
        4581 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3210 as i32 as uint16_t,
        4715 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1130 as i32 as uint16_t,
        3895 as i32 as uint16_t,
        1444 as i32 as uint16_t,
        118 as i32 as uint16_t,
        0 as i32 as uint16_t,
        328 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1188 as i32 as uint16_t,
        1646 as i32 as uint16_t,
        2104 as i32 as uint16_t,
        863 as i32 as uint16_t,
        1055 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2140 as i32 as uint16_t,
        4672 as i32 as uint16_t,
        4079 as i32 as uint16_t,
        1650 as i32 as uint16_t,
        1961 as i32 as uint16_t,
        993 as i32 as uint16_t,
        0 as i32 as uint16_t,
        375 as i32 as uint16_t,
        3954 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2196 as i32 as uint16_t,
        963 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2727 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4349 as i32 as uint16_t,
        2474 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2352 as i32 as uint16_t,
        3781 as i32 as uint16_t,
        2116 as i32 as uint16_t,
        3509 as i32 as uint16_t,
        1144 as i32 as uint16_t,
        246 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3334 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1038 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        70 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1339 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2162 as i32 as uint16_t,
        180 as i32 as uint16_t,
        0 as i32 as uint16_t,
        710 as i32 as uint16_t,
        103 as i32 as uint16_t,
        861 as i32 as uint16_t,
        4521 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3550 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3970 as i32 as uint16_t,
        608 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2088 as i32 as uint16_t,
        1693 as i32 as uint16_t,
        1607 as i32 as uint16_t,
        2331 as i32 as uint16_t,
        4189 as i32 as uint16_t,
        773 as i32 as uint16_t,
        2025 as i32 as uint16_t,
        872 as i32 as uint16_t,
        31 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1919 as i32 as uint16_t,
        2265 as i32 as uint16_t,
        962 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2497 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4664 as i32 as uint16_t,
        2501 as i32 as uint16_t,
        249 as i32 as uint16_t,
        1695 as i32 as uint16_t,
        817 as i32 as uint16_t,
        3380 as i32 as uint16_t,
        3386 as i32 as uint16_t,
        1188 as i32 as uint16_t,
        3559 as i32 as uint16_t,
        845 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3553 as i32 as uint16_t,
        1614 as i32 as uint16_t,
        2032 as i32 as uint16_t,
        4660 as i32 as uint16_t,
        2512 as i32 as uint16_t,
        3175 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        868 as i32 as uint16_t,
        847 as i32 as uint16_t,
        727 as i32 as uint16_t,
        1218 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1002 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2821 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2978 as i32 as uint16_t,
        3699 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1940 as i32 as uint16_t,
        0 as i32 as uint16_t,
        69 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1113 as i32 as uint16_t,
        1694 as i32 as uint16_t,
        0 as i32 as uint16_t,
        750 as i32 as uint16_t,
        4266 as i32 as uint16_t,
        643 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4647 as i32 as uint16_t,
        2975 as i32 as uint16_t,
        4556 as i32 as uint16_t,
        3997 as i32 as uint16_t,
        1197 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        811 as i32 as uint16_t,
        2419 as i32 as uint16_t,
        1849 as i32 as uint16_t,
        2695 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2687 as i32 as uint16_t,
        1528 as i32 as uint16_t,
        1158 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3487 as i32 as uint16_t,
        3584 as i32 as uint16_t,
        2061 as i32 as uint16_t,
        2591 as i32 as uint16_t,
        4274 as i32 as uint16_t,
        176 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4138 as i32 as uint16_t,
        4401 as i32 as uint16_t,
        1112 as i32 as uint16_t,
        2610 as i32 as uint16_t,
        167 as i32 as uint16_t,
        2658 as i32 as uint16_t,
        2591 as i32 as uint16_t,
        1768 as i32 as uint16_t,
        2699 as i32 as uint16_t,
        3958 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        221 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        38 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1237 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2439 as i32 as uint16_t,
        877 as i32 as uint16_t,
        1858 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2070 as i32 as uint16_t,
        3062 as i32 as uint16_t,
        2181 as i32 as uint16_t,
        608 as i32 as uint16_t,
        0 as i32 as uint16_t,
        29 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        880 as i32 as uint16_t,
        1679 as i32 as uint16_t,
        72 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1756 as i32 as uint16_t,
        2193 as i32 as uint16_t,
        384 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4181 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3608 as i32 as uint16_t,
        1326 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3699 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3503 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1735 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        358 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        972 as i32 as uint16_t,
        1063 as i32 as uint16_t,
        2668 as i32 as uint16_t,
        3960 as i32 as uint16_t,
        2786 as i32 as uint16_t,
        1862 as i32 as uint16_t,
        2093 as i32 as uint16_t,
        1845 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1931 as i32 as uint16_t,
        995 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3570 as i32 as uint16_t,
        364 as i32 as uint16_t,
        1054 as i32 as uint16_t,
        2914 as i32 as uint16_t,
        1957 as i32 as uint16_t,
        842 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1903 as i32 as uint16_t,
        4485 as i32 as uint16_t,
        1076 as i32 as uint16_t,
        1656 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2265 as i32 as uint16_t,
        811 as i32 as uint16_t,
        3941 as i32 as uint16_t,
        957 as i32 as uint16_t,
        757 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3404 as i32 as uint16_t,
        3369 as i32 as uint16_t,
        0 as i32 as uint16_t,
        244 as i32 as uint16_t,
        553 as i32 as uint16_t,
        1362 as i32 as uint16_t,
        0 as i32 as uint16_t,
        958 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1755 as i32 as uint16_t,
        1901 as i32 as uint16_t,
        1792 as i32 as uint16_t,
        3757 as i32 as uint16_t,
        789 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3437 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3932 as i32 as uint16_t,
        0 as i32 as uint16_t,
        407 as i32 as uint16_t,
        3403 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1177 as i32 as uint16_t,
        1399 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1726 as i32 as uint16_t,
        1234 as i32 as uint16_t,
        0 as i32 as uint16_t,
        189 as i32 as uint16_t,
        0 as i32 as uint16_t,
        518 as i32 as uint16_t,
        719 as i32 as uint16_t,
        4596 as i32 as uint16_t,
        3647 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1291 as i32 as uint16_t,
        25 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4187 as i32 as uint16_t,
        94 as i32 as uint16_t,
        3004 as i32 as uint16_t,
        2954 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2016 as i32 as uint16_t,
        3532 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2204 as i32 as uint16_t,
        1180 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2310 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3341 as i32 as uint16_t,
        4423 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        844 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2873 as i32 as uint16_t,
        3035 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2831 as i32 as uint16_t,
        4 as i32 as uint16_t,
        948 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        566 as i32 as uint16_t,
        1199 as i32 as uint16_t,
        1194 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2456 as i32 as uint16_t,
        3581 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1850 as i32 as uint16_t,
        238 as i32 as uint16_t,
        1484 as i32 as uint16_t,
        4036 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1949 as i32 as uint16_t,
        4581 as i32 as uint16_t,
        2960 as i32 as uint16_t,
        3716 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1885 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        601 as i32 as uint16_t,
        2318 as i32 as uint16_t,
        370 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2275 as i32 as uint16_t,
        3911 as i32 as uint16_t,
        983 as i32 as uint16_t,
        4570 as i32 as uint16_t,
        3494 as i32 as uint16_t,
        15 as i32 as uint16_t,
        4610 as i32 as uint16_t,
        1993 as i32 as uint16_t,
        0 as i32 as uint16_t,
        743 as i32 as uint16_t,
        3104 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2771 as i32 as uint16_t,
        2214 as i32 as uint16_t,
        2709 as i32 as uint16_t,
        14 as i32 as uint16_t,
        2365 as i32 as uint16_t,
        702 as i32 as uint16_t,
        0 as i32 as uint16_t,
        803 as i32 as uint16_t,
        0 as i32 as uint16_t,
        627 as i32 as uint16_t,
        3921 as i32 as uint16_t,
        1501 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        984 as i32 as uint16_t,
        1249 as i32 as uint16_t,
        2615 as i32 as uint16_t,
        836 as i32 as uint16_t,
        1043 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1374 as i32 as uint16_t,
        843 as i32 as uint16_t,
        2195 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        228 as i32 as uint16_t,
        137 as i32 as uint16_t,
        1412 as i32 as uint16_t,
        1678 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1149 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2071 as i32 as uint16_t,
        3584 as i32 as uint16_t,
        84 as i32 as uint16_t,
        1292 as i32 as uint16_t,
        711 as i32 as uint16_t,
        24 as i32 as uint16_t,
        0 as i32 as uint16_t,
        646 as i32 as uint16_t,
        79 as i32 as uint16_t,
        1083 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2266 as i32 as uint16_t,
        2305 as i32 as uint16_t,
        1212 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1363 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4428 as i32 as uint16_t,
        3377 as i32 as uint16_t,
        3576 as i32 as uint16_t,
        2382 as i32 as uint16_t,
        3798 as i32 as uint16_t,
        2500 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2645 as i32 as uint16_t,
        2628 as i32 as uint16_t,
        978 as i32 as uint16_t,
        1701 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2000 as i32 as uint16_t,
        402 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2802 as i32 as uint16_t,
        999 as i32 as uint16_t,
        1674 as i32 as uint16_t,
        8 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1907 as i32 as uint16_t,
        2781 as i32 as uint16_t,
        1564 as i32 as uint16_t,
        3544 as i32 as uint16_t,
        1228 as i32 as uint16_t,
        839 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4335 as i32 as uint16_t,
        3413 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3299 as i32 as uint16_t,
        2507 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2262 as i32 as uint16_t,
        0 as i32 as uint16_t,
        790 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3672 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3533 as i32 as uint16_t,
        670 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1393 as i32 as uint16_t,
        746 as i32 as uint16_t,
        2492 as i32 as uint16_t,
        713 as i32 as uint16_t,
        853 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2241 as i32 as uint16_t,
        1669 as i32 as uint16_t,
        0 as i32 as uint16_t,
        114 as i32 as uint16_t,
        366 as i32 as uint16_t,
        1320 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        884 as i32 as uint16_t,
        2181 as i32 as uint16_t,
        2489 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2885 as i32 as uint16_t,
        2508 as i32 as uint16_t,
        0 as i32 as uint16_t,
        317 as i32 as uint16_t,
        2181 as i32 as uint16_t,
        2890 as i32 as uint16_t,
        3096 as i32 as uint16_t,
        1270 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4718 as i32 as uint16_t,
        922 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1374 as i32 as uint16_t,
        238 as i32 as uint16_t,
        1630 as i32 as uint16_t,
        0 as i32 as uint16_t,
        404 as i32 as uint16_t,
        4567 as i32 as uint16_t,
        940 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2164 as i32 as uint16_t,
        2168 as i32 as uint16_t,
        3419 as i32 as uint16_t,
        0 as i32 as uint16_t,
        876 as i32 as uint16_t,
        1069 as i32 as uint16_t,
        946 as i32 as uint16_t,
        418 as i32 as uint16_t,
        1307 as i32 as uint16_t,
        0 as i32 as uint16_t,
        802 as i32 as uint16_t,
        1110 as i32 as uint16_t,
        1090 as i32 as uint16_t,
        4617 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1912 as i32 as uint16_t,
        582 as i32 as uint16_t,
        2559 as i32 as uint16_t,
        1671 as i32 as uint16_t,
        504 as i32 as uint16_t,
        3516 as i32 as uint16_t,
        1076 as i32 as uint16_t,
        3092 as i32 as uint16_t,
        3788 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4113 as i32 as uint16_t,
        1496 as i32 as uint16_t,
        0 as i32 as uint16_t,
        518 as i32 as uint16_t,
        284 as i32 as uint16_t,
        4067 as i32 as uint16_t,
        454 as i32 as uint16_t,
        2210 as i32 as uint16_t,
        89 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2354 as i32 as uint16_t,
        1694 as i32 as uint16_t,
        4053 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1055 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2794 as i32 as uint16_t,
        2685 as i32 as uint16_t,
        1623 as i32 as uint16_t,
        1683 as i32 as uint16_t,
        417 as i32 as uint16_t,
        1791 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4294 as i32 as uint16_t,
        1874 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3481 as i32 as uint16_t,
        1579 as i32 as uint16_t,
        466 as i32 as uint16_t,
        2244 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4007 as i32 as uint16_t,
        1961 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        460 as i32 as uint16_t,
        1946 as i32 as uint16_t,
        490 as i32 as uint16_t,
        508 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1619 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3757 as i32 as uint16_t,
        2147 as i32 as uint16_t,
        1059 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1201 as i32 as uint16_t,
        443 as i32 as uint16_t,
        0 as i32 as uint16_t,
        723 as i32 as uint16_t,
        872 as i32 as uint16_t,
        873 as i32 as uint16_t,
        913 as i32 as uint16_t,
        3753 as i32 as uint16_t,
        0 as i32 as uint16_t,
        24 as i32 as uint16_t,
        739 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2216 as i32 as uint16_t,
        3475 as i32 as uint16_t,
        831 as i32 as uint16_t,
        4259 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4037 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        526 as i32 as uint16_t,
        4702 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1204 as i32 as uint16_t,
        3819 as i32 as uint16_t,
        2230 as i32 as uint16_t,
        0 as i32 as uint16_t,
        914 as i32 as uint16_t,
        0 as i32 as uint16_t,
        430 as i32 as uint16_t,
        2408 as i32 as uint16_t,
        3673 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3234 as i32 as uint16_t,
        3541 as i32 as uint16_t,
        3932 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1355 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3061 as i32 as uint16_t,
        2170 as i32 as uint16_t,
        0 as i32 as uint16_t,
        488 as i32 as uint16_t,
        0 as i32 as uint16_t,
        3964 as i32 as uint16_t,
        1286 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2193 as i32 as uint16_t,
        333 as i32 as uint16_t,
        295 as i32 as uint16_t,
        3552 as i32 as uint16_t,
        4081 as i32 as uint16_t,
        3204 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1094 as i32 as uint16_t,
        3640 as i32 as uint16_t,
        1553 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1305 as i32 as uint16_t,
        3915 as i32 as uint16_t,
        100 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        4189 as i32 as uint16_t,
        2095 as i32 as uint16_t,
        1923 as i32 as uint16_t,
        2472 as i32 as uint16_t,
        0 as i32 as uint16_t,
        428 as i32 as uint16_t,
        2977 as i32 as uint16_t,
        0 as i32 as uint16_t,
        665 as i32 as uint16_t,
        2493 as i32 as uint16_t,
        255 as i32 as uint16_t,
        927 as i32 as uint16_t,
        778 as i32 as uint16_t,
        1614 as i32 as uint16_t,
        1723 as i32 as uint16_t,
        920 as i32 as uint16_t,
        0 as i32 as uint16_t,
        26 as i32 as uint16_t,
        343 as i32 as uint16_t,
        199 as i32 as uint16_t,
        213 as i32 as uint16_t,
        1928 as i32 as uint16_t,
        78 as i32 as uint16_t,
        3113 as i32 as uint16_t,
        1156 as i32 as uint16_t,
        4255 as i32 as uint16_t,
        2726 as i32 as uint16_t,
        1137 as i32 as uint16_t,
        2272 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        1379 as i32 as uint16_t,
        1631 as i32 as uint16_t,
        2260 as i32 as uint16_t,
        0 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2726 as i32 as uint16_t,
        0 as i32 as uint16_t,
        2414 as i32 as uint16_t,
    ];
    #[inline]
    pub unsafe fn keysym_name_perfect_hash(mut key: *const i8) -> usize {
        unsafe {
            let mut T1: *const i8 = b"gQEXVgBVbDK59TnjkSMO7UnyrqsrcaA4\0".as_ptr() as *const i8;
            let mut T2: *const i8 = b"AB6xkcvEK5OHbYOD14cPYBxnVAoDGTPL\0".as_ptr() as *const i8;
            let mut h1: usize = 0 as usize;
            let mut h2: usize = 0 as usize;
            let mut i: usize = 0 as usize;
            while *key.offset(i as isize) as i32 != '\0' as i32 {
                h1 = h1.wrapping_add(
                    (*T1.offset(i.wrapping_rem(32 as usize) as isize) as i32
                        * *key.offset(i as isize) as i32) as usize,
                );
                h2 = h2.wrapping_add(
                    (*T2.offset(i.wrapping_rem(32 as usize) as isize) as i32
                        * *key.offset(i as isize) as i32) as usize,
                );
                i = i.wrapping_add(1);
            }
            return ((keysym_name_G[h1.wrapping_rem(4728 as usize) as usize] as i32
                + keysym_name_G[h2.wrapping_rem(4728 as usize) as usize] as i32)
                % 4728 as i32) as usize;
        }
    }
    pub static mut name_to_keysym: [name_keysym; 2635] = [
        name_keysym {
            keysym: 0x30 as xkb_keysym_t,
            offset: 0 as uint16_t,
        },
        name_keysym {
            keysym: 0x31 as xkb_keysym_t,
            offset: 2 as uint16_t,
        },
        name_keysym {
            keysym: 0x32 as xkb_keysym_t,
            offset: 4 as uint16_t,
        },
        name_keysym {
            keysym: 0x33 as xkb_keysym_t,
            offset: 6 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd10 as xkb_keysym_t,
            offset: 8 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0e as xkb_keysym_t,
            offset: 23 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd05 as xkb_keysym_t,
            offset: 33 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd19 as xkb_keysym_t,
            offset: 46 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd15 as xkb_keysym_t,
            offset: 64 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0f as xkb_keysym_t,
            offset: 74 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1c as xkb_keysym_t,
            offset: 91 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1a as xkb_keysym_t,
            offset: 109 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd01 as xkb_keysym_t,
            offset: 125 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1e as xkb_keysym_t,
            offset: 140 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd06 as xkb_keysym_t,
            offset: 151 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd07 as xkb_keysym_t,
            offset: 165 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1b as xkb_keysym_t,
            offset: 181 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd02 as xkb_keysym_t,
            offset: 195 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd13 as xkb_keysym_t,
            offset: 210 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd12 as xkb_keysym_t,
            offset: 221 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd11 as xkb_keysym_t,
            offset: 231 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd04 as xkb_keysym_t,
            offset: 245 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0a as xkb_keysym_t,
            offset: 256 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0b as xkb_keysym_t,
            offset: 265 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0c as xkb_keysym_t,
            offset: 274 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd16 as xkb_keysym_t,
            offset: 283 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1d as xkb_keysym_t,
            offset: 293 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd09 as xkb_keysym_t,
            offset: 310 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd18 as xkb_keysym_t,
            offset: 320 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd08 as xkb_keysym_t,
            offset: 332 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd03 as xkb_keysym_t,
            offset: 343 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd14 as xkb_keysym_t,
            offset: 355 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd17 as xkb_keysym_t,
            offset: 365 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0d as xkb_keysym_t,
            offset: 376 as uint16_t,
        },
        name_keysym {
            keysym: 0x34 as xkb_keysym_t,
            offset: 386 as uint16_t,
        },
        name_keysym {
            keysym: 0x35 as xkb_keysym_t,
            offset: 388 as uint16_t,
        },
        name_keysym {
            keysym: 0x36 as xkb_keysym_t,
            offset: 390 as uint16_t,
        },
        name_keysym {
            keysym: 0x37 as xkb_keysym_t,
            offset: 392 as uint16_t,
        },
        name_keysym {
            keysym: 0x38 as xkb_keysym_t,
            offset: 394 as uint16_t,
        },
        name_keysym {
            keysym: 0x39 as xkb_keysym_t,
            offset: 396 as uint16_t,
        },
        name_keysym {
            keysym: 0x41 as xkb_keysym_t,
            offset: 398 as uint16_t,
        },
        name_keysym {
            keysym: 0x61 as xkb_keysym_t,
            offset: 400 as uint16_t,
        },
        name_keysym {
            keysym: 0xc1 as xkb_keysym_t,
            offset: 402 as uint16_t,
        },
        name_keysym {
            keysym: 0xe1 as xkb_keysym_t,
            offset: 409 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea0 as xkb_keysym_t,
            offset: 416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea1 as xkb_keysym_t,
            offset: 426 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ff as xkb_keysym_t,
            offset: 436 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c3 as xkb_keysym_t,
            offset: 445 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e3 as xkb_keysym_t,
            offset: 452 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eae as xkb_keysym_t,
            offset: 459 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eaf as xkb_keysym_t,
            offset: 471 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb6 as xkb_keysym_t,
            offset: 483 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb7 as xkb_keysym_t,
            offset: 498 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb0 as xkb_keysym_t,
            offset: 513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb1 as xkb_keysym_t,
            offset: 525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb2 as xkb_keysym_t,
            offset: 537 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb3 as xkb_keysym_t,
            offset: 548 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb4 as xkb_keysym_t,
            offset: 559 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb5 as xkb_keysym_t,
            offset: 571 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe70 as xkb_keysym_t,
            offset: 583 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe71 as xkb_keysym_t,
            offset: 598 as uint16_t,
        },
        name_keysym {
            keysym: 0xc2 as xkb_keysym_t,
            offset: 622 as uint16_t,
        },
        name_keysym {
            keysym: 0xe2 as xkb_keysym_t,
            offset: 634 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea4 as xkb_keysym_t,
            offset: 646 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea5 as xkb_keysym_t,
            offset: 663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eac as xkb_keysym_t,
            offset: 680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ead as xkb_keysym_t,
            offset: 700 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea6 as xkb_keysym_t,
            offset: 720 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea7 as xkb_keysym_t,
            offset: 737 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea8 as xkb_keysym_t,
            offset: 754 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea9 as xkb_keysym_t,
            offset: 770 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eaa as xkb_keysym_t,
            offset: 786 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eab as xkb_keysym_t,
            offset: 803 as uint16_t,
        },
        name_keysym {
            keysym: 0xb4 as xkb_keysym_t,
            offset: 820 as uint16_t,
        },
        name_keysym {
            keysym: 0xc4 as xkb_keysym_t,
            offset: 826 as uint16_t,
        },
        name_keysym {
            keysym: 0xe4 as xkb_keysym_t,
            offset: 837 as uint16_t,
        },
        name_keysym {
            keysym: 0xc6 as xkb_keysym_t,
            offset: 848 as uint16_t,
        },
        name_keysym {
            keysym: 0xe6 as xkb_keysym_t,
            offset: 851 as uint16_t,
        },
        name_keysym {
            keysym: 0xc0 as xkb_keysym_t,
            offset: 854 as uint16_t,
        },
        name_keysym {
            keysym: 0xe0 as xkb_keysym_t,
            offset: 861 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea2 as xkb_keysym_t,
            offset: 868 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea3 as xkb_keysym_t,
            offset: 874 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe9 as xkb_keysym_t,
            offset: 880 as uint16_t,
        },
        name_keysym {
            keysym: 0xffea as xkb_keysym_t,
            offset: 886 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c0 as xkb_keysym_t,
            offset: 892 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e0 as xkb_keysym_t,
            offset: 900 as uint16_t,
        },
        name_keysym {
            keysym: 0x26 as xkb_keysym_t,
            offset: 908 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a1 as xkb_keysym_t,
            offset: 918 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b1 as xkb_keysym_t,
            offset: 926 as uint16_t,
        },
        name_keysym {
            keysym: 0x27 as xkb_keysym_t,
            offset: 934 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002248 as xkb_keysym_t,
            offset: 945 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c8 as xkb_keysym_t,
            offset: 954 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000660 as xkb_keysym_t,
            offset: 966 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000661 as xkb_keysym_t,
            offset: 975 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000662 as xkb_keysym_t,
            offset: 984 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000663 as xkb_keysym_t,
            offset: 993 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000664 as xkb_keysym_t,
            offset: 1002 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000665 as xkb_keysym_t,
            offset: 1011 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000666 as xkb_keysym_t,
            offset: 1020 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000667 as xkb_keysym_t,
            offset: 1029 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000668 as xkb_keysym_t,
            offset: 1038 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000669 as xkb_keysym_t,
            offset: 1047 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d9 as xkb_keysym_t,
            offset: 1056 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c7 as xkb_keysym_t,
            offset: 1067 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e9 as xkb_keysym_t,
            offset: 1079 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c8 as xkb_keysym_t,
            offset: 1098 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ac as xkb_keysym_t,
            offset: 1109 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d6 as xkb_keysym_t,
            offset: 1122 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cf as xkb_keysym_t,
            offset: 1133 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ef as xkb_keysym_t,
            offset: 1144 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ec as xkb_keysym_t,
            offset: 1157 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000688 as xkb_keysym_t,
            offset: 1173 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006cc as xkb_keysym_t,
            offset: 1185 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ee as xkb_keysym_t,
            offset: 1202 as uint16_t,
        },
        name_keysym {
            keysym: 0x5eb as xkb_keysym_t,
            offset: 1215 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e1 as xkb_keysym_t,
            offset: 1231 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006d4 as xkb_keysym_t,
            offset: 1242 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006af as xkb_keysym_t,
            offset: 1258 as uint16_t,
        },
        name_keysym {
            keysym: 0x5da as xkb_keysym_t,
            offset: 1269 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e7 as xkb_keysym_t,
            offset: 1282 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cd as xkb_keysym_t,
            offset: 1292 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c1 as xkb_keysym_t,
            offset: 1303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000654 as xkb_keysym_t,
            offset: 1316 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000655 as xkb_keysym_t,
            offset: 1335 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c3 as xkb_keysym_t,
            offset: 1354 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c4 as xkb_keysym_t,
            offset: 1373 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c6 as xkb_keysym_t,
            offset: 1391 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c5 as xkb_keysym_t,
            offset: 1409 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e7 as xkb_keysym_t,
            offset: 1431 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006be as xkb_keysym_t,
            offset: 1442 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006c1 as xkb_keysym_t,
            offset: 1465 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cc as xkb_keysym_t,
            offset: 1481 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000698 as xkb_keysym_t,
            offset: 1493 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e3 as xkb_keysym_t,
            offset: 1504 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f0 as xkb_keysym_t,
            offset: 1515 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ed as xkb_keysym_t,
            offset: 1528 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006a9 as xkb_keysym_t,
            offset: 1544 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ce as xkb_keysym_t,
            offset: 1557 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e4 as xkb_keysym_t,
            offset: 1569 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000653 as xkb_keysym_t,
            offset: 1580 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c2 as xkb_keysym_t,
            offset: 1599 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e5 as xkb_keysym_t,
            offset: 1618 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e6 as xkb_keysym_t,
            offset: 1630 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006ba as xkb_keysym_t,
            offset: 1642 as uint16_t,
        },
        name_keysym {
            keysym: 0x100067e as xkb_keysym_t,
            offset: 1661 as uint16_t,
        },
        name_keysym {
            keysym: 0x100066a as xkb_keysym_t,
            offset: 1672 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e2 as xkb_keysym_t,
            offset: 1687 as uint16_t,
        },
        name_keysym {
            keysym: 0x5bf as xkb_keysym_t,
            offset: 1698 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d1 as xkb_keysym_t,
            offset: 1719 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000691 as xkb_keysym_t,
            offset: 1729 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d5 as xkb_keysym_t,
            offset: 1741 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d3 as xkb_keysym_t,
            offset: 1752 as uint16_t,
        },
        name_keysym {
            keysym: 0x5bb as xkb_keysym_t,
            offset: 1764 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f1 as xkb_keysym_t,
            offset: 1781 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d4 as xkb_keysym_t,
            offset: 1795 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f2 as xkb_keysym_t,
            offset: 1808 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000670 as xkb_keysym_t,
            offset: 1821 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 1845 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d7 as xkb_keysym_t,
            offset: 1859 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e0 as xkb_keysym_t,
            offset: 1870 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000686 as xkb_keysym_t,
            offset: 1885 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ca as xkb_keysym_t,
            offset: 1898 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c9 as xkb_keysym_t,
            offset: 1909 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d0 as xkb_keysym_t,
            offset: 1927 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cb as xkb_keysym_t,
            offset: 1939 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000679 as xkb_keysym_t,
            offset: 1951 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006a4 as xkb_keysym_t,
            offset: 1963 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e8 as xkb_keysym_t,
            offset: 1974 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ea as xkb_keysym_t,
            offset: 1985 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006d2 as xkb_keysym_t,
            offset: 1996 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d8 as xkb_keysym_t,
            offset: 2013 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d2 as xkb_keysym_t,
            offset: 2024 as uint16_t,
        },
        name_keysym {
            keysym: 0xc5 as xkb_keysym_t,
            offset: 2036 as uint16_t,
        },
        name_keysym {
            keysym: 0xe5 as xkb_keysym_t,
            offset: 2042 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055b as xkb_keysym_t,
            offset: 2048 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055c as xkb_keysym_t,
            offset: 2064 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055a as xkb_keysym_t,
            offset: 2080 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000538 as xkb_keysym_t,
            offset: 2100 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000568 as xkb_keysym_t,
            offset: 2112 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000531 as xkb_keysym_t,
            offset: 2124 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000561 as xkb_keysym_t,
            offset: 2137 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000532 as xkb_keysym_t,
            offset: 2150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000562 as xkb_keysym_t,
            offset: 2163 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055d as xkb_keysym_t,
            offset: 2176 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000549 as xkb_keysym_t,
            offset: 2189 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000579 as xkb_keysym_t,
            offset: 2202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000534 as xkb_keysym_t,
            offset: 2215 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000564 as xkb_keysym_t,
            offset: 2227 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000541 as xkb_keysym_t,
            offset: 2239 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000571 as xkb_keysym_t,
            offset: 2252 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000537 as xkb_keysym_t,
            offset: 2265 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000567 as xkb_keysym_t,
            offset: 2276 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055c as xkb_keysym_t,
            offset: 2287 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000556 as xkb_keysym_t,
            offset: 2303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000586 as xkb_keysym_t,
            offset: 2315 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000589 as xkb_keysym_t,
            offset: 2327 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000542 as xkb_keysym_t,
            offset: 2346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000572 as xkb_keysym_t,
            offset: 2360 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000533 as xkb_keysym_t,
            offset: 2374 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000563 as xkb_keysym_t,
            offset: 2387 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000545 as xkb_keysym_t,
            offset: 2400 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000575 as xkb_keysym_t,
            offset: 2412 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000540 as xkb_keysym_t,
            offset: 2424 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000570 as xkb_keysym_t,
            offset: 2436 as uint16_t,
        },
        name_keysym {
            keysym: 0x100058a as xkb_keysym_t,
            offset: 2448 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053b as xkb_keysym_t,
            offset: 2464 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056b as xkb_keysym_t,
            offset: 2477 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054b as xkb_keysym_t,
            offset: 2490 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057b as xkb_keysym_t,
            offset: 2502 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000554 as xkb_keysym_t,
            offset: 2514 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000584 as xkb_keysym_t,
            offset: 2526 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053f as xkb_keysym_t,
            offset: 2538 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056f as xkb_keysym_t,
            offset: 2551 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053d as xkb_keysym_t,
            offset: 2564 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056d as xkb_keysym_t,
            offset: 2577 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000587 as xkb_keysym_t,
            offset: 2590 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053c as xkb_keysym_t,
            offset: 2611 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056c as xkb_keysym_t,
            offset: 2625 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000544 as xkb_keysym_t,
            offset: 2639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000574 as xkb_keysym_t,
            offset: 2652 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000546 as xkb_keysym_t,
            offset: 2665 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000576 as xkb_keysym_t,
            offset: 2677 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000555 as xkb_keysym_t,
            offset: 2689 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000585 as xkb_keysym_t,
            offset: 2700 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055e as xkb_keysym_t,
            offset: 2711 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054a as xkb_keysym_t,
            offset: 2727 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057a as xkb_keysym_t,
            offset: 2739 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000553 as xkb_keysym_t,
            offset: 2751 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000583 as xkb_keysym_t,
            offset: 2765 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055e as xkb_keysym_t,
            offset: 2779 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054c as xkb_keysym_t,
            offset: 2797 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057c as xkb_keysym_t,
            offset: 2809 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000550 as xkb_keysym_t,
            offset: 2821 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000580 as xkb_keysym_t,
            offset: 2833 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054d as xkb_keysym_t,
            offset: 2845 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057d as xkb_keysym_t,
            offset: 2857 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055d as xkb_keysym_t,
            offset: 2869 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000547 as xkb_keysym_t,
            offset: 2894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000577 as xkb_keysym_t,
            offset: 2907 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055b as xkb_keysym_t,
            offset: 2920 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000543 as xkb_keysym_t,
            offset: 2936 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000573 as xkb_keysym_t,
            offset: 2950 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000539 as xkb_keysym_t,
            offset: 2964 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000569 as xkb_keysym_t,
            offset: 2976 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053e as xkb_keysym_t,
            offset: 2988 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056e as xkb_keysym_t,
            offset: 3001 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000551 as xkb_keysym_t,
            offset: 3014 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000581 as xkb_keysym_t,
            offset: 3027 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054f as xkb_keysym_t,
            offset: 3040 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057f as xkb_keysym_t,
            offset: 3054 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000589 as xkb_keysym_t,
            offset: 3068 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054e as xkb_keysym_t,
            offset: 3086 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057e as xkb_keysym_t,
            offset: 3099 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000548 as xkb_keysym_t,
            offset: 3112 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000578 as xkb_keysym_t,
            offset: 3124 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000552 as xkb_keysym_t,
            offset: 3136 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000582 as xkb_keysym_t,
            offset: 3150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000535 as xkb_keysym_t,
            offset: 3164 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000565 as xkb_keysym_t,
            offset: 3178 as uint16_t,
        },
        name_keysym {
            keysym: 0x100058a as xkb_keysym_t,
            offset: 3192 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000536 as xkb_keysym_t,
            offset: 3210 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000566 as xkb_keysym_t,
            offset: 3222 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053a as xkb_keysym_t,
            offset: 3234 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056a as xkb_keysym_t,
            offset: 3247 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e as xkb_keysym_t,
            offset: 3260 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e as xkb_keysym_t,
            offset: 3272 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a as xkb_keysym_t,
            offset: 3283 as uint16_t,
        },
        name_keysym {
            keysym: 0x40 as xkb_keysym_t,
            offset: 3292 as uint16_t,
        },
        name_keysym {
            keysym: 0xc3 as xkb_keysym_t,
            offset: 3295 as uint16_t,
        },
        name_keysym {
            keysym: 0xe3 as xkb_keysym_t,
            offset: 3302 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe7a as xkb_keysym_t,
            offset: 3309 as uint16_t,
        },
        name_keysym {
            keysym: 0x42 as xkb_keysym_t,
            offset: 3328 as uint16_t,
        },
        name_keysym {
            keysym: 0x62 as xkb_keysym_t,
            offset: 3330 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e02 as xkb_keysym_t,
            offset: 3332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e03 as xkb_keysym_t,
            offset: 3342 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c as xkb_keysym_t,
            offset: 3352 as uint16_t,
        },
        name_keysym {
            keysym: 0xff08 as xkb_keysym_t,
            offset: 3362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff74 as xkb_keysym_t,
            offset: 3372 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf4 as xkb_keysym_t,
            offset: 3380 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c as xkb_keysym_t,
            offset: 3392 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002235 as xkb_keysym_t,
            offset: 3396 as uint16_t,
        },
        name_keysym {
            keysym: 0xff58 as xkb_keysym_t,
            offset: 3404 as uint16_t,
        },
        name_keysym {
            keysym: 0x9df as xkb_keysym_t,
            offset: 3410 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000fc as xkb_keysym_t,
            offset: 3416 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a5 as xkb_keysym_t,
            offset: 3422 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ac as xkb_keysym_t,
            offset: 3434 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a8 as xkb_keysym_t,
            offset: 3448 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b2 as xkb_keysym_t,
            offset: 3465 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ae as xkb_keysym_t,
            offset: 3482 as uint16_t,
        },
        name_keysym {
            keysym: 0x8aa as xkb_keysym_t,
            offset: 3497 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b6 as xkb_keysym_t,
            offset: 3515 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f6 as xkb_keysym_t,
            offset: 3533 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b4 as xkb_keysym_t,
            offset: 3538 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe74 as xkb_keysym_t,
            offset: 3564 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b as xkb_keysym_t,
            offset: 3582 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d as xkb_keysym_t,
            offset: 3592 as uint16_t,
        },
        name_keysym {
            keysym: 0x5b as xkb_keysym_t,
            offset: 3603 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d as xkb_keysym_t,
            offset: 3615 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002800 as xkb_keysym_t,
            offset: 3628 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff1 as xkb_keysym_t,
            offset: 3642 as uint16_t,
        },
        name_keysym {
            keysym: 0xfffa as xkb_keysym_t,
            offset: 3656 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff2 as xkb_keysym_t,
            offset: 3671 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff3 as xkb_keysym_t,
            offset: 3685 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff4 as xkb_keysym_t,
            offset: 3699 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff5 as xkb_keysym_t,
            offset: 3713 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff6 as xkb_keysym_t,
            offset: 3727 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff7 as xkb_keysym_t,
            offset: 3741 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff8 as xkb_keysym_t,
            offset: 3755 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff9 as xkb_keysym_t,
            offset: 3769 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002801 as xkb_keysym_t,
            offset: 3783 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002803 as xkb_keysym_t,
            offset: 3798 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002807 as xkb_keysym_t,
            offset: 3814 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280f as xkb_keysym_t,
            offset: 3831 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281f as xkb_keysym_t,
            offset: 3849 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283f as xkb_keysym_t,
            offset: 3868 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287f as xkb_keysym_t,
            offset: 3888 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ff as xkb_keysym_t,
            offset: 3909 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bf as xkb_keysym_t,
            offset: 3931 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285f as xkb_keysym_t,
            offset: 3952 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028df as xkb_keysym_t,
            offset: 3972 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289f as xkb_keysym_t,
            offset: 3993 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282f as xkb_keysym_t,
            offset: 4013 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286f as xkb_keysym_t,
            offset: 4032 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ef as xkb_keysym_t,
            offset: 4052 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028af as xkb_keysym_t,
            offset: 4073 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284f as xkb_keysym_t,
            offset: 4093 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cf as xkb_keysym_t,
            offset: 4112 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288f as xkb_keysym_t,
            offset: 4132 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002817 as xkb_keysym_t,
            offset: 4151 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002837 as xkb_keysym_t,
            offset: 4169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002877 as xkb_keysym_t,
            offset: 4188 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f7 as xkb_keysym_t,
            offset: 4208 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b7 as xkb_keysym_t,
            offset: 4229 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002857 as xkb_keysym_t,
            offset: 4249 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d7 as xkb_keysym_t,
            offset: 4268 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002897 as xkb_keysym_t,
            offset: 4288 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002827 as xkb_keysym_t,
            offset: 4307 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002867 as xkb_keysym_t,
            offset: 4325 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e7 as xkb_keysym_t,
            offset: 4344 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a7 as xkb_keysym_t,
            offset: 4364 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002847 as xkb_keysym_t,
            offset: 4383 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c7 as xkb_keysym_t,
            offset: 4401 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002887 as xkb_keysym_t,
            offset: 4420 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280b as xkb_keysym_t,
            offset: 4438 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281b as xkb_keysym_t,
            offset: 4455 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283b as xkb_keysym_t,
            offset: 4473 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287b as xkb_keysym_t,
            offset: 4492 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fb as xkb_keysym_t,
            offset: 4512 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bb as xkb_keysym_t,
            offset: 4533 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285b as xkb_keysym_t,
            offset: 4553 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028db as xkb_keysym_t,
            offset: 4572 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289b as xkb_keysym_t,
            offset: 4592 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282b as xkb_keysym_t,
            offset: 4611 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286b as xkb_keysym_t,
            offset: 4629 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028eb as xkb_keysym_t,
            offset: 4648 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ab as xkb_keysym_t,
            offset: 4668 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284b as xkb_keysym_t,
            offset: 4687 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cb as xkb_keysym_t,
            offset: 4705 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288b as xkb_keysym_t,
            offset: 4724 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002813 as xkb_keysym_t,
            offset: 4742 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002833 as xkb_keysym_t,
            offset: 4759 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002873 as xkb_keysym_t,
            offset: 4777 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f3 as xkb_keysym_t,
            offset: 4796 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b3 as xkb_keysym_t,
            offset: 4816 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002853 as xkb_keysym_t,
            offset: 4835 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d3 as xkb_keysym_t,
            offset: 4853 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002893 as xkb_keysym_t,
            offset: 4872 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002823 as xkb_keysym_t,
            offset: 4890 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002863 as xkb_keysym_t,
            offset: 4907 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e3 as xkb_keysym_t,
            offset: 4925 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a3 as xkb_keysym_t,
            offset: 4944 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002843 as xkb_keysym_t,
            offset: 4962 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c3 as xkb_keysym_t,
            offset: 4979 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002883 as xkb_keysym_t,
            offset: 4997 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002805 as xkb_keysym_t,
            offset: 5014 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280d as xkb_keysym_t,
            offset: 5030 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281d as xkb_keysym_t,
            offset: 5047 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283d as xkb_keysym_t,
            offset: 5065 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287d as xkb_keysym_t,
            offset: 5084 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fd as xkb_keysym_t,
            offset: 5104 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bd as xkb_keysym_t,
            offset: 5125 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285d as xkb_keysym_t,
            offset: 5145 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028dd as xkb_keysym_t,
            offset: 5164 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289d as xkb_keysym_t,
            offset: 5184 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282d as xkb_keysym_t,
            offset: 5203 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286d as xkb_keysym_t,
            offset: 5221 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ed as xkb_keysym_t,
            offset: 5240 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ad as xkb_keysym_t,
            offset: 5260 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284d as xkb_keysym_t,
            offset: 5279 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cd as xkb_keysym_t,
            offset: 5297 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288d as xkb_keysym_t,
            offset: 5316 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002815 as xkb_keysym_t,
            offset: 5334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002835 as xkb_keysym_t,
            offset: 5351 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002875 as xkb_keysym_t,
            offset: 5369 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f5 as xkb_keysym_t,
            offset: 5388 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b5 as xkb_keysym_t,
            offset: 5408 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002855 as xkb_keysym_t,
            offset: 5427 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d5 as xkb_keysym_t,
            offset: 5445 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002895 as xkb_keysym_t,
            offset: 5464 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002825 as xkb_keysym_t,
            offset: 5482 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002865 as xkb_keysym_t,
            offset: 5499 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e5 as xkb_keysym_t,
            offset: 5517 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a5 as xkb_keysym_t,
            offset: 5536 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002845 as xkb_keysym_t,
            offset: 5554 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c5 as xkb_keysym_t,
            offset: 5571 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002885 as xkb_keysym_t,
            offset: 5589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002809 as xkb_keysym_t,
            offset: 5606 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002819 as xkb_keysym_t,
            offset: 5622 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002839 as xkb_keysym_t,
            offset: 5639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002879 as xkb_keysym_t,
            offset: 5657 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f9 as xkb_keysym_t,
            offset: 5676 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b9 as xkb_keysym_t,
            offset: 5696 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002859 as xkb_keysym_t,
            offset: 5715 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d9 as xkb_keysym_t,
            offset: 5733 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002899 as xkb_keysym_t,
            offset: 5752 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002829 as xkb_keysym_t,
            offset: 5770 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002869 as xkb_keysym_t,
            offset: 5787 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e9 as xkb_keysym_t,
            offset: 5805 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a9 as xkb_keysym_t,
            offset: 5824 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002849 as xkb_keysym_t,
            offset: 5842 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c9 as xkb_keysym_t,
            offset: 5859 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002889 as xkb_keysym_t,
            offset: 5877 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002811 as xkb_keysym_t,
            offset: 5894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002831 as xkb_keysym_t,
            offset: 5910 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002871 as xkb_keysym_t,
            offset: 5927 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f1 as xkb_keysym_t,
            offset: 5945 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b1 as xkb_keysym_t,
            offset: 5964 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002851 as xkb_keysym_t,
            offset: 5982 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d1 as xkb_keysym_t,
            offset: 5999 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002891 as xkb_keysym_t,
            offset: 6017 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002821 as xkb_keysym_t,
            offset: 6034 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002861 as xkb_keysym_t,
            offset: 6050 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e1 as xkb_keysym_t,
            offset: 6067 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a1 as xkb_keysym_t,
            offset: 6085 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002841 as xkb_keysym_t,
            offset: 6102 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c1 as xkb_keysym_t,
            offset: 6118 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002881 as xkb_keysym_t,
            offset: 6135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002802 as xkb_keysym_t,
            offset: 6151 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002806 as xkb_keysym_t,
            offset: 6166 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280e as xkb_keysym_t,
            offset: 6182 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281e as xkb_keysym_t,
            offset: 6199 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283e as xkb_keysym_t,
            offset: 6217 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287e as xkb_keysym_t,
            offset: 6236 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fe as xkb_keysym_t,
            offset: 6256 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028be as xkb_keysym_t,
            offset: 6277 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285e as xkb_keysym_t,
            offset: 6297 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028de as xkb_keysym_t,
            offset: 6316 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289e as xkb_keysym_t,
            offset: 6336 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282e as xkb_keysym_t,
            offset: 6355 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286e as xkb_keysym_t,
            offset: 6373 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ee as xkb_keysym_t,
            offset: 6392 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ae as xkb_keysym_t,
            offset: 6412 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284e as xkb_keysym_t,
            offset: 6431 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ce as xkb_keysym_t,
            offset: 6449 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288e as xkb_keysym_t,
            offset: 6468 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002816 as xkb_keysym_t,
            offset: 6486 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002836 as xkb_keysym_t,
            offset: 6503 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002876 as xkb_keysym_t,
            offset: 6521 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f6 as xkb_keysym_t,
            offset: 6540 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b6 as xkb_keysym_t,
            offset: 6560 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002856 as xkb_keysym_t,
            offset: 6579 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d6 as xkb_keysym_t,
            offset: 6597 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002896 as xkb_keysym_t,
            offset: 6616 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002826 as xkb_keysym_t,
            offset: 6634 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002866 as xkb_keysym_t,
            offset: 6651 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e6 as xkb_keysym_t,
            offset: 6669 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a6 as xkb_keysym_t,
            offset: 6688 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002846 as xkb_keysym_t,
            offset: 6706 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c6 as xkb_keysym_t,
            offset: 6723 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002886 as xkb_keysym_t,
            offset: 6741 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280a as xkb_keysym_t,
            offset: 6758 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281a as xkb_keysym_t,
            offset: 6774 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283a as xkb_keysym_t,
            offset: 6791 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287a as xkb_keysym_t,
            offset: 6809 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fa as xkb_keysym_t,
            offset: 6828 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ba as xkb_keysym_t,
            offset: 6848 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285a as xkb_keysym_t,
            offset: 6867 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028da as xkb_keysym_t,
            offset: 6885 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289a as xkb_keysym_t,
            offset: 6904 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282a as xkb_keysym_t,
            offset: 6922 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286a as xkb_keysym_t,
            offset: 6939 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ea as xkb_keysym_t,
            offset: 6957 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028aa as xkb_keysym_t,
            offset: 6976 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284a as xkb_keysym_t,
            offset: 6994 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ca as xkb_keysym_t,
            offset: 7011 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288a as xkb_keysym_t,
            offset: 7029 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002812 as xkb_keysym_t,
            offset: 7046 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002832 as xkb_keysym_t,
            offset: 7062 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002872 as xkb_keysym_t,
            offset: 7079 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f2 as xkb_keysym_t,
            offset: 7097 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b2 as xkb_keysym_t,
            offset: 7116 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002852 as xkb_keysym_t,
            offset: 7134 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d2 as xkb_keysym_t,
            offset: 7151 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002892 as xkb_keysym_t,
            offset: 7169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002822 as xkb_keysym_t,
            offset: 7186 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002862 as xkb_keysym_t,
            offset: 7202 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e2 as xkb_keysym_t,
            offset: 7219 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a2 as xkb_keysym_t,
            offset: 7237 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002842 as xkb_keysym_t,
            offset: 7254 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c2 as xkb_keysym_t,
            offset: 7270 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002882 as xkb_keysym_t,
            offset: 7287 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002804 as xkb_keysym_t,
            offset: 7303 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280c as xkb_keysym_t,
            offset: 7318 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281c as xkb_keysym_t,
            offset: 7334 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283c as xkb_keysym_t,
            offset: 7351 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287c as xkb_keysym_t,
            offset: 7369 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fc as xkb_keysym_t,
            offset: 7388 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bc as xkb_keysym_t,
            offset: 7408 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285c as xkb_keysym_t,
            offset: 7427 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028dc as xkb_keysym_t,
            offset: 7445 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289c as xkb_keysym_t,
            offset: 7464 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282c as xkb_keysym_t,
            offset: 7482 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286c as xkb_keysym_t,
            offset: 7499 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ec as xkb_keysym_t,
            offset: 7517 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ac as xkb_keysym_t,
            offset: 7536 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284c as xkb_keysym_t,
            offset: 7554 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cc as xkb_keysym_t,
            offset: 7571 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288c as xkb_keysym_t,
            offset: 7589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002814 as xkb_keysym_t,
            offset: 7606 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002834 as xkb_keysym_t,
            offset: 7622 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002874 as xkb_keysym_t,
            offset: 7639 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f4 as xkb_keysym_t,
            offset: 7657 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b4 as xkb_keysym_t,
            offset: 7676 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002854 as xkb_keysym_t,
            offset: 7694 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d4 as xkb_keysym_t,
            offset: 7711 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002894 as xkb_keysym_t,
            offset: 7729 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002824 as xkb_keysym_t,
            offset: 7746 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002864 as xkb_keysym_t,
            offset: 7762 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e4 as xkb_keysym_t,
            offset: 7779 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a4 as xkb_keysym_t,
            offset: 7797 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002844 as xkb_keysym_t,
            offset: 7814 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c4 as xkb_keysym_t,
            offset: 7830 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002884 as xkb_keysym_t,
            offset: 7847 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002808 as xkb_keysym_t,
            offset: 7863 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002818 as xkb_keysym_t,
            offset: 7878 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002838 as xkb_keysym_t,
            offset: 7894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002878 as xkb_keysym_t,
            offset: 7911 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f8 as xkb_keysym_t,
            offset: 7929 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b8 as xkb_keysym_t,
            offset: 7948 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002858 as xkb_keysym_t,
            offset: 7966 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d8 as xkb_keysym_t,
            offset: 7983 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002898 as xkb_keysym_t,
            offset: 8001 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002828 as xkb_keysym_t,
            offset: 8018 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002868 as xkb_keysym_t,
            offset: 8034 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e8 as xkb_keysym_t,
            offset: 8051 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a8 as xkb_keysym_t,
            offset: 8069 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002848 as xkb_keysym_t,
            offset: 8086 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c8 as xkb_keysym_t,
            offset: 8102 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002888 as xkb_keysym_t,
            offset: 8119 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002810 as xkb_keysym_t,
            offset: 8135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002830 as xkb_keysym_t,
            offset: 8150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002870 as xkb_keysym_t,
            offset: 8166 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f0 as xkb_keysym_t,
            offset: 8183 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b0 as xkb_keysym_t,
            offset: 8201 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002850 as xkb_keysym_t,
            offset: 8218 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d0 as xkb_keysym_t,
            offset: 8234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002890 as xkb_keysym_t,
            offset: 8251 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002820 as xkb_keysym_t,
            offset: 8267 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002860 as xkb_keysym_t,
            offset: 8282 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e0 as xkb_keysym_t,
            offset: 8298 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a0 as xkb_keysym_t,
            offset: 8315 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002840 as xkb_keysym_t,
            offset: 8331 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c0 as xkb_keysym_t,
            offset: 8346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002880 as xkb_keysym_t,
            offset: 8362 as uint16_t,
        },
        name_keysym {
            keysym: 0xff6b as xkb_keysym_t,
            offset: 8377 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a2 as xkb_keysym_t,
            offset: 8383 as uint16_t,
        },
        name_keysym {
            keysym: 0xa6 as xkb_keysym_t,
            offset: 8389 as uint16_t,
        },
        name_keysym {
            keysym: 0x6be as xkb_keysym_t,
            offset: 8399 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ae as xkb_keysym_t,
            offset: 8419 as uint16_t,
        },
        name_keysym {
            keysym: 0x43 as xkb_keysym_t,
            offset: 8439 as uint16_t,
        },
        name_keysym {
            keysym: 0x63 as xkb_keysym_t,
            offset: 8441 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea5 as xkb_keysym_t,
            offset: 8443 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea4 as xkb_keysym_t,
            offset: 8447 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea3 as xkb_keysym_t,
            offset: 8451 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c5 as xkb_keysym_t,
            offset: 8455 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e5 as xkb_keysym_t,
            offset: 8465 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c6 as xkb_keysym_t,
            offset: 8475 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e6 as xkb_keysym_t,
            offset: 8482 as uint16_t,
        },
        name_keysym {
            keysym: 0xff69 as xkb_keysym_t,
            offset: 8489 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe5 as xkb_keysym_t,
            offset: 8496 as uint16_t,
        },
        name_keysym {
            keysym: 0xab8 as xkb_keysym_t,
            offset: 8506 as uint16_t,
        },
        name_keysym {
            keysym: 0xafc as xkb_keysym_t,
            offset: 8513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b7 as xkb_keysym_t,
            offset: 8519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c8 as xkb_keysym_t,
            offset: 8525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e8 as xkb_keysym_t,
            offset: 8532 as uint16_t,
        },
        name_keysym {
            keysym: 0xc7 as xkb_keysym_t,
            offset: 8539 as uint16_t,
        },
        name_keysym {
            keysym: 0xe7 as xkb_keysym_t,
            offset: 8548 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c6 as xkb_keysym_t,
            offset: 8557 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e6 as xkb_keysym_t,
            offset: 8569 as uint16_t,
        },
        name_keysym {
            keysym: 0xb8 as xkb_keysym_t,
            offset: 8581 as uint16_t,
        },
        name_keysym {
            keysym: 0xa2 as xkb_keysym_t,
            offset: 8589 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea2 as xkb_keysym_t,
            offset: 8594 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea1 as xkb_keysym_t,
            offset: 8597 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea0 as xkb_keysym_t,
            offset: 8600 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e1 as xkb_keysym_t,
            offset: 8603 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf3 as xkb_keysym_t,
            offset: 8616 as uint16_t,
        },
        name_keysym {
            keysym: 0xbcf as xkb_keysym_t,
            offset: 8626 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0b as xkb_keysym_t,
            offset: 8633 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6f as xkb_keysym_t,
            offset: 8639 as uint16_t,
        },
        name_keysym {
            keysym: 0xaec as xkb_keysym_t,
            offset: 8649 as uint16_t,
        },
        name_keysym {
            keysym: 0xff37 as xkb_keysym_t,
            offset: 8654 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a as xkb_keysym_t,
            offset: 8664 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a1 as xkb_keysym_t,
            offset: 8670 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000301 as xkb_keysym_t,
            offset: 8680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000323 as xkb_keysym_t,
            offset: 8696 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000300 as xkb_keysym_t,
            offset: 8715 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000309 as xkb_keysym_t,
            offset: 8731 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000303 as xkb_keysym_t,
            offset: 8746 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c as xkb_keysym_t,
            offset: 8762 as uint16_t,
        },
        name_keysym {
            keysym: 0x100220b as xkb_keysym_t,
            offset: 8768 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe3 as xkb_keysym_t,
            offset: 8779 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe4 as xkb_keysym_t,
            offset: 8789 as uint16_t,
        },
        name_keysym {
            keysym: 0xa9 as xkb_keysym_t,
            offset: 8799 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e4 as xkb_keysym_t,
            offset: 8809 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ee as xkb_keysym_t,
            offset: 8812 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a2 as xkb_keysym_t,
            offset: 8826 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221b as xkb_keysym_t,
            offset: 8839 as uint16_t,
        },
        name_keysym {
            keysym: 0xa4 as xkb_keysym_t,
            offset: 8848 as uint16_t,
        },
        name_keysym {
            keysym: 0xaff as xkb_keysym_t,
            offset: 8857 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e1 as xkb_keysym_t,
            offset: 8864 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c1 as xkb_keysym_t,
            offset: 8875 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e2 as xkb_keysym_t,
            offset: 8886 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c2 as xkb_keysym_t,
            offset: 8898 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fe as xkb_keysym_t,
            offset: 8910 as uint16_t,
        },
        name_keysym {
            keysym: 0x6de as xkb_keysym_t,
            offset: 8923 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b6 as xkb_keysym_t,
            offset: 8936 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b7 as xkb_keysym_t,
            offset: 8959 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b8 as xkb_keysym_t,
            offset: 8982 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b9 as xkb_keysym_t,
            offset: 9006 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e4 as xkb_keysym_t,
            offset: 9030 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c4 as xkb_keysym_t,
            offset: 9042 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bf as xkb_keysym_t,
            offset: 9054 as uint16_t,
        },
        name_keysym {
            keysym: 0x6af as xkb_keysym_t,
            offset: 9068 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fc as xkb_keysym_t,
            offset: 9082 as uint16_t,
        },
        name_keysym {
            keysym: 0x6dc as xkb_keysym_t,
            offset: 9093 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e6 as xkb_keysym_t,
            offset: 9104 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c6 as xkb_keysym_t,
            offset: 9116 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ec as xkb_keysym_t,
            offset: 9128 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cc as xkb_keysym_t,
            offset: 9140 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ed as xkb_keysym_t,
            offset: 9152 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cd as xkb_keysym_t,
            offset: 9164 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ee as xkb_keysym_t,
            offset: 9176 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ce as xkb_keysym_t,
            offset: 9188 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004a2 as xkb_keysym_t,
            offset: 9200 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004a3 as xkb_keysym_t,
            offset: 9222 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f2 as xkb_keysym_t,
            offset: 9244 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d2 as xkb_keysym_t,
            offset: 9256 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f3 as xkb_keysym_t,
            offset: 9268 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d3 as xkb_keysym_t,
            offset: 9280 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e7 as xkb_keysym_t,
            offset: 9292 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c7 as xkb_keysym_t,
            offset: 9305 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000492 as xkb_keysym_t,
            offset: 9318 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000493 as xkb_keysym_t,
            offset: 9335 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e8 as xkb_keysym_t,
            offset: 9352 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c8 as xkb_keysym_t,
            offset: 9364 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b2 as xkb_keysym_t,
            offset: 9376 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b3 as xkb_keysym_t,
            offset: 9398 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ff as xkb_keysym_t,
            offset: 9420 as uint16_t,
        },
        name_keysym {
            keysym: 0x6df as xkb_keysym_t,
            offset: 9438 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e9 as xkb_keysym_t,
            offset: 9456 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c9 as xkb_keysym_t,
            offset: 9467 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e2 as xkb_keysym_t,
            offset: 9478 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e3 as xkb_keysym_t,
            offset: 9496 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e5 as xkb_keysym_t,
            offset: 9514 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c5 as xkb_keysym_t,
            offset: 9526 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b3 as xkb_keysym_t,
            offset: 9538 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a3 as xkb_keysym_t,
            offset: 9550 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b8 as xkb_keysym_t,
            offset: 9562 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a8 as xkb_keysym_t,
            offset: 9574 as uint16_t,
        },
        name_keysym {
            keysym: 0x6eb as xkb_keysym_t,
            offset: 9586 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cb as xkb_keysym_t,
            offset: 9598 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049a as xkb_keysym_t,
            offset: 9610 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049b as xkb_keysym_t,
            offset: 9632 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049c as xkb_keysym_t,
            offset: 9654 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049d as xkb_keysym_t,
            offset: 9677 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b9 as xkb_keysym_t,
            offset: 9700 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a9 as xkb_keysym_t,
            offset: 9713 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ba as xkb_keysym_t,
            offset: 9726 as uint16_t,
        },
        name_keysym {
            keysym: 0x6aa as xkb_keysym_t,
            offset: 9739 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ef as xkb_keysym_t,
            offset: 9752 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cf as xkb_keysym_t,
            offset: 9763 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e8 as xkb_keysym_t,
            offset: 9774 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e9 as xkb_keysym_t,
            offset: 9789 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f0 as xkb_keysym_t,
            offset: 9804 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d0 as xkb_keysym_t,
            offset: 9816 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004d8 as xkb_keysym_t,
            offset: 9828 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004d9 as xkb_keysym_t,
            offset: 9843 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fb as xkb_keysym_t,
            offset: 9858 as uint16_t,
        },
        name_keysym {
            keysym: 0x6db as xkb_keysym_t,
            offset: 9871 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fd as xkb_keysym_t,
            offset: 9884 as uint16_t,
        },
        name_keysym {
            keysym: 0x6dd as xkb_keysym_t,
            offset: 9899 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ba as xkb_keysym_t,
            offset: 9914 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004bb as xkb_keysym_t,
            offset: 9928 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ea as xkb_keysym_t,
            offset: 9942 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ca as xkb_keysym_t,
            offset: 9958 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f8 as xkb_keysym_t,
            offset: 9974 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d8 as xkb_keysym_t,
            offset: 9992 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f4 as xkb_keysym_t,
            offset: 10010 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d4 as xkb_keysym_t,
            offset: 10022 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e3 as xkb_keysym_t,
            offset: 10034 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c3 as xkb_keysym_t,
            offset: 10047 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f5 as xkb_keysym_t,
            offset: 10060 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d5 as xkb_keysym_t,
            offset: 10071 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ee as xkb_keysym_t,
            offset: 10082 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ef as xkb_keysym_t,
            offset: 10100 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ae as xkb_keysym_t,
            offset: 10118 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004af as xkb_keysym_t,
            offset: 10138 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b0 as xkb_keysym_t,
            offset: 10158 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b1 as xkb_keysym_t,
            offset: 10182 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f7 as xkb_keysym_t,
            offset: 10206 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d7 as xkb_keysym_t,
            offset: 10218 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f1 as xkb_keysym_t,
            offset: 10230 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d1 as xkb_keysym_t,
            offset: 10242 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f9 as xkb_keysym_t,
            offset: 10254 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d9 as xkb_keysym_t,
            offset: 10268 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e0 as xkb_keysym_t,
            offset: 10282 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c0 as xkb_keysym_t,
            offset: 10294 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fa as xkb_keysym_t,
            offset: 10306 as uint16_t,
        },
        name_keysym {
            keysym: 0x6da as xkb_keysym_t,
            offset: 10318 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f6 as xkb_keysym_t,
            offset: 10330 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d6 as xkb_keysym_t,
            offset: 10343 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000496 as xkb_keysym_t,
            offset: 10356 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000497 as xkb_keysym_t,
            offset: 10379 as uint16_t,
        },
        name_keysym {
            keysym: 0x44 as xkb_keysym_t,
            offset: 10402 as uint16_t,
        },
        name_keysym {
            keysym: 0x64 as xkb_keysym_t,
            offset: 10404 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e0a as xkb_keysym_t,
            offset: 10406 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e0b as xkb_keysym_t,
            offset: 10416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe27 as xkb_keysym_t,
            offset: 10426 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf1 as xkb_keysym_t,
            offset: 10440 as uint16_t,
        },
        name_keysym {
            keysym: 0x1cf as xkb_keysym_t,
            offset: 10447 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ef as xkb_keysym_t,
            offset: 10454 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe2c as xkb_keysym_t,
            offset: 10461 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe5e as xkb_keysym_t,
            offset: 10477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe22 as xkb_keysym_t,
            offset: 10496 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe81 as xkb_keysym_t,
            offset: 10507 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe80 as xkb_keysym_t,
            offset: 10514 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe64 as xkb_keysym_t,
            offset: 10521 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe56 as xkb_keysym_t,
            offset: 10537 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe65 as xkb_keysym_t,
            offset: 10551 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe58 as xkb_keysym_t,
            offset: 10575 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe91 as xkb_keysym_t,
            offset: 10590 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe51 as xkb_keysym_t,
            offset: 10613 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8e as xkb_keysym_t,
            offset: 10624 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6b as xkb_keysym_t,
            offset: 10640 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe69 as xkb_keysym_t,
            offset: 10656 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6e as xkb_keysym_t,
            offset: 10677 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6c as xkb_keysym_t,
            offset: 10693 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe60 as xkb_keysym_t,
            offset: 10713 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe68 as xkb_keysym_t,
            offset: 10727 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe67 as xkb_keysym_t,
            offset: 10744 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6a as xkb_keysym_t,
            offset: 10759 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe92 as xkb_keysym_t,
            offset: 10775 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe55 as xkb_keysym_t,
            offset: 10798 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8b as xkb_keysym_t,
            offset: 10809 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5a as xkb_keysym_t,
            offset: 10828 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5b as xkb_keysym_t,
            offset: 10839 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe52 as xkb_keysym_t,
            offset: 10852 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6f as xkb_keysym_t,
            offset: 10868 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe65 as xkb_keysym_t,
            offset: 10882 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe57 as xkb_keysym_t,
            offset: 10893 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe59 as xkb_keysym_t,
            offset: 10908 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe66 as xkb_keysym_t,
            offset: 10925 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe83 as xkb_keysym_t,
            offset: 10942 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe82 as xkb_keysym_t,
            offset: 10949 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe50 as xkb_keysym_t,
            offset: 10956 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8c as xkb_keysym_t,
            offset: 10967 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8d as xkb_keysym_t,
            offset: 10978 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe61 as xkb_keysym_t,
            offset: 10989 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe62 as xkb_keysym_t,
            offset: 10999 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe85 as xkb_keysym_t,
            offset: 11009 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe84 as xkb_keysym_t,
            offset: 11016 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6d as xkb_keysym_t,
            offset: 11023 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5d as xkb_keysym_t,
            offset: 11042 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe93 as xkb_keysym_t,
            offset: 11052 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe90 as xkb_keysym_t,
            offset: 11076 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe54 as xkb_keysym_t,
            offset: 11089 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe87 as xkb_keysym_t,
            offset: 11101 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe86 as xkb_keysym_t,
            offset: 11108 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5c as xkb_keysym_t,
            offset: 11115 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe53 as xkb_keysym_t,
            offset: 11127 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe64 as xkb_keysym_t,
            offset: 11144 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8b as xkb_keysym_t,
            offset: 11155 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8a as xkb_keysym_t,
            offset: 11166 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5f as xkb_keysym_t,
            offset: 11177 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8a as xkb_keysym_t,
            offset: 11199 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe63 as xkb_keysym_t,
            offset: 11216 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe53 as xkb_keysym_t,
            offset: 11228 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe89 as xkb_keysym_t,
            offset: 11239 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe88 as xkb_keysym_t,
            offset: 11246 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5e as xkb_keysym_t,
            offset: 11253 as uint16_t,
        },
        name_keysym {
            keysym: 0xabd as xkb_keysym_t,
            offset: 11271 as uint16_t,
        },
        name_keysym {
            keysym: 0xb0 as xkb_keysym_t,
            offset: 11284 as uint16_t,
        },
        name_keysym {
            keysym: 0xffff as xkb_keysym_t,
            offset: 11291 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff73 as xkb_keysym_t,
            offset: 11298 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff71 as xkb_keysym_t,
            offset: 11309 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe60 as xkb_keysym_t,
            offset: 11320 as uint16_t,
        },
        name_keysym {
            keysym: 0xa8 as xkb_keysym_t,
            offset: 11334 as uint16_t,
        },
        name_keysym {
            keysym: 0xaed as xkb_keysym_t,
            offset: 11344 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa5 as xkb_keysym_t,
            offset: 11352 as uint16_t,
        },
        name_keysym {
            keysym: 0x100222c as xkb_keysym_t,
            offset: 11363 as uint16_t,
        },
        name_keysym {
            keysym: 0xf7 as xkb_keysym_t,
            offset: 11373 as uint16_t,
        },
        name_keysym {
            keysym: 0x24 as xkb_keysym_t,
            offset: 11382 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020ab as xkb_keysym_t,
            offset: 11389 as uint16_t,
        },
        name_keysym {
            keysym: 0xaaf as xkb_keysym_t,
            offset: 11398 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bd as xkb_keysym_t,
            offset: 11414 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf2 as xkb_keysym_t,
            offset: 11426 as uint16_t,
        },
        name_keysym {
            keysym: 0xafe as xkb_keysym_t,
            offset: 11439 as uint16_t,
        },
        name_keysym {
            keysym: 0xff54 as xkb_keysym_t,
            offset: 11458 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fe as xkb_keysym_t,
            offset: 11463 as uint16_t,
        },
        name_keysym {
            keysym: 0xba8 as xkb_keysym_t,
            offset: 11473 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd6 as xkb_keysym_t,
            offset: 11483 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc4 as xkb_keysym_t,
            offset: 11492 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc2 as xkb_keysym_t,
            offset: 11502 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff00 as xkb_keysym_t,
            offset: 11511 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000feb0 as xkb_keysym_t,
            offset: 11519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d0 as xkb_keysym_t,
            offset: 11532 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f0 as xkb_keysym_t,
            offset: 11540 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe7e as xkb_keysym_t,
            offset: 11548 as uint16_t,
        },
        name_keysym {
            keysym: 0x45 as xkb_keysym_t,
            offset: 11555 as uint16_t,
        },
        name_keysym {
            keysym: 0x65 as xkb_keysym_t,
            offset: 11557 as uint16_t,
        },
        name_keysym {
            keysym: 0x3cc as xkb_keysym_t,
            offset: 11559 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ec as xkb_keysym_t,
            offset: 11569 as uint16_t,
        },
        name_keysym {
            keysym: 0xc9 as xkb_keysym_t,
            offset: 11579 as uint16_t,
        },
        name_keysym {
            keysym: 0xe9 as xkb_keysym_t,
            offset: 11586 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb8 as xkb_keysym_t,
            offset: 11593 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb9 as xkb_keysym_t,
            offset: 11603 as uint16_t,
        },
        name_keysym {
            keysym: 0x1cc as xkb_keysym_t,
            offset: 11613 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ec as xkb_keysym_t,
            offset: 11620 as uint16_t,
        },
        name_keysym {
            keysym: 0xca as xkb_keysym_t,
            offset: 11627 as uint16_t,
        },
        name_keysym {
            keysym: 0xea as xkb_keysym_t,
            offset: 11639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebe as xkb_keysym_t,
            offset: 11651 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebf as xkb_keysym_t,
            offset: 11668 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec6 as xkb_keysym_t,
            offset: 11685 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec7 as xkb_keysym_t,
            offset: 11705 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec0 as xkb_keysym_t,
            offset: 11725 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec1 as xkb_keysym_t,
            offset: 11742 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec2 as xkb_keysym_t,
            offset: 11759 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec3 as xkb_keysym_t,
            offset: 11775 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec4 as xkb_keysym_t,
            offset: 11791 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec5 as xkb_keysym_t,
            offset: 11808 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a0 as xkb_keysym_t,
            offset: 11825 as uint16_t,
        },
        name_keysym {
            keysym: 0xcb as xkb_keysym_t,
            offset: 11833 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb as xkb_keysym_t,
            offset: 11844 as uint16_t,
        },
        name_keysym {
            keysym: 0xc8 as xkb_keysym_t,
            offset: 11855 as uint16_t,
        },
        name_keysym {
            keysym: 0xe8 as xkb_keysym_t,
            offset: 11862 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eba as xkb_keysym_t,
            offset: 11869 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebb as xkb_keysym_t,
            offset: 11875 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002088 as xkb_keysym_t,
            offset: 11881 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002078 as xkb_keysym_t,
            offset: 11896 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2f as xkb_keysym_t,
            offset: 11910 as uint16_t,
        },
        name_keysym {
            keysym: 0xff30 as xkb_keysym_t,
            offset: 11921 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002208 as xkb_keysym_t,
            offset: 11933 as uint16_t,
        },
        name_keysym {
            keysym: 0xaae as xkb_keysym_t,
            offset: 11943 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa3 as xkb_keysym_t,
            offset: 11952 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa4 as xkb_keysym_t,
            offset: 11961 as uint16_t,
        },
        name_keysym {
            keysym: 0x3aa as xkb_keysym_t,
            offset: 11970 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ba as xkb_keysym_t,
            offset: 11978 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa9 as xkb_keysym_t,
            offset: 11986 as uint16_t,
        },
        name_keysym {
            keysym: 0xade as xkb_keysym_t,
            offset: 11993 as uint16_t,
        },
        name_keysym {
            keysym: 0xadf as xkb_keysym_t,
            offset: 12008 as uint16_t,
        },
        name_keysym {
            keysym: 0xace as xkb_keysym_t,
            offset: 12021 as uint16_t,
        },
        name_keysym {
            keysym: 0xacf as xkb_keysym_t,
            offset: 12034 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002205 as xkb_keysym_t,
            offset: 12050 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa1 as xkb_keysym_t,
            offset: 12059 as uint16_t,
        },
        name_keysym {
            keysym: 0xff57 as xkb_keysym_t,
            offset: 12067 as uint16_t,
        },
        name_keysym {
            keysym: 0xaaa as xkb_keysym_t,
            offset: 12071 as uint16_t,
        },
        name_keysym {
            keysym: 0xae6 as xkb_keysym_t,
            offset: 12078 as uint16_t,
        },
        name_keysym {
            keysym: 0xae7 as xkb_keysym_t,
            offset: 12097 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bd as xkb_keysym_t,
            offset: 12114 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bf as xkb_keysym_t,
            offset: 12118 as uint16_t,
        },
        name_keysym {
            keysym: 0xae0 as xkb_keysym_t,
            offset: 12122 as uint16_t,
        },
        name_keysym {
            keysym: 0xae1 as xkb_keysym_t,
            offset: 12139 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa2 as xkb_keysym_t,
            offset: 12158 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ca as xkb_keysym_t,
            offset: 12166 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ea as xkb_keysym_t,
            offset: 12174 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d as xkb_keysym_t,
            offset: 12182 as uint16_t,
        },
        name_keysym {
            keysym: 0xff1b as xkb_keysym_t,
            offset: 12188 as uint16_t,
        },
        name_keysym {
            keysym: 0xd0 as xkb_keysym_t,
            offset: 12195 as uint16_t,
        },
        name_keysym {
            keysym: 0xd0 as xkb_keysym_t,
            offset: 12199 as uint16_t,
        },
        name_keysym {
            keysym: 0xf0 as xkb_keysym_t,
            offset: 12203 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebc as xkb_keysym_t,
            offset: 12207 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebd as xkb_keysym_t,
            offset: 12214 as uint16_t,
        },
        name_keysym {
            keysym: 0x20ac as xkb_keysym_t,
            offset: 12221 as uint16_t,
        },
        name_keysym {
            keysym: 0x21 as xkb_keysym_t,
            offset: 12230 as uint16_t,
        },
        name_keysym {
            keysym: 0xa1 as xkb_keysym_t,
            offset: 12237 as uint16_t,
        },
        name_keysym {
            keysym: 0xff62 as xkb_keysym_t,
            offset: 12248 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff76 as xkb_keysym_t,
            offset: 12256 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff77 as xkb_keysym_t,
            offset: 12267 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b7 as xkb_keysym_t,
            offset: 12278 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000292 as xkb_keysym_t,
            offset: 12282 as uint16_t,
        },
        name_keysym {
            keysym: 0x46 as xkb_keysym_t,
            offset: 12286 as uint16_t,
        },
        name_keysym {
            keysym: 0x66 as xkb_keysym_t,
            offset: 12288 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbe as xkb_keysym_t,
            offset: 12290 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc7 as xkb_keysym_t,
            offset: 12293 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc8 as xkb_keysym_t,
            offset: 12297 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc9 as xkb_keysym_t,
            offset: 12301 as uint16_t,
        },
        name_keysym {
            keysym: 0xffca as xkb_keysym_t,
            offset: 12305 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcb as xkb_keysym_t,
            offset: 12309 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcc as xkb_keysym_t,
            offset: 12313 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcd as xkb_keysym_t,
            offset: 12317 as uint16_t,
        },
        name_keysym {
            keysym: 0xffce as xkb_keysym_t,
            offset: 12321 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcf as xkb_keysym_t,
            offset: 12325 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd0 as xkb_keysym_t,
            offset: 12329 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbf as xkb_keysym_t,
            offset: 12333 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd1 as xkb_keysym_t,
            offset: 12336 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd2 as xkb_keysym_t,
            offset: 12340 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd3 as xkb_keysym_t,
            offset: 12344 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd4 as xkb_keysym_t,
            offset: 12348 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd5 as xkb_keysym_t,
            offset: 12352 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd6 as xkb_keysym_t,
            offset: 12356 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd7 as xkb_keysym_t,
            offset: 12360 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd8 as xkb_keysym_t,
            offset: 12364 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd9 as xkb_keysym_t,
            offset: 12368 as uint16_t,
        },
        name_keysym {
            keysym: 0xffda as xkb_keysym_t,
            offset: 12372 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc0 as xkb_keysym_t,
            offset: 12376 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdb as xkb_keysym_t,
            offset: 12379 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdc as xkb_keysym_t,
            offset: 12383 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdd as xkb_keysym_t,
            offset: 12387 as uint16_t,
        },
        name_keysym {
            keysym: 0xffde as xkb_keysym_t,
            offset: 12391 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdf as xkb_keysym_t,
            offset: 12395 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe0 as xkb_keysym_t,
            offset: 12399 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc1 as xkb_keysym_t,
            offset: 12403 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc2 as xkb_keysym_t,
            offset: 12406 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc3 as xkb_keysym_t,
            offset: 12409 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc4 as xkb_keysym_t,
            offset: 12412 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc5 as xkb_keysym_t,
            offset: 12415 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc6 as xkb_keysym_t,
            offset: 12418 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e1e as xkb_keysym_t,
            offset: 12421 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e1f as xkb_keysym_t,
            offset: 12431 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f0 as xkb_keysym_t,
            offset: 12441 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f1 as xkb_keysym_t,
            offset: 12449 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f2 as xkb_keysym_t,
            offset: 12457 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f3 as xkb_keysym_t,
            offset: 12465 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f4 as xkb_keysym_t,
            offset: 12473 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f5 as xkb_keysym_t,
            offset: 12481 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f6 as xkb_keysym_t,
            offset: 12489 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f7 as xkb_keysym_t,
            offset: 12497 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f8 as xkb_keysym_t,
            offset: 12505 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f9 as xkb_keysym_t,
            offset: 12513 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006cc as xkb_keysym_t,
            offset: 12521 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf8 as xkb_keysym_t,
            offset: 12531 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e3 as xkb_keysym_t,
            offset: 12544 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a3 as xkb_keysym_t,
            offset: 12547 as uint16_t,
        },
        name_keysym {
            keysym: 0xabb as xkb_keysym_t,
            offset: 12558 as uint16_t,
        },
        name_keysym {
            keysym: 0xadc as xkb_keysym_t,
            offset: 12566 as uint16_t,
        },
        name_keysym {
            keysym: 0xadb as xkb_keysym_t,
            offset: 12586 as uint16_t,
        },
        name_keysym {
            keysym: 0xadd as xkb_keysym_t,
            offset: 12603 as uint16_t,
        },
        name_keysym {
            keysym: 0xae9 as xkb_keysym_t,
            offset: 12624 as uint16_t,
        },
        name_keysym {
            keysym: 0xae8 as xkb_keysym_t,
            offset: 12644 as uint16_t,
        },
        name_keysym {
            keysym: 0xff68 as xkb_keysym_t,
            offset: 12662 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed0 as xkb_keysym_t,
            offset: 12667 as uint16_t,
        },
        name_keysym {
            keysym: 0xac5 as xkb_keysym_t,
            offset: 12688 as uint16_t,
        },
        name_keysym {
            keysym: 0xab7 as xkb_keysym_t,
            offset: 12700 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002085 as xkb_keysym_t,
            offset: 12711 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002075 as xkb_keysym_t,
            offset: 12725 as uint16_t,
        },
        name_keysym {
            keysym: 0xab5 as xkb_keysym_t,
            offset: 12738 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002084 as xkb_keysym_t,
            offset: 12749 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002074 as xkb_keysym_t,
            offset: 12763 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221c as xkb_keysym_t,
            offset: 12776 as uint16_t,
        },
        name_keysym {
            keysym: 0x8f6 as xkb_keysym_t,
            offset: 12787 as uint16_t,
        },
        name_keysym {
            keysym: 0x47 as xkb_keysym_t,
            offset: 12796 as uint16_t,
        },
        name_keysym {
            keysym: 0x67 as xkb_keysym_t,
            offset: 12798 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d5 as xkb_keysym_t,
            offset: 12800 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f5 as xkb_keysym_t,
            offset: 12810 as uint16_t,
        },
        name_keysym {
            keysym: 0x2ab as xkb_keysym_t,
            offset: 12820 as uint16_t,
        },
        name_keysym {
            keysym: 0x2bb as xkb_keysym_t,
            offset: 12827 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001e6 as xkb_keysym_t,
            offset: 12834 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001e7 as xkb_keysym_t,
            offset: 12841 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ab as xkb_keysym_t,
            offset: 12848 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bb as xkb_keysym_t,
            offset: 12857 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d8 as xkb_keysym_t,
            offset: 12866 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f8 as xkb_keysym_t,
            offset: 12878 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d0 as xkb_keysym_t,
            offset: 12890 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d1 as xkb_keysym_t,
            offset: 12902 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ea as xkb_keysym_t,
            offset: 12915 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ed as xkb_keysym_t,
            offset: 12928 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e9 as xkb_keysym_t,
            offset: 12942 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ec as xkb_keysym_t,
            offset: 12956 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d3 as xkb_keysym_t,
            offset: 12969 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d4 as xkb_keysym_t,
            offset: 12982 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f6 as xkb_keysym_t,
            offset: 12994 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d2 as xkb_keysym_t,
            offset: 13006 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e6 as xkb_keysym_t,
            offset: 13019 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f0 as xkb_keysym_t,
            offset: 13033 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f4 as xkb_keysym_t,
            offset: 13046 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f1 as xkb_keysym_t,
            offset: 13059 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f2 as xkb_keysym_t,
            offset: 13071 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f5 as xkb_keysym_t,
            offset: 13084 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d8 as xkb_keysym_t,
            offset: 13097 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ef as xkb_keysym_t,
            offset: 13109 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010eb as xkb_keysym_t,
            offset: 13123 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d9 as xkb_keysym_t,
            offset: 13136 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e5 as xkb_keysym_t,
            offset: 13149 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010da as xkb_keysym_t,
            offset: 13163 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010db as xkb_keysym_t,
            offset: 13176 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010dc as xkb_keysym_t,
            offset: 13189 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010dd as xkb_keysym_t,
            offset: 13202 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010de as xkb_keysym_t,
            offset: 13214 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e4 as xkb_keysym_t,
            offset: 13227 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e7 as xkb_keysym_t,
            offset: 13241 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e0 as xkb_keysym_t,
            offset: 13254 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e1 as xkb_keysym_t,
            offset: 13267 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e8 as xkb_keysym_t,
            offset: 13280 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d7 as xkb_keysym_t,
            offset: 13294 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e2 as xkb_keysym_t,
            offset: 13307 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e3 as xkb_keysym_t,
            offset: 13320 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d5 as xkb_keysym_t,
            offset: 13332 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f3 as xkb_keysym_t,
            offset: 13345 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ee as xkb_keysym_t,
            offset: 13357 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d6 as xkb_keysym_t,
            offset: 13370 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010df as xkb_keysym_t,
            offset: 13383 as uint16_t,
        },
        name_keysym {
            keysym: 0x60 as xkb_keysym_t,
            offset: 13397 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e as xkb_keysym_t,
            offset: 13403 as uint16_t,
        },
        name_keysym {
            keysym: 0x8be as xkb_keysym_t,
            offset: 13411 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ae as xkb_keysym_t,
            offset: 13428 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c1 as xkb_keysym_t,
            offset: 13449 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e1 as xkb_keysym_t,
            offset: 13461 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a1 as xkb_keysym_t,
            offset: 13473 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b1 as xkb_keysym_t,
            offset: 13491 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c2 as xkb_keysym_t,
            offset: 13509 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e2 as xkb_keysym_t,
            offset: 13520 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d7 as xkb_keysym_t,
            offset: 13531 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f7 as xkb_keysym_t,
            offset: 13541 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c4 as xkb_keysym_t,
            offset: 13551 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e4 as xkb_keysym_t,
            offset: 13563 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c5 as xkb_keysym_t,
            offset: 13575 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e5 as xkb_keysym_t,
            offset: 13589 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a2 as xkb_keysym_t,
            offset: 13603 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b2 as xkb_keysym_t,
            offset: 13623 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c7 as xkb_keysym_t,
            offset: 13643 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e7 as xkb_keysym_t,
            offset: 13653 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a3 as xkb_keysym_t,
            offset: 13663 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b3 as xkb_keysym_t,
            offset: 13679 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f3 as xkb_keysym_t,
            offset: 13695 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c3 as xkb_keysym_t,
            offset: 13717 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e3 as xkb_keysym_t,
            offset: 13729 as uint16_t,
        },
        name_keysym {
            keysym: 0x7af as xkb_keysym_t,
            offset: 13741 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c9 as xkb_keysym_t,
            offset: 13756 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e9 as xkb_keysym_t,
            offset: 13767 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a4 as xkb_keysym_t,
            offset: 13778 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b4 as xkb_keysym_t,
            offset: 13795 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b6 as xkb_keysym_t,
            offset: 13812 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a5 as xkb_keysym_t,
            offset: 13837 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a5 as xkb_keysym_t,
            offset: 13857 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b5 as xkb_keysym_t,
            offset: 13876 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ca as xkb_keysym_t,
            offset: 13895 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ea as xkb_keysym_t,
            offset: 13907 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cb as xkb_keysym_t,
            offset: 13919 as uint16_t,
        },
        name_keysym {
            keysym: 0x7eb as xkb_keysym_t,
            offset: 13932 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cb as xkb_keysym_t,
            offset: 13945 as uint16_t,
        },
        name_keysym {
            keysym: 0x7eb as xkb_keysym_t,
            offset: 13957 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cc as xkb_keysym_t,
            offset: 13969 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ec as xkb_keysym_t,
            offset: 13978 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cd as xkb_keysym_t,
            offset: 13987 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ed as xkb_keysym_t,
            offset: 13996 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d9 as xkb_keysym_t,
            offset: 14005 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f9 as xkb_keysym_t,
            offset: 14017 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ab as xkb_keysym_t,
            offset: 14029 as uint16_t,
        },
        name_keysym {
            keysym: 0x7bb as xkb_keysym_t,
            offset: 14047 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cf as xkb_keysym_t,
            offset: 14065 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ef as xkb_keysym_t,
            offset: 14079 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a7 as xkb_keysym_t,
            offset: 14093 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b7 as xkb_keysym_t,
            offset: 14113 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d6 as xkb_keysym_t,
            offset: 14133 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f6 as xkb_keysym_t,
            offset: 14143 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d0 as xkb_keysym_t,
            offset: 14153 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f0 as xkb_keysym_t,
            offset: 14162 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d8 as xkb_keysym_t,
            offset: 14171 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f8 as xkb_keysym_t,
            offset: 14181 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d1 as xkb_keysym_t,
            offset: 14191 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f1 as xkb_keysym_t,
            offset: 14201 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d2 as xkb_keysym_t,
            offset: 14211 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f2 as xkb_keysym_t,
            offset: 14223 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 14235 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d4 as xkb_keysym_t,
            offset: 14248 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f4 as xkb_keysym_t,
            offset: 14258 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c8 as xkb_keysym_t,
            offset: 14268 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e8 as xkb_keysym_t,
            offset: 14280 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d5 as xkb_keysym_t,
            offset: 14292 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f5 as xkb_keysym_t,
            offset: 14306 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a8 as xkb_keysym_t,
            offset: 14320 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b8 as xkb_keysym_t,
            offset: 14340 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ba as xkb_keysym_t,
            offset: 14360 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a9 as xkb_keysym_t,
            offset: 14388 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b9 as xkb_keysym_t,
            offset: 14410 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ce as xkb_keysym_t,
            offset: 14432 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ee as xkb_keysym_t,
            offset: 14441 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c6 as xkb_keysym_t,
            offset: 14450 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e6 as xkb_keysym_t,
            offset: 14461 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000be as xkb_keysym_t,
            offset: 14472 as uint16_t,
        },
        name_keysym {
            keysym: 0xab as xkb_keysym_t,
            offset: 14480 as uint16_t,
        },
        name_keysym {
            keysym: 0xbb as xkb_keysym_t,
            offset: 14494 as uint16_t,
        },
        name_keysym {
            keysym: 0xab as xkb_keysym_t,
            offset: 14509 as uint16_t,
        },
        name_keysym {
            keysym: 0xbb as xkb_keysym_t,
            offset: 14523 as uint16_t,
        },
        name_keysym {
            keysym: 0x48 as xkb_keysym_t,
            offset: 14538 as uint16_t,
        },
        name_keysym {
            keysym: 0x68 as xkb_keysym_t,
            offset: 14540 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa8 as xkb_keysym_t,
            offset: 14542 as uint16_t,
        },
        name_keysym {
            keysym: 0xff31 as xkb_keysym_t,
            offset: 14552 as uint16_t,
        },
        name_keysym {
            keysym: 0xebf as xkb_keysym_t,
            offset: 14559 as uint16_t,
        },
        name_keysym {
            keysym: 0xec0 as xkb_keysym_t,
            offset: 14568 as uint16_t,
        },
        name_keysym {
            keysym: 0xef6 as xkb_keysym_t,
            offset: 14578 as uint16_t,
        },
        name_keysym {
            keysym: 0xef7 as xkb_keysym_t,
            offset: 14591 as uint16_t,
        },
        name_keysym {
            keysym: 0xff39 as xkb_keysym_t,
            offset: 14605 as uint16_t,
        },
        name_keysym {
            keysym: 0xeba as xkb_keysym_t,
            offset: 14618 as uint16_t,
        },
        name_keysym {
            keysym: 0xff37 as xkb_keysym_t,
            offset: 14631 as uint16_t,
        },
        name_keysym {
            keysym: 0xea7 as xkb_keysym_t,
            offset: 14648 as uint16_t,
        },
        name_keysym {
            keysym: 0xec4 as xkb_keysym_t,
            offset: 14662 as uint16_t,
        },
        name_keysym {
            keysym: 0xff33 as xkb_keysym_t,
            offset: 14671 as uint16_t,
        },
        name_keysym {
            keysym: 0xec3 as xkb_keysym_t,
            offset: 14682 as uint16_t,
        },
        name_keysym {
            keysym: 0xed1 as xkb_keysym_t,
            offset: 14692 as uint16_t,
        },
        name_keysym {
            keysym: 0xff34 as xkb_keysym_t,
            offset: 14702 as uint16_t,
        },
        name_keysym {
            keysym: 0xebe as xkb_keysym_t,
            offset: 14715 as uint16_t,
        },
        name_keysym {
            keysym: 0xed3 as xkb_keysym_t,
            offset: 14728 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb7 as xkb_keysym_t,
            offset: 14737 as uint16_t,
        },
        name_keysym {
            keysym: 0xeea as xkb_keysym_t,
            offset: 14750 as uint16_t,
        },
        name_keysym {
            keysym: 0xeda as xkb_keysym_t,
            offset: 14765 as uint16_t,
        },
        name_keysym {
            keysym: 0xeee as xkb_keysym_t,
            offset: 14781 as uint16_t,
        },
        name_keysym {
            keysym: 0xee8 as xkb_keysym_t,
            offset: 14796 as uint16_t,
        },
        name_keysym {
            keysym: 0xee9 as xkb_keysym_t,
            offset: 14811 as uint16_t,
        },
        name_keysym {
            keysym: 0xeeb as xkb_keysym_t,
            offset: 14826 as uint16_t,
        },
        name_keysym {
            keysym: 0xed4 as xkb_keysym_t,
            offset: 14842 as uint16_t,
        },
        name_keysym {
            keysym: 0xed6 as xkb_keysym_t,
            offset: 14858 as uint16_t,
        },
        name_keysym {
            keysym: 0xef9 as xkb_keysym_t,
            offset: 14878 as uint16_t,
        },
        name_keysym {
            keysym: 0xee3 as xkb_keysym_t,
            offset: 14905 as uint16_t,
        },
        name_keysym {
            keysym: 0xed7 as xkb_keysym_t,
            offset: 14920 as uint16_t,
        },
        name_keysym {
            keysym: 0xed9 as xkb_keysym_t,
            offset: 14935 as uint16_t,
        },
        name_keysym {
            keysym: 0xed8 as xkb_keysym_t,
            offset: 14955 as uint16_t,
        },
        name_keysym {
            keysym: 0xef8 as xkb_keysym_t,
            offset: 14975 as uint16_t,
        },
        name_keysym {
            keysym: 0xeed as xkb_keysym_t,
            offset: 14992 as uint16_t,
        },
        name_keysym {
            keysym: 0xee4 as xkb_keysym_t,
            offset: 15008 as uint16_t,
        },
        name_keysym {
            keysym: 0xee5 as xkb_keysym_t,
            offset: 15023 as uint16_t,
        },
        name_keysym {
            keysym: 0xedb as xkb_keysym_t,
            offset: 15042 as uint16_t,
        },
        name_keysym {
            keysym: 0xee2 as xkb_keysym_t,
            offset: 15057 as uint16_t,
        },
        name_keysym {
            keysym: 0xedc as xkb_keysym_t,
            offset: 15077 as uint16_t,
        },
        name_keysym {
            keysym: 0xedd as xkb_keysym_t,
            offset: 15098 as uint16_t,
        },
        name_keysym {
            keysym: 0xee1 as xkb_keysym_t,
            offset: 15118 as uint16_t,
        },
        name_keysym {
            keysym: 0xede as xkb_keysym_t,
            offset: 15139 as uint16_t,
        },
        name_keysym {
            keysym: 0xedf as xkb_keysym_t,
            offset: 15159 as uint16_t,
        },
        name_keysym {
            keysym: 0xee0 as xkb_keysym_t,
            offset: 15178 as uint16_t,
        },
        name_keysym {
            keysym: 0xee6 as xkb_keysym_t,
            offset: 15198 as uint16_t,
        },
        name_keysym {
            keysym: 0xed5 as xkb_keysym_t,
            offset: 15212 as uint16_t,
        },
        name_keysym {
            keysym: 0xee7 as xkb_keysym_t,
            offset: 15233 as uint16_t,
        },
        name_keysym {
            keysym: 0xeec as xkb_keysym_t,
            offset: 15252 as uint16_t,
        },
        name_keysym {
            keysym: 0xefa as xkb_keysym_t,
            offset: 15267 as uint16_t,
        },
        name_keysym {
            keysym: 0xff35 as xkb_keysym_t,
            offset: 15288 as uint16_t,
        },
        name_keysym {
            keysym: 0xff38 as xkb_keysym_t,
            offset: 15300 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb8 as xkb_keysym_t,
            offset: 15314 as uint16_t,
        },
        name_keysym {
            keysym: 0xebb as xkb_keysym_t,
            offset: 15327 as uint16_t,
        },
        name_keysym {
            keysym: 0xea1 as xkb_keysym_t,
            offset: 15341 as uint16_t,
        },
        name_keysym {
            keysym: 0xea3 as xkb_keysym_t,
            offset: 15355 as uint16_t,
        },
        name_keysym {
            keysym: 0xef3 as xkb_keysym_t,
            offset: 15373 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb1 as xkb_keysym_t,
            offset: 15398 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3d as xkb_keysym_t,
            offset: 15411 as uint16_t,
        },
        name_keysym {
            keysym: 0xea4 as xkb_keysym_t,
            offset: 15436 as uint16_t,
        },
        name_keysym {
            keysym: 0xea6 as xkb_keysym_t,
            offset: 15449 as uint16_t,
        },
        name_keysym {
            keysym: 0xea5 as xkb_keysym_t,
            offset: 15467 as uint16_t,
        },
        name_keysym {
            keysym: 0xec7 as xkb_keysym_t,
            offset: 15485 as uint16_t,
        },
        name_keysym {
            keysym: 0xeca as xkb_keysym_t,
            offset: 15494 as uint16_t,
        },
        name_keysym {
            keysym: 0xef2 as xkb_keysym_t,
            offset: 15504 as uint16_t,
        },
        name_keysym {
            keysym: 0xebd as xkb_keysym_t,
            offset: 15519 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb2 as xkb_keysym_t,
            offset: 15533 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb4 as xkb_keysym_t,
            offset: 15546 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3b as xkb_keysym_t,
            offset: 15563 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3a as xkb_keysym_t,
            offset: 15580 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3e as xkb_keysym_t,
            offset: 15596 as uint16_t,
        },
        name_keysym {
            keysym: 0xea9 as xkb_keysym_t,
            offset: 15621 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb0 as xkb_keysym_t,
            offset: 15634 as uint16_t,
        },
        name_keysym {
            keysym: 0xeaa as xkb_keysym_t,
            offset: 15652 as uint16_t,
        },
        name_keysym {
            keysym: 0xeab as xkb_keysym_t,
            offset: 15671 as uint16_t,
        },
        name_keysym {
            keysym: 0xeaf as xkb_keysym_t,
            offset: 15689 as uint16_t,
        },
        name_keysym {
            keysym: 0xeac as xkb_keysym_t,
            offset: 15708 as uint16_t,
        },
        name_keysym {
            keysym: 0xead as xkb_keysym_t,
            offset: 15726 as uint16_t,
        },
        name_keysym {
            keysym: 0xeae as xkb_keysym_t,
            offset: 15743 as uint16_t,
        },
        name_keysym {
            keysym: 0xeef as xkb_keysym_t,
            offset: 15761 as uint16_t,
        },
        name_keysym {
            keysym: 0xff36 as xkb_keysym_t,
            offset: 15785 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3c as xkb_keysym_t,
            offset: 15799 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb5 as xkb_keysym_t,
            offset: 15822 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3f as xkb_keysym_t,
            offset: 15834 as uint16_t,
        },
        name_keysym {
            keysym: 0xea8 as xkb_keysym_t,
            offset: 15849 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb9 as xkb_keysym_t,
            offset: 15868 as uint16_t,
        },
        name_keysym {
            keysym: 0xea2 as xkb_keysym_t,
            offset: 15886 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb3 as xkb_keysym_t,
            offset: 15905 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb6 as xkb_keysym_t,
            offset: 15923 as uint16_t,
        },
        name_keysym {
            keysym: 0xff32 as xkb_keysym_t,
            offset: 15940 as uint16_t,
        },
        name_keysym {
            keysym: 0xef0 as xkb_keysym_t,
            offset: 15953 as uint16_t,
        },
        name_keysym {
            keysym: 0xef4 as xkb_keysym_t,
            offset: 15978 as uint16_t,
        },
        name_keysym {
            keysym: 0xef1 as xkb_keysym_t,
            offset: 16004 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 16029 as uint16_t,
        },
        name_keysym {
            keysym: 0xebc as xkb_keysym_t,
            offset: 16043 as uint16_t,
        },
        name_keysym {
            keysym: 0xecc as xkb_keysym_t,
            offset: 16056 as uint16_t,
        },
        name_keysym {
            keysym: 0xec8 as xkb_keysym_t,
            offset: 16065 as uint16_t,
        },
        name_keysym {
            keysym: 0xec9 as xkb_keysym_t,
            offset: 16075 as uint16_t,
        },
        name_keysym {
            keysym: 0xece as xkb_keysym_t,
            offset: 16086 as uint16_t,
        },
        name_keysym {
            keysym: 0xecd as xkb_keysym_t,
            offset: 16096 as uint16_t,
        },
        name_keysym {
            keysym: 0xecf as xkb_keysym_t,
            offset: 16107 as uint16_t,
        },
        name_keysym {
            keysym: 0xec1 as xkb_keysym_t,
            offset: 16117 as uint16_t,
        },
        name_keysym {
            keysym: 0xec2 as xkb_keysym_t,
            offset: 16127 as uint16_t,
        },
        name_keysym {
            keysym: 0xec6 as xkb_keysym_t,
            offset: 16138 as uint16_t,
        },
        name_keysym {
            keysym: 0xec5 as xkb_keysym_t,
            offset: 16148 as uint16_t,
        },
        name_keysym {
            keysym: 0xef5 as xkb_keysym_t,
            offset: 16159 as uint16_t,
        },
        name_keysym {
            keysym: 0xed2 as xkb_keysym_t,
            offset: 16178 as uint16_t,
        },
        name_keysym {
            keysym: 0xecb as xkb_keysym_t,
            offset: 16188 as uint16_t,
        },
        name_keysym {
            keysym: 0xed0 as xkb_keysym_t,
            offset: 16198 as uint16_t,
        },
        name_keysym {
            keysym: 0xff29 as xkb_keysym_t,
            offset: 16208 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a6 as xkb_keysym_t,
            offset: 16216 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b6 as xkb_keysym_t,
            offset: 16228 as uint16_t,
        },
        name_keysym {
            keysym: 0xaee as xkb_keysym_t,
            offset: 16240 as uint16_t,
        },
        name_keysym {
            keysym: 0xce0 as xkb_keysym_t,
            offset: 16246 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf2 as xkb_keysym_t,
            offset: 16259 as uint16_t,
        },
        name_keysym {
            keysym: 0xce1 as xkb_keysym_t,
            offset: 16271 as uint16_t,
        },
        name_keysym {
            keysym: 0xce1 as xkb_keysym_t,
            offset: 16282 as uint16_t,
        },
        name_keysym {
            keysym: 0xce7 as xkb_keysym_t,
            offset: 16294 as uint16_t,
        },
        name_keysym {
            keysym: 0xce3 as xkb_keysym_t,
            offset: 16306 as uint16_t,
        },
        name_keysym {
            keysym: 0xce3 as xkb_keysym_t,
            offset: 16319 as uint16_t,
        },
        name_keysym {
            keysym: 0xcdf as xkb_keysym_t,
            offset: 16333 as uint16_t,
        },
        name_keysym {
            keysym: 0xcea as xkb_keysym_t,
            offset: 16354 as uint16_t,
        },
        name_keysym {
            keysym: 0xced as xkb_keysym_t,
            offset: 16371 as uint16_t,
        },
        name_keysym {
            keysym: 0xcef as xkb_keysym_t,
            offset: 16387 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf3 as xkb_keysym_t,
            offset: 16403 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf5 as xkb_keysym_t,
            offset: 16418 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf5 as xkb_keysym_t,
            offset: 16435 as uint16_t,
        },
        name_keysym {
            keysym: 0xce2 as xkb_keysym_t,
            offset: 16452 as uint16_t,
        },
        name_keysym {
            keysym: 0xce2 as xkb_keysym_t,
            offset: 16465 as uint16_t,
        },
        name_keysym {
            keysym: 0xce4 as xkb_keysym_t,
            offset: 16479 as uint16_t,
        },
        name_keysym {
            keysym: 0xce7 as xkb_keysym_t,
            offset: 16489 as uint16_t,
        },
        name_keysym {
            keysym: 0xceb as xkb_keysym_t,
            offset: 16500 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf7 as xkb_keysym_t,
            offset: 16512 as uint16_t,
        },
        name_keysym {
            keysym: 0xcec as xkb_keysym_t,
            offset: 16523 as uint16_t,
        },
        name_keysym {
            keysym: 0xcee as xkb_keysym_t,
            offset: 16536 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf0 as xkb_keysym_t,
            offset: 16547 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf4 as xkb_keysym_t,
            offset: 16558 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf7 as xkb_keysym_t,
            offset: 16568 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf8 as xkb_keysym_t,
            offset: 16580 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf1 as xkb_keysym_t,
            offset: 16592 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf1 as xkb_keysym_t,
            offset: 16606 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf9 as xkb_keysym_t,
            offset: 16620 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 16632 as uint16_t,
        },
        name_keysym {
            keysym: 0xcfa as xkb_keysym_t,
            offset: 16646 as uint16_t,
        },
        name_keysym {
            keysym: 0xcfa as xkb_keysym_t,
            offset: 16657 as uint16_t,
        },
        name_keysym {
            keysym: 0xce8 as xkb_keysym_t,
            offset: 16668 as uint16_t,
        },
        name_keysym {
            keysym: 0xce8 as xkb_keysym_t,
            offset: 16679 as uint16_t,
        },
        name_keysym {
            keysym: 0xce5 as xkb_keysym_t,
            offset: 16691 as uint16_t,
        },
        name_keysym {
            keysym: 0xce9 as xkb_keysym_t,
            offset: 16702 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf6 as xkb_keysym_t,
            offset: 16713 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf6 as xkb_keysym_t,
            offset: 16725 as uint16_t,
        },
        name_keysym {
            keysym: 0xce6 as xkb_keysym_t,
            offset: 16737 as uint16_t,
        },
        name_keysym {
            keysym: 0xce6 as xkb_keysym_t,
            offset: 16749 as uint16_t,
        },
        name_keysym {
            keysym: 0xff6a as xkb_keysym_t,
            offset: 16762 as uint16_t,
        },
        name_keysym {
            keysym: 0xff23 as xkb_keysym_t,
            offset: 16767 as uint16_t,
        },
        name_keysym {
            keysym: 0xff23 as xkb_keysym_t,
            offset: 16774 as uint16_t,
        },
        name_keysym {
            keysym: 0xada as xkb_keysym_t,
            offset: 16786 as uint16_t,
        },
        name_keysym {
            keysym: 0xff25 as xkb_keysym_t,
            offset: 16795 as uint16_t,
        },
        name_keysym {
            keysym: 0xff27 as xkb_keysym_t,
            offset: 16804 as uint16_t,
        },
        name_keysym {
            keysym: 0xff50 as xkb_keysym_t,
            offset: 16822 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a3 as xkb_keysym_t,
            offset: 16827 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ef as xkb_keysym_t,
            offset: 16842 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f0 as xkb_keysym_t,
            offset: 16857 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f1 as xkb_keysym_t,
            offset: 16872 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f2 as xkb_keysym_t,
            offset: 16887 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f3 as xkb_keysym_t,
            offset: 16902 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff74 as xkb_keysym_t,
            offset: 16917 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000fc as xkb_keysym_t,
            offset: 16927 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6f as xkb_keysym_t,
            offset: 16935 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff73 as xkb_keysym_t,
            offset: 16947 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff71 as xkb_keysym_t,
            offset: 16960 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000be as xkb_keysym_t,
            offset: 16973 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff72 as xkb_keysym_t,
            offset: 16983 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff70 as xkb_keysym_t,
            offset: 16996 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ee as xkb_keysym_t,
            offset: 17009 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff75 as xkb_keysym_t,
            offset: 17014 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000af as xkb_keysym_t,
            offset: 17027 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000f6 as xkb_keysym_t,
            offset: 17034 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff48 as xkb_keysym_t,
            offset: 17046 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff49 as xkb_keysym_t,
            offset: 17058 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a8 as xkb_keysym_t,
            offset: 17070 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000aa as xkb_keysym_t,
            offset: 17083 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ac as xkb_keysym_t,
            offset: 17102 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ab as xkb_keysym_t,
            offset: 17120 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a9 as xkb_keysym_t,
            offset: 17137 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6c as xkb_keysym_t,
            offset: 17150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6d as xkb_keysym_t,
            offset: 17158 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6e as xkb_keysym_t,
            offset: 17167 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ee as xkb_keysym_t,
            offset: 17174 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a1 as xkb_keysym_t,
            offset: 17187 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b1 as xkb_keysym_t,
            offset: 17195 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e2 as xkb_keysym_t,
            offset: 17203 as uint16_t,
        },
        name_keysym {
            keysym: 0xffed as xkb_keysym_t,
            offset: 17206 as uint16_t,
        },
        name_keysym {
            keysym: 0xffee as xkb_keysym_t,
            offset: 17214 as uint16_t,
        },
        name_keysym {
            keysym: 0xad as xkb_keysym_t,
            offset: 17222 as uint16_t,
        },
        name_keysym {
            keysym: 0x49 as xkb_keysym_t,
            offset: 17229 as uint16_t,
        },
        name_keysym {
            keysym: 0x69 as xkb_keysym_t,
            offset: 17231 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a9 as xkb_keysym_t,
            offset: 17233 as uint16_t,
        },
        name_keysym {
            keysym: 0xcd as xkb_keysym_t,
            offset: 17243 as uint16_t,
        },
        name_keysym {
            keysym: 0xed as xkb_keysym_t,
            offset: 17250 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eca as xkb_keysym_t,
            offset: 17257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecb as xkb_keysym_t,
            offset: 17267 as uint16_t,
        },
        name_keysym {
            keysym: 0x100012c as xkb_keysym_t,
            offset: 17277 as uint16_t,
        },
        name_keysym {
            keysym: 0x100012d as xkb_keysym_t,
            offset: 17284 as uint16_t,
        },
        name_keysym {
            keysym: 0xce as xkb_keysym_t,
            offset: 17291 as uint16_t,
        },
        name_keysym {
            keysym: 0xee as xkb_keysym_t,
            offset: 17303 as uint16_t,
        },
        name_keysym {
            keysym: 0x8cf as xkb_keysym_t,
            offset: 17315 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf as xkb_keysym_t,
            offset: 17325 as uint16_t,
        },
        name_keysym {
            keysym: 0xef as xkb_keysym_t,
            offset: 17336 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b9 as xkb_keysym_t,
            offset: 17347 as uint16_t,
        },
        name_keysym {
            keysym: 0x8cd as xkb_keysym_t,
            offset: 17356 as uint16_t,
        },
        name_keysym {
            keysym: 0xcc as xkb_keysym_t,
            offset: 17365 as uint16_t,
        },
        name_keysym {
            keysym: 0xec as xkb_keysym_t,
            offset: 17372 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec8 as xkb_keysym_t,
            offset: 17379 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec9 as xkb_keysym_t,
            offset: 17385 as uint16_t,
        },
        name_keysym {
            keysym: 0x3cf as xkb_keysym_t,
            offset: 17391 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ef as xkb_keysym_t,
            offset: 17399 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ce as xkb_keysym_t,
            offset: 17407 as uint16_t,
        },
        name_keysym {
            keysym: 0x8da as xkb_keysym_t,
            offset: 17415 as uint16_t,
        },
        name_keysym {
            keysym: 0x8db as xkb_keysym_t,
            offset: 17426 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c2 as xkb_keysym_t,
            offset: 17435 as uint16_t,
        },
        name_keysym {
            keysym: 0xff63 as xkb_keysym_t,
            offset: 17444 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff72 as xkb_keysym_t,
            offset: 17451 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff70 as xkb_keysym_t,
            offset: 17462 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bf as xkb_keysym_t,
            offset: 17473 as uint16_t,
        },
        name_keysym {
            keysym: 0x8dc as xkb_keysym_t,
            offset: 17482 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ee as xkb_keysym_t,
            offset: 17495 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c7 as xkb_keysym_t,
            offset: 17498 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e7 as xkb_keysym_t,
            offset: 17506 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe33 as xkb_keysym_t,
            offset: 17514 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe30 as xkb_keysym_t,
            offset: 17532 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe31 as xkb_keysym_t,
            offset: 17557 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe32 as xkb_keysym_t,
            offset: 17585 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe34 as xkb_keysym_t,
            offset: 17599 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2f as xkb_keysym_t,
            offset: 17609 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2c as xkb_keysym_t,
            offset: 17630 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2d as xkb_keysym_t,
            offset: 17651 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2e as xkb_keysym_t,
            offset: 17673 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0c as xkb_keysym_t,
            offset: 17692 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0d as xkb_keysym_t,
            offset: 17708 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe06 as xkb_keysym_t,
            offset: 17729 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe07 as xkb_keysym_t,
            offset: 17745 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 17760 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0e as xkb_keysym_t,
            offset: 17776 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0f as xkb_keysym_t,
            offset: 17791 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe20 as xkb_keysym_t,
            offset: 17811 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe02 as xkb_keysym_t,
            offset: 17824 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe04 as xkb_keysym_t,
            offset: 17841 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe05 as xkb_keysym_t,
            offset: 17858 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe03 as xkb_keysym_t,
            offset: 17874 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe12 as xkb_keysym_t,
            offset: 17891 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe13 as xkb_keysym_t,
            offset: 17908 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe11 as xkb_keysym_t,
            offset: 17924 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe01 as xkb_keysym_t,
            offset: 17941 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe22 as xkb_keysym_t,
            offset: 17950 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe21 as xkb_keysym_t,
            offset: 17969 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe08 as xkb_keysym_t,
            offset: 17986 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe09 as xkb_keysym_t,
            offset: 18001 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe24 as xkb_keysym_t,
            offset: 18021 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe23 as xkb_keysym_t,
            offset: 18043 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe25 as xkb_keysym_t,
            offset: 18063 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe26 as xkb_keysym_t,
            offset: 18086 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0a as xkb_keysym_t,
            offset: 18110 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0b as xkb_keysym_t,
            offset: 18125 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2b as xkb_keysym_t,
            offset: 18145 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe29 as xkb_keysym_t,
            offset: 18170 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2a as xkb_keysym_t,
            offset: 18194 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe27 as xkb_keysym_t,
            offset: 18219 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe28 as xkb_keysym_t,
            offset: 18239 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a5 as xkb_keysym_t,
            offset: 18260 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b5 as xkb_keysym_t,
            offset: 18267 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a as xkb_keysym_t,
            offset: 18274 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a as xkb_keysym_t,
            offset: 18276 as uint16_t,
        },
        name_keysym {
            keysym: 0x2ac as xkb_keysym_t,
            offset: 18278 as uint16_t,
        },
        name_keysym {
            keysym: 0x2bc as xkb_keysym_t,
            offset: 18290 as uint16_t,
        },
        name_keysym {
            keysym: 0xbca as xkb_keysym_t,
            offset: 18302 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b as xkb_keysym_t,
            offset: 18306 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b as xkb_keysym_t,
            offset: 18308 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b1 as xkb_keysym_t,
            offset: 18310 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a7 as xkb_keysym_t,
            offset: 18317 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c1 as xkb_keysym_t,
            offset: 18324 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a3 as xkb_keysym_t,
            offset: 18333 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a4 as xkb_keysym_t,
            offset: 18353 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a5 as xkb_keysym_t,
            offset: 18364 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b4 as xkb_keysym_t,
            offset: 18381 as uint16_t,
        },
        name_keysym {
            keysym: 0x4aa as xkb_keysym_t,
            offset: 18388 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cc as xkb_keysym_t,
            offset: 18395 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a1 as xkb_keysym_t,
            offset: 18403 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ca as xkb_keysym_t,
            offset: 18417 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cd as xkb_keysym_t,
            offset: 18425 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cb as xkb_keysym_t,
            offset: 18433 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ce as xkb_keysym_t,
            offset: 18441 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cc as xkb_keysym_t,
            offset: 18449 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b2 as xkb_keysym_t,
            offset: 18457 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a8 as xkb_keysym_t,
            offset: 18464 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b6 as xkb_keysym_t,
            offset: 18471 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b9 as xkb_keysym_t,
            offset: 18479 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b7 as xkb_keysym_t,
            offset: 18487 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ba as xkb_keysym_t,
            offset: 18495 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b8 as xkb_keysym_t,
            offset: 18503 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2d as xkb_keysym_t,
            offset: 18511 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cf as xkb_keysym_t,
            offset: 18521 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d2 as xkb_keysym_t,
            offset: 18529 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d0 as xkb_keysym_t,
            offset: 18537 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a5 as xkb_keysym_t,
            offset: 18545 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d3 as xkb_keysym_t,
            offset: 18560 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d1 as xkb_keysym_t,
            offset: 18568 as uint16_t,
        },
        name_keysym {
            keysym: 0x4dd as xkb_keysym_t,
            offset: 18576 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c5 as xkb_keysym_t,
            offset: 18583 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c8 as xkb_keysym_t,
            offset: 18591 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c6 as xkb_keysym_t,
            offset: 18599 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c9 as xkb_keysym_t,
            offset: 18607 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c7 as xkb_keysym_t,
            offset: 18615 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b5 as xkb_keysym_t,
            offset: 18623 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ab as xkb_keysym_t,
            offset: 18630 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a2 as xkb_keysym_t,
            offset: 18637 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d7 as xkb_keysym_t,
            offset: 18657 as uint16_t,
        },
        name_keysym {
            keysym: 0x4da as xkb_keysym_t,
            offset: 18665 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d8 as xkb_keysym_t,
            offset: 18673 as uint16_t,
        },
        name_keysym {
            keysym: 0x4db as xkb_keysym_t,
            offset: 18681 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d9 as xkb_keysym_t,
            offset: 18689 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bb as xkb_keysym_t,
            offset: 18697 as uint16_t,
        },
        name_keysym {
            keysym: 0x4be as xkb_keysym_t,
            offset: 18705 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bc as xkb_keysym_t,
            offset: 18713 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2e as xkb_keysym_t,
            offset: 18722 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bf as xkb_keysym_t,
            offset: 18733 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bd as xkb_keysym_t,
            offset: 18741 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 18749 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c0 as xkb_keysym_t,
            offset: 18761 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c3 as xkb_keysym_t,
            offset: 18769 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c1 as xkb_keysym_t,
            offset: 18777 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c4 as xkb_keysym_t,
            offset: 18785 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c2 as xkb_keysym_t,
            offset: 18793 as uint16_t,
        },
        name_keysym {
            keysym: 0x4af as xkb_keysym_t,
            offset: 18802 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c2 as xkb_keysym_t,
            offset: 18811 as uint16_t,
        },
        name_keysym {
            keysym: 0x4af as xkb_keysym_t,
            offset: 18819 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b3 as xkb_keysym_t,
            offset: 18827 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a9 as xkb_keysym_t,
            offset: 18834 as uint16_t,
        },
        name_keysym {
            keysym: 0x4dc as xkb_keysym_t,
            offset: 18841 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a6 as xkb_keysym_t,
            offset: 18849 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d4 as xkb_keysym_t,
            offset: 18857 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ac as xkb_keysym_t,
            offset: 18865 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d6 as xkb_keysym_t,
            offset: 18873 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ae as xkb_keysym_t,
            offset: 18881 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d5 as xkb_keysym_t,
            offset: 18889 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ad as xkb_keysym_t,
            offset: 18897 as uint16_t,
        },
        name_keysym {
            keysym: 0xff21 as xkb_keysym_t,
            offset: 18905 as uint16_t,
        },
        name_keysym {
            keysym: 0xff37 as xkb_keysym_t,
            offset: 18911 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a2 as xkb_keysym_t,
            offset: 18924 as uint16_t,
        },
        name_keysym {
            keysym: 0xff26 as xkb_keysym_t,
            offset: 18930 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d3 as xkb_keysym_t,
            offset: 18939 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f3 as xkb_keysym_t,
            offset: 18948 as uint16_t,
        },
        name_keysym {
            keysym: 0xeff as xkb_keysym_t,
            offset: 18957 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb0 as xkb_keysym_t,
            offset: 18968 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb1 as xkb_keysym_t,
            offset: 18973 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb2 as xkb_keysym_t,
            offset: 18978 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb3 as xkb_keysym_t,
            offset: 18983 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb4 as xkb_keysym_t,
            offset: 18988 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb5 as xkb_keysym_t,
            offset: 18993 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb6 as xkb_keysym_t,
            offset: 18998 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb7 as xkb_keysym_t,
            offset: 19003 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb8 as xkb_keysym_t,
            offset: 19008 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb9 as xkb_keysym_t,
            offset: 19013 as uint16_t,
        },
        name_keysym {
            keysym: 0xffab as xkb_keysym_t,
            offset: 19018 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff75 as xkb_keysym_t,
            offset: 19025 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9d as xkb_keysym_t,
            offset: 19036 as uint16_t,
        },
        name_keysym {
            keysym: 0xffae as xkb_keysym_t,
            offset: 19045 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9f as xkb_keysym_t,
            offset: 19056 as uint16_t,
        },
        name_keysym {
            keysym: 0xffaf as xkb_keysym_t,
            offset: 19066 as uint16_t,
        },
        name_keysym {
            keysym: 0xff99 as xkb_keysym_t,
            offset: 19076 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9c as xkb_keysym_t,
            offset: 19084 as uint16_t,
        },
        name_keysym {
            keysym: 0xff8d as xkb_keysym_t,
            offset: 19091 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbd as xkb_keysym_t,
            offset: 19100 as uint16_t,
        },
        name_keysym {
            keysym: 0xff91 as xkb_keysym_t,
            offset: 19109 as uint16_t,
        },
        name_keysym {
            keysym: 0xff92 as xkb_keysym_t,
            offset: 19115 as uint16_t,
        },
        name_keysym {
            keysym: 0xff93 as xkb_keysym_t,
            offset: 19121 as uint16_t,
        },
        name_keysym {
            keysym: 0xff94 as xkb_keysym_t,
            offset: 19127 as uint16_t,
        },
        name_keysym {
            keysym: 0xff95 as xkb_keysym_t,
            offset: 19133 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9e as xkb_keysym_t,
            offset: 19141 as uint16_t,
        },
        name_keysym {
            keysym: 0xff96 as xkb_keysym_t,
            offset: 19151 as uint16_t,
        },
        name_keysym {
            keysym: 0xffaa as xkb_keysym_t,
            offset: 19159 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9b as xkb_keysym_t,
            offset: 19171 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9b as xkb_keysym_t,
            offset: 19179 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9a as xkb_keysym_t,
            offset: 19192 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9a as xkb_keysym_t,
            offset: 19203 as uint16_t,
        },
        name_keysym {
            keysym: 0xff98 as xkb_keysym_t,
            offset: 19212 as uint16_t,
        },
        name_keysym {
            keysym: 0xffac as xkb_keysym_t,
            offset: 19221 as uint16_t,
        },
        name_keysym {
            keysym: 0xff80 as xkb_keysym_t,
            offset: 19234 as uint16_t,
        },
        name_keysym {
            keysym: 0xffad as xkb_keysym_t,
            offset: 19243 as uint16_t,
        },
        name_keysym {
            keysym: 0xff89 as xkb_keysym_t,
            offset: 19255 as uint16_t,
        },
        name_keysym {
            keysym: 0xff97 as xkb_keysym_t,
            offset: 19262 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a2 as xkb_keysym_t,
            offset: 19268 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c as xkb_keysym_t,
            offset: 19272 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c as xkb_keysym_t,
            offset: 19274 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc8 as xkb_keysym_t,
            offset: 19276 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd1 as xkb_keysym_t,
            offset: 19279 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc9 as xkb_keysym_t,
            offset: 19283 as uint16_t,
        },
        name_keysym {
            keysym: 0xffca as xkb_keysym_t,
            offset: 19286 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcb as xkb_keysym_t,
            offset: 19289 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcc as xkb_keysym_t,
            offset: 19292 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcd as xkb_keysym_t,
            offset: 19295 as uint16_t,
        },
        name_keysym {
            keysym: 0xffce as xkb_keysym_t,
            offset: 19298 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcf as xkb_keysym_t,
            offset: 19301 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd0 as xkb_keysym_t,
            offset: 19304 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c5 as xkb_keysym_t,
            offset: 19307 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e5 as xkb_keysym_t,
            offset: 19314 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed4 as xkb_keysym_t,
            offset: 19321 as uint16_t,
        },
        name_keysym {
            keysym: 0xad9 as xkb_keysym_t,
            offset: 19341 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e36 as xkb_keysym_t,
            offset: 19352 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e37 as xkb_keysym_t,
            offset: 19362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a5 as xkb_keysym_t,
            offset: 19372 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b5 as xkb_keysym_t,
            offset: 19379 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a6 as xkb_keysym_t,
            offset: 19386 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b6 as xkb_keysym_t,
            offset: 19395 as uint16_t,
        },
        name_keysym {
            keysym: 0xff51 as xkb_keysym_t,
            offset: 19404 as uint16_t,
        },
        name_keysym {
            keysym: 0xabc as xkb_keysym_t,
            offset: 19409 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fb as xkb_keysym_t,
            offset: 19426 as uint16_t,
        },
        name_keysym {
            keysym: 0xba3 as xkb_keysym_t,
            offset: 19436 as uint16_t,
        },
        name_keysym {
            keysym: 0xad2 as xkb_keysym_t,
            offset: 19446 as uint16_t,
        },
        name_keysym {
            keysym: 0x8af as xkb_keysym_t,
            offset: 19466 as uint16_t,
        },
        name_keysym {
            keysym: 0xacc as xkb_keysym_t,
            offset: 19487 as uint16_t,
        },
        name_keysym {
            keysym: 0xaea as xkb_keysym_t,
            offset: 19504 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a1 as xkb_keysym_t,
            offset: 19516 as uint16_t,
        },
        name_keysym {
            keysym: 0xbda as xkb_keysym_t,
            offset: 19528 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002039 as xkb_keysym_t,
            offset: 19537 as uint16_t,
        },
        name_keysym {
            keysym: 0xad0 as xkb_keysym_t,
            offset: 19562 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f4 as xkb_keysym_t,
            offset: 19582 as uint16_t,
        },
        name_keysym {
            keysym: 0xbdc as xkb_keysym_t,
            offset: 19588 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c as xkb_keysym_t,
            offset: 19597 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bc as xkb_keysym_t,
            offset: 19602 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e5 as xkb_keysym_t,
            offset: 19616 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0a as xkb_keysym_t,
            offset: 19619 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000af as xkb_keysym_t,
            offset: 19628 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a4 as xkb_keysym_t,
            offset: 19633 as uint16_t,
        },
        name_keysym {
            keysym: 0x8de as xkb_keysym_t,
            offset: 19642 as uint16_t,
        },
        name_keysym {
            keysym: 0x8df as xkb_keysym_t,
            offset: 19653 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000f6 as xkb_keysym_t,
            offset: 19663 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ed as xkb_keysym_t,
            offset: 19673 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ea as xkb_keysym_t,
            offset: 19687 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a3 as xkb_keysym_t,
            offset: 19702 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b3 as xkb_keysym_t,
            offset: 19710 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d as xkb_keysym_t,
            offset: 19718 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d as xkb_keysym_t,
            offset: 19720 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e40 as xkb_keysym_t,
            offset: 19722 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e41 as xkb_keysym_t,
            offset: 19732 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b5 as xkb_keysym_t,
            offset: 19742 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a5 as xkb_keysym_t,
            offset: 19756 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b2 as xkb_keysym_t,
            offset: 19770 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a2 as xkb_keysym_t,
            offset: 19784 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bc as xkb_keysym_t,
            offset: 19798 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ac as xkb_keysym_t,
            offset: 19812 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf as xkb_keysym_t,
            offset: 19826 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3e as xkb_keysym_t,
            offset: 19833 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf7 as xkb_keysym_t,
            offset: 19842 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf0 as xkb_keysym_t,
            offset: 19853 as uint16_t,
        },
        name_keysym {
            keysym: 0xabf as xkb_keysym_t,
            offset: 19866 as uint16_t,
        },
        name_keysym {
            keysym: 0xba as xkb_keysym_t,
            offset: 19873 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2c as xkb_keysym_t,
            offset: 19883 as uint16_t,
        },
        name_keysym {
            keysym: 0xff67 as xkb_keysym_t,
            offset: 19890 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe7 as xkb_keysym_t,
            offset: 19895 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe8 as xkb_keysym_t,
            offset: 19902 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a5 as xkb_keysym_t,
            offset: 19909 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d as xkb_keysym_t,
            offset: 19918 as uint16_t,
        },
        name_keysym {
            keysym: 0xad6 as xkb_keysym_t,
            offset: 19924 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 19932 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe77 as xkb_keysym_t,
            offset: 19944 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe76 as xkb_keysym_t,
            offset: 19967 as uint16_t,
        },
        name_keysym {
            keysym: 0xb5 as xkb_keysym_t,
            offset: 19984 as uint16_t,
        },
        name_keysym {
            keysym: 0xff22 as xkb_keysym_t,
            offset: 19987 as uint16_t,
        },
        name_keysym {
            keysym: 0xff20 as xkb_keysym_t,
            offset: 19996 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3d as xkb_keysym_t,
            offset: 20006 as uint16_t,
        },
        name_keysym {
            keysym: 0xd7 as xkb_keysym_t,
            offset: 20024 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf6 as xkb_keysym_t,
            offset: 20033 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf5 as xkb_keysym_t,
            offset: 20045 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a8 as xkb_keysym_t,
            offset: 20058 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000aa as xkb_keysym_t,
            offset: 20069 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ac as xkb_keysym_t,
            offset: 20086 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ab as xkb_keysym_t,
            offset: 20102 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a9 as xkb_keysym_t,
            offset: 20117 as uint16_t,
        },
        name_keysym {
            keysym: 0x4e as xkb_keysym_t,
            offset: 20128 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e as xkb_keysym_t,
            offset: 20130 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c5 as xkb_keysym_t,
            offset: 20132 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d1 as xkb_keysym_t,
            offset: 20138 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f1 as xkb_keysym_t,
            offset: 20145 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a6 as xkb_keysym_t,
            offset: 20152 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d2 as xkb_keysym_t,
            offset: 20162 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f2 as xkb_keysym_t,
            offset: 20169 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d1 as xkb_keysym_t,
            offset: 20176 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f1 as xkb_keysym_t,
            offset: 20185 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020aa as xkb_keysym_t,
            offset: 20194 as uint16_t,
        },
        name_keysym {
            keysym: 0xff56 as xkb_keysym_t,
            offset: 20208 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed2 as xkb_keysym_t,
            offset: 20213 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002089 as xkb_keysym_t,
            offset: 20233 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002079 as xkb_keysym_t,
            offset: 20247 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e8 as xkb_keysym_t,
            offset: 20260 as uint16_t,
        },
        name_keysym {
            keysym: 0xa0 as xkb_keysym_t,
            offset: 20263 as uint16_t,
        },
        name_keysym {
            keysym: 0 as xkb_keysym_t,
            offset: 20276 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002247 as xkb_keysym_t,
            offset: 20285 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002209 as xkb_keysym_t,
            offset: 20297 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bd as xkb_keysym_t,
            offset: 20310 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002262 as xkb_keysym_t,
            offset: 20319 as uint16_t,
        },
        name_keysym {
            keysym: 0xac as xkb_keysym_t,
            offset: 20332 as uint16_t,
        },
        name_keysym {
            keysym: 0xd1 as xkb_keysym_t,
            offset: 20340 as uint16_t,
        },
        name_keysym {
            keysym: 0xf1 as xkb_keysym_t,
            offset: 20347 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7f as xkb_keysym_t,
            offset: 20354 as uint16_t,
        },
        name_keysym {
            keysym: 0x23 as xkb_keysym_t,
            offset: 20363 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b0 as xkb_keysym_t,
            offset: 20374 as uint16_t,
        },
        name_keysym {
            keysym: 0x4f as xkb_keysym_t,
            offset: 20385 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f as xkb_keysym_t,
            offset: 20387 as uint16_t,
        },
        name_keysym {
            keysym: 0xd3 as xkb_keysym_t,
            offset: 20389 as uint16_t,
        },
        name_keysym {
            keysym: 0xf3 as xkb_keysym_t,
            offset: 20396 as uint16_t,
        },
        name_keysym {
            keysym: 0x100019f as xkb_keysym_t,
            offset: 20403 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000275 as xkb_keysym_t,
            offset: 20411 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecc as xkb_keysym_t,
            offset: 20419 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecd as xkb_keysym_t,
            offset: 20429 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001d1 as xkb_keysym_t,
            offset: 20439 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001d2 as xkb_keysym_t,
            offset: 20446 as uint16_t,
        },
        name_keysym {
            keysym: 0xd4 as xkb_keysym_t,
            offset: 20453 as uint16_t,
        },
        name_keysym {
            keysym: 0xf4 as xkb_keysym_t,
            offset: 20465 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed0 as xkb_keysym_t,
            offset: 20477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed1 as xkb_keysym_t,
            offset: 20494 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed8 as xkb_keysym_t,
            offset: 20511 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed9 as xkb_keysym_t,
            offset: 20531 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed2 as xkb_keysym_t,
            offset: 20551 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed3 as xkb_keysym_t,
            offset: 20568 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed4 as xkb_keysym_t,
            offset: 20585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed5 as xkb_keysym_t,
            offset: 20601 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed6 as xkb_keysym_t,
            offset: 20617 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed7 as xkb_keysym_t,
            offset: 20634 as uint16_t,
        },
        name_keysym {
            keysym: 0xd6 as xkb_keysym_t,
            offset: 20651 as uint16_t,
        },
        name_keysym {
            keysym: 0xf6 as xkb_keysym_t,
            offset: 20662 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d5 as xkb_keysym_t,
            offset: 20673 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f5 as xkb_keysym_t,
            offset: 20686 as uint16_t,
        },
        name_keysym {
            keysym: 0x13bc as xkb_keysym_t,
            offset: 20699 as uint16_t,
        },
        name_keysym {
            keysym: 0x13bd as xkb_keysym_t,
            offset: 20702 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b2 as xkb_keysym_t,
            offset: 20705 as uint16_t,
        },
        name_keysym {
            keysym: 0xd2 as xkb_keysym_t,
            offset: 20712 as uint16_t,
        },
        name_keysym {
            keysym: 0xf2 as xkb_keysym_t,
            offset: 20719 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ece as xkb_keysym_t,
            offset: 20726 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecf as xkb_keysym_t,
            offset: 20732 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001a0 as xkb_keysym_t,
            offset: 20738 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001a1 as xkb_keysym_t,
            offset: 20744 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eda as xkb_keysym_t,
            offset: 20750 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edb as xkb_keysym_t,
            offset: 20761 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee2 as xkb_keysym_t,
            offset: 20772 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee3 as xkb_keysym_t,
            offset: 20786 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edc as xkb_keysym_t,
            offset: 20800 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edd as xkb_keysym_t,
            offset: 20811 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ede as xkb_keysym_t,
            offset: 20822 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edf as xkb_keysym_t,
            offset: 20832 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee0 as xkb_keysym_t,
            offset: 20842 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee1 as xkb_keysym_t,
            offset: 20853 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d2 as xkb_keysym_t,
            offset: 20864 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f2 as xkb_keysym_t,
            offset: 20872 as uint16_t,
        },
        name_keysym {
            keysym: 0xac3 as xkb_keysym_t,
            offset: 20880 as uint16_t,
        },
        name_keysym {
            keysym: 0xab2 as xkb_keysym_t,
            offset: 20890 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd as xkb_keysym_t,
            offset: 20899 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc as xkb_keysym_t,
            offset: 20907 as uint16_t,
        },
        name_keysym {
            keysym: 0xab6 as xkb_keysym_t,
            offset: 20918 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002081 as xkb_keysym_t,
            offset: 20927 as uint16_t,
        },
        name_keysym {
            keysym: 0xb9 as xkb_keysym_t,
            offset: 20940 as uint16_t,
        },
        name_keysym {
            keysym: 0xab0 as xkb_keysym_t,
            offset: 20952 as uint16_t,
        },
        name_keysym {
            keysym: 0xd8 as xkb_keysym_t,
            offset: 20961 as uint16_t,
        },
        name_keysym {
            keysym: 0xf8 as xkb_keysym_t,
            offset: 20970 as uint16_t,
        },
        name_keysym {
            keysym: 0xae2 as xkb_keysym_t,
            offset: 20979 as uint16_t,
        },
        name_keysym {
            keysym: 0xae5 as xkb_keysym_t,
            offset: 20994 as uint16_t,
        },
        name_keysym {
            keysym: 0xae4 as xkb_keysym_t,
            offset: 21003 as uint16_t,
        },
        name_keysym {
            keysym: 0xae3 as xkb_keysym_t,
            offset: 21021 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa as xkb_keysym_t,
            offset: 21037 as uint16_t,
        },
        name_keysym {
            keysym: 0xba as xkb_keysym_t,
            offset: 21049 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff44 as xkb_keysym_t,
            offset: 21062 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff31 as xkb_keysym_t,
            offset: 21074 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff08 as xkb_keysym_t,
            offset: 21085 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff07 as xkb_keysym_t,
            offset: 21098 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5a as xkb_keysym_t,
            offset: 21109 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff58 as xkb_keysym_t,
            offset: 21122 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff69 as xkb_keysym_t,
            offset: 21135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff0b as xkb_keysym_t,
            offset: 21145 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff02 as xkb_keysym_t,
            offset: 21154 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff03 as xkb_keysym_t,
            offset: 21162 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ffff as xkb_keysym_t,
            offset: 21169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff72 as xkb_keysym_t,
            offset: 21179 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff54 as xkb_keysym_t,
            offset: 21194 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff59 as xkb_keysym_t,
            offset: 21202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff57 as xkb_keysym_t,
            offset: 21213 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff1b as xkb_keysym_t,
            offset: 21224 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff74 as xkb_keysym_t,
            offset: 21234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff6a as xkb_keysym_t,
            offset: 21244 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff63 as xkb_keysym_t,
            offset: 21252 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff51 as xkb_keysym_t,
            offset: 21262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff67 as xkb_keysym_t,
            offset: 21270 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff45 as xkb_keysym_t,
            offset: 21278 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5e as xkb_keysym_t,
            offset: 21289 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5c as xkb_keysym_t,
            offset: 21302 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff42 as xkb_keysym_t,
            offset: 21314 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff40 as xkb_keysym_t,
            offset: 21326 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff43 as xkb_keysym_t,
            offset: 21338 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff41 as xkb_keysym_t,
            offset: 21351 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff04 as xkb_keysym_t,
            offset: 21361 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5d as xkb_keysym_t,
            offset: 21370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5b as xkb_keysym_t,
            offset: 21383 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff32 as xkb_keysym_t,
            offset: 21395 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff33 as xkb_keysym_t,
            offset: 21411 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff73 as xkb_keysym_t,
            offset: 21425 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff78 as xkb_keysym_t,
            offset: 21437 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff53 as xkb_keysym_t,
            offset: 21448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff60 as xkb_keysym_t,
            offset: 21457 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff71 as xkb_keysym_t,
            offset: 21467 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff65 as xkb_keysym_t,
            offset: 21480 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff52 as xkb_keysym_t,
            offset: 21488 as uint16_t,
        },
        name_keysym {
            keysym: 0xd8 as xkb_keysym_t,
            offset: 21494 as uint16_t,
        },
        name_keysym {
            keysym: 0xf8 as xkb_keysym_t,
            offset: 21501 as uint16_t,
        },
        name_keysym {
            keysym: 0xd5 as xkb_keysym_t,
            offset: 21508 as uint16_t,
        },
        name_keysym {
            keysym: 0xf5 as xkb_keysym_t,
            offset: 21515 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc0 as xkb_keysym_t,
            offset: 21522 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe78 as xkb_keysym_t,
            offset: 21530 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe79 as xkb_keysym_t,
            offset: 21546 as uint16_t,
        },
        name_keysym {
            keysym: 0x47e as xkb_keysym_t,
            offset: 21562 as uint16_t,
        },
        name_keysym {
            keysym: 0x50 as xkb_keysym_t,
            offset: 21571 as uint16_t,
        },
        name_keysym {
            keysym: 0x70 as xkb_keysym_t,
            offset: 21573 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e56 as xkb_keysym_t,
            offset: 21575 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e57 as xkb_keysym_t,
            offset: 21585 as uint16_t,
        },
        name_keysym {
            keysym: 0xff56 as xkb_keysym_t,
            offset: 21595 as uint16_t,
        },
        name_keysym {
            keysym: 0xff55 as xkb_keysym_t,
            offset: 21605 as uint16_t,
        },
        name_keysym {
            keysym: 0xb6 as xkb_keysym_t,
            offset: 21613 as uint16_t,
        },
        name_keysym {
            keysym: 0x28 as xkb_keysym_t,
            offset: 21623 as uint16_t,
        },
        name_keysym {
            keysym: 0x29 as xkb_keysym_t,
            offset: 21633 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002202 as xkb_keysym_t,
            offset: 21644 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ef as xkb_keysym_t,
            offset: 21661 as uint16_t,
        },
        name_keysym {
            keysym: 0xff13 as xkb_keysym_t,
            offset: 21679 as uint16_t,
        },
        name_keysym {
            keysym: 0x25 as xkb_keysym_t,
            offset: 21685 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e as xkb_keysym_t,
            offset: 21693 as uint16_t,
        },
        name_keysym {
            keysym: 0xb7 as xkb_keysym_t,
            offset: 21700 as uint16_t,
        },
        name_keysym {
            keysym: 0xad5 as xkb_keysym_t,
            offset: 21715 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a7 as xkb_keysym_t,
            offset: 21724 as uint16_t,
        },
        name_keysym {
            keysym: 0xafb as xkb_keysym_t,
            offset: 21735 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b as xkb_keysym_t,
            offset: 21755 as uint16_t,
        },
        name_keysym {
            keysym: 0xb1 as xkb_keysym_t,
            offset: 21760 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefa as xkb_keysym_t,
            offset: 21770 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee9 as xkb_keysym_t,
            offset: 21789 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeea as xkb_keysym_t,
            offset: 21805 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeeb as xkb_keysym_t,
            offset: 21821 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeec as xkb_keysym_t,
            offset: 21837 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeed as xkb_keysym_t,
            offset: 21853 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee8 as xkb_keysym_t,
            offset: 21869 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeef as xkb_keysym_t,
            offset: 21889 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef0 as xkb_keysym_t,
            offset: 21907 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef1 as xkb_keysym_t,
            offset: 21925 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef2 as xkb_keysym_t,
            offset: 21943 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef3 as xkb_keysym_t,
            offset: 21961 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeee as xkb_keysym_t,
            offset: 21979 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefb as xkb_keysym_t,
            offset: 22001 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefc as xkb_keysym_t,
            offset: 22021 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee3 as xkb_keysym_t,
            offset: 22041 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee6 as xkb_keysym_t,
            offset: 22054 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee7 as xkb_keysym_t,
            offset: 22071 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef5 as xkb_keysym_t,
            offset: 22089 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef6 as xkb_keysym_t,
            offset: 22103 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef7 as xkb_keysym_t,
            offset: 22117 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef8 as xkb_keysym_t,
            offset: 22131 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefd as xkb_keysym_t,
            offset: 22145 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef4 as xkb_keysym_t,
            offset: 22159 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef9 as xkb_keysym_t,
            offset: 22177 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee0 as xkb_keysym_t,
            offset: 22196 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee1 as xkb_keysym_t,
            offset: 22209 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee2 as xkb_keysym_t,
            offset: 22223 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee4 as xkb_keysym_t,
            offset: 22234 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee5 as xkb_keysym_t,
            offset: 22249 as uint16_t,
        },
        name_keysym {
            keysym: 0xad4 as xkb_keysym_t,
            offset: 22265 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed1 as xkb_keysym_t,
            offset: 22278 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3e as xkb_keysym_t,
            offset: 22298 as uint16_t,
        },
        name_keysym {
            keysym: 0xff61 as xkb_keysym_t,
            offset: 22316 as uint16_t,
        },
        name_keysym {
            keysym: 0xff55 as xkb_keysym_t,
            offset: 22322 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b0 as xkb_keysym_t,
            offset: 22328 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa6 as xkb_keysym_t,
            offset: 22343 as uint16_t,
        },
        name_keysym {
            keysym: 0x51 as xkb_keysym_t,
            offset: 22354 as uint16_t,
        },
        name_keysym {
            keysym: 0x71 as xkb_keysym_t,
            offset: 22356 as uint16_t,
        },
        name_keysym {
            keysym: 0xbcc as xkb_keysym_t,
            offset: 22358 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f as xkb_keysym_t,
            offset: 22363 as uint16_t,
        },
        name_keysym {
            keysym: 0xbf as xkb_keysym_t,
            offset: 22372 as uint16_t,
        },
        name_keysym {
            keysym: 0x22 as xkb_keysym_t,
            offset: 22385 as uint16_t,
        },
        name_keysym {
            keysym: 0x60 as xkb_keysym_t,
            offset: 22394 as uint16_t,
        },
        name_keysym {
            keysym: 0x27 as xkb_keysym_t,
            offset: 22404 as uint16_t,
        },
        name_keysym {
            keysym: 0x52 as xkb_keysym_t,
            offset: 22415 as uint16_t,
        },
        name_keysym {
            keysym: 0x72 as xkb_keysym_t,
            offset: 22417 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd2 as xkb_keysym_t,
            offset: 22419 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdb as xkb_keysym_t,
            offset: 22422 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdc as xkb_keysym_t,
            offset: 22426 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdd as xkb_keysym_t,
            offset: 22430 as uint16_t,
        },
        name_keysym {
            keysym: 0xffde as xkb_keysym_t,
            offset: 22434 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdf as xkb_keysym_t,
            offset: 22438 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe0 as xkb_keysym_t,
            offset: 22442 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd3 as xkb_keysym_t,
            offset: 22446 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd4 as xkb_keysym_t,
            offset: 22449 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd5 as xkb_keysym_t,
            offset: 22452 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd6 as xkb_keysym_t,
            offset: 22455 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd7 as xkb_keysym_t,
            offset: 22458 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd8 as xkb_keysym_t,
            offset: 22461 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd9 as xkb_keysym_t,
            offset: 22464 as uint16_t,
        },
        name_keysym {
            keysym: 0xffda as xkb_keysym_t,
            offset: 22467 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c0 as xkb_keysym_t,
            offset: 22470 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e0 as xkb_keysym_t,
            offset: 22477 as uint16_t,
        },
        name_keysym {
            keysym: 0x8d6 as xkb_keysym_t,
            offset: 22484 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d8 as xkb_keysym_t,
            offset: 22492 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f8 as xkb_keysym_t,
            offset: 22499 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a3 as xkb_keysym_t,
            offset: 22506 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b3 as xkb_keysym_t,
            offset: 22515 as uint16_t,
        },
        name_keysym {
            keysym: 0xff66 as xkb_keysym_t,
            offset: 22524 as uint16_t,
        },
        name_keysym {
            keysym: 0xae as xkb_keysym_t,
            offset: 22529 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe72 as xkb_keysym_t,
            offset: 22540 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6c as xkb_keysym_t,
            offset: 22558 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0d as xkb_keysym_t,
            offset: 22564 as uint16_t,
        },
        name_keysym {
            keysym: 0xff53 as xkb_keysym_t,
            offset: 22571 as uint16_t,
        },
        name_keysym {
            keysym: 0xabe as xkb_keysym_t,
            offset: 22577 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fd as xkb_keysym_t,
            offset: 22595 as uint16_t,
        },
        name_keysym {
            keysym: 0xba6 as xkb_keysym_t,
            offset: 22606 as uint16_t,
        },
        name_keysym {
            keysym: 0xad3 as xkb_keysym_t,
            offset: 22617 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b0 as xkb_keysym_t,
            offset: 22638 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b7 as xkb_keysym_t,
            offset: 22660 as uint16_t,
        },
        name_keysym {
            keysym: 0xacd as xkb_keysym_t,
            offset: 22681 as uint16_t,
        },
        name_keysym {
            keysym: 0xaeb as xkb_keysym_t,
            offset: 22699 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd8 as xkb_keysym_t,
            offset: 22712 as uint16_t,
        },
        name_keysym {
            keysym: 0x100203a as xkb_keysym_t,
            offset: 22722 as uint16_t,
        },
        name_keysym {
            keysym: 0xad1 as xkb_keysym_t,
            offset: 22748 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f5 as xkb_keysym_t,
            offset: 22769 as uint16_t,
        },
        name_keysym {
            keysym: 0xbfc as xkb_keysym_t,
            offset: 22776 as uint16_t,
        },
        name_keysym {
            keysym: 0xff24 as xkb_keysym_t,
            offset: 22786 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a8 as xkb_keysym_t,
            offset: 22793 as uint16_t,
        },
        name_keysym {
            keysym: 0x53 as xkb_keysym_t,
            offset: 22803 as uint16_t,
        },
        name_keysym {
            keysym: 0x73 as xkb_keysym_t,
            offset: 22805 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e60 as xkb_keysym_t,
            offset: 22807 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e61 as xkb_keysym_t,
            offset: 22817 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a6 as xkb_keysym_t,
            offset: 22827 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b6 as xkb_keysym_t,
            offset: 22834 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a9 as xkb_keysym_t,
            offset: 22841 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b9 as xkb_keysym_t,
            offset: 22848 as uint16_t,
        },
        name_keysym {
            keysym: 0x1aa as xkb_keysym_t,
            offset: 22855 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ba as xkb_keysym_t,
            offset: 22864 as uint16_t,
        },
        name_keysym {
            keysym: 0x100018f as xkb_keysym_t,
            offset: 22873 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000259 as xkb_keysym_t,
            offset: 22879 as uint16_t,
        },
        name_keysym {
            keysym: 0x2de as xkb_keysym_t,
            offset: 22885 as uint16_t,
        },
        name_keysym {
            keysym: 0x2fe as xkb_keysym_t,
            offset: 22897 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 22909 as uint16_t,
        },
        name_keysym {
            keysym: 0xff14 as xkb_keysym_t,
            offset: 22923 as uint16_t,
        },
        name_keysym {
            keysym: 0xad7 as xkb_keysym_t,
            offset: 22935 as uint16_t,
        },
        name_keysym {
            keysym: 0xa7 as xkb_keysym_t,
            offset: 22943 as uint16_t,
        },
        name_keysym {
            keysym: 0xff60 as xkb_keysym_t,
            offset: 22951 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b as xkb_keysym_t,
            offset: 22958 as uint16_t,
        },
        name_keysym {
            keysym: 0x4df as xkb_keysym_t,
            offset: 22968 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b1 as xkb_keysym_t,
            offset: 22984 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a1 as xkb_keysym_t,
            offset: 22996 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bf as xkb_keysym_t,
            offset: 23008 as uint16_t,
        },
        name_keysym {
            keysym: 0x6af as xkb_keysym_t,
            offset: 23020 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b8 as xkb_keysym_t,
            offset: 23032 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a8 as xkb_keysym_t,
            offset: 23043 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b9 as xkb_keysym_t,
            offset: 23054 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a9 as xkb_keysym_t,
            offset: 23066 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ba as xkb_keysym_t,
            offset: 23078 as uint16_t,
        },
        name_keysym {
            keysym: 0x6aa as xkb_keysym_t,
            offset: 23090 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bb as xkb_keysym_t,
            offset: 23102 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ab as xkb_keysym_t,
            offset: 23115 as uint16_t,
        },
        name_keysym {
            keysym: 0xac6 as xkb_keysym_t,
            offset: 23128 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002087 as xkb_keysym_t,
            offset: 23141 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002077 as xkb_keysym_t,
            offset: 23156 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe1 as xkb_keysym_t,
            offset: 23170 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe6 as xkb_keysym_t,
            offset: 23178 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe2 as xkb_keysym_t,
            offset: 23189 as uint16_t,
        },
        name_keysym {
            keysym: 0xaca as xkb_keysym_t,
            offset: 23197 as uint16_t,
        },
        name_keysym {
            keysym: 0xaac as xkb_keysym_t,
            offset: 23211 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c9 as xkb_keysym_t,
            offset: 23223 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3c as xkb_keysym_t,
            offset: 23236 as uint16_t,
        },
        name_keysym {
            keysym: 0xafd as xkb_keysym_t,
            offset: 23252 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d85 as xkb_keysym_t,
            offset: 23271 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d86 as xkb_keysym_t,
            offset: 23278 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dcf as xkb_keysym_t,
            offset: 23286 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d87 as xkb_keysym_t,
            offset: 23295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd0 as xkb_keysym_t,
            offset: 23303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d88 as xkb_keysym_t,
            offset: 23312 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd1 as xkb_keysym_t,
            offset: 23321 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d93 as xkb_keysym_t,
            offset: 23331 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddb as xkb_keysym_t,
            offset: 23339 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dca as xkb_keysym_t,
            offset: 23348 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d96 as xkb_keysym_t,
            offset: 23356 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dde as xkb_keysym_t,
            offset: 23364 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db6 as xkb_keysym_t,
            offset: 23373 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db7 as xkb_keysym_t,
            offset: 23381 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da0 as xkb_keysym_t,
            offset: 23390 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da1 as xkb_keysym_t,
            offset: 23398 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da9 as xkb_keysym_t,
            offset: 23407 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000daa as xkb_keysym_t,
            offset: 23416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000daf as xkb_keysym_t,
            offset: 23426 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db0 as xkb_keysym_t,
            offset: 23435 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d91 as xkb_keysym_t,
            offset: 23445 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd9 as xkb_keysym_t,
            offset: 23452 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d92 as xkb_keysym_t,
            offset: 23460 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dda as xkb_keysym_t,
            offset: 23468 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc6 as xkb_keysym_t,
            offset: 23477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9c as xkb_keysym_t,
            offset: 23485 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9d as xkb_keysym_t,
            offset: 23493 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d83 as xkb_keysym_t,
            offset: 23502 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc4 as xkb_keysym_t,
            offset: 23510 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d89 as xkb_keysym_t,
            offset: 23518 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd2 as xkb_keysym_t,
            offset: 23525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8a as xkb_keysym_t,
            offset: 23533 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd3 as xkb_keysym_t,
            offset: 23541 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da2 as xkb_keysym_t,
            offset: 23550 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da3 as xkb_keysym_t,
            offset: 23558 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da5 as xkb_keysym_t,
            offset: 23567 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9a as xkb_keysym_t,
            offset: 23577 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9b as xkb_keysym_t,
            offset: 23585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df4 as xkb_keysym_t,
            offset: 23594 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dbd as xkb_keysym_t,
            offset: 23610 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc5 as xkb_keysym_t,
            offset: 23618 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8f as xkb_keysym_t,
            offset: 23627 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddf as xkb_keysym_t,
            offset: 23635 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d90 as xkb_keysym_t,
            offset: 23644 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df3 as xkb_keysym_t,
            offset: 23653 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db8 as xkb_keysym_t,
            offset: 23663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db9 as xkb_keysym_t,
            offset: 23671 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db1 as xkb_keysym_t,
            offset: 23680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dac as xkb_keysym_t,
            offset: 23688 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db3 as xkb_keysym_t,
            offset: 23698 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d82 as xkb_keysym_t,
            offset: 23708 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9e as xkb_keysym_t,
            offset: 23716 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9f as xkb_keysym_t,
            offset: 23725 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da6 as xkb_keysym_t,
            offset: 23734 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dab as xkb_keysym_t,
            offset: 23743 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da4 as xkb_keysym_t,
            offset: 23752 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d94 as xkb_keysym_t,
            offset: 23761 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddc as xkb_keysym_t,
            offset: 23768 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d95 as xkb_keysym_t,
            offset: 23776 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddd as xkb_keysym_t,
            offset: 23784 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db4 as xkb_keysym_t,
            offset: 23793 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db5 as xkb_keysym_t,
            offset: 23801 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dbb as xkb_keysym_t,
            offset: 23810 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8d as xkb_keysym_t,
            offset: 23818 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8e as xkb_keysym_t,
            offset: 23826 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd8 as xkb_keysym_t,
            offset: 23835 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df2 as xkb_keysym_t,
            offset: 23844 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc3 as xkb_keysym_t,
            offset: 23854 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc1 as xkb_keysym_t,
            offset: 23862 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc2 as xkb_keysym_t,
            offset: 23871 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dad as xkb_keysym_t,
            offset: 23881 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dae as xkb_keysym_t,
            offset: 23890 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da7 as xkb_keysym_t,
            offset: 23900 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da8 as xkb_keysym_t,
            offset: 23909 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8b as xkb_keysym_t,
            offset: 23919 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd4 as xkb_keysym_t,
            offset: 23926 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8c as xkb_keysym_t,
            offset: 23934 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd6 as xkb_keysym_t,
            offset: 23942 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc0 as xkb_keysym_t,
            offset: 23951 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dba as xkb_keysym_t,
            offset: 23959 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002086 as xkb_keysym_t,
            offset: 23967 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002076 as xkb_keysym_t,
            offset: 23980 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f as xkb_keysym_t,
            offset: 23992 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe73 as xkb_keysym_t,
            offset: 23998 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e0 as xkb_keysym_t,
            offset: 24014 as uint16_t,
        },
        name_keysym {
            keysym: 0x20 as xkb_keysym_t,
            offset: 24027 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221a as xkb_keysym_t,
            offset: 24033 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e9e as xkb_keysym_t,
            offset: 24044 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf as xkb_keysym_t,
            offset: 24051 as uint16_t,
        },
        name_keysym {
            keysym: 0xa3 as xkb_keysym_t,
            offset: 24058 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe75 as xkb_keysym_t,
            offset: 24067 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002263 as xkb_keysym_t,
            offset: 24085 as uint16_t,
        },
        name_keysym {
            keysym: 0xff66 as xkb_keysym_t,
            offset: 24094 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 24103 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff77 as xkb_keysym_t,
            offset: 24115 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff78 as xkb_keysym_t,
            offset: 24135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff79 as xkb_keysym_t,
            offset: 24148 as uint16_t,
        },
        name_keysym {
            keysym: 0xff20 as xkb_keysym_t,
            offset: 24168 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff72 as xkb_keysym_t,
            offset: 24179 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff75 as xkb_keysym_t,
            offset: 24187 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff10 as xkb_keysym_t,
            offset: 24194 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff11 as xkb_keysym_t,
            offset: 24201 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff03 as xkb_keysym_t,
            offset: 24208 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff05 as xkb_keysym_t,
            offset: 24220 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff01 as xkb_keysym_t,
            offset: 24234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff04 as xkb_keysym_t,
            offset: 24247 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff00 as xkb_keysym_t,
            offset: 24263 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff02 as xkb_keysym_t,
            offset: 24275 as uint16_t,
        },
        name_keysym {
            keysym: 0xff68 as xkb_keysym_t,
            offset: 24287 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff71 as xkb_keysym_t,
            offset: 24295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff73 as xkb_keysym_t,
            offset: 24304 as uint16_t,
        },
        name_keysym {
            keysym: 0xff56 as xkb_keysym_t,
            offset: 24312 as uint16_t,
        },
        name_keysym {
            keysym: 0xff55 as xkb_keysym_t,
            offset: 24324 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff74 as xkb_keysym_t,
            offset: 24334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff76 as xkb_keysym_t,
            offset: 24343 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7d as xkb_keysym_t,
            offset: 24358 as uint16_t,
        },
        name_keysym {
            keysym: 0xff61 as xkb_keysym_t,
            offset: 24378 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff70 as xkb_keysym_t,
            offset: 24394 as uint16_t,
        },
        name_keysym {
            keysym: 0xff69 as xkb_keysym_t,
            offset: 24403 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff60 as xkb_keysym_t,
            offset: 24411 as uint16_t,
        },
        name_keysym {
            keysym: 0xff65 as xkb_keysym_t,
            offset: 24422 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7a as xkb_keysym_t,
            offset: 24430 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7b as xkb_keysym_t,
            offset: 24446 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7c as xkb_keysym_t,
            offset: 24470 as uint16_t,
        },
        name_keysym {
            keysym: 0xffeb as xkb_keysym_t,
            offset: 24494 as uint16_t,
        },
        name_keysym {
            keysym: 0xffec as xkb_keysym_t,
            offset: 24502 as uint16_t,
        },
        name_keysym {
            keysym: 0xff15 as xkb_keysym_t,
            offset: 24510 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6d as xkb_keysym_t,
            offset: 24518 as uint16_t,
        },
        name_keysym {
            keysym: 0x54 as xkb_keysym_t,
            offset: 24525 as uint16_t,
        },
        name_keysym {
            keysym: 0x74 as xkb_keysym_t,
            offset: 24527 as uint16_t,
        },
        name_keysym {
            keysym: 0xff09 as xkb_keysym_t,
            offset: 24529 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e6a as xkb_keysym_t,
            offset: 24533 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e6b as xkb_keysym_t,
            offset: 24543 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ab as xkb_keysym_t,
            offset: 24553 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bb as xkb_keysym_t,
            offset: 24560 as uint16_t,
        },
        name_keysym {
            keysym: 0x1de as xkb_keysym_t,
            offset: 24567 as uint16_t,
        },
        name_keysym {
            keysym: 0x1fe as xkb_keysym_t,
            offset: 24576 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf9 as xkb_keysym_t,
            offset: 24585 as uint16_t,
        },
        name_keysym {
            keysym: 0xafa as xkb_keysym_t,
            offset: 24595 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed5 as xkb_keysym_t,
            offset: 24613 as uint16_t,
        },
        name_keysym {
            keysym: 0xddf as xkb_keysym_t,
            offset: 24630 as uint16_t,
        },
        name_keysym {
            keysym: 0xdba as xkb_keysym_t,
            offset: 24640 as uint16_t,
        },
        name_keysym {
            keysym: 0xda8 as xkb_keysym_t,
            offset: 24654 as uint16_t,
        },
        name_keysym {
            keysym: 0xdaa as xkb_keysym_t,
            offset: 24667 as uint16_t,
        },
        name_keysym {
            keysym: 0xda9 as xkb_keysym_t,
            offset: 24681 as uint16_t,
        },
        name_keysym {
            keysym: 0xdac as xkb_keysym_t,
            offset: 24695 as uint16_t,
        },
        name_keysym {
            keysym: 0xdae as xkb_keysym_t,
            offset: 24708 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb4 as xkb_keysym_t,
            offset: 24721 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbd as xkb_keysym_t,
            offset: 24732 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbf as xkb_keysym_t,
            offset: 24742 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcb as xkb_keysym_t,
            offset: 24753 as uint16_t,
        },
        name_keysym {
            keysym: 0xdce as xkb_keysym_t,
            offset: 24764 as uint16_t,
        },
        name_keysym {
            keysym: 0xda2 as xkb_keysym_t,
            offset: 24778 as uint16_t,
        },
        name_keysym {
            keysym: 0xda5 as xkb_keysym_t,
            offset: 24791 as uint16_t,
        },
        name_keysym {
            keysym: 0xda3 as xkb_keysym_t,
            offset: 24804 as uint16_t,
        },
        name_keysym {
            keysym: 0xda4 as xkb_keysym_t,
            offset: 24818 as uint16_t,
        },
        name_keysym {
            keysym: 0xda6 as xkb_keysym_t,
            offset: 24832 as uint16_t,
        },
        name_keysym {
            keysym: 0xda1 as xkb_keysym_t,
            offset: 24848 as uint16_t,
        },
        name_keysym {
            keysym: 0xde5 as xkb_keysym_t,
            offset: 24859 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf7 as xkb_keysym_t,
            offset: 24876 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf5 as xkb_keysym_t,
            offset: 24889 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf6 as xkb_keysym_t,
            offset: 24900 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf9 as xkb_keysym_t,
            offset: 24912 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf1 as xkb_keysym_t,
            offset: 24924 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf8 as xkb_keysym_t,
            offset: 24937 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf3 as xkb_keysym_t,
            offset: 24950 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf4 as xkb_keysym_t,
            offset: 24962 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf2 as xkb_keysym_t,
            offset: 24973 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf0 as xkb_keysym_t,
            offset: 24986 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcc as xkb_keysym_t,
            offset: 24998 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc5 as xkb_keysym_t,
            offset: 25011 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc6 as xkb_keysym_t,
            offset: 25023 as uint16_t,
        },
        name_keysym {
            keysym: 0xdeb as xkb_keysym_t,
            offset: 25031 as uint16_t,
        },
        name_keysym {
            keysym: 0xde8 as xkb_keysym_t,
            offset: 25048 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd1 as xkb_keysym_t,
            offset: 25059 as uint16_t,
        },
        name_keysym {
            keysym: 0xdde as xkb_keysym_t,
            offset: 25075 as uint16_t,
        },
        name_keysym {
            keysym: 0xde7 as xkb_keysym_t,
            offset: 25098 as uint16_t,
        },
        name_keysym {
            keysym: 0xde9 as xkb_keysym_t,
            offset: 25113 as uint16_t,
        },
        name_keysym {
            keysym: 0xdea as xkb_keysym_t,
            offset: 25125 as uint16_t,
        },
        name_keysym {
            keysym: 0xde6 as xkb_keysym_t,
            offset: 25137 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc1 as xkb_keysym_t,
            offset: 25151 as uint16_t,
        },
        name_keysym {
            keysym: 0xda7 as xkb_keysym_t,
            offset: 25161 as uint16_t,
        },
        name_keysym {
            keysym: 0xded as xkb_keysym_t,
            offset: 25173 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb3 as xkb_keysym_t,
            offset: 25187 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb9 as xkb_keysym_t,
            offset: 25198 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcd as xkb_keysym_t,
            offset: 25208 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcf as xkb_keysym_t,
            offset: 25218 as uint16_t,
        },
        name_keysym {
            keysym: 0xdda as xkb_keysym_t,
            offset: 25233 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbe as xkb_keysym_t,
            offset: 25246 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbc as xkb_keysym_t,
            offset: 25259 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc0 as xkb_keysym_t,
            offset: 25273 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbb as xkb_keysym_t,
            offset: 25289 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc3 as xkb_keysym_t,
            offset: 25300 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc4 as xkb_keysym_t,
            offset: 25311 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd0 as xkb_keysym_t,
            offset: 25319 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd2 as xkb_keysym_t,
            offset: 25330 as uint16_t,
        },
        name_keysym {
            keysym: 0xde1 as xkb_keysym_t,
            offset: 25342 as uint16_t,
        },
        name_keysym {
            keysym: 0xde4 as xkb_keysym_t,
            offset: 25354 as uint16_t,
        },
        name_keysym {
            keysym: 0xde3 as xkb_keysym_t,
            offset: 25374 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd3 as xkb_keysym_t,
            offset: 25393 as uint16_t,
        },
        name_keysym {
            keysym: 0xde0 as xkb_keysym_t,
            offset: 25405 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd4 as xkb_keysym_t,
            offset: 25416 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd5 as xkb_keysym_t,
            offset: 25427 as uint16_t,
        },
        name_keysym {
            keysym: 0xde2 as xkb_keysym_t,
            offset: 25439 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd8 as xkb_keysym_t,
            offset: 25450 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd6 as xkb_keysym_t,
            offset: 25461 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd7 as xkb_keysym_t,
            offset: 25473 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd9 as xkb_keysym_t,
            offset: 25486 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc9 as xkb_keysym_t,
            offset: 25498 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc8 as xkb_keysym_t,
            offset: 25510 as uint16_t,
        },
        name_keysym {
            keysym: 0xdab as xkb_keysym_t,
            offset: 25522 as uint16_t,
        },
        name_keysym {
            keysym: 0xdca as xkb_keysym_t,
            offset: 25532 as uint16_t,
        },
        name_keysym {
            keysym: 0xdec as xkb_keysym_t,
            offset: 25543 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb1 as xkb_keysym_t,
            offset: 25560 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb2 as xkb_keysym_t,
            offset: 25579 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb7 as xkb_keysym_t,
            offset: 25595 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb0 as xkb_keysym_t,
            offset: 25610 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb8 as xkb_keysym_t,
            offset: 25623 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb6 as xkb_keysym_t,
            offset: 25637 as uint16_t,
        },
        name_keysym {
            keysym: 0xdaf as xkb_keysym_t,
            offset: 25651 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb5 as xkb_keysym_t,
            offset: 25664 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc7 as xkb_keysym_t,
            offset: 25675 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc2 as xkb_keysym_t,
            offset: 25687 as uint16_t,
        },
        name_keysym {
            keysym: 0xdad as xkb_keysym_t,
            offset: 25698 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c0 as xkb_keysym_t,
            offset: 25710 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa7 as xkb_keysym_t,
            offset: 25720 as uint16_t,
        },
        name_keysym {
            keysym: 0xde as xkb_keysym_t,
            offset: 25730 as uint16_t,
        },
        name_keysym {
            keysym: 0xde as xkb_keysym_t,
            offset: 25736 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe as xkb_keysym_t,
            offset: 25742 as uint16_t,
        },
        name_keysym {
            keysym: 0xac4 as xkb_keysym_t,
            offset: 25748 as uint16_t,
        },
        name_keysym {
            keysym: 0xab4 as xkb_keysym_t,
            offset: 25761 as uint16_t,
        },
        name_keysym {
            keysym: 0xbe as xkb_keysym_t,
            offset: 25773 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002083 as xkb_keysym_t,
            offset: 25787 as uint16_t,
        },
        name_keysym {
            keysym: 0xb3 as xkb_keysym_t,
            offset: 25802 as uint16_t,
        },
        name_keysym {
            keysym: 0x100222d as xkb_keysym_t,
            offset: 25816 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a4 as xkb_keysym_t,
            offset: 25826 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ab as xkb_keysym_t,
            offset: 25838 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a2 as xkb_keysym_t,
            offset: 25852 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a7 as xkb_keysym_t,
            offset: 25867 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b1 as xkb_keysym_t,
            offset: 25884 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ad as xkb_keysym_t,
            offset: 25901 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a9 as xkb_keysym_t,
            offset: 25916 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b5 as xkb_keysym_t,
            offset: 25934 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f7 as xkb_keysym_t,
            offset: 25952 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b3 as xkb_keysym_t,
            offset: 25957 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2b as xkb_keysym_t,
            offset: 25983 as uint16_t,
        },
        name_keysym {
            keysym: 0xac9 as xkb_keysym_t,
            offset: 25991 as uint16_t,
        },
        name_keysym {
            keysym: 0xacb as xkb_keysym_t,
            offset: 26001 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ac as xkb_keysym_t,
            offset: 26019 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bc as xkb_keysym_t,
            offset: 26026 as uint16_t,
        },
        name_keysym {
            keysym: 0xab3 as xkb_keysym_t,
            offset: 26033 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002082 as xkb_keysym_t,
            offset: 26043 as uint16_t,
        },
        name_keysym {
            keysym: 0xb2 as xkb_keysym_t,
            offset: 26056 as uint16_t,
        },
        name_keysym {
            keysym: 0xab1 as xkb_keysym_t,
            offset: 26068 as uint16_t,
        },
        name_keysym {
            keysym: 0x55 as xkb_keysym_t,
            offset: 26078 as uint16_t,
        },
        name_keysym {
            keysym: 0x75 as xkb_keysym_t,
            offset: 26080 as uint16_t,
        },
        name_keysym {
            keysym: 0xda as xkb_keysym_t,
            offset: 26082 as uint16_t,
        },
        name_keysym {
            keysym: 0xfa as xkb_keysym_t,
            offset: 26089 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee4 as xkb_keysym_t,
            offset: 26096 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee5 as xkb_keysym_t,
            offset: 26106 as uint16_t,
        },
        name_keysym {
            keysym: 0x2dd as xkb_keysym_t,
            offset: 26116 as uint16_t,
        },
        name_keysym {
            keysym: 0x2fd as xkb_keysym_t,
            offset: 26123 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb as xkb_keysym_t,
            offset: 26130 as uint16_t,
        },
        name_keysym {
            keysym: 0xfb as xkb_keysym_t,
            offset: 26142 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc as xkb_keysym_t,
            offset: 26154 as uint16_t,
        },
        name_keysym {
            keysym: 0xfc as xkb_keysym_t,
            offset: 26165 as uint16_t,
        },
        name_keysym {
            keysym: 0x1db as xkb_keysym_t,
            offset: 26176 as uint16_t,
        },
        name_keysym {
            keysym: 0x1fb as xkb_keysym_t,
            offset: 26189 as uint16_t,
        },
        name_keysym {
            keysym: 0xd9 as xkb_keysym_t,
            offset: 26202 as uint16_t,
        },
        name_keysym {
            keysym: 0xf9 as xkb_keysym_t,
            offset: 26209 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee6 as xkb_keysym_t,
            offset: 26216 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee7 as xkb_keysym_t,
            offset: 26222 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001af as xkb_keysym_t,
            offset: 26228 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b0 as xkb_keysym_t,
            offset: 26234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee8 as xkb_keysym_t,
            offset: 26240 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee9 as xkb_keysym_t,
            offset: 26251 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef0 as xkb_keysym_t,
            offset: 26262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef1 as xkb_keysym_t,
            offset: 26276 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eea as xkb_keysym_t,
            offset: 26290 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eeb as xkb_keysym_t,
            offset: 26301 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eec as xkb_keysym_t,
            offset: 26312 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eed as xkb_keysym_t,
            offset: 26322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eee as xkb_keysym_t,
            offset: 26332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eef as xkb_keysym_t,
            offset: 26343 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bd as xkb_keysym_t,
            offset: 26354 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ad as xkb_keysym_t,
            offset: 26380 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b6 as xkb_keysym_t,
            offset: 26406 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a6 as xkb_keysym_t,
            offset: 26418 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b4 as xkb_keysym_t,
            offset: 26430 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a4 as xkb_keysym_t,
            offset: 26443 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b7 as xkb_keysym_t,
            offset: 26456 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a7 as xkb_keysym_t,
            offset: 26469 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b6 as xkb_keysym_t,
            offset: 26482 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a6 as xkb_keysym_t,
            offset: 26493 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b4 as xkb_keysym_t,
            offset: 26504 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a4 as xkb_keysym_t,
            offset: 26516 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b7 as xkb_keysym_t,
            offset: 26528 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a7 as xkb_keysym_t,
            offset: 26540 as uint16_t,
        },
        name_keysym {
            keysym: 0x3de as xkb_keysym_t,
            offset: 26552 as uint16_t,
        },
        name_keysym {
            keysym: 0x3fe as xkb_keysym_t,
            offset: 26560 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc6 as xkb_keysym_t,
            offset: 26568 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f as xkb_keysym_t,
            offset: 26577 as uint16_t,
        },
        name_keysym {
            keysym: 0xff65 as xkb_keysym_t,
            offset: 26588 as uint16_t,
        },
        name_keysym {
            keysym: 0x8dd as xkb_keysym_t,
            offset: 26593 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d9 as xkb_keysym_t,
            offset: 26599 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f9 as xkb_keysym_t,
            offset: 26607 as uint16_t,
        },
        name_keysym {
            keysym: 0xff52 as xkb_keysym_t,
            offset: 26615 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fc as xkb_keysym_t,
            offset: 26618 as uint16_t,
        },
        name_keysym {
            keysym: 0xba9 as xkb_keysym_t,
            offset: 26626 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ec as xkb_keysym_t,
            offset: 26634 as uint16_t,
        },
        name_keysym {
            keysym: 0x9eb as xkb_keysym_t,
            offset: 26647 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc3 as xkb_keysym_t,
            offset: 26661 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd3 as xkb_keysym_t,
            offset: 26668 as uint16_t,
        },
        name_keysym {
            keysym: 0xbce as xkb_keysym_t,
            offset: 26676 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d9 as xkb_keysym_t,
            offset: 26683 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f9 as xkb_keysym_t,
            offset: 26689 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6e as xkb_keysym_t,
            offset: 26695 as uint16_t,
        },
        name_keysym {
            keysym: 0x3dd as xkb_keysym_t,
            offset: 26700 as uint16_t,
        },
        name_keysym {
            keysym: 0x3fd as xkb_keysym_t,
            offset: 26707 as uint16_t,
        },
        name_keysym {
            keysym: 0x56 as xkb_keysym_t,
            offset: 26714 as uint16_t,
        },
        name_keysym {
            keysym: 0x76 as xkb_keysym_t,
            offset: 26716 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c1 as xkb_keysym_t,
            offset: 26718 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f8 as xkb_keysym_t,
            offset: 26728 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a6 as xkb_keysym_t,
            offset: 26736 as uint16_t,
        },
        name_keysym {
            keysym: 0x4de as xkb_keysym_t,
            offset: 26750 as uint16_t,
        },
        name_keysym {
            keysym: 0xffffff as xkb_keysym_t,
            offset: 26762 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e9 as xkb_keysym_t,
            offset: 26773 as uint16_t,
        },
        name_keysym {
            keysym: 0x57 as xkb_keysym_t,
            offset: 26776 as uint16_t,
        },
        name_keysym {
            keysym: 0x77 as xkb_keysym_t,
            offset: 26778 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e82 as xkb_keysym_t,
            offset: 26780 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e83 as xkb_keysym_t,
            offset: 26787 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000174 as xkb_keysym_t,
            offset: 26794 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000175 as xkb_keysym_t,
            offset: 26806 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e84 as xkb_keysym_t,
            offset: 26818 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e85 as xkb_keysym_t,
            offset: 26829 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e80 as xkb_keysym_t,
            offset: 26840 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e81 as xkb_keysym_t,
            offset: 26847 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a9 as xkb_keysym_t,
            offset: 26854 as uint16_t,
        },
        name_keysym {
            keysym: 0x58 as xkb_keysym_t,
            offset: 26862 as uint16_t,
        },
        name_keysym {
            keysym: 0x78 as xkb_keysym_t,
            offset: 26864 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e8a as xkb_keysym_t,
            offset: 26866 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e8b as xkb_keysym_t,
            offset: 26876 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b9 as xkb_keysym_t,
            offset: 26886 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b8 as xkb_keysym_t,
            offset: 26905 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126f as xkb_keysym_t,
            offset: 26922 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124e as xkb_keysym_t,
            offset: 26933 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff39 as xkb_keysym_t,
            offset: 26951 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ad as xkb_keysym_t,
            offset: 26967 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081230 as xkb_keysym_t,
            offset: 26983 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff50 as xkb_keysym_t,
            offset: 26997 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff51 as xkb_keysym_t,
            offset: 27017 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081244 as xkb_keysym_t,
            offset: 27038 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081177 as xkb_keysym_t,
            offset: 27052 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081247 as xkb_keysym_t,
            offset: 27068 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121c as xkb_keysym_t,
            offset: 27082 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121b as xkb_keysym_t,
            offset: 27099 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121d as xkb_keysym_t,
            offset: 27115 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081188 as xkb_keysym_t,
            offset: 27135 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810d1 as xkb_keysym_t,
            offset: 27145 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081175 as xkb_keysym_t,
            offset: 27164 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9b as xkb_keysym_t,
            offset: 27185 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126e as xkb_keysym_t,
            offset: 27205 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff97 as xkb_keysym_t,
            offset: 27219 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff11 as xkb_keysym_t,
            offset: 27236 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff32 as xkb_keysym_t,
            offset: 27257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb2 as xkb_keysym_t,
            offset: 27272 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff12 as xkb_keysym_t,
            offset: 27289 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff17 as xkb_keysym_t,
            offset: 27303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff31 as xkb_keysym_t,
            offset: 27317 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff14 as xkb_keysym_t,
            offset: 27332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb6 as xkb_keysym_t,
            offset: 27346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff16 as xkb_keysym_t,
            offset: 27362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff13 as xkb_keysym_t,
            offset: 27376 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff99 as xkb_keysym_t,
            offset: 27397 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1c as xkb_keysym_t,
            offset: 27417 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff98 as xkb_keysym_t,
            offset: 27433 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3e as xkb_keysym_t,
            offset: 27449 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff15 as xkb_keysym_t,
            offset: 27465 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127d as xkb_keysym_t,
            offset: 27479 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8d as xkb_keysym_t,
            offset: 27505 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff26 as xkb_keysym_t,
            offset: 27514 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3f as xkb_keysym_t,
            offset: 27523 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff93 as xkb_keysym_t,
            offset: 27539 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa6 as xkb_keysym_t,
            offset: 27551 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff94 as xkb_keysym_t,
            offset: 27560 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff52 as xkb_keysym_t,
            offset: 27574 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008119b as xkb_keysym_t,
            offset: 27583 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3b as xkb_keysym_t,
            offset: 27593 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810f4 as xkb_keysym_t,
            offset: 27614 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081251 as xkb_keysym_t,
            offset: 27633 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081250 as xkb_keysym_t,
            offset: 27651 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081240 as xkb_keysym_t,
            offset: 27669 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff54 as xkb_keysym_t,
            offset: 27686 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1d as xkb_keysym_t,
            offset: 27701 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff20 as xkb_keysym_t,
            offset: 27716 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124c as xkb_keysym_t,
            offset: 27729 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124b as xkb_keysym_t,
            offset: 27753 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124d as xkb_keysym_t,
            offset: 27776 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081218 as xkb_keysym_t,
            offset: 27799 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081210 as xkb_keysym_t,
            offset: 27814 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081219 as xkb_keysym_t,
            offset: 27830 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121a as xkb_keysym_t,
            offset: 27845 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081217 as xkb_keysym_t,
            offset: 27861 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081215 as xkb_keysym_t,
            offset: 27874 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081216 as xkb_keysym_t,
            offset: 27891 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff53 as xkb_keysym_t,
            offset: 27909 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081193 as xkb_keysym_t,
            offset: 27916 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081192 as xkb_keysym_t,
            offset: 27932 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff55 as xkb_keysym_t,
            offset: 27946 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe21 as xkb_keysym_t,
            offset: 27956 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081286 as xkb_keysym_t,
            offset: 27970 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff56 as xkb_keysym_t,
            offset: 27987 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3d as xkb_keysym_t,
            offset: 27997 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b6 as xkb_keysym_t,
            offset: 28011 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff22 as xkb_keysym_t,
            offset: 28027 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081243 as xkb_keysym_t,
            offset: 28046 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff57 as xkb_keysym_t,
            offset: 28063 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff58 as xkb_keysym_t,
            offset: 28072 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9c as xkb_keysym_t,
            offset: 28080 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081277 as xkb_keysym_t,
            offset: 28095 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811aa as xkb_keysym_t,
            offset: 28104 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124a as xkb_keysym_t,
            offset: 28117 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff59 as xkb_keysym_t,
            offset: 28129 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810f5 as xkb_keysym_t,
            offset: 28141 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811af as xkb_keysym_t,
            offset: 28156 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5b as xkb_keysym_t,
            offset: 28174 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124f as xkb_keysym_t,
            offset: 28188 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5a as xkb_keysym_t,
            offset: 28205 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081283 as xkb_keysym_t,
            offset: 28213 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081185 as xkb_keysym_t,
            offset: 28232 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a6 as xkb_keysym_t,
            offset: 28240 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2c as xkb_keysym_t,
            offset: 28251 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081253 as xkb_keysym_t,
            offset: 28261 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081252 as xkb_keysym_t,
            offset: 28292 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081249 as xkb_keysym_t,
            offset: 28322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5c as xkb_keysym_t,
            offset: 28338 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810ae as xkb_keysym_t,
            offset: 28348 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5d as xkb_keysym_t,
            offset: 28357 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081275 as xkb_keysym_t,
            offset: 28370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff30 as xkb_keysym_t,
            offset: 28386 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3c as xkb_keysym_t,
            offset: 28400 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081281 as xkb_keysym_t,
            offset: 28412 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d0 as xkb_keysym_t,
            offset: 28429 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811de as xkb_keysym_t,
            offset: 28436 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811df as xkb_keysym_t,
            offset: 28445 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e4 as xkb_keysym_t,
            offset: 28454 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e0 as xkb_keysym_t,
            offset: 28463 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e1 as xkb_keysym_t,
            offset: 28472 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d1 as xkb_keysym_t,
            offset: 28481 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e2 as xkb_keysym_t,
            offset: 28492 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d2 as xkb_keysym_t,
            offset: 28501 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811db as xkb_keysym_t,
            offset: 28511 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811dc as xkb_keysym_t,
            offset: 28522 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811dd as xkb_keysym_t,
            offset: 28533 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d3 as xkb_keysym_t,
            offset: 28544 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d4 as xkb_keysym_t,
            offset: 28554 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d5 as xkb_keysym_t,
            offset: 28564 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d6 as xkb_keysym_t,
            offset: 28574 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d7 as xkb_keysym_t,
            offset: 28584 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d8 as xkb_keysym_t,
            offset: 28594 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d9 as xkb_keysym_t,
            offset: 28604 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811da as xkb_keysym_t,
            offset: 28614 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e3 as xkb_keysym_t,
            offset: 28624 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e5 as xkb_keysym_t,
            offset: 28633 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff27 as xkb_keysym_t,
            offset: 28650 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9d as xkb_keysym_t,
            offset: 28662 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9e as xkb_keysym_t,
            offset: 28676 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb8 as xkb_keysym_t,
            offset: 28693 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5e as xkb_keysym_t,
            offset: 28708 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5f as xkb_keysym_t,
            offset: 28717 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081162 as xkb_keysym_t,
            offset: 28724 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a8 as xkb_keysym_t,
            offset: 28733 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa4 as xkb_keysym_t,
            offset: 28752 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811be as xkb_keysym_t,
            offset: 28762 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa8 as xkb_keysym_t,
            offset: 28778 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff37 as xkb_keysym_t,
            offset: 28792 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff18 as xkb_keysym_t,
            offset: 28804 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3a as xkb_keysym_t,
            offset: 28817 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ba as xkb_keysym_t,
            offset: 28830 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081166 as xkb_keysym_t,
            offset: 28841 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff60 as xkb_keysym_t,
            offset: 28850 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081242 as xkb_keysym_t,
            offset: 28861 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff06 as xkb_keysym_t,
            offset: 28873 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff05 as xkb_keysym_t,
            offset: 28895 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081264 as xkb_keysym_t,
            offset: 28915 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081265 as xkb_keysym_t,
            offset: 28940 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081261 as xkb_keysym_t,
            offset: 28965 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081263 as xkb_keysym_t,
            offset: 28988 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081260 as xkb_keysym_t,
            offset: 29016 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081262 as xkb_keysym_t,
            offset: 29039 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b8 as xkb_keysym_t,
            offset: 29067 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b9 as xkb_keysym_t,
            offset: 29083 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ba as xkb_keysym_t,
            offset: 29099 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bb as xkb_keysym_t,
            offset: 29115 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bc as xkb_keysym_t,
            offset: 29131 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff04 as xkb_keysym_t,
            offset: 29147 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb3 as xkb_keysym_t,
            offset: 29165 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff40 as xkb_keysym_t,
            offset: 29178 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff41 as xkb_keysym_t,
            offset: 29190 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff42 as xkb_keysym_t,
            offset: 29202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff43 as xkb_keysym_t,
            offset: 29214 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff44 as xkb_keysym_t,
            offset: 29226 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff45 as xkb_keysym_t,
            offset: 29238 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff46 as xkb_keysym_t,
            offset: 29250 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff47 as xkb_keysym_t,
            offset: 29262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff48 as xkb_keysym_t,
            offset: 29274 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff49 as xkb_keysym_t,
            offset: 29286 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4a as xkb_keysym_t,
            offset: 29298 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4b as xkb_keysym_t,
            offset: 29310 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4c as xkb_keysym_t,
            offset: 29322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4d as xkb_keysym_t,
            offset: 29334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4e as xkb_keysym_t,
            offset: 29346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4f as xkb_keysym_t,
            offset: 29358 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081269 as xkb_keysym_t,
            offset: 29370 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081268 as xkb_keysym_t,
            offset: 29383 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff35 as xkb_keysym_t,
            offset: 29394 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121e as xkb_keysym_t,
            offset: 29408 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bf as xkb_keysym_t,
            offset: 29425 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe25 as xkb_keysym_t,
            offset: 29439 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff61 as xkb_keysym_t,
            offset: 29455 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe24 as xkb_keysym_t,
            offset: 29466 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081290 as xkb_keysym_t,
            offset: 29484 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081299 as xkb_keysym_t,
            offset: 29495 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129a as xkb_keysym_t,
            offset: 29507 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129b as xkb_keysym_t,
            offset: 29519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129c as xkb_keysym_t,
            offset: 29531 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129d as xkb_keysym_t,
            offset: 29543 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129e as xkb_keysym_t,
            offset: 29555 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129f as xkb_keysym_t,
            offset: 29567 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a0 as xkb_keysym_t,
            offset: 29579 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a1 as xkb_keysym_t,
            offset: 29591 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a2 as xkb_keysym_t,
            offset: 29603 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081291 as xkb_keysym_t,
            offset: 29615 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a3 as xkb_keysym_t,
            offset: 29626 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a4 as xkb_keysym_t,
            offset: 29638 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a5 as xkb_keysym_t,
            offset: 29650 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a6 as xkb_keysym_t,
            offset: 29662 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a7 as xkb_keysym_t,
            offset: 29674 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a8 as xkb_keysym_t,
            offset: 29686 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a9 as xkb_keysym_t,
            offset: 29698 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812aa as xkb_keysym_t,
            offset: 29710 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ab as xkb_keysym_t,
            offset: 29722 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ac as xkb_keysym_t,
            offset: 29734 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081292 as xkb_keysym_t,
            offset: 29746 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ad as xkb_keysym_t,
            offset: 29757 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081293 as xkb_keysym_t,
            offset: 29769 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081294 as xkb_keysym_t,
            offset: 29780 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081295 as xkb_keysym_t,
            offset: 29791 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081296 as xkb_keysym_t,
            offset: 29802 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081297 as xkb_keysym_t,
            offset: 29813 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081298 as xkb_keysym_t,
            offset: 29824 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b3 as xkb_keysym_t,
            offset: 29835 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b4 as xkb_keysym_t,
            offset: 29852 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b5 as xkb_keysym_t,
            offset: 29869 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b2 as xkb_keysym_t,
            offset: 29886 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b0 as xkb_keysym_t,
            offset: 29907 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b1 as xkb_keysym_t,
            offset: 29928 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff19 as xkb_keysym_t,
            offset: 29948 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff90 as xkb_keysym_t,
            offset: 29957 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff62 as xkb_keysym_t,
            offset: 29973 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127e as xkb_keysym_t,
            offset: 29984 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081170 as xkb_keysym_t,
            offset: 30001 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081183 as xkb_keysym_t,
            offset: 30023 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810a4 as xkb_keysym_t,
            offset: 30039 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081199 as xkb_keysym_t,
            offset: 30058 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b7 as xkb_keysym_t,
            offset: 30076 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081186 as xkb_keysym_t,
            offset: 30092 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117a as xkb_keysym_t,
            offset: 30117 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff53 as xkb_keysym_t,
            offset: 30138 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081185 as xkb_keysym_t,
            offset: 30156 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008116e as xkb_keysym_t,
            offset: 30175 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081178 as xkb_keysym_t,
            offset: 30195 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008116a as xkb_keysym_t,
            offset: 30213 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081181 as xkb_keysym_t,
            offset: 30241 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117d as xkb_keysym_t,
            offset: 30262 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081180 as xkb_keysym_t,
            offset: 30287 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081184 as xkb_keysym_t,
            offset: 30307 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081182 as xkb_keysym_t,
            offset: 30331 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081179 as xkb_keysym_t,
            offset: 30352 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117b as xkb_keysym_t,
            offset: 30370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117c as xkb_keysym_t,
            offset: 30389 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081171 as xkb_keysym_t,
            offset: 30412 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126b as xkb_keysym_t,
            offset: 30431 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff63 as xkb_keysym_t,
            offset: 30448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1e as xkb_keysym_t,
            offset: 30460 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff65 as xkb_keysym_t,
            offset: 30469 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff66 as xkb_keysym_t,
            offset: 30480 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8e as xkb_keysym_t,
            offset: 30491 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff01 as xkb_keysym_t,
            offset: 30505 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810f4 as xkb_keysym_t,
            offset: 30518 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff07 as xkb_keysym_t,
            offset: 30540 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff03 as xkb_keysym_t,
            offset: 30563 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff02 as xkb_keysym_t,
            offset: 30585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff92 as xkb_keysym_t,
            offset: 30605 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff33 as xkb_keysym_t,
            offset: 30615 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff67 as xkb_keysym_t,
            offset: 30630 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081280 as xkb_keysym_t,
            offset: 30642 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081288 as xkb_keysym_t,
            offset: 30655 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff68 as xkb_keysym_t,
            offset: 30667 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff69 as xkb_keysym_t,
            offset: 30675 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe22 as xkb_keysym_t,
            offset: 30684 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127b as xkb_keysym_t,
            offset: 30699 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081270 as xkb_keysym_t,
            offset: 30715 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bc as xkb_keysym_t,
            offset: 30732 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008119d as xkb_keysym_t,
            offset: 30755 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081200 as xkb_keysym_t,
            offset: 30775 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081201 as xkb_keysym_t,
            offset: 30788 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126c as xkb_keysym_t,
            offset: 30801 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126d as xkb_keysym_t,
            offset: 30815 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081202 as xkb_keysym_t,
            offset: 30829 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081203 as xkb_keysym_t,
            offset: 30842 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081204 as xkb_keysym_t,
            offset: 30855 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081205 as xkb_keysym_t,
            offset: 30868 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081206 as xkb_keysym_t,
            offset: 30881 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081207 as xkb_keysym_t,
            offset: 30894 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081208 as xkb_keysym_t,
            offset: 30907 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081209 as xkb_keysym_t,
            offset: 30920 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120c as xkb_keysym_t,
            offset: 30933 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120d as xkb_keysym_t,
            offset: 30946 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120e as xkb_keysym_t,
            offset: 30959 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120f as xkb_keysym_t,
            offset: 30972 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120b as xkb_keysym_t,
            offset: 30985 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120a as xkb_keysym_t,
            offset: 31002 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6a as xkb_keysym_t,
            offset: 31018 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081160 as xkb_keysym_t,
            offset: 31033 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081278 as xkb_keysym_t,
            offset: 31040 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6b as xkb_keysym_t,
            offset: 31061 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff38 as xkb_keysym_t,
            offset: 31070 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6c as xkb_keysym_t,
            offset: 31082 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6d as xkb_keysym_t,
            offset: 31093 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081272 as xkb_keysym_t,
            offset: 31103 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bd as xkb_keysym_t,
            offset: 31119 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6e as xkb_keysym_t,
            offset: 31139 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bd as xkb_keysym_t,
            offset: 31149 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff91 as xkb_keysym_t,
            offset: 31165 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff21 as xkb_keysym_t,
            offset: 31178 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2a as xkb_keysym_t,
            offset: 31192 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a9 as xkb_keysym_t,
            offset: 31205 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe23 as xkb_keysym_t,
            offset: 31222 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127c as xkb_keysym_t,
            offset: 31237 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081279 as xkb_keysym_t,
            offset: 31257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff70 as xkb_keysym_t,
            offset: 31281 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081284 as xkb_keysym_t,
            offset: 31287 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa3 as xkb_keysym_t,
            offset: 31304 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff29 as xkb_keysym_t,
            offset: 31312 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081232 as xkb_keysym_t,
            offset: 31324 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff73 as xkb_keysym_t,
            offset: 31346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff72 as xkb_keysym_t,
            offset: 31357 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb5 as xkb_keysym_t,
            offset: 31367 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081267 as xkb_keysym_t,
            offset: 31378 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081266 as xkb_keysym_t,
            offset: 31392 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff24 as xkb_keysym_t,
            offset: 31404 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff25 as xkb_keysym_t,
            offset: 31419 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff23 as xkb_keysym_t,
            offset: 31435 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126a as xkb_keysym_t,
            offset: 31448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff74 as xkb_keysym_t,
            offset: 31461 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff76 as xkb_keysym_t,
            offset: 31479 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb7 as xkb_keysym_t,
            offset: 31494 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff75 as xkb_keysym_t,
            offset: 31517 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff77 as xkb_keysym_t,
            offset: 31532 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2d as xkb_keysym_t,
            offset: 31541 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081245 as xkb_keysym_t,
            offset: 31557 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7a as xkb_keysym_t,
            offset: 31573 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff79 as xkb_keysym_t,
            offset: 31589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff78 as xkb_keysym_t,
            offset: 31604 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1b as xkb_keysym_t,
            offset: 31617 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa0 as xkb_keysym_t,
            offset: 31628 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127a as xkb_keysym_t,
            offset: 31639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7b as xkb_keysym_t,
            offset: 31663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff36 as xkb_keysym_t,
            offset: 31672 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081287 as xkb_keysym_t,
            offset: 31681 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081282 as xkb_keysym_t,
            offset: 31697 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2f as xkb_keysym_t,
            offset: 31718 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081276 as xkb_keysym_t,
            offset: 31728 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127f as xkb_keysym_t,
            offset: 31744 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7c as xkb_keysym_t,
            offset: 31752 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b0 as xkb_keysym_t,
            offset: 31762 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7d as xkb_keysym_t,
            offset: 31777 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810dc as xkb_keysym_t,
            offset: 31793 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff10 as xkb_keysym_t,
            offset: 31803 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1a as xkb_keysym_t,
            offset: 31815 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff28 as xkb_keysym_t,
            offset: 31825 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081271 as xkb_keysym_t,
            offset: 31834 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9a as xkb_keysym_t,
            offset: 31849 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7e as xkb_keysym_t,
            offset: 31862 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa7 as xkb_keysym_t,
            offset: 31874 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe01 as xkb_keysym_t,
            offset: 31886 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0a as xkb_keysym_t,
            offset: 31902 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0b as xkb_keysym_t,
            offset: 31919 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0c as xkb_keysym_t,
            offset: 31936 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe02 as xkb_keysym_t,
            offset: 31953 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe03 as xkb_keysym_t,
            offset: 31969 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe04 as xkb_keysym_t,
            offset: 31985 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe05 as xkb_keysym_t,
            offset: 32001 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe06 as xkb_keysym_t,
            offset: 32017 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe07 as xkb_keysym_t,
            offset: 32033 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe08 as xkb_keysym_t,
            offset: 32049 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe09 as xkb_keysym_t,
            offset: 32065 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081241 as xkb_keysym_t,
            offset: 32081 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7f as xkb_keysym_t,
            offset: 32097 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff80 as xkb_keysym_t,
            offset: 32110 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9f as xkb_keysym_t,
            offset: 32123 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1f as xkb_keysym_t,
            offset: 32132 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff81 as xkb_keysym_t,
            offset: 32145 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa2 as xkb_keysym_t,
            offset: 32155 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb1 as xkb_keysym_t,
            offset: 32167 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb0 as xkb_keysym_t,
            offset: 32183 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa9 as xkb_keysym_t,
            offset: 32198 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081285 as xkb_keysym_t,
            offset: 32217 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff82 as xkb_keysym_t,
            offset: 32238 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe20 as xkb_keysym_t,
            offset: 32249 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081274 as xkb_keysym_t,
            offset: 32260 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff85 as xkb_keysym_t,
            offset: 32271 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff86 as xkb_keysym_t,
            offset: 32283 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff84 as xkb_keysym_t,
            offset: 32295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff96 as xkb_keysym_t,
            offset: 32306 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff34 as xkb_keysym_t,
            offset: 32314 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081168 as xkb_keysym_t,
            offset: 32329 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff87 as xkb_keysym_t,
            offset: 32344 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a0 as xkb_keysym_t,
            offset: 32354 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa1 as xkb_keysym_t,
            offset: 32369 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081273 as xkb_keysym_t,
            offset: 32378 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081246 as xkb_keysym_t,
            offset: 32386 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ac as xkb_keysym_t,
            offset: 32403 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2b as xkb_keysym_t,
            offset: 32417 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8f as xkb_keysym_t,
            offset: 32428 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff88 as xkb_keysym_t,
            offset: 32439 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff95 as xkb_keysym_t,
            offset: 32455 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff89 as xkb_keysym_t,
            offset: 32464 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081211 as xkb_keysym_t,
            offset: 32473 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb4 as xkb_keysym_t,
            offset: 32487 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2e as xkb_keysym_t,
            offset: 32496 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8a as xkb_keysym_t,
            offset: 32504 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa5 as xkb_keysym_t,
            offset: 32513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8b as xkb_keysym_t,
            offset: 32524 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8c as xkb_keysym_t,
            offset: 32535 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a4 as xkb_keysym_t,
            offset: 32547 as uint16_t,
        },
        name_keysym {
            keysym: 0x59 as xkb_keysym_t,
            offset: 32561 as uint16_t,
        },
        name_keysym {
            keysym: 0x79 as xkb_keysym_t,
            offset: 32563 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd as xkb_keysym_t,
            offset: 32565 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd as xkb_keysym_t,
            offset: 32572 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef4 as xkb_keysym_t,
            offset: 32579 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef5 as xkb_keysym_t,
            offset: 32589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000176 as xkb_keysym_t,
            offset: 32599 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000177 as xkb_keysym_t,
            offset: 32611 as uint16_t,
        },
        name_keysym {
            keysym: 0x13be as xkb_keysym_t,
            offset: 32623 as uint16_t,
        },
        name_keysym {
            keysym: 0xff as xkb_keysym_t,
            offset: 32634 as uint16_t,
        },
        name_keysym {
            keysym: 0xa5 as xkb_keysym_t,
            offset: 32645 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef2 as xkb_keysym_t,
            offset: 32649 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef3 as xkb_keysym_t,
            offset: 32656 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef6 as xkb_keysym_t,
            offset: 32663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef7 as xkb_keysym_t,
            offset: 32669 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef8 as xkb_keysym_t,
            offset: 32675 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef9 as xkb_keysym_t,
            offset: 32682 as uint16_t,
        },
        name_keysym {
            keysym: 0x5a as xkb_keysym_t,
            offset: 32689 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a as xkb_keysym_t,
            offset: 32691 as uint16_t,
        },
        name_keysym {
            keysym: 0x1af as xkb_keysym_t,
            offset: 32693 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bf as xkb_keysym_t,
            offset: 32703 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ac as xkb_keysym_t,
            offset: 32713 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bc as xkb_keysym_t,
            offset: 32720 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ae as xkb_keysym_t,
            offset: 32727 as uint16_t,
        },
        name_keysym {
            keysym: 0x1be as xkb_keysym_t,
            offset: 32734 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3d as xkb_keysym_t,
            offset: 32741 as uint16_t,
        },
        name_keysym {
            keysym: 0xff28 as xkb_keysym_t,
            offset: 32750 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2a as xkb_keysym_t,
            offset: 32758 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002080 as xkb_keysym_t,
            offset: 32774 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002070 as xkb_keysym_t,
            offset: 32788 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b5 as xkb_keysym_t,
            offset: 32801 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b6 as xkb_keysym_t,
            offset: 32809 as uint16_t,
        },
    ];
    pub static mut keysym_to_name: [name_keysym; 2502] = [
        name_keysym {
            keysym: 0 as xkb_keysym_t,
            offset: 20276 as uint16_t,
        },
        name_keysym {
            keysym: 0x20 as xkb_keysym_t,
            offset: 24027 as uint16_t,
        },
        name_keysym {
            keysym: 0x21 as xkb_keysym_t,
            offset: 12230 as uint16_t,
        },
        name_keysym {
            keysym: 0x22 as xkb_keysym_t,
            offset: 22385 as uint16_t,
        },
        name_keysym {
            keysym: 0x23 as xkb_keysym_t,
            offset: 20363 as uint16_t,
        },
        name_keysym {
            keysym: 0x24 as xkb_keysym_t,
            offset: 11382 as uint16_t,
        },
        name_keysym {
            keysym: 0x25 as xkb_keysym_t,
            offset: 21685 as uint16_t,
        },
        name_keysym {
            keysym: 0x26 as xkb_keysym_t,
            offset: 908 as uint16_t,
        },
        name_keysym {
            keysym: 0x27 as xkb_keysym_t,
            offset: 934 as uint16_t,
        },
        name_keysym {
            keysym: 0x28 as xkb_keysym_t,
            offset: 21623 as uint16_t,
        },
        name_keysym {
            keysym: 0x29 as xkb_keysym_t,
            offset: 21633 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a as xkb_keysym_t,
            offset: 3283 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b as xkb_keysym_t,
            offset: 21755 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c as xkb_keysym_t,
            offset: 8762 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d as xkb_keysym_t,
            offset: 19918 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e as xkb_keysym_t,
            offset: 21693 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f as xkb_keysym_t,
            offset: 23992 as uint16_t,
        },
        name_keysym {
            keysym: 0x30 as xkb_keysym_t,
            offset: 0 as uint16_t,
        },
        name_keysym {
            keysym: 0x31 as xkb_keysym_t,
            offset: 2 as uint16_t,
        },
        name_keysym {
            keysym: 0x32 as xkb_keysym_t,
            offset: 4 as uint16_t,
        },
        name_keysym {
            keysym: 0x33 as xkb_keysym_t,
            offset: 6 as uint16_t,
        },
        name_keysym {
            keysym: 0x34 as xkb_keysym_t,
            offset: 386 as uint16_t,
        },
        name_keysym {
            keysym: 0x35 as xkb_keysym_t,
            offset: 388 as uint16_t,
        },
        name_keysym {
            keysym: 0x36 as xkb_keysym_t,
            offset: 390 as uint16_t,
        },
        name_keysym {
            keysym: 0x37 as xkb_keysym_t,
            offset: 392 as uint16_t,
        },
        name_keysym {
            keysym: 0x38 as xkb_keysym_t,
            offset: 394 as uint16_t,
        },
        name_keysym {
            keysym: 0x39 as xkb_keysym_t,
            offset: 396 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a as xkb_keysym_t,
            offset: 8664 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b as xkb_keysym_t,
            offset: 22958 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c as xkb_keysym_t,
            offset: 19597 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d as xkb_keysym_t,
            offset: 12182 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e as xkb_keysym_t,
            offset: 13403 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f as xkb_keysym_t,
            offset: 22363 as uint16_t,
        },
        name_keysym {
            keysym: 0x40 as xkb_keysym_t,
            offset: 3292 as uint16_t,
        },
        name_keysym {
            keysym: 0x41 as xkb_keysym_t,
            offset: 398 as uint16_t,
        },
        name_keysym {
            keysym: 0x42 as xkb_keysym_t,
            offset: 3328 as uint16_t,
        },
        name_keysym {
            keysym: 0x43 as xkb_keysym_t,
            offset: 8439 as uint16_t,
        },
        name_keysym {
            keysym: 0x44 as xkb_keysym_t,
            offset: 10402 as uint16_t,
        },
        name_keysym {
            keysym: 0x45 as xkb_keysym_t,
            offset: 11555 as uint16_t,
        },
        name_keysym {
            keysym: 0x46 as xkb_keysym_t,
            offset: 12286 as uint16_t,
        },
        name_keysym {
            keysym: 0x47 as xkb_keysym_t,
            offset: 12796 as uint16_t,
        },
        name_keysym {
            keysym: 0x48 as xkb_keysym_t,
            offset: 14538 as uint16_t,
        },
        name_keysym {
            keysym: 0x49 as xkb_keysym_t,
            offset: 17229 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a as xkb_keysym_t,
            offset: 18274 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b as xkb_keysym_t,
            offset: 18306 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c as xkb_keysym_t,
            offset: 19272 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d as xkb_keysym_t,
            offset: 19718 as uint16_t,
        },
        name_keysym {
            keysym: 0x4e as xkb_keysym_t,
            offset: 20128 as uint16_t,
        },
        name_keysym {
            keysym: 0x4f as xkb_keysym_t,
            offset: 20385 as uint16_t,
        },
        name_keysym {
            keysym: 0x50 as xkb_keysym_t,
            offset: 21571 as uint16_t,
        },
        name_keysym {
            keysym: 0x51 as xkb_keysym_t,
            offset: 22354 as uint16_t,
        },
        name_keysym {
            keysym: 0x52 as xkb_keysym_t,
            offset: 22415 as uint16_t,
        },
        name_keysym {
            keysym: 0x53 as xkb_keysym_t,
            offset: 22803 as uint16_t,
        },
        name_keysym {
            keysym: 0x54 as xkb_keysym_t,
            offset: 24525 as uint16_t,
        },
        name_keysym {
            keysym: 0x55 as xkb_keysym_t,
            offset: 26078 as uint16_t,
        },
        name_keysym {
            keysym: 0x56 as xkb_keysym_t,
            offset: 26714 as uint16_t,
        },
        name_keysym {
            keysym: 0x57 as xkb_keysym_t,
            offset: 26776 as uint16_t,
        },
        name_keysym {
            keysym: 0x58 as xkb_keysym_t,
            offset: 26862 as uint16_t,
        },
        name_keysym {
            keysym: 0x59 as xkb_keysym_t,
            offset: 32561 as uint16_t,
        },
        name_keysym {
            keysym: 0x5a as xkb_keysym_t,
            offset: 32689 as uint16_t,
        },
        name_keysym {
            keysym: 0x5b as xkb_keysym_t,
            offset: 3603 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c as xkb_keysym_t,
            offset: 3352 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d as xkb_keysym_t,
            offset: 3615 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e as xkb_keysym_t,
            offset: 3260 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f as xkb_keysym_t,
            offset: 26577 as uint16_t,
        },
        name_keysym {
            keysym: 0x60 as xkb_keysym_t,
            offset: 13397 as uint16_t,
        },
        name_keysym {
            keysym: 0x61 as xkb_keysym_t,
            offset: 400 as uint16_t,
        },
        name_keysym {
            keysym: 0x62 as xkb_keysym_t,
            offset: 3330 as uint16_t,
        },
        name_keysym {
            keysym: 0x63 as xkb_keysym_t,
            offset: 8441 as uint16_t,
        },
        name_keysym {
            keysym: 0x64 as xkb_keysym_t,
            offset: 10404 as uint16_t,
        },
        name_keysym {
            keysym: 0x65 as xkb_keysym_t,
            offset: 11557 as uint16_t,
        },
        name_keysym {
            keysym: 0x66 as xkb_keysym_t,
            offset: 12288 as uint16_t,
        },
        name_keysym {
            keysym: 0x67 as xkb_keysym_t,
            offset: 12798 as uint16_t,
        },
        name_keysym {
            keysym: 0x68 as xkb_keysym_t,
            offset: 14540 as uint16_t,
        },
        name_keysym {
            keysym: 0x69 as xkb_keysym_t,
            offset: 17231 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a as xkb_keysym_t,
            offset: 18276 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b as xkb_keysym_t,
            offset: 18308 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c as xkb_keysym_t,
            offset: 19274 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d as xkb_keysym_t,
            offset: 19720 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e as xkb_keysym_t,
            offset: 20130 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f as xkb_keysym_t,
            offset: 20387 as uint16_t,
        },
        name_keysym {
            keysym: 0x70 as xkb_keysym_t,
            offset: 21573 as uint16_t,
        },
        name_keysym {
            keysym: 0x71 as xkb_keysym_t,
            offset: 22356 as uint16_t,
        },
        name_keysym {
            keysym: 0x72 as xkb_keysym_t,
            offset: 22417 as uint16_t,
        },
        name_keysym {
            keysym: 0x73 as xkb_keysym_t,
            offset: 22805 as uint16_t,
        },
        name_keysym {
            keysym: 0x74 as xkb_keysym_t,
            offset: 24527 as uint16_t,
        },
        name_keysym {
            keysym: 0x75 as xkb_keysym_t,
            offset: 26080 as uint16_t,
        },
        name_keysym {
            keysym: 0x76 as xkb_keysym_t,
            offset: 26716 as uint16_t,
        },
        name_keysym {
            keysym: 0x77 as xkb_keysym_t,
            offset: 26778 as uint16_t,
        },
        name_keysym {
            keysym: 0x78 as xkb_keysym_t,
            offset: 26864 as uint16_t,
        },
        name_keysym {
            keysym: 0x79 as xkb_keysym_t,
            offset: 32563 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a as xkb_keysym_t,
            offset: 32691 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b as xkb_keysym_t,
            offset: 3582 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c as xkb_keysym_t,
            offset: 3392 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d as xkb_keysym_t,
            offset: 3592 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e as xkb_keysym_t,
            offset: 3272 as uint16_t,
        },
        name_keysym {
            keysym: 0xa0 as xkb_keysym_t,
            offset: 20263 as uint16_t,
        },
        name_keysym {
            keysym: 0xa1 as xkb_keysym_t,
            offset: 12237 as uint16_t,
        },
        name_keysym {
            keysym: 0xa2 as xkb_keysym_t,
            offset: 8589 as uint16_t,
        },
        name_keysym {
            keysym: 0xa3 as xkb_keysym_t,
            offset: 24058 as uint16_t,
        },
        name_keysym {
            keysym: 0xa4 as xkb_keysym_t,
            offset: 8848 as uint16_t,
        },
        name_keysym {
            keysym: 0xa5 as xkb_keysym_t,
            offset: 32645 as uint16_t,
        },
        name_keysym {
            keysym: 0xa6 as xkb_keysym_t,
            offset: 8389 as uint16_t,
        },
        name_keysym {
            keysym: 0xa7 as xkb_keysym_t,
            offset: 22943 as uint16_t,
        },
        name_keysym {
            keysym: 0xa8 as xkb_keysym_t,
            offset: 11334 as uint16_t,
        },
        name_keysym {
            keysym: 0xa9 as xkb_keysym_t,
            offset: 8799 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa as xkb_keysym_t,
            offset: 21037 as uint16_t,
        },
        name_keysym {
            keysym: 0xab as xkb_keysym_t,
            offset: 14509 as uint16_t,
        },
        name_keysym {
            keysym: 0xac as xkb_keysym_t,
            offset: 20332 as uint16_t,
        },
        name_keysym {
            keysym: 0xad as xkb_keysym_t,
            offset: 17222 as uint16_t,
        },
        name_keysym {
            keysym: 0xae as xkb_keysym_t,
            offset: 22529 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf as xkb_keysym_t,
            offset: 19826 as uint16_t,
        },
        name_keysym {
            keysym: 0xb0 as xkb_keysym_t,
            offset: 11284 as uint16_t,
        },
        name_keysym {
            keysym: 0xb1 as xkb_keysym_t,
            offset: 21760 as uint16_t,
        },
        name_keysym {
            keysym: 0xb2 as xkb_keysym_t,
            offset: 26056 as uint16_t,
        },
        name_keysym {
            keysym: 0xb3 as xkb_keysym_t,
            offset: 25802 as uint16_t,
        },
        name_keysym {
            keysym: 0xb4 as xkb_keysym_t,
            offset: 820 as uint16_t,
        },
        name_keysym {
            keysym: 0xb5 as xkb_keysym_t,
            offset: 19984 as uint16_t,
        },
        name_keysym {
            keysym: 0xb6 as xkb_keysym_t,
            offset: 21613 as uint16_t,
        },
        name_keysym {
            keysym: 0xb7 as xkb_keysym_t,
            offset: 21700 as uint16_t,
        },
        name_keysym {
            keysym: 0xb8 as xkb_keysym_t,
            offset: 8581 as uint16_t,
        },
        name_keysym {
            keysym: 0xb9 as xkb_keysym_t,
            offset: 20940 as uint16_t,
        },
        name_keysym {
            keysym: 0xba as xkb_keysym_t,
            offset: 19873 as uint16_t,
        },
        name_keysym {
            keysym: 0xbb as xkb_keysym_t,
            offset: 14523 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc as xkb_keysym_t,
            offset: 20907 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd as xkb_keysym_t,
            offset: 20899 as uint16_t,
        },
        name_keysym {
            keysym: 0xbe as xkb_keysym_t,
            offset: 25773 as uint16_t,
        },
        name_keysym {
            keysym: 0xbf as xkb_keysym_t,
            offset: 22372 as uint16_t,
        },
        name_keysym {
            keysym: 0xc0 as xkb_keysym_t,
            offset: 854 as uint16_t,
        },
        name_keysym {
            keysym: 0xc1 as xkb_keysym_t,
            offset: 402 as uint16_t,
        },
        name_keysym {
            keysym: 0xc2 as xkb_keysym_t,
            offset: 622 as uint16_t,
        },
        name_keysym {
            keysym: 0xc3 as xkb_keysym_t,
            offset: 3295 as uint16_t,
        },
        name_keysym {
            keysym: 0xc4 as xkb_keysym_t,
            offset: 826 as uint16_t,
        },
        name_keysym {
            keysym: 0xc5 as xkb_keysym_t,
            offset: 2036 as uint16_t,
        },
        name_keysym {
            keysym: 0xc6 as xkb_keysym_t,
            offset: 848 as uint16_t,
        },
        name_keysym {
            keysym: 0xc7 as xkb_keysym_t,
            offset: 8539 as uint16_t,
        },
        name_keysym {
            keysym: 0xc8 as xkb_keysym_t,
            offset: 11855 as uint16_t,
        },
        name_keysym {
            keysym: 0xc9 as xkb_keysym_t,
            offset: 11579 as uint16_t,
        },
        name_keysym {
            keysym: 0xca as xkb_keysym_t,
            offset: 11627 as uint16_t,
        },
        name_keysym {
            keysym: 0xcb as xkb_keysym_t,
            offset: 11833 as uint16_t,
        },
        name_keysym {
            keysym: 0xcc as xkb_keysym_t,
            offset: 17365 as uint16_t,
        },
        name_keysym {
            keysym: 0xcd as xkb_keysym_t,
            offset: 17243 as uint16_t,
        },
        name_keysym {
            keysym: 0xce as xkb_keysym_t,
            offset: 17291 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf as xkb_keysym_t,
            offset: 17325 as uint16_t,
        },
        name_keysym {
            keysym: 0xd0 as xkb_keysym_t,
            offset: 12195 as uint16_t,
        },
        name_keysym {
            keysym: 0xd1 as xkb_keysym_t,
            offset: 20340 as uint16_t,
        },
        name_keysym {
            keysym: 0xd2 as xkb_keysym_t,
            offset: 20712 as uint16_t,
        },
        name_keysym {
            keysym: 0xd3 as xkb_keysym_t,
            offset: 20389 as uint16_t,
        },
        name_keysym {
            keysym: 0xd4 as xkb_keysym_t,
            offset: 20453 as uint16_t,
        },
        name_keysym {
            keysym: 0xd5 as xkb_keysym_t,
            offset: 21508 as uint16_t,
        },
        name_keysym {
            keysym: 0xd6 as xkb_keysym_t,
            offset: 20651 as uint16_t,
        },
        name_keysym {
            keysym: 0xd7 as xkb_keysym_t,
            offset: 20024 as uint16_t,
        },
        name_keysym {
            keysym: 0xd8 as xkb_keysym_t,
            offset: 21494 as uint16_t,
        },
        name_keysym {
            keysym: 0xd9 as xkb_keysym_t,
            offset: 26202 as uint16_t,
        },
        name_keysym {
            keysym: 0xda as xkb_keysym_t,
            offset: 26082 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb as xkb_keysym_t,
            offset: 26130 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc as xkb_keysym_t,
            offset: 26154 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd as xkb_keysym_t,
            offset: 32565 as uint16_t,
        },
        name_keysym {
            keysym: 0xde as xkb_keysym_t,
            offset: 25730 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf as xkb_keysym_t,
            offset: 24051 as uint16_t,
        },
        name_keysym {
            keysym: 0xe0 as xkb_keysym_t,
            offset: 861 as uint16_t,
        },
        name_keysym {
            keysym: 0xe1 as xkb_keysym_t,
            offset: 409 as uint16_t,
        },
        name_keysym {
            keysym: 0xe2 as xkb_keysym_t,
            offset: 634 as uint16_t,
        },
        name_keysym {
            keysym: 0xe3 as xkb_keysym_t,
            offset: 3302 as uint16_t,
        },
        name_keysym {
            keysym: 0xe4 as xkb_keysym_t,
            offset: 837 as uint16_t,
        },
        name_keysym {
            keysym: 0xe5 as xkb_keysym_t,
            offset: 2042 as uint16_t,
        },
        name_keysym {
            keysym: 0xe6 as xkb_keysym_t,
            offset: 851 as uint16_t,
        },
        name_keysym {
            keysym: 0xe7 as xkb_keysym_t,
            offset: 8548 as uint16_t,
        },
        name_keysym {
            keysym: 0xe8 as xkb_keysym_t,
            offset: 11862 as uint16_t,
        },
        name_keysym {
            keysym: 0xe9 as xkb_keysym_t,
            offset: 11586 as uint16_t,
        },
        name_keysym {
            keysym: 0xea as xkb_keysym_t,
            offset: 11639 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb as xkb_keysym_t,
            offset: 11844 as uint16_t,
        },
        name_keysym {
            keysym: 0xec as xkb_keysym_t,
            offset: 17372 as uint16_t,
        },
        name_keysym {
            keysym: 0xed as xkb_keysym_t,
            offset: 17250 as uint16_t,
        },
        name_keysym {
            keysym: 0xee as xkb_keysym_t,
            offset: 17303 as uint16_t,
        },
        name_keysym {
            keysym: 0xef as xkb_keysym_t,
            offset: 17336 as uint16_t,
        },
        name_keysym {
            keysym: 0xf0 as xkb_keysym_t,
            offset: 12203 as uint16_t,
        },
        name_keysym {
            keysym: 0xf1 as xkb_keysym_t,
            offset: 20347 as uint16_t,
        },
        name_keysym {
            keysym: 0xf2 as xkb_keysym_t,
            offset: 20719 as uint16_t,
        },
        name_keysym {
            keysym: 0xf3 as xkb_keysym_t,
            offset: 20396 as uint16_t,
        },
        name_keysym {
            keysym: 0xf4 as xkb_keysym_t,
            offset: 20465 as uint16_t,
        },
        name_keysym {
            keysym: 0xf5 as xkb_keysym_t,
            offset: 21515 as uint16_t,
        },
        name_keysym {
            keysym: 0xf6 as xkb_keysym_t,
            offset: 20662 as uint16_t,
        },
        name_keysym {
            keysym: 0xf7 as xkb_keysym_t,
            offset: 11373 as uint16_t,
        },
        name_keysym {
            keysym: 0xf8 as xkb_keysym_t,
            offset: 21501 as uint16_t,
        },
        name_keysym {
            keysym: 0xf9 as xkb_keysym_t,
            offset: 26209 as uint16_t,
        },
        name_keysym {
            keysym: 0xfa as xkb_keysym_t,
            offset: 26089 as uint16_t,
        },
        name_keysym {
            keysym: 0xfb as xkb_keysym_t,
            offset: 26142 as uint16_t,
        },
        name_keysym {
            keysym: 0xfc as xkb_keysym_t,
            offset: 26165 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd as xkb_keysym_t,
            offset: 32572 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe as xkb_keysym_t,
            offset: 25742 as uint16_t,
        },
        name_keysym {
            keysym: 0xff as xkb_keysym_t,
            offset: 32634 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a1 as xkb_keysym_t,
            offset: 918 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a2 as xkb_keysym_t,
            offset: 8383 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a3 as xkb_keysym_t,
            offset: 19702 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a5 as xkb_keysym_t,
            offset: 19372 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a6 as xkb_keysym_t,
            offset: 22827 as uint16_t,
        },
        name_keysym {
            keysym: 0x1a9 as xkb_keysym_t,
            offset: 22841 as uint16_t,
        },
        name_keysym {
            keysym: 0x1aa as xkb_keysym_t,
            offset: 22855 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ab as xkb_keysym_t,
            offset: 24553 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ac as xkb_keysym_t,
            offset: 32713 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ae as xkb_keysym_t,
            offset: 32727 as uint16_t,
        },
        name_keysym {
            keysym: 0x1af as xkb_keysym_t,
            offset: 32693 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b1 as xkb_keysym_t,
            offset: 926 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b2 as xkb_keysym_t,
            offset: 20705 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b3 as xkb_keysym_t,
            offset: 19710 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b5 as xkb_keysym_t,
            offset: 19379 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b6 as xkb_keysym_t,
            offset: 22834 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b7 as xkb_keysym_t,
            offset: 8519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1b9 as xkb_keysym_t,
            offset: 22848 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ba as xkb_keysym_t,
            offset: 22864 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bb as xkb_keysym_t,
            offset: 24560 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bc as xkb_keysym_t,
            offset: 32720 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bd as xkb_keysym_t,
            offset: 11414 as uint16_t,
        },
        name_keysym {
            keysym: 0x1be as xkb_keysym_t,
            offset: 32734 as uint16_t,
        },
        name_keysym {
            keysym: 0x1bf as xkb_keysym_t,
            offset: 32703 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c0 as xkb_keysym_t,
            offset: 22470 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c3 as xkb_keysym_t,
            offset: 445 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c5 as xkb_keysym_t,
            offset: 19307 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c6 as xkb_keysym_t,
            offset: 8475 as uint16_t,
        },
        name_keysym {
            keysym: 0x1c8 as xkb_keysym_t,
            offset: 8525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ca as xkb_keysym_t,
            offset: 12166 as uint16_t,
        },
        name_keysym {
            keysym: 0x1cc as xkb_keysym_t,
            offset: 11613 as uint16_t,
        },
        name_keysym {
            keysym: 0x1cf as xkb_keysym_t,
            offset: 10447 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d0 as xkb_keysym_t,
            offset: 11532 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d1 as xkb_keysym_t,
            offset: 20138 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d2 as xkb_keysym_t,
            offset: 20162 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d5 as xkb_keysym_t,
            offset: 20673 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d8 as xkb_keysym_t,
            offset: 22492 as uint16_t,
        },
        name_keysym {
            keysym: 0x1d9 as xkb_keysym_t,
            offset: 26683 as uint16_t,
        },
        name_keysym {
            keysym: 0x1db as xkb_keysym_t,
            offset: 26176 as uint16_t,
        },
        name_keysym {
            keysym: 0x1de as xkb_keysym_t,
            offset: 24567 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e0 as xkb_keysym_t,
            offset: 22477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e3 as xkb_keysym_t,
            offset: 452 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e5 as xkb_keysym_t,
            offset: 19314 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e6 as xkb_keysym_t,
            offset: 8482 as uint16_t,
        },
        name_keysym {
            keysym: 0x1e8 as xkb_keysym_t,
            offset: 8532 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ea as xkb_keysym_t,
            offset: 12174 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ec as xkb_keysym_t,
            offset: 11620 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ef as xkb_keysym_t,
            offset: 10454 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f0 as xkb_keysym_t,
            offset: 11540 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f1 as xkb_keysym_t,
            offset: 20145 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f2 as xkb_keysym_t,
            offset: 20169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f5 as xkb_keysym_t,
            offset: 20686 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f8 as xkb_keysym_t,
            offset: 22499 as uint16_t,
        },
        name_keysym {
            keysym: 0x1f9 as xkb_keysym_t,
            offset: 26689 as uint16_t,
        },
        name_keysym {
            keysym: 0x1fb as xkb_keysym_t,
            offset: 26189 as uint16_t,
        },
        name_keysym {
            keysym: 0x1fe as xkb_keysym_t,
            offset: 24576 as uint16_t,
        },
        name_keysym {
            keysym: 0x1ff as xkb_keysym_t,
            offset: 436 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a1 as xkb_keysym_t,
            offset: 17187 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a6 as xkb_keysym_t,
            offset: 16216 as uint16_t,
        },
        name_keysym {
            keysym: 0x2a9 as xkb_keysym_t,
            offset: 17233 as uint16_t,
        },
        name_keysym {
            keysym: 0x2ab as xkb_keysym_t,
            offset: 12820 as uint16_t,
        },
        name_keysym {
            keysym: 0x2ac as xkb_keysym_t,
            offset: 18278 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b1 as xkb_keysym_t,
            offset: 17195 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b6 as xkb_keysym_t,
            offset: 16228 as uint16_t,
        },
        name_keysym {
            keysym: 0x2b9 as xkb_keysym_t,
            offset: 17347 as uint16_t,
        },
        name_keysym {
            keysym: 0x2bb as xkb_keysym_t,
            offset: 12827 as uint16_t,
        },
        name_keysym {
            keysym: 0x2bc as xkb_keysym_t,
            offset: 18290 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c5 as xkb_keysym_t,
            offset: 8455 as uint16_t,
        },
        name_keysym {
            keysym: 0x2c6 as xkb_keysym_t,
            offset: 8557 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d5 as xkb_keysym_t,
            offset: 12800 as uint16_t,
        },
        name_keysym {
            keysym: 0x2d8 as xkb_keysym_t,
            offset: 12866 as uint16_t,
        },
        name_keysym {
            keysym: 0x2dd as xkb_keysym_t,
            offset: 26116 as uint16_t,
        },
        name_keysym {
            keysym: 0x2de as xkb_keysym_t,
            offset: 22885 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e5 as xkb_keysym_t,
            offset: 8465 as uint16_t,
        },
        name_keysym {
            keysym: 0x2e6 as xkb_keysym_t,
            offset: 8569 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f5 as xkb_keysym_t,
            offset: 12810 as uint16_t,
        },
        name_keysym {
            keysym: 0x2f8 as xkb_keysym_t,
            offset: 12878 as uint16_t,
        },
        name_keysym {
            keysym: 0x2fd as xkb_keysym_t,
            offset: 26123 as uint16_t,
        },
        name_keysym {
            keysym: 0x2fe as xkb_keysym_t,
            offset: 22897 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a2 as xkb_keysym_t,
            offset: 19268 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a3 as xkb_keysym_t,
            offset: 22506 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a5 as xkb_keysym_t,
            offset: 18260 as uint16_t,
        },
        name_keysym {
            keysym: 0x3a6 as xkb_keysym_t,
            offset: 19386 as uint16_t,
        },
        name_keysym {
            keysym: 0x3aa as xkb_keysym_t,
            offset: 11970 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ab as xkb_keysym_t,
            offset: 12848 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ac as xkb_keysym_t,
            offset: 26019 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b3 as xkb_keysym_t,
            offset: 22515 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b5 as xkb_keysym_t,
            offset: 18267 as uint16_t,
        },
        name_keysym {
            keysym: 0x3b6 as xkb_keysym_t,
            offset: 19395 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ba as xkb_keysym_t,
            offset: 11978 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bb as xkb_keysym_t,
            offset: 12857 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bc as xkb_keysym_t,
            offset: 26026 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bd as xkb_keysym_t,
            offset: 12114 as uint16_t,
        },
        name_keysym {
            keysym: 0x3bf as xkb_keysym_t,
            offset: 12118 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c0 as xkb_keysym_t,
            offset: 892 as uint16_t,
        },
        name_keysym {
            keysym: 0x3c7 as xkb_keysym_t,
            offset: 17498 as uint16_t,
        },
        name_keysym {
            keysym: 0x3cc as xkb_keysym_t,
            offset: 11559 as uint16_t,
        },
        name_keysym {
            keysym: 0x3cf as xkb_keysym_t,
            offset: 17391 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d1 as xkb_keysym_t,
            offset: 20176 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d2 as xkb_keysym_t,
            offset: 20864 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d3 as xkb_keysym_t,
            offset: 18939 as uint16_t,
        },
        name_keysym {
            keysym: 0x3d9 as xkb_keysym_t,
            offset: 26599 as uint16_t,
        },
        name_keysym {
            keysym: 0x3dd as xkb_keysym_t,
            offset: 26700 as uint16_t,
        },
        name_keysym {
            keysym: 0x3de as xkb_keysym_t,
            offset: 26552 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e0 as xkb_keysym_t,
            offset: 900 as uint16_t,
        },
        name_keysym {
            keysym: 0x3e7 as xkb_keysym_t,
            offset: 17506 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ec as xkb_keysym_t,
            offset: 11569 as uint16_t,
        },
        name_keysym {
            keysym: 0x3ef as xkb_keysym_t,
            offset: 17399 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f1 as xkb_keysym_t,
            offset: 20185 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f2 as xkb_keysym_t,
            offset: 20872 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f3 as xkb_keysym_t,
            offset: 18948 as uint16_t,
        },
        name_keysym {
            keysym: 0x3f9 as xkb_keysym_t,
            offset: 26607 as uint16_t,
        },
        name_keysym {
            keysym: 0x3fd as xkb_keysym_t,
            offset: 26707 as uint16_t,
        },
        name_keysym {
            keysym: 0x3fe as xkb_keysym_t,
            offset: 26560 as uint16_t,
        },
        name_keysym {
            keysym: 0x47e as xkb_keysym_t,
            offset: 21562 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a1 as xkb_keysym_t,
            offset: 18403 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a2 as xkb_keysym_t,
            offset: 18637 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a3 as xkb_keysym_t,
            offset: 18333 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a4 as xkb_keysym_t,
            offset: 18353 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a5 as xkb_keysym_t,
            offset: 18364 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a6 as xkb_keysym_t,
            offset: 18849 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a7 as xkb_keysym_t,
            offset: 18317 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a8 as xkb_keysym_t,
            offset: 18464 as uint16_t,
        },
        name_keysym {
            keysym: 0x4a9 as xkb_keysym_t,
            offset: 18834 as uint16_t,
        },
        name_keysym {
            keysym: 0x4aa as xkb_keysym_t,
            offset: 18388 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ab as xkb_keysym_t,
            offset: 18630 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ac as xkb_keysym_t,
            offset: 18865 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ad as xkb_keysym_t,
            offset: 18897 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ae as xkb_keysym_t,
            offset: 18881 as uint16_t,
        },
        name_keysym {
            keysym: 0x4af as xkb_keysym_t,
            offset: 18802 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b0 as xkb_keysym_t,
            offset: 22328 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b1 as xkb_keysym_t,
            offset: 18310 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b2 as xkb_keysym_t,
            offset: 18457 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b3 as xkb_keysym_t,
            offset: 18827 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b4 as xkb_keysym_t,
            offset: 18381 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b5 as xkb_keysym_t,
            offset: 18623 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b6 as xkb_keysym_t,
            offset: 18471 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b7 as xkb_keysym_t,
            offset: 18487 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b8 as xkb_keysym_t,
            offset: 18503 as uint16_t,
        },
        name_keysym {
            keysym: 0x4b9 as xkb_keysym_t,
            offset: 18479 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ba as xkb_keysym_t,
            offset: 18495 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bb as xkb_keysym_t,
            offset: 18697 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bc as xkb_keysym_t,
            offset: 18713 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bd as xkb_keysym_t,
            offset: 18741 as uint16_t,
        },
        name_keysym {
            keysym: 0x4be as xkb_keysym_t,
            offset: 18705 as uint16_t,
        },
        name_keysym {
            keysym: 0x4bf as xkb_keysym_t,
            offset: 18733 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c0 as xkb_keysym_t,
            offset: 18761 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c1 as xkb_keysym_t,
            offset: 18324 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c2 as xkb_keysym_t,
            offset: 18793 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c3 as xkb_keysym_t,
            offset: 18769 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c4 as xkb_keysym_t,
            offset: 18785 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c5 as xkb_keysym_t,
            offset: 18583 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c6 as xkb_keysym_t,
            offset: 18599 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c7 as xkb_keysym_t,
            offset: 18615 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c8 as xkb_keysym_t,
            offset: 18591 as uint16_t,
        },
        name_keysym {
            keysym: 0x4c9 as xkb_keysym_t,
            offset: 18607 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ca as xkb_keysym_t,
            offset: 18417 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cb as xkb_keysym_t,
            offset: 18433 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cc as xkb_keysym_t,
            offset: 18395 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cd as xkb_keysym_t,
            offset: 18425 as uint16_t,
        },
        name_keysym {
            keysym: 0x4ce as xkb_keysym_t,
            offset: 18441 as uint16_t,
        },
        name_keysym {
            keysym: 0x4cf as xkb_keysym_t,
            offset: 18521 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d0 as xkb_keysym_t,
            offset: 18537 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d1 as xkb_keysym_t,
            offset: 18568 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d2 as xkb_keysym_t,
            offset: 18529 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d3 as xkb_keysym_t,
            offset: 18560 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d4 as xkb_keysym_t,
            offset: 18857 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d5 as xkb_keysym_t,
            offset: 18889 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d6 as xkb_keysym_t,
            offset: 18873 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d7 as xkb_keysym_t,
            offset: 18657 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d8 as xkb_keysym_t,
            offset: 18673 as uint16_t,
        },
        name_keysym {
            keysym: 0x4d9 as xkb_keysym_t,
            offset: 18689 as uint16_t,
        },
        name_keysym {
            keysym: 0x4da as xkb_keysym_t,
            offset: 18665 as uint16_t,
        },
        name_keysym {
            keysym: 0x4db as xkb_keysym_t,
            offset: 18681 as uint16_t,
        },
        name_keysym {
            keysym: 0x4dc as xkb_keysym_t,
            offset: 18841 as uint16_t,
        },
        name_keysym {
            keysym: 0x4dd as xkb_keysym_t,
            offset: 18576 as uint16_t,
        },
        name_keysym {
            keysym: 0x4de as xkb_keysym_t,
            offset: 26750 as uint16_t,
        },
        name_keysym {
            keysym: 0x4df as xkb_keysym_t,
            offset: 22968 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ac as xkb_keysym_t,
            offset: 1109 as uint16_t,
        },
        name_keysym {
            keysym: 0x5bb as xkb_keysym_t,
            offset: 1764 as uint16_t,
        },
        name_keysym {
            keysym: 0x5bf as xkb_keysym_t,
            offset: 1698 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c1 as xkb_keysym_t,
            offset: 1303 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c2 as xkb_keysym_t,
            offset: 1599 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c3 as xkb_keysym_t,
            offset: 1354 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c4 as xkb_keysym_t,
            offset: 1373 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c5 as xkb_keysym_t,
            offset: 1409 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c6 as xkb_keysym_t,
            offset: 1391 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c7 as xkb_keysym_t,
            offset: 1067 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c8 as xkb_keysym_t,
            offset: 1098 as uint16_t,
        },
        name_keysym {
            keysym: 0x5c9 as xkb_keysym_t,
            offset: 1909 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ca as xkb_keysym_t,
            offset: 1898 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cb as xkb_keysym_t,
            offset: 1939 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cc as xkb_keysym_t,
            offset: 1481 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cd as xkb_keysym_t,
            offset: 1292 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ce as xkb_keysym_t,
            offset: 1557 as uint16_t,
        },
        name_keysym {
            keysym: 0x5cf as xkb_keysym_t,
            offset: 1133 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d0 as xkb_keysym_t,
            offset: 1927 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d1 as xkb_keysym_t,
            offset: 1719 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d2 as xkb_keysym_t,
            offset: 2024 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d3 as xkb_keysym_t,
            offset: 1752 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d4 as xkb_keysym_t,
            offset: 1795 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d5 as xkb_keysym_t,
            offset: 1741 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d6 as xkb_keysym_t,
            offset: 1122 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d7 as xkb_keysym_t,
            offset: 1859 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d8 as xkb_keysym_t,
            offset: 2013 as uint16_t,
        },
        name_keysym {
            keysym: 0x5d9 as xkb_keysym_t,
            offset: 1056 as uint16_t,
        },
        name_keysym {
            keysym: 0x5da as xkb_keysym_t,
            offset: 1269 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e0 as xkb_keysym_t,
            offset: 1870 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e1 as xkb_keysym_t,
            offset: 1231 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e2 as xkb_keysym_t,
            offset: 1687 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e3 as xkb_keysym_t,
            offset: 1504 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e4 as xkb_keysym_t,
            offset: 1569 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e5 as xkb_keysym_t,
            offset: 1618 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e6 as xkb_keysym_t,
            offset: 1630 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e7 as xkb_keysym_t,
            offset: 1282 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e8 as xkb_keysym_t,
            offset: 1974 as uint16_t,
        },
        name_keysym {
            keysym: 0x5e9 as xkb_keysym_t,
            offset: 1079 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ea as xkb_keysym_t,
            offset: 1985 as uint16_t,
        },
        name_keysym {
            keysym: 0x5eb as xkb_keysym_t,
            offset: 1215 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ec as xkb_keysym_t,
            offset: 1157 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ed as xkb_keysym_t,
            offset: 1528 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ee as xkb_keysym_t,
            offset: 1202 as uint16_t,
        },
        name_keysym {
            keysym: 0x5ef as xkb_keysym_t,
            offset: 1144 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f0 as xkb_keysym_t,
            offset: 1515 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f1 as xkb_keysym_t,
            offset: 1781 as uint16_t,
        },
        name_keysym {
            keysym: 0x5f2 as xkb_keysym_t,
            offset: 1808 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a1 as xkb_keysym_t,
            offset: 22996 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a2 as xkb_keysym_t,
            offset: 19784 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a3 as xkb_keysym_t,
            offset: 9550 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a4 as xkb_keysym_t,
            offset: 26443 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a5 as xkb_keysym_t,
            offset: 19756 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a6 as xkb_keysym_t,
            offset: 26418 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a7 as xkb_keysym_t,
            offset: 26469 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a8 as xkb_keysym_t,
            offset: 9574 as uint16_t,
        },
        name_keysym {
            keysym: 0x6a9 as xkb_keysym_t,
            offset: 9713 as uint16_t,
        },
        name_keysym {
            keysym: 0x6aa as xkb_keysym_t,
            offset: 9739 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ab as xkb_keysym_t,
            offset: 23115 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ac as xkb_keysym_t,
            offset: 19812 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ad as xkb_keysym_t,
            offset: 26380 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ae as xkb_keysym_t,
            offset: 8419 as uint16_t,
        },
        name_keysym {
            keysym: 0x6af as xkb_keysym_t,
            offset: 9068 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b0 as xkb_keysym_t,
            offset: 20374 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b1 as xkb_keysym_t,
            offset: 22984 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b2 as xkb_keysym_t,
            offset: 19770 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b3 as xkb_keysym_t,
            offset: 9538 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b4 as xkb_keysym_t,
            offset: 26430 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b5 as xkb_keysym_t,
            offset: 19742 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b6 as xkb_keysym_t,
            offset: 26406 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b7 as xkb_keysym_t,
            offset: 26456 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b8 as xkb_keysym_t,
            offset: 9562 as uint16_t,
        },
        name_keysym {
            keysym: 0x6b9 as xkb_keysym_t,
            offset: 9700 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ba as xkb_keysym_t,
            offset: 9726 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bb as xkb_keysym_t,
            offset: 23102 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bc as xkb_keysym_t,
            offset: 19798 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bd as xkb_keysym_t,
            offset: 26354 as uint16_t,
        },
        name_keysym {
            keysym: 0x6be as xkb_keysym_t,
            offset: 8399 as uint16_t,
        },
        name_keysym {
            keysym: 0x6bf as xkb_keysym_t,
            offset: 9054 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c0 as xkb_keysym_t,
            offset: 10294 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c1 as xkb_keysym_t,
            offset: 8875 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c2 as xkb_keysym_t,
            offset: 8898 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c3 as xkb_keysym_t,
            offset: 10047 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c4 as xkb_keysym_t,
            offset: 9042 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c5 as xkb_keysym_t,
            offset: 9526 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c6 as xkb_keysym_t,
            offset: 9116 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c7 as xkb_keysym_t,
            offset: 9305 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c8 as xkb_keysym_t,
            offset: 9364 as uint16_t,
        },
        name_keysym {
            keysym: 0x6c9 as xkb_keysym_t,
            offset: 9467 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ca as xkb_keysym_t,
            offset: 9958 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cb as xkb_keysym_t,
            offset: 9598 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cc as xkb_keysym_t,
            offset: 9140 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cd as xkb_keysym_t,
            offset: 9164 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ce as xkb_keysym_t,
            offset: 9188 as uint16_t,
        },
        name_keysym {
            keysym: 0x6cf as xkb_keysym_t,
            offset: 9763 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d0 as xkb_keysym_t,
            offset: 9816 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d1 as xkb_keysym_t,
            offset: 10242 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d2 as xkb_keysym_t,
            offset: 9256 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d3 as xkb_keysym_t,
            offset: 9280 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d4 as xkb_keysym_t,
            offset: 10022 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d5 as xkb_keysym_t,
            offset: 10071 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d6 as xkb_keysym_t,
            offset: 10343 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d7 as xkb_keysym_t,
            offset: 10218 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d8 as xkb_keysym_t,
            offset: 9992 as uint16_t,
        },
        name_keysym {
            keysym: 0x6d9 as xkb_keysym_t,
            offset: 10268 as uint16_t,
        },
        name_keysym {
            keysym: 0x6da as xkb_keysym_t,
            offset: 10318 as uint16_t,
        },
        name_keysym {
            keysym: 0x6db as xkb_keysym_t,
            offset: 9871 as uint16_t,
        },
        name_keysym {
            keysym: 0x6dc as xkb_keysym_t,
            offset: 9093 as uint16_t,
        },
        name_keysym {
            keysym: 0x6dd as xkb_keysym_t,
            offset: 9899 as uint16_t,
        },
        name_keysym {
            keysym: 0x6de as xkb_keysym_t,
            offset: 8923 as uint16_t,
        },
        name_keysym {
            keysym: 0x6df as xkb_keysym_t,
            offset: 9438 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e0 as xkb_keysym_t,
            offset: 10282 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e1 as xkb_keysym_t,
            offset: 8864 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e2 as xkb_keysym_t,
            offset: 8886 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e3 as xkb_keysym_t,
            offset: 10034 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e4 as xkb_keysym_t,
            offset: 9030 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e5 as xkb_keysym_t,
            offset: 9514 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e6 as xkb_keysym_t,
            offset: 9104 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e7 as xkb_keysym_t,
            offset: 9292 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e8 as xkb_keysym_t,
            offset: 9352 as uint16_t,
        },
        name_keysym {
            keysym: 0x6e9 as xkb_keysym_t,
            offset: 9456 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ea as xkb_keysym_t,
            offset: 9942 as uint16_t,
        },
        name_keysym {
            keysym: 0x6eb as xkb_keysym_t,
            offset: 9586 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ec as xkb_keysym_t,
            offset: 9128 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ed as xkb_keysym_t,
            offset: 9152 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ee as xkb_keysym_t,
            offset: 9176 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ef as xkb_keysym_t,
            offset: 9752 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f0 as xkb_keysym_t,
            offset: 9804 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f1 as xkb_keysym_t,
            offset: 10230 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f2 as xkb_keysym_t,
            offset: 9244 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f3 as xkb_keysym_t,
            offset: 9268 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f4 as xkb_keysym_t,
            offset: 10010 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f5 as xkb_keysym_t,
            offset: 10060 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f6 as xkb_keysym_t,
            offset: 10330 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f7 as xkb_keysym_t,
            offset: 10206 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f8 as xkb_keysym_t,
            offset: 9974 as uint16_t,
        },
        name_keysym {
            keysym: 0x6f9 as xkb_keysym_t,
            offset: 10254 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fa as xkb_keysym_t,
            offset: 10306 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fb as xkb_keysym_t,
            offset: 9858 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fc as xkb_keysym_t,
            offset: 9082 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fd as xkb_keysym_t,
            offset: 9884 as uint16_t,
        },
        name_keysym {
            keysym: 0x6fe as xkb_keysym_t,
            offset: 8910 as uint16_t,
        },
        name_keysym {
            keysym: 0x6ff as xkb_keysym_t,
            offset: 9420 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a1 as xkb_keysym_t,
            offset: 13473 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a2 as xkb_keysym_t,
            offset: 13603 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a3 as xkb_keysym_t,
            offset: 13663 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a4 as xkb_keysym_t,
            offset: 13778 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a5 as xkb_keysym_t,
            offset: 13857 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a7 as xkb_keysym_t,
            offset: 14093 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a8 as xkb_keysym_t,
            offset: 14320 as uint16_t,
        },
        name_keysym {
            keysym: 0x7a9 as xkb_keysym_t,
            offset: 14388 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ab as xkb_keysym_t,
            offset: 14029 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ae as xkb_keysym_t,
            offset: 13428 as uint16_t,
        },
        name_keysym {
            keysym: 0x7af as xkb_keysym_t,
            offset: 13741 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b1 as xkb_keysym_t,
            offset: 13491 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b2 as xkb_keysym_t,
            offset: 13623 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b3 as xkb_keysym_t,
            offset: 13679 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b4 as xkb_keysym_t,
            offset: 13795 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b5 as xkb_keysym_t,
            offset: 13876 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b6 as xkb_keysym_t,
            offset: 13812 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b7 as xkb_keysym_t,
            offset: 14113 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b8 as xkb_keysym_t,
            offset: 14340 as uint16_t,
        },
        name_keysym {
            keysym: 0x7b9 as xkb_keysym_t,
            offset: 14410 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ba as xkb_keysym_t,
            offset: 14360 as uint16_t,
        },
        name_keysym {
            keysym: 0x7bb as xkb_keysym_t,
            offset: 14047 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c1 as xkb_keysym_t,
            offset: 13449 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c2 as xkb_keysym_t,
            offset: 13509 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c3 as xkb_keysym_t,
            offset: 13717 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c4 as xkb_keysym_t,
            offset: 13551 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c5 as xkb_keysym_t,
            offset: 13575 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c6 as xkb_keysym_t,
            offset: 14450 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c7 as xkb_keysym_t,
            offset: 13643 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c8 as xkb_keysym_t,
            offset: 14268 as uint16_t,
        },
        name_keysym {
            keysym: 0x7c9 as xkb_keysym_t,
            offset: 13756 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ca as xkb_keysym_t,
            offset: 13895 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cb as xkb_keysym_t,
            offset: 13945 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cc as xkb_keysym_t,
            offset: 13969 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cd as xkb_keysym_t,
            offset: 13987 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ce as xkb_keysym_t,
            offset: 14432 as uint16_t,
        },
        name_keysym {
            keysym: 0x7cf as xkb_keysym_t,
            offset: 14065 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d0 as xkb_keysym_t,
            offset: 14153 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d1 as xkb_keysym_t,
            offset: 14191 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d2 as xkb_keysym_t,
            offset: 14211 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d4 as xkb_keysym_t,
            offset: 14248 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d5 as xkb_keysym_t,
            offset: 14292 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d6 as xkb_keysym_t,
            offset: 14133 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d7 as xkb_keysym_t,
            offset: 13531 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d8 as xkb_keysym_t,
            offset: 14171 as uint16_t,
        },
        name_keysym {
            keysym: 0x7d9 as xkb_keysym_t,
            offset: 14005 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e1 as xkb_keysym_t,
            offset: 13461 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e2 as xkb_keysym_t,
            offset: 13520 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e3 as xkb_keysym_t,
            offset: 13729 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e4 as xkb_keysym_t,
            offset: 13563 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e5 as xkb_keysym_t,
            offset: 13589 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e6 as xkb_keysym_t,
            offset: 14461 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e7 as xkb_keysym_t,
            offset: 13653 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e8 as xkb_keysym_t,
            offset: 14280 as uint16_t,
        },
        name_keysym {
            keysym: 0x7e9 as xkb_keysym_t,
            offset: 13767 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ea as xkb_keysym_t,
            offset: 13907 as uint16_t,
        },
        name_keysym {
            keysym: 0x7eb as xkb_keysym_t,
            offset: 13957 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ec as xkb_keysym_t,
            offset: 13978 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ed as xkb_keysym_t,
            offset: 13996 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ee as xkb_keysym_t,
            offset: 14441 as uint16_t,
        },
        name_keysym {
            keysym: 0x7ef as xkb_keysym_t,
            offset: 14079 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f0 as xkb_keysym_t,
            offset: 14162 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f1 as xkb_keysym_t,
            offset: 14201 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f2 as xkb_keysym_t,
            offset: 14223 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f3 as xkb_keysym_t,
            offset: 13695 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f4 as xkb_keysym_t,
            offset: 14258 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f5 as xkb_keysym_t,
            offset: 14306 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f6 as xkb_keysym_t,
            offset: 14143 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f7 as xkb_keysym_t,
            offset: 13541 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f8 as xkb_keysym_t,
            offset: 14181 as uint16_t,
        },
        name_keysym {
            keysym: 0x7f9 as xkb_keysym_t,
            offset: 14017 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a1 as xkb_keysym_t,
            offset: 19516 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a2 as xkb_keysym_t,
            offset: 25852 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a3 as xkb_keysym_t,
            offset: 16827 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a4 as xkb_keysym_t,
            offset: 25826 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a5 as xkb_keysym_t,
            offset: 3422 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a6 as xkb_keysym_t,
            offset: 26736 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a7 as xkb_keysym_t,
            offset: 25867 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a8 as xkb_keysym_t,
            offset: 3448 as uint16_t,
        },
        name_keysym {
            keysym: 0x8a9 as xkb_keysym_t,
            offset: 25916 as uint16_t,
        },
        name_keysym {
            keysym: 0x8aa as xkb_keysym_t,
            offset: 3497 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ab as xkb_keysym_t,
            offset: 25838 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ac as xkb_keysym_t,
            offset: 3434 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ad as xkb_keysym_t,
            offset: 25901 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ae as xkb_keysym_t,
            offset: 3482 as uint16_t,
        },
        name_keysym {
            keysym: 0x8af as xkb_keysym_t,
            offset: 19466 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b0 as xkb_keysym_t,
            offset: 22638 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b1 as xkb_keysym_t,
            offset: 25884 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b2 as xkb_keysym_t,
            offset: 3465 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b3 as xkb_keysym_t,
            offset: 25957 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b4 as xkb_keysym_t,
            offset: 3538 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b5 as xkb_keysym_t,
            offset: 25934 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b6 as xkb_keysym_t,
            offset: 3515 as uint16_t,
        },
        name_keysym {
            keysym: 0x8b7 as xkb_keysym_t,
            offset: 22660 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bc as xkb_keysym_t,
            offset: 19602 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bd as xkb_keysym_t,
            offset: 20310 as uint16_t,
        },
        name_keysym {
            keysym: 0x8be as xkb_keysym_t,
            offset: 13411 as uint16_t,
        },
        name_keysym {
            keysym: 0x8bf as xkb_keysym_t,
            offset: 17473 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c0 as xkb_keysym_t,
            offset: 25710 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c1 as xkb_keysym_t,
            offset: 26718 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c2 as xkb_keysym_t,
            offset: 17435 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c5 as xkb_keysym_t,
            offset: 20132 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c8 as xkb_keysym_t,
            offset: 954 as uint16_t,
        },
        name_keysym {
            keysym: 0x8c9 as xkb_keysym_t,
            offset: 23223 as uint16_t,
        },
        name_keysym {
            keysym: 0x8cd as xkb_keysym_t,
            offset: 17356 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ce as xkb_keysym_t,
            offset: 17407 as uint16_t,
        },
        name_keysym {
            keysym: 0x8cf as xkb_keysym_t,
            offset: 17315 as uint16_t,
        },
        name_keysym {
            keysym: 0x8d6 as xkb_keysym_t,
            offset: 22484 as uint16_t,
        },
        name_keysym {
            keysym: 0x8da as xkb_keysym_t,
            offset: 17415 as uint16_t,
        },
        name_keysym {
            keysym: 0x8db as xkb_keysym_t,
            offset: 17426 as uint16_t,
        },
        name_keysym {
            keysym: 0x8dc as xkb_keysym_t,
            offset: 17482 as uint16_t,
        },
        name_keysym {
            keysym: 0x8dd as xkb_keysym_t,
            offset: 26593 as uint16_t,
        },
        name_keysym {
            keysym: 0x8de as xkb_keysym_t,
            offset: 19642 as uint16_t,
        },
        name_keysym {
            keysym: 0x8df as xkb_keysym_t,
            offset: 19653 as uint16_t,
        },
        name_keysym {
            keysym: 0x8ef as xkb_keysym_t,
            offset: 21661 as uint16_t,
        },
        name_keysym {
            keysym: 0x8f6 as xkb_keysym_t,
            offset: 12787 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fb as xkb_keysym_t,
            offset: 19426 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fc as xkb_keysym_t,
            offset: 26618 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fd as xkb_keysym_t,
            offset: 22595 as uint16_t,
        },
        name_keysym {
            keysym: 0x8fe as xkb_keysym_t,
            offset: 11463 as uint16_t,
        },
        name_keysym {
            keysym: 0x9df as xkb_keysym_t,
            offset: 3410 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e0 as xkb_keysym_t,
            offset: 24014 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e1 as xkb_keysym_t,
            offset: 8603 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e2 as xkb_keysym_t,
            offset: 17203 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e3 as xkb_keysym_t,
            offset: 12544 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e4 as xkb_keysym_t,
            offset: 8809 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e5 as xkb_keysym_t,
            offset: 19616 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e8 as xkb_keysym_t,
            offset: 20260 as uint16_t,
        },
        name_keysym {
            keysym: 0x9e9 as xkb_keysym_t,
            offset: 26773 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ea as xkb_keysym_t,
            offset: 19687 as uint16_t,
        },
        name_keysym {
            keysym: 0x9eb as xkb_keysym_t,
            offset: 26647 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ec as xkb_keysym_t,
            offset: 26634 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ed as xkb_keysym_t,
            offset: 19673 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ee as xkb_keysym_t,
            offset: 8812 as uint16_t,
        },
        name_keysym {
            keysym: 0x9ef as xkb_keysym_t,
            offset: 16842 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f0 as xkb_keysym_t,
            offset: 16857 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f1 as xkb_keysym_t,
            offset: 16872 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f2 as xkb_keysym_t,
            offset: 16887 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f3 as xkb_keysym_t,
            offset: 16902 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f4 as xkb_keysym_t,
            offset: 19582 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f5 as xkb_keysym_t,
            offset: 22769 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f6 as xkb_keysym_t,
            offset: 3533 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f7 as xkb_keysym_t,
            offset: 25952 as uint16_t,
        },
        name_keysym {
            keysym: 0x9f8 as xkb_keysym_t,
            offset: 26728 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa1 as xkb_keysym_t,
            offset: 12059 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa2 as xkb_keysym_t,
            offset: 12158 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa3 as xkb_keysym_t,
            offset: 11952 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa4 as xkb_keysym_t,
            offset: 11961 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa5 as xkb_keysym_t,
            offset: 11352 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa6 as xkb_keysym_t,
            offset: 22343 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa7 as xkb_keysym_t,
            offset: 25720 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa8 as xkb_keysym_t,
            offset: 14542 as uint16_t,
        },
        name_keysym {
            keysym: 0xaa9 as xkb_keysym_t,
            offset: 11986 as uint16_t,
        },
        name_keysym {
            keysym: 0xaaa as xkb_keysym_t,
            offset: 12071 as uint16_t,
        },
        name_keysym {
            keysym: 0xaac as xkb_keysym_t,
            offset: 23211 as uint16_t,
        },
        name_keysym {
            keysym: 0xaae as xkb_keysym_t,
            offset: 11943 as uint16_t,
        },
        name_keysym {
            keysym: 0xaaf as xkb_keysym_t,
            offset: 11398 as uint16_t,
        },
        name_keysym {
            keysym: 0xab0 as xkb_keysym_t,
            offset: 20952 as uint16_t,
        },
        name_keysym {
            keysym: 0xab1 as xkb_keysym_t,
            offset: 26068 as uint16_t,
        },
        name_keysym {
            keysym: 0xab2 as xkb_keysym_t,
            offset: 20890 as uint16_t,
        },
        name_keysym {
            keysym: 0xab3 as xkb_keysym_t,
            offset: 26033 as uint16_t,
        },
        name_keysym {
            keysym: 0xab4 as xkb_keysym_t,
            offset: 25761 as uint16_t,
        },
        name_keysym {
            keysym: 0xab5 as xkb_keysym_t,
            offset: 12738 as uint16_t,
        },
        name_keysym {
            keysym: 0xab6 as xkb_keysym_t,
            offset: 20918 as uint16_t,
        },
        name_keysym {
            keysym: 0xab7 as xkb_keysym_t,
            offset: 12700 as uint16_t,
        },
        name_keysym {
            keysym: 0xab8 as xkb_keysym_t,
            offset: 8506 as uint16_t,
        },
        name_keysym {
            keysym: 0xabb as xkb_keysym_t,
            offset: 12558 as uint16_t,
        },
        name_keysym {
            keysym: 0xabc as xkb_keysym_t,
            offset: 19409 as uint16_t,
        },
        name_keysym {
            keysym: 0xabd as xkb_keysym_t,
            offset: 11271 as uint16_t,
        },
        name_keysym {
            keysym: 0xabe as xkb_keysym_t,
            offset: 22577 as uint16_t,
        },
        name_keysym {
            keysym: 0xabf as xkb_keysym_t,
            offset: 19866 as uint16_t,
        },
        name_keysym {
            keysym: 0xac3 as xkb_keysym_t,
            offset: 20880 as uint16_t,
        },
        name_keysym {
            keysym: 0xac4 as xkb_keysym_t,
            offset: 25748 as uint16_t,
        },
        name_keysym {
            keysym: 0xac5 as xkb_keysym_t,
            offset: 12688 as uint16_t,
        },
        name_keysym {
            keysym: 0xac6 as xkb_keysym_t,
            offset: 23128 as uint16_t,
        },
        name_keysym {
            keysym: 0xac9 as xkb_keysym_t,
            offset: 25991 as uint16_t,
        },
        name_keysym {
            keysym: 0xaca as xkb_keysym_t,
            offset: 23197 as uint16_t,
        },
        name_keysym {
            keysym: 0xacb as xkb_keysym_t,
            offset: 26001 as uint16_t,
        },
        name_keysym {
            keysym: 0xacc as xkb_keysym_t,
            offset: 19487 as uint16_t,
        },
        name_keysym {
            keysym: 0xacd as xkb_keysym_t,
            offset: 22681 as uint16_t,
        },
        name_keysym {
            keysym: 0xace as xkb_keysym_t,
            offset: 12021 as uint16_t,
        },
        name_keysym {
            keysym: 0xacf as xkb_keysym_t,
            offset: 12034 as uint16_t,
        },
        name_keysym {
            keysym: 0xad0 as xkb_keysym_t,
            offset: 19562 as uint16_t,
        },
        name_keysym {
            keysym: 0xad1 as xkb_keysym_t,
            offset: 22748 as uint16_t,
        },
        name_keysym {
            keysym: 0xad2 as xkb_keysym_t,
            offset: 19446 as uint16_t,
        },
        name_keysym {
            keysym: 0xad3 as xkb_keysym_t,
            offset: 22617 as uint16_t,
        },
        name_keysym {
            keysym: 0xad4 as xkb_keysym_t,
            offset: 22265 as uint16_t,
        },
        name_keysym {
            keysym: 0xad5 as xkb_keysym_t,
            offset: 21715 as uint16_t,
        },
        name_keysym {
            keysym: 0xad6 as xkb_keysym_t,
            offset: 19924 as uint16_t,
        },
        name_keysym {
            keysym: 0xad7 as xkb_keysym_t,
            offset: 22935 as uint16_t,
        },
        name_keysym {
            keysym: 0xad9 as xkb_keysym_t,
            offset: 19341 as uint16_t,
        },
        name_keysym {
            keysym: 0xada as xkb_keysym_t,
            offset: 16786 as uint16_t,
        },
        name_keysym {
            keysym: 0xadb as xkb_keysym_t,
            offset: 12586 as uint16_t,
        },
        name_keysym {
            keysym: 0xadc as xkb_keysym_t,
            offset: 12566 as uint16_t,
        },
        name_keysym {
            keysym: 0xadd as xkb_keysym_t,
            offset: 12603 as uint16_t,
        },
        name_keysym {
            keysym: 0xade as xkb_keysym_t,
            offset: 11993 as uint16_t,
        },
        name_keysym {
            keysym: 0xadf as xkb_keysym_t,
            offset: 12008 as uint16_t,
        },
        name_keysym {
            keysym: 0xae0 as xkb_keysym_t,
            offset: 12122 as uint16_t,
        },
        name_keysym {
            keysym: 0xae1 as xkb_keysym_t,
            offset: 12139 as uint16_t,
        },
        name_keysym {
            keysym: 0xae2 as xkb_keysym_t,
            offset: 20979 as uint16_t,
        },
        name_keysym {
            keysym: 0xae3 as xkb_keysym_t,
            offset: 21021 as uint16_t,
        },
        name_keysym {
            keysym: 0xae4 as xkb_keysym_t,
            offset: 21003 as uint16_t,
        },
        name_keysym {
            keysym: 0xae5 as xkb_keysym_t,
            offset: 20994 as uint16_t,
        },
        name_keysym {
            keysym: 0xae6 as xkb_keysym_t,
            offset: 12078 as uint16_t,
        },
        name_keysym {
            keysym: 0xae7 as xkb_keysym_t,
            offset: 12097 as uint16_t,
        },
        name_keysym {
            keysym: 0xae8 as xkb_keysym_t,
            offset: 12644 as uint16_t,
        },
        name_keysym {
            keysym: 0xae9 as xkb_keysym_t,
            offset: 12624 as uint16_t,
        },
        name_keysym {
            keysym: 0xaea as xkb_keysym_t,
            offset: 19504 as uint16_t,
        },
        name_keysym {
            keysym: 0xaeb as xkb_keysym_t,
            offset: 22699 as uint16_t,
        },
        name_keysym {
            keysym: 0xaec as xkb_keysym_t,
            offset: 8649 as uint16_t,
        },
        name_keysym {
            keysym: 0xaed as xkb_keysym_t,
            offset: 11344 as uint16_t,
        },
        name_keysym {
            keysym: 0xaee as xkb_keysym_t,
            offset: 16240 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf0 as xkb_keysym_t,
            offset: 19853 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf1 as xkb_keysym_t,
            offset: 10440 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf2 as xkb_keysym_t,
            offset: 11426 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf3 as xkb_keysym_t,
            offset: 8616 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf4 as xkb_keysym_t,
            offset: 3380 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf5 as xkb_keysym_t,
            offset: 20045 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf6 as xkb_keysym_t,
            offset: 20033 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf7 as xkb_keysym_t,
            offset: 19842 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf8 as xkb_keysym_t,
            offset: 12531 as uint16_t,
        },
        name_keysym {
            keysym: 0xaf9 as xkb_keysym_t,
            offset: 24585 as uint16_t,
        },
        name_keysym {
            keysym: 0xafa as xkb_keysym_t,
            offset: 24595 as uint16_t,
        },
        name_keysym {
            keysym: 0xafb as xkb_keysym_t,
            offset: 21735 as uint16_t,
        },
        name_keysym {
            keysym: 0xafc as xkb_keysym_t,
            offset: 8513 as uint16_t,
        },
        name_keysym {
            keysym: 0xafd as xkb_keysym_t,
            offset: 23252 as uint16_t,
        },
        name_keysym {
            keysym: 0xafe as xkb_keysym_t,
            offset: 11439 as uint16_t,
        },
        name_keysym {
            keysym: 0xaff as xkb_keysym_t,
            offset: 8857 as uint16_t,
        },
        name_keysym {
            keysym: 0xba3 as xkb_keysym_t,
            offset: 19436 as uint16_t,
        },
        name_keysym {
            keysym: 0xba6 as xkb_keysym_t,
            offset: 22606 as uint16_t,
        },
        name_keysym {
            keysym: 0xba8 as xkb_keysym_t,
            offset: 11473 as uint16_t,
        },
        name_keysym {
            keysym: 0xba9 as xkb_keysym_t,
            offset: 26626 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc0 as xkb_keysym_t,
            offset: 21522 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc2 as xkb_keysym_t,
            offset: 11502 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc3 as xkb_keysym_t,
            offset: 26661 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc4 as xkb_keysym_t,
            offset: 11492 as uint16_t,
        },
        name_keysym {
            keysym: 0xbc6 as xkb_keysym_t,
            offset: 26568 as uint16_t,
        },
        name_keysym {
            keysym: 0xbca as xkb_keysym_t,
            offset: 18302 as uint16_t,
        },
        name_keysym {
            keysym: 0xbcc as xkb_keysym_t,
            offset: 22358 as uint16_t,
        },
        name_keysym {
            keysym: 0xbce as xkb_keysym_t,
            offset: 26676 as uint16_t,
        },
        name_keysym {
            keysym: 0xbcf as xkb_keysym_t,
            offset: 8626 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd3 as xkb_keysym_t,
            offset: 26668 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd6 as xkb_keysym_t,
            offset: 11483 as uint16_t,
        },
        name_keysym {
            keysym: 0xbd8 as xkb_keysym_t,
            offset: 22712 as uint16_t,
        },
        name_keysym {
            keysym: 0xbda as xkb_keysym_t,
            offset: 19528 as uint16_t,
        },
        name_keysym {
            keysym: 0xbdc as xkb_keysym_t,
            offset: 19588 as uint16_t,
        },
        name_keysym {
            keysym: 0xbfc as xkb_keysym_t,
            offset: 22776 as uint16_t,
        },
        name_keysym {
            keysym: 0xcdf as xkb_keysym_t,
            offset: 16333 as uint16_t,
        },
        name_keysym {
            keysym: 0xce0 as xkb_keysym_t,
            offset: 16246 as uint16_t,
        },
        name_keysym {
            keysym: 0xce1 as xkb_keysym_t,
            offset: 16271 as uint16_t,
        },
        name_keysym {
            keysym: 0xce2 as xkb_keysym_t,
            offset: 16452 as uint16_t,
        },
        name_keysym {
            keysym: 0xce3 as xkb_keysym_t,
            offset: 16306 as uint16_t,
        },
        name_keysym {
            keysym: 0xce4 as xkb_keysym_t,
            offset: 16479 as uint16_t,
        },
        name_keysym {
            keysym: 0xce5 as xkb_keysym_t,
            offset: 16691 as uint16_t,
        },
        name_keysym {
            keysym: 0xce6 as xkb_keysym_t,
            offset: 16737 as uint16_t,
        },
        name_keysym {
            keysym: 0xce7 as xkb_keysym_t,
            offset: 16294 as uint16_t,
        },
        name_keysym {
            keysym: 0xce8 as xkb_keysym_t,
            offset: 16668 as uint16_t,
        },
        name_keysym {
            keysym: 0xce9 as xkb_keysym_t,
            offset: 16702 as uint16_t,
        },
        name_keysym {
            keysym: 0xcea as xkb_keysym_t,
            offset: 16354 as uint16_t,
        },
        name_keysym {
            keysym: 0xceb as xkb_keysym_t,
            offset: 16500 as uint16_t,
        },
        name_keysym {
            keysym: 0xcec as xkb_keysym_t,
            offset: 16523 as uint16_t,
        },
        name_keysym {
            keysym: 0xced as xkb_keysym_t,
            offset: 16371 as uint16_t,
        },
        name_keysym {
            keysym: 0xcee as xkb_keysym_t,
            offset: 16536 as uint16_t,
        },
        name_keysym {
            keysym: 0xcef as xkb_keysym_t,
            offset: 16387 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf0 as xkb_keysym_t,
            offset: 16547 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf1 as xkb_keysym_t,
            offset: 16592 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf2 as xkb_keysym_t,
            offset: 16259 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf3 as xkb_keysym_t,
            offset: 16403 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf4 as xkb_keysym_t,
            offset: 16558 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf5 as xkb_keysym_t,
            offset: 16418 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf6 as xkb_keysym_t,
            offset: 16713 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf7 as xkb_keysym_t,
            offset: 16568 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf8 as xkb_keysym_t,
            offset: 16580 as uint16_t,
        },
        name_keysym {
            keysym: 0xcf9 as xkb_keysym_t,
            offset: 16620 as uint16_t,
        },
        name_keysym {
            keysym: 0xcfa as xkb_keysym_t,
            offset: 16657 as uint16_t,
        },
        name_keysym {
            keysym: 0xda1 as xkb_keysym_t,
            offset: 24848 as uint16_t,
        },
        name_keysym {
            keysym: 0xda2 as xkb_keysym_t,
            offset: 24778 as uint16_t,
        },
        name_keysym {
            keysym: 0xda3 as xkb_keysym_t,
            offset: 24804 as uint16_t,
        },
        name_keysym {
            keysym: 0xda4 as xkb_keysym_t,
            offset: 24818 as uint16_t,
        },
        name_keysym {
            keysym: 0xda5 as xkb_keysym_t,
            offset: 24791 as uint16_t,
        },
        name_keysym {
            keysym: 0xda6 as xkb_keysym_t,
            offset: 24832 as uint16_t,
        },
        name_keysym {
            keysym: 0xda7 as xkb_keysym_t,
            offset: 25161 as uint16_t,
        },
        name_keysym {
            keysym: 0xda8 as xkb_keysym_t,
            offset: 24654 as uint16_t,
        },
        name_keysym {
            keysym: 0xda9 as xkb_keysym_t,
            offset: 24681 as uint16_t,
        },
        name_keysym {
            keysym: 0xdaa as xkb_keysym_t,
            offset: 24667 as uint16_t,
        },
        name_keysym {
            keysym: 0xdab as xkb_keysym_t,
            offset: 25522 as uint16_t,
        },
        name_keysym {
            keysym: 0xdac as xkb_keysym_t,
            offset: 24695 as uint16_t,
        },
        name_keysym {
            keysym: 0xdad as xkb_keysym_t,
            offset: 25698 as uint16_t,
        },
        name_keysym {
            keysym: 0xdae as xkb_keysym_t,
            offset: 24708 as uint16_t,
        },
        name_keysym {
            keysym: 0xdaf as xkb_keysym_t,
            offset: 25651 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb0 as xkb_keysym_t,
            offset: 25610 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb1 as xkb_keysym_t,
            offset: 25560 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb2 as xkb_keysym_t,
            offset: 25579 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb3 as xkb_keysym_t,
            offset: 25187 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb4 as xkb_keysym_t,
            offset: 24721 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb5 as xkb_keysym_t,
            offset: 25664 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb6 as xkb_keysym_t,
            offset: 25637 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb7 as xkb_keysym_t,
            offset: 25595 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb8 as xkb_keysym_t,
            offset: 25623 as uint16_t,
        },
        name_keysym {
            keysym: 0xdb9 as xkb_keysym_t,
            offset: 25198 as uint16_t,
        },
        name_keysym {
            keysym: 0xdba as xkb_keysym_t,
            offset: 24640 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbb as xkb_keysym_t,
            offset: 25289 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbc as xkb_keysym_t,
            offset: 25259 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbd as xkb_keysym_t,
            offset: 24732 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbe as xkb_keysym_t,
            offset: 25246 as uint16_t,
        },
        name_keysym {
            keysym: 0xdbf as xkb_keysym_t,
            offset: 24742 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc0 as xkb_keysym_t,
            offset: 25273 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc1 as xkb_keysym_t,
            offset: 25151 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc2 as xkb_keysym_t,
            offset: 25687 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc3 as xkb_keysym_t,
            offset: 25300 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc4 as xkb_keysym_t,
            offset: 25311 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc5 as xkb_keysym_t,
            offset: 25011 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc6 as xkb_keysym_t,
            offset: 25023 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc7 as xkb_keysym_t,
            offset: 25675 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc8 as xkb_keysym_t,
            offset: 25510 as uint16_t,
        },
        name_keysym {
            keysym: 0xdc9 as xkb_keysym_t,
            offset: 25498 as uint16_t,
        },
        name_keysym {
            keysym: 0xdca as xkb_keysym_t,
            offset: 25532 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcb as xkb_keysym_t,
            offset: 24753 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcc as xkb_keysym_t,
            offset: 24998 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcd as xkb_keysym_t,
            offset: 25208 as uint16_t,
        },
        name_keysym {
            keysym: 0xdce as xkb_keysym_t,
            offset: 24764 as uint16_t,
        },
        name_keysym {
            keysym: 0xdcf as xkb_keysym_t,
            offset: 25218 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd0 as xkb_keysym_t,
            offset: 25319 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd1 as xkb_keysym_t,
            offset: 25059 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd2 as xkb_keysym_t,
            offset: 25330 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd3 as xkb_keysym_t,
            offset: 25393 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd4 as xkb_keysym_t,
            offset: 25416 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd5 as xkb_keysym_t,
            offset: 25427 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd6 as xkb_keysym_t,
            offset: 25461 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd7 as xkb_keysym_t,
            offset: 25473 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd8 as xkb_keysym_t,
            offset: 25450 as uint16_t,
        },
        name_keysym {
            keysym: 0xdd9 as xkb_keysym_t,
            offset: 25486 as uint16_t,
        },
        name_keysym {
            keysym: 0xdda as xkb_keysym_t,
            offset: 25233 as uint16_t,
        },
        name_keysym {
            keysym: 0xdde as xkb_keysym_t,
            offset: 25075 as uint16_t,
        },
        name_keysym {
            keysym: 0xddf as xkb_keysym_t,
            offset: 24630 as uint16_t,
        },
        name_keysym {
            keysym: 0xde0 as xkb_keysym_t,
            offset: 25405 as uint16_t,
        },
        name_keysym {
            keysym: 0xde1 as xkb_keysym_t,
            offset: 25342 as uint16_t,
        },
        name_keysym {
            keysym: 0xde2 as xkb_keysym_t,
            offset: 25439 as uint16_t,
        },
        name_keysym {
            keysym: 0xde3 as xkb_keysym_t,
            offset: 25374 as uint16_t,
        },
        name_keysym {
            keysym: 0xde4 as xkb_keysym_t,
            offset: 25354 as uint16_t,
        },
        name_keysym {
            keysym: 0xde5 as xkb_keysym_t,
            offset: 24859 as uint16_t,
        },
        name_keysym {
            keysym: 0xde6 as xkb_keysym_t,
            offset: 25137 as uint16_t,
        },
        name_keysym {
            keysym: 0xde7 as xkb_keysym_t,
            offset: 25098 as uint16_t,
        },
        name_keysym {
            keysym: 0xde8 as xkb_keysym_t,
            offset: 25048 as uint16_t,
        },
        name_keysym {
            keysym: 0xde9 as xkb_keysym_t,
            offset: 25113 as uint16_t,
        },
        name_keysym {
            keysym: 0xdea as xkb_keysym_t,
            offset: 25125 as uint16_t,
        },
        name_keysym {
            keysym: 0xdeb as xkb_keysym_t,
            offset: 25031 as uint16_t,
        },
        name_keysym {
            keysym: 0xdec as xkb_keysym_t,
            offset: 25543 as uint16_t,
        },
        name_keysym {
            keysym: 0xded as xkb_keysym_t,
            offset: 25173 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf0 as xkb_keysym_t,
            offset: 24986 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf1 as xkb_keysym_t,
            offset: 24924 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf2 as xkb_keysym_t,
            offset: 24973 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf3 as xkb_keysym_t,
            offset: 24950 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf4 as xkb_keysym_t,
            offset: 24962 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf5 as xkb_keysym_t,
            offset: 24889 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf6 as xkb_keysym_t,
            offset: 24900 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf7 as xkb_keysym_t,
            offset: 24876 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf8 as xkb_keysym_t,
            offset: 24937 as uint16_t,
        },
        name_keysym {
            keysym: 0xdf9 as xkb_keysym_t,
            offset: 24912 as uint16_t,
        },
        name_keysym {
            keysym: 0xea1 as xkb_keysym_t,
            offset: 15341 as uint16_t,
        },
        name_keysym {
            keysym: 0xea2 as xkb_keysym_t,
            offset: 15886 as uint16_t,
        },
        name_keysym {
            keysym: 0xea3 as xkb_keysym_t,
            offset: 15355 as uint16_t,
        },
        name_keysym {
            keysym: 0xea4 as xkb_keysym_t,
            offset: 15436 as uint16_t,
        },
        name_keysym {
            keysym: 0xea5 as xkb_keysym_t,
            offset: 15467 as uint16_t,
        },
        name_keysym {
            keysym: 0xea6 as xkb_keysym_t,
            offset: 15449 as uint16_t,
        },
        name_keysym {
            keysym: 0xea7 as xkb_keysym_t,
            offset: 14648 as uint16_t,
        },
        name_keysym {
            keysym: 0xea8 as xkb_keysym_t,
            offset: 15849 as uint16_t,
        },
        name_keysym {
            keysym: 0xea9 as xkb_keysym_t,
            offset: 15621 as uint16_t,
        },
        name_keysym {
            keysym: 0xeaa as xkb_keysym_t,
            offset: 15652 as uint16_t,
        },
        name_keysym {
            keysym: 0xeab as xkb_keysym_t,
            offset: 15671 as uint16_t,
        },
        name_keysym {
            keysym: 0xeac as xkb_keysym_t,
            offset: 15708 as uint16_t,
        },
        name_keysym {
            keysym: 0xead as xkb_keysym_t,
            offset: 15726 as uint16_t,
        },
        name_keysym {
            keysym: 0xeae as xkb_keysym_t,
            offset: 15743 as uint16_t,
        },
        name_keysym {
            keysym: 0xeaf as xkb_keysym_t,
            offset: 15689 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb0 as xkb_keysym_t,
            offset: 15634 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb1 as xkb_keysym_t,
            offset: 15398 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb2 as xkb_keysym_t,
            offset: 15533 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb3 as xkb_keysym_t,
            offset: 15905 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb4 as xkb_keysym_t,
            offset: 15546 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb5 as xkb_keysym_t,
            offset: 15822 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb6 as xkb_keysym_t,
            offset: 15923 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb7 as xkb_keysym_t,
            offset: 14737 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb8 as xkb_keysym_t,
            offset: 15314 as uint16_t,
        },
        name_keysym {
            keysym: 0xeb9 as xkb_keysym_t,
            offset: 15868 as uint16_t,
        },
        name_keysym {
            keysym: 0xeba as xkb_keysym_t,
            offset: 14618 as uint16_t,
        },
        name_keysym {
            keysym: 0xebb as xkb_keysym_t,
            offset: 15327 as uint16_t,
        },
        name_keysym {
            keysym: 0xebc as xkb_keysym_t,
            offset: 16043 as uint16_t,
        },
        name_keysym {
            keysym: 0xebd as xkb_keysym_t,
            offset: 15519 as uint16_t,
        },
        name_keysym {
            keysym: 0xebe as xkb_keysym_t,
            offset: 14715 as uint16_t,
        },
        name_keysym {
            keysym: 0xebf as xkb_keysym_t,
            offset: 14559 as uint16_t,
        },
        name_keysym {
            keysym: 0xec0 as xkb_keysym_t,
            offset: 14568 as uint16_t,
        },
        name_keysym {
            keysym: 0xec1 as xkb_keysym_t,
            offset: 16117 as uint16_t,
        },
        name_keysym {
            keysym: 0xec2 as xkb_keysym_t,
            offset: 16127 as uint16_t,
        },
        name_keysym {
            keysym: 0xec3 as xkb_keysym_t,
            offset: 14682 as uint16_t,
        },
        name_keysym {
            keysym: 0xec4 as xkb_keysym_t,
            offset: 14662 as uint16_t,
        },
        name_keysym {
            keysym: 0xec5 as xkb_keysym_t,
            offset: 16148 as uint16_t,
        },
        name_keysym {
            keysym: 0xec6 as xkb_keysym_t,
            offset: 16138 as uint16_t,
        },
        name_keysym {
            keysym: 0xec7 as xkb_keysym_t,
            offset: 15485 as uint16_t,
        },
        name_keysym {
            keysym: 0xec8 as xkb_keysym_t,
            offset: 16065 as uint16_t,
        },
        name_keysym {
            keysym: 0xec9 as xkb_keysym_t,
            offset: 16075 as uint16_t,
        },
        name_keysym {
            keysym: 0xeca as xkb_keysym_t,
            offset: 15494 as uint16_t,
        },
        name_keysym {
            keysym: 0xecb as xkb_keysym_t,
            offset: 16188 as uint16_t,
        },
        name_keysym {
            keysym: 0xecc as xkb_keysym_t,
            offset: 16056 as uint16_t,
        },
        name_keysym {
            keysym: 0xecd as xkb_keysym_t,
            offset: 16096 as uint16_t,
        },
        name_keysym {
            keysym: 0xece as xkb_keysym_t,
            offset: 16086 as uint16_t,
        },
        name_keysym {
            keysym: 0xecf as xkb_keysym_t,
            offset: 16107 as uint16_t,
        },
        name_keysym {
            keysym: 0xed0 as xkb_keysym_t,
            offset: 16198 as uint16_t,
        },
        name_keysym {
            keysym: 0xed1 as xkb_keysym_t,
            offset: 14692 as uint16_t,
        },
        name_keysym {
            keysym: 0xed2 as xkb_keysym_t,
            offset: 16178 as uint16_t,
        },
        name_keysym {
            keysym: 0xed3 as xkb_keysym_t,
            offset: 14728 as uint16_t,
        },
        name_keysym {
            keysym: 0xed4 as xkb_keysym_t,
            offset: 14842 as uint16_t,
        },
        name_keysym {
            keysym: 0xed5 as xkb_keysym_t,
            offset: 15212 as uint16_t,
        },
        name_keysym {
            keysym: 0xed6 as xkb_keysym_t,
            offset: 14858 as uint16_t,
        },
        name_keysym {
            keysym: 0xed7 as xkb_keysym_t,
            offset: 14920 as uint16_t,
        },
        name_keysym {
            keysym: 0xed8 as xkb_keysym_t,
            offset: 14955 as uint16_t,
        },
        name_keysym {
            keysym: 0xed9 as xkb_keysym_t,
            offset: 14935 as uint16_t,
        },
        name_keysym {
            keysym: 0xeda as xkb_keysym_t,
            offset: 14765 as uint16_t,
        },
        name_keysym {
            keysym: 0xedb as xkb_keysym_t,
            offset: 15042 as uint16_t,
        },
        name_keysym {
            keysym: 0xedc as xkb_keysym_t,
            offset: 15077 as uint16_t,
        },
        name_keysym {
            keysym: 0xedd as xkb_keysym_t,
            offset: 15098 as uint16_t,
        },
        name_keysym {
            keysym: 0xede as xkb_keysym_t,
            offset: 15139 as uint16_t,
        },
        name_keysym {
            keysym: 0xedf as xkb_keysym_t,
            offset: 15159 as uint16_t,
        },
        name_keysym {
            keysym: 0xee0 as xkb_keysym_t,
            offset: 15178 as uint16_t,
        },
        name_keysym {
            keysym: 0xee1 as xkb_keysym_t,
            offset: 15118 as uint16_t,
        },
        name_keysym {
            keysym: 0xee2 as xkb_keysym_t,
            offset: 15057 as uint16_t,
        },
        name_keysym {
            keysym: 0xee3 as xkb_keysym_t,
            offset: 14905 as uint16_t,
        },
        name_keysym {
            keysym: 0xee4 as xkb_keysym_t,
            offset: 15008 as uint16_t,
        },
        name_keysym {
            keysym: 0xee5 as xkb_keysym_t,
            offset: 15023 as uint16_t,
        },
        name_keysym {
            keysym: 0xee6 as xkb_keysym_t,
            offset: 15198 as uint16_t,
        },
        name_keysym {
            keysym: 0xee7 as xkb_keysym_t,
            offset: 15233 as uint16_t,
        },
        name_keysym {
            keysym: 0xee8 as xkb_keysym_t,
            offset: 14796 as uint16_t,
        },
        name_keysym {
            keysym: 0xee9 as xkb_keysym_t,
            offset: 14811 as uint16_t,
        },
        name_keysym {
            keysym: 0xeea as xkb_keysym_t,
            offset: 14750 as uint16_t,
        },
        name_keysym {
            keysym: 0xeeb as xkb_keysym_t,
            offset: 14826 as uint16_t,
        },
        name_keysym {
            keysym: 0xeec as xkb_keysym_t,
            offset: 15252 as uint16_t,
        },
        name_keysym {
            keysym: 0xeed as xkb_keysym_t,
            offset: 14992 as uint16_t,
        },
        name_keysym {
            keysym: 0xeee as xkb_keysym_t,
            offset: 14781 as uint16_t,
        },
        name_keysym {
            keysym: 0xeef as xkb_keysym_t,
            offset: 15761 as uint16_t,
        },
        name_keysym {
            keysym: 0xef0 as xkb_keysym_t,
            offset: 15953 as uint16_t,
        },
        name_keysym {
            keysym: 0xef1 as xkb_keysym_t,
            offset: 16004 as uint16_t,
        },
        name_keysym {
            keysym: 0xef2 as xkb_keysym_t,
            offset: 15504 as uint16_t,
        },
        name_keysym {
            keysym: 0xef3 as xkb_keysym_t,
            offset: 15373 as uint16_t,
        },
        name_keysym {
            keysym: 0xef4 as xkb_keysym_t,
            offset: 15978 as uint16_t,
        },
        name_keysym {
            keysym: 0xef5 as xkb_keysym_t,
            offset: 16159 as uint16_t,
        },
        name_keysym {
            keysym: 0xef6 as xkb_keysym_t,
            offset: 14578 as uint16_t,
        },
        name_keysym {
            keysym: 0xef7 as xkb_keysym_t,
            offset: 14591 as uint16_t,
        },
        name_keysym {
            keysym: 0xef8 as xkb_keysym_t,
            offset: 14975 as uint16_t,
        },
        name_keysym {
            keysym: 0xef9 as xkb_keysym_t,
            offset: 14878 as uint16_t,
        },
        name_keysym {
            keysym: 0xefa as xkb_keysym_t,
            offset: 15267 as uint16_t,
        },
        name_keysym {
            keysym: 0xeff as xkb_keysym_t,
            offset: 18957 as uint16_t,
        },
        name_keysym {
            keysym: 0x13bc as xkb_keysym_t,
            offset: 20699 as uint16_t,
        },
        name_keysym {
            keysym: 0x13bd as xkb_keysym_t,
            offset: 20702 as uint16_t,
        },
        name_keysym {
            keysym: 0x13be as xkb_keysym_t,
            offset: 32623 as uint16_t,
        },
        name_keysym {
            keysym: 0x20ac as xkb_keysym_t,
            offset: 12221 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd01 as xkb_keysym_t,
            offset: 125 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd02 as xkb_keysym_t,
            offset: 195 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd03 as xkb_keysym_t,
            offset: 343 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd04 as xkb_keysym_t,
            offset: 245 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd05 as xkb_keysym_t,
            offset: 33 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd06 as xkb_keysym_t,
            offset: 151 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd07 as xkb_keysym_t,
            offset: 165 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd08 as xkb_keysym_t,
            offset: 332 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd09 as xkb_keysym_t,
            offset: 310 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0a as xkb_keysym_t,
            offset: 256 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0b as xkb_keysym_t,
            offset: 265 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0c as xkb_keysym_t,
            offset: 274 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0d as xkb_keysym_t,
            offset: 376 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0e as xkb_keysym_t,
            offset: 23 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd0f as xkb_keysym_t,
            offset: 74 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd10 as xkb_keysym_t,
            offset: 8 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd11 as xkb_keysym_t,
            offset: 231 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd12 as xkb_keysym_t,
            offset: 221 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd13 as xkb_keysym_t,
            offset: 210 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd14 as xkb_keysym_t,
            offset: 355 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd15 as xkb_keysym_t,
            offset: 64 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd16 as xkb_keysym_t,
            offset: 283 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd17 as xkb_keysym_t,
            offset: 365 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd18 as xkb_keysym_t,
            offset: 320 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd19 as xkb_keysym_t,
            offset: 46 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1a as xkb_keysym_t,
            offset: 109 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1b as xkb_keysym_t,
            offset: 181 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1c as xkb_keysym_t,
            offset: 91 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1d as xkb_keysym_t,
            offset: 293 as uint16_t,
        },
        name_keysym {
            keysym: 0xfd1e as xkb_keysym_t,
            offset: 140 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe01 as xkb_keysym_t,
            offset: 17941 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe02 as xkb_keysym_t,
            offset: 17824 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe03 as xkb_keysym_t,
            offset: 17874 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe04 as xkb_keysym_t,
            offset: 17841 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe05 as xkb_keysym_t,
            offset: 17858 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe06 as xkb_keysym_t,
            offset: 17729 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe07 as xkb_keysym_t,
            offset: 17745 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe08 as xkb_keysym_t,
            offset: 17986 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe09 as xkb_keysym_t,
            offset: 18001 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0a as xkb_keysym_t,
            offset: 18110 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0b as xkb_keysym_t,
            offset: 18125 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0c as xkb_keysym_t,
            offset: 17692 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0d as xkb_keysym_t,
            offset: 17708 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0e as xkb_keysym_t,
            offset: 17776 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe0f as xkb_keysym_t,
            offset: 17791 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe11 as xkb_keysym_t,
            offset: 17924 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe12 as xkb_keysym_t,
            offset: 17891 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe13 as xkb_keysym_t,
            offset: 17908 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe20 as xkb_keysym_t,
            offset: 17811 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe21 as xkb_keysym_t,
            offset: 17969 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe22 as xkb_keysym_t,
            offset: 17950 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe23 as xkb_keysym_t,
            offset: 18043 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe24 as xkb_keysym_t,
            offset: 18021 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe25 as xkb_keysym_t,
            offset: 18063 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe26 as xkb_keysym_t,
            offset: 18086 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe27 as xkb_keysym_t,
            offset: 18219 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe28 as xkb_keysym_t,
            offset: 18239 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe29 as xkb_keysym_t,
            offset: 18170 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2a as xkb_keysym_t,
            offset: 18194 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2b as xkb_keysym_t,
            offset: 18145 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2c as xkb_keysym_t,
            offset: 17630 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2d as xkb_keysym_t,
            offset: 17651 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2e as xkb_keysym_t,
            offset: 17673 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe2f as xkb_keysym_t,
            offset: 17609 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe30 as xkb_keysym_t,
            offset: 17532 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe31 as xkb_keysym_t,
            offset: 17557 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe32 as xkb_keysym_t,
            offset: 17585 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe33 as xkb_keysym_t,
            offset: 17514 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe34 as xkb_keysym_t,
            offset: 17599 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe50 as xkb_keysym_t,
            offset: 10956 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe51 as xkb_keysym_t,
            offset: 10613 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe52 as xkb_keysym_t,
            offset: 10852 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe53 as xkb_keysym_t,
            offset: 11228 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe54 as xkb_keysym_t,
            offset: 11089 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe55 as xkb_keysym_t,
            offset: 10798 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe56 as xkb_keysym_t,
            offset: 10537 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe57 as xkb_keysym_t,
            offset: 10893 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe58 as xkb_keysym_t,
            offset: 10575 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe59 as xkb_keysym_t,
            offset: 10908 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5a as xkb_keysym_t,
            offset: 10828 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5b as xkb_keysym_t,
            offset: 10839 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5c as xkb_keysym_t,
            offset: 11115 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5d as xkb_keysym_t,
            offset: 11042 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5e as xkb_keysym_t,
            offset: 11253 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe5f as xkb_keysym_t,
            offset: 11177 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe60 as xkb_keysym_t,
            offset: 10713 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe61 as xkb_keysym_t,
            offset: 10989 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe62 as xkb_keysym_t,
            offset: 10999 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe63 as xkb_keysym_t,
            offset: 11216 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe64 as xkb_keysym_t,
            offset: 10521 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe65 as xkb_keysym_t,
            offset: 10551 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe66 as xkb_keysym_t,
            offset: 10925 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe67 as xkb_keysym_t,
            offset: 10744 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe68 as xkb_keysym_t,
            offset: 10727 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe69 as xkb_keysym_t,
            offset: 10656 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6a as xkb_keysym_t,
            offset: 10759 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6b as xkb_keysym_t,
            offset: 10640 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6c as xkb_keysym_t,
            offset: 10693 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6d as xkb_keysym_t,
            offset: 11023 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6e as xkb_keysym_t,
            offset: 10677 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe6f as xkb_keysym_t,
            offset: 10868 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe70 as xkb_keysym_t,
            offset: 583 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe71 as xkb_keysym_t,
            offset: 598 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe72 as xkb_keysym_t,
            offset: 22540 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe73 as xkb_keysym_t,
            offset: 23998 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe74 as xkb_keysym_t,
            offset: 3564 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe75 as xkb_keysym_t,
            offset: 24067 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe76 as xkb_keysym_t,
            offset: 19967 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe77 as xkb_keysym_t,
            offset: 19944 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe78 as xkb_keysym_t,
            offset: 21530 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe79 as xkb_keysym_t,
            offset: 21546 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe7a as xkb_keysym_t,
            offset: 3309 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe80 as xkb_keysym_t,
            offset: 10514 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe81 as xkb_keysym_t,
            offset: 10507 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe82 as xkb_keysym_t,
            offset: 10949 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe83 as xkb_keysym_t,
            offset: 10942 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe84 as xkb_keysym_t,
            offset: 11016 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe85 as xkb_keysym_t,
            offset: 11009 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe86 as xkb_keysym_t,
            offset: 11108 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe87 as xkb_keysym_t,
            offset: 11101 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe88 as xkb_keysym_t,
            offset: 11246 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe89 as xkb_keysym_t,
            offset: 11239 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8a as xkb_keysym_t,
            offset: 11199 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8b as xkb_keysym_t,
            offset: 10809 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8c as xkb_keysym_t,
            offset: 10967 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8d as xkb_keysym_t,
            offset: 10978 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe8e as xkb_keysym_t,
            offset: 10624 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe90 as xkb_keysym_t,
            offset: 11076 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe91 as xkb_keysym_t,
            offset: 10590 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe92 as xkb_keysym_t,
            offset: 10775 as uint16_t,
        },
        name_keysym {
            keysym: 0xfe93 as xkb_keysym_t,
            offset: 11052 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea0 as xkb_keysym_t,
            offset: 8600 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea1 as xkb_keysym_t,
            offset: 8597 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea2 as xkb_keysym_t,
            offset: 8594 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea3 as xkb_keysym_t,
            offset: 8451 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea4 as xkb_keysym_t,
            offset: 8447 as uint16_t,
        },
        name_keysym {
            keysym: 0xfea5 as xkb_keysym_t,
            offset: 8443 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed0 as xkb_keysym_t,
            offset: 12667 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed1 as xkb_keysym_t,
            offset: 22278 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed2 as xkb_keysym_t,
            offset: 20213 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed4 as xkb_keysym_t,
            offset: 19321 as uint16_t,
        },
        name_keysym {
            keysym: 0xfed5 as xkb_keysym_t,
            offset: 24613 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee0 as xkb_keysym_t,
            offset: 22196 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee1 as xkb_keysym_t,
            offset: 22209 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee2 as xkb_keysym_t,
            offset: 22223 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee3 as xkb_keysym_t,
            offset: 22041 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee4 as xkb_keysym_t,
            offset: 22234 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee5 as xkb_keysym_t,
            offset: 22249 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee6 as xkb_keysym_t,
            offset: 22054 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee7 as xkb_keysym_t,
            offset: 22071 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee8 as xkb_keysym_t,
            offset: 21869 as uint16_t,
        },
        name_keysym {
            keysym: 0xfee9 as xkb_keysym_t,
            offset: 21789 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeea as xkb_keysym_t,
            offset: 21805 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeeb as xkb_keysym_t,
            offset: 21821 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeec as xkb_keysym_t,
            offset: 21837 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeed as xkb_keysym_t,
            offset: 21853 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeee as xkb_keysym_t,
            offset: 21979 as uint16_t,
        },
        name_keysym {
            keysym: 0xfeef as xkb_keysym_t,
            offset: 21889 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef0 as xkb_keysym_t,
            offset: 21907 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef1 as xkb_keysym_t,
            offset: 21925 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef2 as xkb_keysym_t,
            offset: 21943 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef3 as xkb_keysym_t,
            offset: 21961 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef4 as xkb_keysym_t,
            offset: 22159 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef5 as xkb_keysym_t,
            offset: 22089 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef6 as xkb_keysym_t,
            offset: 22103 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef7 as xkb_keysym_t,
            offset: 22117 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef8 as xkb_keysym_t,
            offset: 22131 as uint16_t,
        },
        name_keysym {
            keysym: 0xfef9 as xkb_keysym_t,
            offset: 22177 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefa as xkb_keysym_t,
            offset: 21770 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefb as xkb_keysym_t,
            offset: 22001 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefc as xkb_keysym_t,
            offset: 22021 as uint16_t,
        },
        name_keysym {
            keysym: 0xfefd as xkb_keysym_t,
            offset: 22145 as uint16_t,
        },
        name_keysym {
            keysym: 0xff08 as xkb_keysym_t,
            offset: 3362 as uint16_t,
        },
        name_keysym {
            keysym: 0xff09 as xkb_keysym_t,
            offset: 24529 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0a as xkb_keysym_t,
            offset: 19619 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0b as xkb_keysym_t,
            offset: 8633 as uint16_t,
        },
        name_keysym {
            keysym: 0xff0d as xkb_keysym_t,
            offset: 22564 as uint16_t,
        },
        name_keysym {
            keysym: 0xff13 as xkb_keysym_t,
            offset: 21679 as uint16_t,
        },
        name_keysym {
            keysym: 0xff14 as xkb_keysym_t,
            offset: 22923 as uint16_t,
        },
        name_keysym {
            keysym: 0xff15 as xkb_keysym_t,
            offset: 24510 as uint16_t,
        },
        name_keysym {
            keysym: 0xff1b as xkb_keysym_t,
            offset: 12188 as uint16_t,
        },
        name_keysym {
            keysym: 0xff20 as xkb_keysym_t,
            offset: 19996 as uint16_t,
        },
        name_keysym {
            keysym: 0xff21 as xkb_keysym_t,
            offset: 18905 as uint16_t,
        },
        name_keysym {
            keysym: 0xff22 as xkb_keysym_t,
            offset: 19987 as uint16_t,
        },
        name_keysym {
            keysym: 0xff23 as xkb_keysym_t,
            offset: 16774 as uint16_t,
        },
        name_keysym {
            keysym: 0xff24 as xkb_keysym_t,
            offset: 22786 as uint16_t,
        },
        name_keysym {
            keysym: 0xff25 as xkb_keysym_t,
            offset: 16795 as uint16_t,
        },
        name_keysym {
            keysym: 0xff26 as xkb_keysym_t,
            offset: 18930 as uint16_t,
        },
        name_keysym {
            keysym: 0xff27 as xkb_keysym_t,
            offset: 16804 as uint16_t,
        },
        name_keysym {
            keysym: 0xff28 as xkb_keysym_t,
            offset: 32750 as uint16_t,
        },
        name_keysym {
            keysym: 0xff29 as xkb_keysym_t,
            offset: 16208 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2a as xkb_keysym_t,
            offset: 32758 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2b as xkb_keysym_t,
            offset: 25983 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2c as xkb_keysym_t,
            offset: 19883 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2d as xkb_keysym_t,
            offset: 18511 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2e as xkb_keysym_t,
            offset: 18722 as uint16_t,
        },
        name_keysym {
            keysym: 0xff2f as xkb_keysym_t,
            offset: 11910 as uint16_t,
        },
        name_keysym {
            keysym: 0xff30 as xkb_keysym_t,
            offset: 11921 as uint16_t,
        },
        name_keysym {
            keysym: 0xff31 as xkb_keysym_t,
            offset: 14552 as uint16_t,
        },
        name_keysym {
            keysym: 0xff32 as xkb_keysym_t,
            offset: 15940 as uint16_t,
        },
        name_keysym {
            keysym: 0xff33 as xkb_keysym_t,
            offset: 14671 as uint16_t,
        },
        name_keysym {
            keysym: 0xff34 as xkb_keysym_t,
            offset: 14702 as uint16_t,
        },
        name_keysym {
            keysym: 0xff35 as xkb_keysym_t,
            offset: 15288 as uint16_t,
        },
        name_keysym {
            keysym: 0xff36 as xkb_keysym_t,
            offset: 15785 as uint16_t,
        },
        name_keysym {
            keysym: 0xff37 as xkb_keysym_t,
            offset: 8654 as uint16_t,
        },
        name_keysym {
            keysym: 0xff38 as xkb_keysym_t,
            offset: 15300 as uint16_t,
        },
        name_keysym {
            keysym: 0xff39 as xkb_keysym_t,
            offset: 14605 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3a as xkb_keysym_t,
            offset: 15580 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3b as xkb_keysym_t,
            offset: 15563 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3c as xkb_keysym_t,
            offset: 23236 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3d as xkb_keysym_t,
            offset: 20006 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3e as xkb_keysym_t,
            offset: 22298 as uint16_t,
        },
        name_keysym {
            keysym: 0xff3f as xkb_keysym_t,
            offset: 15834 as uint16_t,
        },
        name_keysym {
            keysym: 0xff50 as xkb_keysym_t,
            offset: 16822 as uint16_t,
        },
        name_keysym {
            keysym: 0xff51 as xkb_keysym_t,
            offset: 19404 as uint16_t,
        },
        name_keysym {
            keysym: 0xff52 as xkb_keysym_t,
            offset: 26615 as uint16_t,
        },
        name_keysym {
            keysym: 0xff53 as xkb_keysym_t,
            offset: 22571 as uint16_t,
        },
        name_keysym {
            keysym: 0xff54 as xkb_keysym_t,
            offset: 11458 as uint16_t,
        },
        name_keysym {
            keysym: 0xff55 as xkb_keysym_t,
            offset: 22322 as uint16_t,
        },
        name_keysym {
            keysym: 0xff56 as xkb_keysym_t,
            offset: 20208 as uint16_t,
        },
        name_keysym {
            keysym: 0xff57 as xkb_keysym_t,
            offset: 12067 as uint16_t,
        },
        name_keysym {
            keysym: 0xff58 as xkb_keysym_t,
            offset: 3404 as uint16_t,
        },
        name_keysym {
            keysym: 0xff60 as xkb_keysym_t,
            offset: 22951 as uint16_t,
        },
        name_keysym {
            keysym: 0xff61 as xkb_keysym_t,
            offset: 22316 as uint16_t,
        },
        name_keysym {
            keysym: 0xff62 as xkb_keysym_t,
            offset: 12248 as uint16_t,
        },
        name_keysym {
            keysym: 0xff63 as xkb_keysym_t,
            offset: 17444 as uint16_t,
        },
        name_keysym {
            keysym: 0xff65 as xkb_keysym_t,
            offset: 26588 as uint16_t,
        },
        name_keysym {
            keysym: 0xff66 as xkb_keysym_t,
            offset: 22524 as uint16_t,
        },
        name_keysym {
            keysym: 0xff67 as xkb_keysym_t,
            offset: 19890 as uint16_t,
        },
        name_keysym {
            keysym: 0xff68 as xkb_keysym_t,
            offset: 12662 as uint16_t,
        },
        name_keysym {
            keysym: 0xff69 as xkb_keysym_t,
            offset: 8489 as uint16_t,
        },
        name_keysym {
            keysym: 0xff6a as xkb_keysym_t,
            offset: 16762 as uint16_t,
        },
        name_keysym {
            keysym: 0xff6b as xkb_keysym_t,
            offset: 8377 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 17760 as uint16_t,
        },
        name_keysym {
            keysym: 0xff7f as xkb_keysym_t,
            offset: 20354 as uint16_t,
        },
        name_keysym {
            keysym: 0xff80 as xkb_keysym_t,
            offset: 19234 as uint16_t,
        },
        name_keysym {
            keysym: 0xff89 as xkb_keysym_t,
            offset: 19255 as uint16_t,
        },
        name_keysym {
            keysym: 0xff8d as xkb_keysym_t,
            offset: 19091 as uint16_t,
        },
        name_keysym {
            keysym: 0xff91 as xkb_keysym_t,
            offset: 19109 as uint16_t,
        },
        name_keysym {
            keysym: 0xff92 as xkb_keysym_t,
            offset: 19115 as uint16_t,
        },
        name_keysym {
            keysym: 0xff93 as xkb_keysym_t,
            offset: 19121 as uint16_t,
        },
        name_keysym {
            keysym: 0xff94 as xkb_keysym_t,
            offset: 19127 as uint16_t,
        },
        name_keysym {
            keysym: 0xff95 as xkb_keysym_t,
            offset: 19133 as uint16_t,
        },
        name_keysym {
            keysym: 0xff96 as xkb_keysym_t,
            offset: 19151 as uint16_t,
        },
        name_keysym {
            keysym: 0xff97 as xkb_keysym_t,
            offset: 19262 as uint16_t,
        },
        name_keysym {
            keysym: 0xff98 as xkb_keysym_t,
            offset: 19212 as uint16_t,
        },
        name_keysym {
            keysym: 0xff99 as xkb_keysym_t,
            offset: 19076 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9a as xkb_keysym_t,
            offset: 19203 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9b as xkb_keysym_t,
            offset: 19171 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9c as xkb_keysym_t,
            offset: 19084 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9d as xkb_keysym_t,
            offset: 19036 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9e as xkb_keysym_t,
            offset: 19141 as uint16_t,
        },
        name_keysym {
            keysym: 0xff9f as xkb_keysym_t,
            offset: 19056 as uint16_t,
        },
        name_keysym {
            keysym: 0xffaa as xkb_keysym_t,
            offset: 19159 as uint16_t,
        },
        name_keysym {
            keysym: 0xffab as xkb_keysym_t,
            offset: 19018 as uint16_t,
        },
        name_keysym {
            keysym: 0xffac as xkb_keysym_t,
            offset: 19221 as uint16_t,
        },
        name_keysym {
            keysym: 0xffad as xkb_keysym_t,
            offset: 19243 as uint16_t,
        },
        name_keysym {
            keysym: 0xffae as xkb_keysym_t,
            offset: 19045 as uint16_t,
        },
        name_keysym {
            keysym: 0xffaf as xkb_keysym_t,
            offset: 19066 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb0 as xkb_keysym_t,
            offset: 18968 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb1 as xkb_keysym_t,
            offset: 18973 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb2 as xkb_keysym_t,
            offset: 18978 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb3 as xkb_keysym_t,
            offset: 18983 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb4 as xkb_keysym_t,
            offset: 18988 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb5 as xkb_keysym_t,
            offset: 18993 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb6 as xkb_keysym_t,
            offset: 18998 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb7 as xkb_keysym_t,
            offset: 19003 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb8 as xkb_keysym_t,
            offset: 19008 as uint16_t,
        },
        name_keysym {
            keysym: 0xffb9 as xkb_keysym_t,
            offset: 19013 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbd as xkb_keysym_t,
            offset: 19100 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbe as xkb_keysym_t,
            offset: 12290 as uint16_t,
        },
        name_keysym {
            keysym: 0xffbf as xkb_keysym_t,
            offset: 12333 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc0 as xkb_keysym_t,
            offset: 12376 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc1 as xkb_keysym_t,
            offset: 12403 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc2 as xkb_keysym_t,
            offset: 12406 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc3 as xkb_keysym_t,
            offset: 12409 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc4 as xkb_keysym_t,
            offset: 12412 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc5 as xkb_keysym_t,
            offset: 12415 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc6 as xkb_keysym_t,
            offset: 12418 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc7 as xkb_keysym_t,
            offset: 12293 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc8 as xkb_keysym_t,
            offset: 12297 as uint16_t,
        },
        name_keysym {
            keysym: 0xffc9 as xkb_keysym_t,
            offset: 12301 as uint16_t,
        },
        name_keysym {
            keysym: 0xffca as xkb_keysym_t,
            offset: 12305 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcb as xkb_keysym_t,
            offset: 12309 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcc as xkb_keysym_t,
            offset: 12313 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcd as xkb_keysym_t,
            offset: 12317 as uint16_t,
        },
        name_keysym {
            keysym: 0xffce as xkb_keysym_t,
            offset: 12321 as uint16_t,
        },
        name_keysym {
            keysym: 0xffcf as xkb_keysym_t,
            offset: 12325 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd0 as xkb_keysym_t,
            offset: 12329 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd1 as xkb_keysym_t,
            offset: 12336 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd2 as xkb_keysym_t,
            offset: 12340 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd3 as xkb_keysym_t,
            offset: 12344 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd4 as xkb_keysym_t,
            offset: 12348 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd5 as xkb_keysym_t,
            offset: 12352 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd6 as xkb_keysym_t,
            offset: 12356 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd7 as xkb_keysym_t,
            offset: 12360 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd8 as xkb_keysym_t,
            offset: 12364 as uint16_t,
        },
        name_keysym {
            keysym: 0xffd9 as xkb_keysym_t,
            offset: 12368 as uint16_t,
        },
        name_keysym {
            keysym: 0xffda as xkb_keysym_t,
            offset: 12372 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdb as xkb_keysym_t,
            offset: 12379 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdc as xkb_keysym_t,
            offset: 12383 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdd as xkb_keysym_t,
            offset: 12387 as uint16_t,
        },
        name_keysym {
            keysym: 0xffde as xkb_keysym_t,
            offset: 12391 as uint16_t,
        },
        name_keysym {
            keysym: 0xffdf as xkb_keysym_t,
            offset: 12395 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe0 as xkb_keysym_t,
            offset: 12399 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe1 as xkb_keysym_t,
            offset: 23170 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe2 as xkb_keysym_t,
            offset: 23189 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe3 as xkb_keysym_t,
            offset: 8779 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe4 as xkb_keysym_t,
            offset: 8789 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe5 as xkb_keysym_t,
            offset: 8496 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe6 as xkb_keysym_t,
            offset: 23178 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe7 as xkb_keysym_t,
            offset: 19895 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe8 as xkb_keysym_t,
            offset: 19902 as uint16_t,
        },
        name_keysym {
            keysym: 0xffe9 as xkb_keysym_t,
            offset: 880 as uint16_t,
        },
        name_keysym {
            keysym: 0xffea as xkb_keysym_t,
            offset: 886 as uint16_t,
        },
        name_keysym {
            keysym: 0xffeb as xkb_keysym_t,
            offset: 24494 as uint16_t,
        },
        name_keysym {
            keysym: 0xffec as xkb_keysym_t,
            offset: 24502 as uint16_t,
        },
        name_keysym {
            keysym: 0xffed as xkb_keysym_t,
            offset: 17206 as uint16_t,
        },
        name_keysym {
            keysym: 0xffee as xkb_keysym_t,
            offset: 17214 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff1 as xkb_keysym_t,
            offset: 3642 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff2 as xkb_keysym_t,
            offset: 3671 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff3 as xkb_keysym_t,
            offset: 3685 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff4 as xkb_keysym_t,
            offset: 3699 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff5 as xkb_keysym_t,
            offset: 3713 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff6 as xkb_keysym_t,
            offset: 3727 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff7 as xkb_keysym_t,
            offset: 3741 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff8 as xkb_keysym_t,
            offset: 3755 as uint16_t,
        },
        name_keysym {
            keysym: 0xfff9 as xkb_keysym_t,
            offset: 3769 as uint16_t,
        },
        name_keysym {
            keysym: 0xfffa as xkb_keysym_t,
            offset: 3656 as uint16_t,
        },
        name_keysym {
            keysym: 0xffff as xkb_keysym_t,
            offset: 11291 as uint16_t,
        },
        name_keysym {
            keysym: 0xffffff as xkb_keysym_t,
            offset: 26762 as uint16_t,
        },
        name_keysym {
            keysym: 0x100012c as xkb_keysym_t,
            offset: 17277 as uint16_t,
        },
        name_keysym {
            keysym: 0x100012d as xkb_keysym_t,
            offset: 17284 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000174 as xkb_keysym_t,
            offset: 26794 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000175 as xkb_keysym_t,
            offset: 26806 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000176 as xkb_keysym_t,
            offset: 32599 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000177 as xkb_keysym_t,
            offset: 32611 as uint16_t,
        },
        name_keysym {
            keysym: 0x100018f as xkb_keysym_t,
            offset: 22873 as uint16_t,
        },
        name_keysym {
            keysym: 0x100019f as xkb_keysym_t,
            offset: 20403 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001a0 as xkb_keysym_t,
            offset: 20738 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001a1 as xkb_keysym_t,
            offset: 20744 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001af as xkb_keysym_t,
            offset: 26228 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b0 as xkb_keysym_t,
            offset: 26234 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b5 as xkb_keysym_t,
            offset: 32801 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b6 as xkb_keysym_t,
            offset: 32809 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001b7 as xkb_keysym_t,
            offset: 12278 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001d1 as xkb_keysym_t,
            offset: 20439 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001d2 as xkb_keysym_t,
            offset: 20446 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001e6 as xkb_keysym_t,
            offset: 12834 as uint16_t,
        },
        name_keysym {
            keysym: 0x10001e7 as xkb_keysym_t,
            offset: 12841 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000259 as xkb_keysym_t,
            offset: 22879 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000275 as xkb_keysym_t,
            offset: 20411 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000292 as xkb_keysym_t,
            offset: 12282 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000300 as xkb_keysym_t,
            offset: 8715 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000301 as xkb_keysym_t,
            offset: 8680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000303 as xkb_keysym_t,
            offset: 8746 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000309 as xkb_keysym_t,
            offset: 8731 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000323 as xkb_keysym_t,
            offset: 8696 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000492 as xkb_keysym_t,
            offset: 9318 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000493 as xkb_keysym_t,
            offset: 9335 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000496 as xkb_keysym_t,
            offset: 10356 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000497 as xkb_keysym_t,
            offset: 10379 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049a as xkb_keysym_t,
            offset: 9610 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049b as xkb_keysym_t,
            offset: 9632 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049c as xkb_keysym_t,
            offset: 9654 as uint16_t,
        },
        name_keysym {
            keysym: 0x100049d as xkb_keysym_t,
            offset: 9677 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004a2 as xkb_keysym_t,
            offset: 9200 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004a3 as xkb_keysym_t,
            offset: 9222 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ae as xkb_keysym_t,
            offset: 10118 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004af as xkb_keysym_t,
            offset: 10138 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b0 as xkb_keysym_t,
            offset: 10158 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b1 as xkb_keysym_t,
            offset: 10182 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b2 as xkb_keysym_t,
            offset: 9376 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b3 as xkb_keysym_t,
            offset: 9398 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b6 as xkb_keysym_t,
            offset: 8936 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b7 as xkb_keysym_t,
            offset: 8959 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b8 as xkb_keysym_t,
            offset: 8982 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004b9 as xkb_keysym_t,
            offset: 9006 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ba as xkb_keysym_t,
            offset: 9914 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004bb as xkb_keysym_t,
            offset: 9928 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004d8 as xkb_keysym_t,
            offset: 9828 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004d9 as xkb_keysym_t,
            offset: 9843 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e2 as xkb_keysym_t,
            offset: 9478 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e3 as xkb_keysym_t,
            offset: 9496 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e8 as xkb_keysym_t,
            offset: 9774 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004e9 as xkb_keysym_t,
            offset: 9789 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ee as xkb_keysym_t,
            offset: 10082 as uint16_t,
        },
        name_keysym {
            keysym: 0x10004ef as xkb_keysym_t,
            offset: 10100 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000531 as xkb_keysym_t,
            offset: 2124 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000532 as xkb_keysym_t,
            offset: 2150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000533 as xkb_keysym_t,
            offset: 2374 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000534 as xkb_keysym_t,
            offset: 2215 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000535 as xkb_keysym_t,
            offset: 3164 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000536 as xkb_keysym_t,
            offset: 3210 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000537 as xkb_keysym_t,
            offset: 2265 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000538 as xkb_keysym_t,
            offset: 2100 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000539 as xkb_keysym_t,
            offset: 2964 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053a as xkb_keysym_t,
            offset: 3234 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053b as xkb_keysym_t,
            offset: 2464 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053c as xkb_keysym_t,
            offset: 2611 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053d as xkb_keysym_t,
            offset: 2564 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053e as xkb_keysym_t,
            offset: 2988 as uint16_t,
        },
        name_keysym {
            keysym: 0x100053f as xkb_keysym_t,
            offset: 2538 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000540 as xkb_keysym_t,
            offset: 2424 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000541 as xkb_keysym_t,
            offset: 2239 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000542 as xkb_keysym_t,
            offset: 2346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000543 as xkb_keysym_t,
            offset: 2936 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000544 as xkb_keysym_t,
            offset: 2639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000545 as xkb_keysym_t,
            offset: 2400 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000546 as xkb_keysym_t,
            offset: 2665 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000547 as xkb_keysym_t,
            offset: 2894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000548 as xkb_keysym_t,
            offset: 3112 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000549 as xkb_keysym_t,
            offset: 2189 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054a as xkb_keysym_t,
            offset: 2727 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054b as xkb_keysym_t,
            offset: 2490 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054c as xkb_keysym_t,
            offset: 2797 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054d as xkb_keysym_t,
            offset: 2845 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054e as xkb_keysym_t,
            offset: 3086 as uint16_t,
        },
        name_keysym {
            keysym: 0x100054f as xkb_keysym_t,
            offset: 3040 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000550 as xkb_keysym_t,
            offset: 2821 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000551 as xkb_keysym_t,
            offset: 3014 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000552 as xkb_keysym_t,
            offset: 3136 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000553 as xkb_keysym_t,
            offset: 2751 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000554 as xkb_keysym_t,
            offset: 2514 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000555 as xkb_keysym_t,
            offset: 2689 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000556 as xkb_keysym_t,
            offset: 2303 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055a as xkb_keysym_t,
            offset: 2080 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055b as xkb_keysym_t,
            offset: 2048 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055c as xkb_keysym_t,
            offset: 2287 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055d as xkb_keysym_t,
            offset: 2869 as uint16_t,
        },
        name_keysym {
            keysym: 0x100055e as xkb_keysym_t,
            offset: 2779 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000561 as xkb_keysym_t,
            offset: 2137 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000562 as xkb_keysym_t,
            offset: 2163 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000563 as xkb_keysym_t,
            offset: 2387 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000564 as xkb_keysym_t,
            offset: 2227 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000565 as xkb_keysym_t,
            offset: 3178 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000566 as xkb_keysym_t,
            offset: 3222 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000567 as xkb_keysym_t,
            offset: 2276 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000568 as xkb_keysym_t,
            offset: 2112 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000569 as xkb_keysym_t,
            offset: 2976 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056a as xkb_keysym_t,
            offset: 3247 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056b as xkb_keysym_t,
            offset: 2477 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056c as xkb_keysym_t,
            offset: 2625 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056d as xkb_keysym_t,
            offset: 2577 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056e as xkb_keysym_t,
            offset: 3001 as uint16_t,
        },
        name_keysym {
            keysym: 0x100056f as xkb_keysym_t,
            offset: 2551 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000570 as xkb_keysym_t,
            offset: 2436 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000571 as xkb_keysym_t,
            offset: 2252 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000572 as xkb_keysym_t,
            offset: 2360 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000573 as xkb_keysym_t,
            offset: 2950 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000574 as xkb_keysym_t,
            offset: 2652 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000575 as xkb_keysym_t,
            offset: 2412 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000576 as xkb_keysym_t,
            offset: 2677 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000577 as xkb_keysym_t,
            offset: 2907 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000578 as xkb_keysym_t,
            offset: 3124 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000579 as xkb_keysym_t,
            offset: 2202 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057a as xkb_keysym_t,
            offset: 2739 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057b as xkb_keysym_t,
            offset: 2502 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057c as xkb_keysym_t,
            offset: 2809 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057d as xkb_keysym_t,
            offset: 2857 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057e as xkb_keysym_t,
            offset: 3099 as uint16_t,
        },
        name_keysym {
            keysym: 0x100057f as xkb_keysym_t,
            offset: 3054 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000580 as xkb_keysym_t,
            offset: 2833 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000581 as xkb_keysym_t,
            offset: 3027 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000582 as xkb_keysym_t,
            offset: 3150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000583 as xkb_keysym_t,
            offset: 2765 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000584 as xkb_keysym_t,
            offset: 2526 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000585 as xkb_keysym_t,
            offset: 2700 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000586 as xkb_keysym_t,
            offset: 2315 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000587 as xkb_keysym_t,
            offset: 2590 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000589 as xkb_keysym_t,
            offset: 2327 as uint16_t,
        },
        name_keysym {
            keysym: 0x100058a as xkb_keysym_t,
            offset: 2448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000653 as xkb_keysym_t,
            offset: 1580 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000654 as xkb_keysym_t,
            offset: 1316 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000655 as xkb_keysym_t,
            offset: 1335 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000660 as xkb_keysym_t,
            offset: 966 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000661 as xkb_keysym_t,
            offset: 975 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000662 as xkb_keysym_t,
            offset: 984 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000663 as xkb_keysym_t,
            offset: 993 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000664 as xkb_keysym_t,
            offset: 1002 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000665 as xkb_keysym_t,
            offset: 1011 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000666 as xkb_keysym_t,
            offset: 1020 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000667 as xkb_keysym_t,
            offset: 1029 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000668 as xkb_keysym_t,
            offset: 1038 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000669 as xkb_keysym_t,
            offset: 1047 as uint16_t,
        },
        name_keysym {
            keysym: 0x100066a as xkb_keysym_t,
            offset: 1672 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000670 as xkb_keysym_t,
            offset: 1821 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000679 as xkb_keysym_t,
            offset: 1951 as uint16_t,
        },
        name_keysym {
            keysym: 0x100067e as xkb_keysym_t,
            offset: 1661 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000686 as xkb_keysym_t,
            offset: 1885 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000688 as xkb_keysym_t,
            offset: 1173 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000691 as xkb_keysym_t,
            offset: 1729 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000698 as xkb_keysym_t,
            offset: 1493 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006a4 as xkb_keysym_t,
            offset: 1963 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006a9 as xkb_keysym_t,
            offset: 1544 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006af as xkb_keysym_t,
            offset: 1258 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006ba as xkb_keysym_t,
            offset: 1642 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006be as xkb_keysym_t,
            offset: 1442 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006c1 as xkb_keysym_t,
            offset: 1465 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006cc as xkb_keysym_t,
            offset: 12521 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006d2 as xkb_keysym_t,
            offset: 1996 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006d4 as xkb_keysym_t,
            offset: 1242 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f0 as xkb_keysym_t,
            offset: 12441 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f1 as xkb_keysym_t,
            offset: 12449 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f2 as xkb_keysym_t,
            offset: 12457 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f3 as xkb_keysym_t,
            offset: 12465 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f4 as xkb_keysym_t,
            offset: 12473 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f5 as xkb_keysym_t,
            offset: 12481 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f6 as xkb_keysym_t,
            offset: 12489 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f7 as xkb_keysym_t,
            offset: 12497 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f8 as xkb_keysym_t,
            offset: 12505 as uint16_t,
        },
        name_keysym {
            keysym: 0x10006f9 as xkb_keysym_t,
            offset: 12513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d82 as xkb_keysym_t,
            offset: 23708 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d83 as xkb_keysym_t,
            offset: 23502 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d85 as xkb_keysym_t,
            offset: 23271 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d86 as xkb_keysym_t,
            offset: 23278 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d87 as xkb_keysym_t,
            offset: 23295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d88 as xkb_keysym_t,
            offset: 23312 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d89 as xkb_keysym_t,
            offset: 23518 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8a as xkb_keysym_t,
            offset: 23533 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8b as xkb_keysym_t,
            offset: 23919 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8c as xkb_keysym_t,
            offset: 23934 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8d as xkb_keysym_t,
            offset: 23818 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8e as xkb_keysym_t,
            offset: 23826 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d8f as xkb_keysym_t,
            offset: 23627 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d90 as xkb_keysym_t,
            offset: 23644 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d91 as xkb_keysym_t,
            offset: 23445 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d92 as xkb_keysym_t,
            offset: 23460 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d93 as xkb_keysym_t,
            offset: 23331 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d94 as xkb_keysym_t,
            offset: 23761 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d95 as xkb_keysym_t,
            offset: 23776 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d96 as xkb_keysym_t,
            offset: 23356 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9a as xkb_keysym_t,
            offset: 23577 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9b as xkb_keysym_t,
            offset: 23585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9c as xkb_keysym_t,
            offset: 23485 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9d as xkb_keysym_t,
            offset: 23493 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9e as xkb_keysym_t,
            offset: 23716 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000d9f as xkb_keysym_t,
            offset: 23725 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da0 as xkb_keysym_t,
            offset: 23390 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da1 as xkb_keysym_t,
            offset: 23398 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da2 as xkb_keysym_t,
            offset: 23550 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da3 as xkb_keysym_t,
            offset: 23558 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da4 as xkb_keysym_t,
            offset: 23752 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da5 as xkb_keysym_t,
            offset: 23567 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da6 as xkb_keysym_t,
            offset: 23734 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da7 as xkb_keysym_t,
            offset: 23900 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da8 as xkb_keysym_t,
            offset: 23909 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000da9 as xkb_keysym_t,
            offset: 23407 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000daa as xkb_keysym_t,
            offset: 23416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dab as xkb_keysym_t,
            offset: 23743 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dac as xkb_keysym_t,
            offset: 23688 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dad as xkb_keysym_t,
            offset: 23881 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dae as xkb_keysym_t,
            offset: 23890 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000daf as xkb_keysym_t,
            offset: 23426 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db0 as xkb_keysym_t,
            offset: 23435 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db1 as xkb_keysym_t,
            offset: 23680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db3 as xkb_keysym_t,
            offset: 23698 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db4 as xkb_keysym_t,
            offset: 23793 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db5 as xkb_keysym_t,
            offset: 23801 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db6 as xkb_keysym_t,
            offset: 23373 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db7 as xkb_keysym_t,
            offset: 23381 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db8 as xkb_keysym_t,
            offset: 23663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000db9 as xkb_keysym_t,
            offset: 23671 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dba as xkb_keysym_t,
            offset: 23959 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dbb as xkb_keysym_t,
            offset: 23810 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dbd as xkb_keysym_t,
            offset: 23610 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc0 as xkb_keysym_t,
            offset: 23951 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc1 as xkb_keysym_t,
            offset: 23862 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc2 as xkb_keysym_t,
            offset: 23871 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc3 as xkb_keysym_t,
            offset: 23854 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc4 as xkb_keysym_t,
            offset: 23510 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc5 as xkb_keysym_t,
            offset: 23618 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dc6 as xkb_keysym_t,
            offset: 23477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dca as xkb_keysym_t,
            offset: 23348 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dcf as xkb_keysym_t,
            offset: 23286 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd0 as xkb_keysym_t,
            offset: 23303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd1 as xkb_keysym_t,
            offset: 23321 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd2 as xkb_keysym_t,
            offset: 23525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd3 as xkb_keysym_t,
            offset: 23541 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd4 as xkb_keysym_t,
            offset: 23926 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd6 as xkb_keysym_t,
            offset: 23942 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd8 as xkb_keysym_t,
            offset: 23835 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dd9 as xkb_keysym_t,
            offset: 23452 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dda as xkb_keysym_t,
            offset: 23468 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddb as xkb_keysym_t,
            offset: 23339 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddc as xkb_keysym_t,
            offset: 23768 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddd as xkb_keysym_t,
            offset: 23784 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000dde as xkb_keysym_t,
            offset: 23364 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ddf as xkb_keysym_t,
            offset: 23635 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df2 as xkb_keysym_t,
            offset: 23844 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df3 as xkb_keysym_t,
            offset: 23653 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000df4 as xkb_keysym_t,
            offset: 23594 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d0 as xkb_keysym_t,
            offset: 12890 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d1 as xkb_keysym_t,
            offset: 12902 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d2 as xkb_keysym_t,
            offset: 13006 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d3 as xkb_keysym_t,
            offset: 12969 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d4 as xkb_keysym_t,
            offset: 12982 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d5 as xkb_keysym_t,
            offset: 13332 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d6 as xkb_keysym_t,
            offset: 13370 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d7 as xkb_keysym_t,
            offset: 13294 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d8 as xkb_keysym_t,
            offset: 13097 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010d9 as xkb_keysym_t,
            offset: 13136 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010da as xkb_keysym_t,
            offset: 13163 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010db as xkb_keysym_t,
            offset: 13176 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010dc as xkb_keysym_t,
            offset: 13189 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010dd as xkb_keysym_t,
            offset: 13202 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010de as xkb_keysym_t,
            offset: 13214 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010df as xkb_keysym_t,
            offset: 13383 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e0 as xkb_keysym_t,
            offset: 13254 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e1 as xkb_keysym_t,
            offset: 13267 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e2 as xkb_keysym_t,
            offset: 13307 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e3 as xkb_keysym_t,
            offset: 13320 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e4 as xkb_keysym_t,
            offset: 13227 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e5 as xkb_keysym_t,
            offset: 13149 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e6 as xkb_keysym_t,
            offset: 13019 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e7 as xkb_keysym_t,
            offset: 13241 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e8 as xkb_keysym_t,
            offset: 13280 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010e9 as xkb_keysym_t,
            offset: 12942 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ea as xkb_keysym_t,
            offset: 12915 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010eb as xkb_keysym_t,
            offset: 13123 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ec as xkb_keysym_t,
            offset: 12956 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ed as xkb_keysym_t,
            offset: 12928 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ee as xkb_keysym_t,
            offset: 13357 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010ef as xkb_keysym_t,
            offset: 13109 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f0 as xkb_keysym_t,
            offset: 13033 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f1 as xkb_keysym_t,
            offset: 13059 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f2 as xkb_keysym_t,
            offset: 13071 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f3 as xkb_keysym_t,
            offset: 13345 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f4 as xkb_keysym_t,
            offset: 13046 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f5 as xkb_keysym_t,
            offset: 13084 as uint16_t,
        },
        name_keysym {
            keysym: 0x10010f6 as xkb_keysym_t,
            offset: 12994 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e02 as xkb_keysym_t,
            offset: 3332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e03 as xkb_keysym_t,
            offset: 3342 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e0a as xkb_keysym_t,
            offset: 10406 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e0b as xkb_keysym_t,
            offset: 10416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e1e as xkb_keysym_t,
            offset: 12421 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e1f as xkb_keysym_t,
            offset: 12431 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e36 as xkb_keysym_t,
            offset: 19352 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e37 as xkb_keysym_t,
            offset: 19362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e40 as xkb_keysym_t,
            offset: 19722 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e41 as xkb_keysym_t,
            offset: 19732 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e56 as xkb_keysym_t,
            offset: 21575 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e57 as xkb_keysym_t,
            offset: 21585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e60 as xkb_keysym_t,
            offset: 22807 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e61 as xkb_keysym_t,
            offset: 22817 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e6a as xkb_keysym_t,
            offset: 24533 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e6b as xkb_keysym_t,
            offset: 24543 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e80 as xkb_keysym_t,
            offset: 26840 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e81 as xkb_keysym_t,
            offset: 26847 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e82 as xkb_keysym_t,
            offset: 26780 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e83 as xkb_keysym_t,
            offset: 26787 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e84 as xkb_keysym_t,
            offset: 26818 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e85 as xkb_keysym_t,
            offset: 26829 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e8a as xkb_keysym_t,
            offset: 26866 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e8b as xkb_keysym_t,
            offset: 26876 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001e9e as xkb_keysym_t,
            offset: 24044 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea0 as xkb_keysym_t,
            offset: 416 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea1 as xkb_keysym_t,
            offset: 426 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea2 as xkb_keysym_t,
            offset: 868 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea3 as xkb_keysym_t,
            offset: 874 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea4 as xkb_keysym_t,
            offset: 646 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea5 as xkb_keysym_t,
            offset: 663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea6 as xkb_keysym_t,
            offset: 720 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea7 as xkb_keysym_t,
            offset: 737 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea8 as xkb_keysym_t,
            offset: 754 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ea9 as xkb_keysym_t,
            offset: 770 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eaa as xkb_keysym_t,
            offset: 786 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eab as xkb_keysym_t,
            offset: 803 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eac as xkb_keysym_t,
            offset: 680 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ead as xkb_keysym_t,
            offset: 700 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eae as xkb_keysym_t,
            offset: 459 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eaf as xkb_keysym_t,
            offset: 471 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb0 as xkb_keysym_t,
            offset: 513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb1 as xkb_keysym_t,
            offset: 525 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb2 as xkb_keysym_t,
            offset: 537 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb3 as xkb_keysym_t,
            offset: 548 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb4 as xkb_keysym_t,
            offset: 559 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb5 as xkb_keysym_t,
            offset: 571 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb6 as xkb_keysym_t,
            offset: 483 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb7 as xkb_keysym_t,
            offset: 498 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb8 as xkb_keysym_t,
            offset: 11593 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eb9 as xkb_keysym_t,
            offset: 11603 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eba as xkb_keysym_t,
            offset: 11869 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebb as xkb_keysym_t,
            offset: 11875 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebc as xkb_keysym_t,
            offset: 12207 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebd as xkb_keysym_t,
            offset: 12214 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebe as xkb_keysym_t,
            offset: 11651 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ebf as xkb_keysym_t,
            offset: 11668 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec0 as xkb_keysym_t,
            offset: 11725 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec1 as xkb_keysym_t,
            offset: 11742 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec2 as xkb_keysym_t,
            offset: 11759 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec3 as xkb_keysym_t,
            offset: 11775 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec4 as xkb_keysym_t,
            offset: 11791 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec5 as xkb_keysym_t,
            offset: 11808 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec6 as xkb_keysym_t,
            offset: 11685 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec7 as xkb_keysym_t,
            offset: 11705 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec8 as xkb_keysym_t,
            offset: 17379 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ec9 as xkb_keysym_t,
            offset: 17385 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eca as xkb_keysym_t,
            offset: 17257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecb as xkb_keysym_t,
            offset: 17267 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecc as xkb_keysym_t,
            offset: 20419 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecd as xkb_keysym_t,
            offset: 20429 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ece as xkb_keysym_t,
            offset: 20726 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ecf as xkb_keysym_t,
            offset: 20732 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed0 as xkb_keysym_t,
            offset: 20477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed1 as xkb_keysym_t,
            offset: 20494 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed2 as xkb_keysym_t,
            offset: 20551 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed3 as xkb_keysym_t,
            offset: 20568 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed4 as xkb_keysym_t,
            offset: 20585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed5 as xkb_keysym_t,
            offset: 20601 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed6 as xkb_keysym_t,
            offset: 20617 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed7 as xkb_keysym_t,
            offset: 20634 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed8 as xkb_keysym_t,
            offset: 20511 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ed9 as xkb_keysym_t,
            offset: 20531 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eda as xkb_keysym_t,
            offset: 20750 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edb as xkb_keysym_t,
            offset: 20761 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edc as xkb_keysym_t,
            offset: 20800 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edd as xkb_keysym_t,
            offset: 20811 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ede as xkb_keysym_t,
            offset: 20822 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001edf as xkb_keysym_t,
            offset: 20832 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee0 as xkb_keysym_t,
            offset: 20842 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee1 as xkb_keysym_t,
            offset: 20853 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee2 as xkb_keysym_t,
            offset: 20772 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee3 as xkb_keysym_t,
            offset: 20786 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee4 as xkb_keysym_t,
            offset: 26096 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee5 as xkb_keysym_t,
            offset: 26106 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee6 as xkb_keysym_t,
            offset: 26216 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee7 as xkb_keysym_t,
            offset: 26222 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee8 as xkb_keysym_t,
            offset: 26240 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ee9 as xkb_keysym_t,
            offset: 26251 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eea as xkb_keysym_t,
            offset: 26290 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eeb as xkb_keysym_t,
            offset: 26301 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eec as xkb_keysym_t,
            offset: 26312 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eed as xkb_keysym_t,
            offset: 26322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eee as xkb_keysym_t,
            offset: 26332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001eef as xkb_keysym_t,
            offset: 26343 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef0 as xkb_keysym_t,
            offset: 26262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef1 as xkb_keysym_t,
            offset: 26276 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef2 as xkb_keysym_t,
            offset: 32649 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef3 as xkb_keysym_t,
            offset: 32656 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef4 as xkb_keysym_t,
            offset: 32579 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef5 as xkb_keysym_t,
            offset: 32589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef6 as xkb_keysym_t,
            offset: 32663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef7 as xkb_keysym_t,
            offset: 32669 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef8 as xkb_keysym_t,
            offset: 32675 as uint16_t,
        },
        name_keysym {
            keysym: 0x1001ef9 as xkb_keysym_t,
            offset: 32682 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002039 as xkb_keysym_t,
            offset: 19537 as uint16_t,
        },
        name_keysym {
            keysym: 0x100203a as xkb_keysym_t,
            offset: 22722 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002070 as xkb_keysym_t,
            offset: 32788 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002074 as xkb_keysym_t,
            offset: 12763 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002075 as xkb_keysym_t,
            offset: 12725 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002076 as xkb_keysym_t,
            offset: 23980 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002077 as xkb_keysym_t,
            offset: 23156 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002078 as xkb_keysym_t,
            offset: 11896 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002079 as xkb_keysym_t,
            offset: 20247 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002080 as xkb_keysym_t,
            offset: 32774 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002081 as xkb_keysym_t,
            offset: 20927 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002082 as xkb_keysym_t,
            offset: 26043 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002083 as xkb_keysym_t,
            offset: 25787 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002084 as xkb_keysym_t,
            offset: 12749 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002085 as xkb_keysym_t,
            offset: 12711 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002086 as xkb_keysym_t,
            offset: 23967 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002087 as xkb_keysym_t,
            offset: 23141 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002088 as xkb_keysym_t,
            offset: 11881 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002089 as xkb_keysym_t,
            offset: 20233 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a0 as xkb_keysym_t,
            offset: 11825 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a1 as xkb_keysym_t,
            offset: 8670 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a2 as xkb_keysym_t,
            offset: 8826 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a3 as xkb_keysym_t,
            offset: 12547 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a4 as xkb_keysym_t,
            offset: 19633 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a5 as xkb_keysym_t,
            offset: 19909 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a6 as xkb_keysym_t,
            offset: 20152 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a7 as xkb_keysym_t,
            offset: 21724 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a8 as xkb_keysym_t,
            offset: 22793 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020a9 as xkb_keysym_t,
            offset: 26854 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020aa as xkb_keysym_t,
            offset: 20194 as uint16_t,
        },
        name_keysym {
            keysym: 0x10020ab as xkb_keysym_t,
            offset: 11389 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002202 as xkb_keysym_t,
            offset: 21644 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002205 as xkb_keysym_t,
            offset: 12050 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002208 as xkb_keysym_t,
            offset: 11933 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002209 as xkb_keysym_t,
            offset: 20297 as uint16_t,
        },
        name_keysym {
            keysym: 0x100220b as xkb_keysym_t,
            offset: 8768 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221a as xkb_keysym_t,
            offset: 24033 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221b as xkb_keysym_t,
            offset: 8839 as uint16_t,
        },
        name_keysym {
            keysym: 0x100221c as xkb_keysym_t,
            offset: 12776 as uint16_t,
        },
        name_keysym {
            keysym: 0x100222c as xkb_keysym_t,
            offset: 11363 as uint16_t,
        },
        name_keysym {
            keysym: 0x100222d as xkb_keysym_t,
            offset: 25816 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002235 as xkb_keysym_t,
            offset: 3396 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002247 as xkb_keysym_t,
            offset: 20285 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002248 as xkb_keysym_t,
            offset: 945 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002262 as xkb_keysym_t,
            offset: 20319 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002263 as xkb_keysym_t,
            offset: 24085 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002800 as xkb_keysym_t,
            offset: 3628 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002801 as xkb_keysym_t,
            offset: 3783 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002802 as xkb_keysym_t,
            offset: 6151 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002803 as xkb_keysym_t,
            offset: 3798 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002804 as xkb_keysym_t,
            offset: 7303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002805 as xkb_keysym_t,
            offset: 5014 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002806 as xkb_keysym_t,
            offset: 6166 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002807 as xkb_keysym_t,
            offset: 3814 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002808 as xkb_keysym_t,
            offset: 7863 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002809 as xkb_keysym_t,
            offset: 5606 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280a as xkb_keysym_t,
            offset: 6758 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280b as xkb_keysym_t,
            offset: 4438 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280c as xkb_keysym_t,
            offset: 7318 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280d as xkb_keysym_t,
            offset: 5030 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280e as xkb_keysym_t,
            offset: 6182 as uint16_t,
        },
        name_keysym {
            keysym: 0x100280f as xkb_keysym_t,
            offset: 3831 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002810 as xkb_keysym_t,
            offset: 8135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002811 as xkb_keysym_t,
            offset: 5894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002812 as xkb_keysym_t,
            offset: 7046 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002813 as xkb_keysym_t,
            offset: 4742 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002814 as xkb_keysym_t,
            offset: 7606 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002815 as xkb_keysym_t,
            offset: 5334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002816 as xkb_keysym_t,
            offset: 6486 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002817 as xkb_keysym_t,
            offset: 4151 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002818 as xkb_keysym_t,
            offset: 7878 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002819 as xkb_keysym_t,
            offset: 5622 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281a as xkb_keysym_t,
            offset: 6774 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281b as xkb_keysym_t,
            offset: 4455 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281c as xkb_keysym_t,
            offset: 7334 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281d as xkb_keysym_t,
            offset: 5047 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281e as xkb_keysym_t,
            offset: 6199 as uint16_t,
        },
        name_keysym {
            keysym: 0x100281f as xkb_keysym_t,
            offset: 3849 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002820 as xkb_keysym_t,
            offset: 8267 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002821 as xkb_keysym_t,
            offset: 6034 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002822 as xkb_keysym_t,
            offset: 7186 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002823 as xkb_keysym_t,
            offset: 4890 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002824 as xkb_keysym_t,
            offset: 7746 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002825 as xkb_keysym_t,
            offset: 5482 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002826 as xkb_keysym_t,
            offset: 6634 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002827 as xkb_keysym_t,
            offset: 4307 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002828 as xkb_keysym_t,
            offset: 8018 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002829 as xkb_keysym_t,
            offset: 5770 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282a as xkb_keysym_t,
            offset: 6922 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282b as xkb_keysym_t,
            offset: 4611 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282c as xkb_keysym_t,
            offset: 7482 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282d as xkb_keysym_t,
            offset: 5203 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282e as xkb_keysym_t,
            offset: 6355 as uint16_t,
        },
        name_keysym {
            keysym: 0x100282f as xkb_keysym_t,
            offset: 4013 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002830 as xkb_keysym_t,
            offset: 8150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002831 as xkb_keysym_t,
            offset: 5910 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002832 as xkb_keysym_t,
            offset: 7062 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002833 as xkb_keysym_t,
            offset: 4759 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002834 as xkb_keysym_t,
            offset: 7622 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002835 as xkb_keysym_t,
            offset: 5351 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002836 as xkb_keysym_t,
            offset: 6503 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002837 as xkb_keysym_t,
            offset: 4169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002838 as xkb_keysym_t,
            offset: 7894 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002839 as xkb_keysym_t,
            offset: 5639 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283a as xkb_keysym_t,
            offset: 6791 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283b as xkb_keysym_t,
            offset: 4473 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283c as xkb_keysym_t,
            offset: 7351 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283d as xkb_keysym_t,
            offset: 5065 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283e as xkb_keysym_t,
            offset: 6217 as uint16_t,
        },
        name_keysym {
            keysym: 0x100283f as xkb_keysym_t,
            offset: 3868 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002840 as xkb_keysym_t,
            offset: 8331 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002841 as xkb_keysym_t,
            offset: 6102 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002842 as xkb_keysym_t,
            offset: 7254 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002843 as xkb_keysym_t,
            offset: 4962 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002844 as xkb_keysym_t,
            offset: 7814 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002845 as xkb_keysym_t,
            offset: 5554 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002846 as xkb_keysym_t,
            offset: 6706 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002847 as xkb_keysym_t,
            offset: 4383 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002848 as xkb_keysym_t,
            offset: 8086 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002849 as xkb_keysym_t,
            offset: 5842 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284a as xkb_keysym_t,
            offset: 6994 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284b as xkb_keysym_t,
            offset: 4687 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284c as xkb_keysym_t,
            offset: 7554 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284d as xkb_keysym_t,
            offset: 5279 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284e as xkb_keysym_t,
            offset: 6431 as uint16_t,
        },
        name_keysym {
            keysym: 0x100284f as xkb_keysym_t,
            offset: 4093 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002850 as xkb_keysym_t,
            offset: 8218 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002851 as xkb_keysym_t,
            offset: 5982 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002852 as xkb_keysym_t,
            offset: 7134 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002853 as xkb_keysym_t,
            offset: 4835 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002854 as xkb_keysym_t,
            offset: 7694 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002855 as xkb_keysym_t,
            offset: 5427 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002856 as xkb_keysym_t,
            offset: 6579 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002857 as xkb_keysym_t,
            offset: 4249 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002858 as xkb_keysym_t,
            offset: 7966 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002859 as xkb_keysym_t,
            offset: 5715 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285a as xkb_keysym_t,
            offset: 6867 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285b as xkb_keysym_t,
            offset: 4553 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285c as xkb_keysym_t,
            offset: 7427 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285d as xkb_keysym_t,
            offset: 5145 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285e as xkb_keysym_t,
            offset: 6297 as uint16_t,
        },
        name_keysym {
            keysym: 0x100285f as xkb_keysym_t,
            offset: 3952 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002860 as xkb_keysym_t,
            offset: 8282 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002861 as xkb_keysym_t,
            offset: 6050 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002862 as xkb_keysym_t,
            offset: 7202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002863 as xkb_keysym_t,
            offset: 4907 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002864 as xkb_keysym_t,
            offset: 7762 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002865 as xkb_keysym_t,
            offset: 5499 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002866 as xkb_keysym_t,
            offset: 6651 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002867 as xkb_keysym_t,
            offset: 4325 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002868 as xkb_keysym_t,
            offset: 8034 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002869 as xkb_keysym_t,
            offset: 5787 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286a as xkb_keysym_t,
            offset: 6939 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286b as xkb_keysym_t,
            offset: 4629 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286c as xkb_keysym_t,
            offset: 7499 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286d as xkb_keysym_t,
            offset: 5221 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286e as xkb_keysym_t,
            offset: 6373 as uint16_t,
        },
        name_keysym {
            keysym: 0x100286f as xkb_keysym_t,
            offset: 4032 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002870 as xkb_keysym_t,
            offset: 8166 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002871 as xkb_keysym_t,
            offset: 5927 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002872 as xkb_keysym_t,
            offset: 7079 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002873 as xkb_keysym_t,
            offset: 4777 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002874 as xkb_keysym_t,
            offset: 7639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002875 as xkb_keysym_t,
            offset: 5369 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002876 as xkb_keysym_t,
            offset: 6521 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002877 as xkb_keysym_t,
            offset: 4188 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002878 as xkb_keysym_t,
            offset: 7911 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002879 as xkb_keysym_t,
            offset: 5657 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287a as xkb_keysym_t,
            offset: 6809 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287b as xkb_keysym_t,
            offset: 4492 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287c as xkb_keysym_t,
            offset: 7369 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287d as xkb_keysym_t,
            offset: 5084 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287e as xkb_keysym_t,
            offset: 6236 as uint16_t,
        },
        name_keysym {
            keysym: 0x100287f as xkb_keysym_t,
            offset: 3888 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002880 as xkb_keysym_t,
            offset: 8362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002881 as xkb_keysym_t,
            offset: 6135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002882 as xkb_keysym_t,
            offset: 7287 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002883 as xkb_keysym_t,
            offset: 4997 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002884 as xkb_keysym_t,
            offset: 7847 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002885 as xkb_keysym_t,
            offset: 5589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002886 as xkb_keysym_t,
            offset: 6741 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002887 as xkb_keysym_t,
            offset: 4420 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002888 as xkb_keysym_t,
            offset: 8119 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002889 as xkb_keysym_t,
            offset: 5877 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288a as xkb_keysym_t,
            offset: 7029 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288b as xkb_keysym_t,
            offset: 4724 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288c as xkb_keysym_t,
            offset: 7589 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288d as xkb_keysym_t,
            offset: 5316 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288e as xkb_keysym_t,
            offset: 6468 as uint16_t,
        },
        name_keysym {
            keysym: 0x100288f as xkb_keysym_t,
            offset: 4132 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002890 as xkb_keysym_t,
            offset: 8251 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002891 as xkb_keysym_t,
            offset: 6017 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002892 as xkb_keysym_t,
            offset: 7169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002893 as xkb_keysym_t,
            offset: 4872 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002894 as xkb_keysym_t,
            offset: 7729 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002895 as xkb_keysym_t,
            offset: 5464 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002896 as xkb_keysym_t,
            offset: 6616 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002897 as xkb_keysym_t,
            offset: 4288 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002898 as xkb_keysym_t,
            offset: 8001 as uint16_t,
        },
        name_keysym {
            keysym: 0x1002899 as xkb_keysym_t,
            offset: 5752 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289a as xkb_keysym_t,
            offset: 6904 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289b as xkb_keysym_t,
            offset: 4592 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289c as xkb_keysym_t,
            offset: 7464 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289d as xkb_keysym_t,
            offset: 5184 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289e as xkb_keysym_t,
            offset: 6336 as uint16_t,
        },
        name_keysym {
            keysym: 0x100289f as xkb_keysym_t,
            offset: 3993 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a0 as xkb_keysym_t,
            offset: 8315 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a1 as xkb_keysym_t,
            offset: 6085 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a2 as xkb_keysym_t,
            offset: 7237 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a3 as xkb_keysym_t,
            offset: 4944 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a4 as xkb_keysym_t,
            offset: 7797 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a5 as xkb_keysym_t,
            offset: 5536 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a6 as xkb_keysym_t,
            offset: 6688 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a7 as xkb_keysym_t,
            offset: 4364 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a8 as xkb_keysym_t,
            offset: 8069 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028a9 as xkb_keysym_t,
            offset: 5824 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028aa as xkb_keysym_t,
            offset: 6976 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ab as xkb_keysym_t,
            offset: 4668 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ac as xkb_keysym_t,
            offset: 7536 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ad as xkb_keysym_t,
            offset: 5260 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ae as xkb_keysym_t,
            offset: 6412 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028af as xkb_keysym_t,
            offset: 4073 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b0 as xkb_keysym_t,
            offset: 8201 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b1 as xkb_keysym_t,
            offset: 5964 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b2 as xkb_keysym_t,
            offset: 7116 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b3 as xkb_keysym_t,
            offset: 4816 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b4 as xkb_keysym_t,
            offset: 7676 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b5 as xkb_keysym_t,
            offset: 5408 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b6 as xkb_keysym_t,
            offset: 6560 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b7 as xkb_keysym_t,
            offset: 4229 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b8 as xkb_keysym_t,
            offset: 7948 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028b9 as xkb_keysym_t,
            offset: 5696 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ba as xkb_keysym_t,
            offset: 6848 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bb as xkb_keysym_t,
            offset: 4533 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bc as xkb_keysym_t,
            offset: 7408 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bd as xkb_keysym_t,
            offset: 5125 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028be as xkb_keysym_t,
            offset: 6277 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028bf as xkb_keysym_t,
            offset: 3931 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c0 as xkb_keysym_t,
            offset: 8346 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c1 as xkb_keysym_t,
            offset: 6118 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c2 as xkb_keysym_t,
            offset: 7270 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c3 as xkb_keysym_t,
            offset: 4979 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c4 as xkb_keysym_t,
            offset: 7830 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c5 as xkb_keysym_t,
            offset: 5571 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c6 as xkb_keysym_t,
            offset: 6723 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c7 as xkb_keysym_t,
            offset: 4401 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c8 as xkb_keysym_t,
            offset: 8102 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028c9 as xkb_keysym_t,
            offset: 5859 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ca as xkb_keysym_t,
            offset: 7011 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cb as xkb_keysym_t,
            offset: 4705 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cc as xkb_keysym_t,
            offset: 7571 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cd as xkb_keysym_t,
            offset: 5297 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ce as xkb_keysym_t,
            offset: 6449 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028cf as xkb_keysym_t,
            offset: 4112 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d0 as xkb_keysym_t,
            offset: 8234 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d1 as xkb_keysym_t,
            offset: 5999 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d2 as xkb_keysym_t,
            offset: 7151 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d3 as xkb_keysym_t,
            offset: 4853 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d4 as xkb_keysym_t,
            offset: 7711 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d5 as xkb_keysym_t,
            offset: 5445 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d6 as xkb_keysym_t,
            offset: 6597 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d7 as xkb_keysym_t,
            offset: 4268 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d8 as xkb_keysym_t,
            offset: 7983 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028d9 as xkb_keysym_t,
            offset: 5733 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028da as xkb_keysym_t,
            offset: 6885 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028db as xkb_keysym_t,
            offset: 4572 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028dc as xkb_keysym_t,
            offset: 7445 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028dd as xkb_keysym_t,
            offset: 5164 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028de as xkb_keysym_t,
            offset: 6316 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028df as xkb_keysym_t,
            offset: 3972 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e0 as xkb_keysym_t,
            offset: 8298 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e1 as xkb_keysym_t,
            offset: 6067 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e2 as xkb_keysym_t,
            offset: 7219 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e3 as xkb_keysym_t,
            offset: 4925 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e4 as xkb_keysym_t,
            offset: 7779 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e5 as xkb_keysym_t,
            offset: 5517 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e6 as xkb_keysym_t,
            offset: 6669 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e7 as xkb_keysym_t,
            offset: 4344 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e8 as xkb_keysym_t,
            offset: 8051 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028e9 as xkb_keysym_t,
            offset: 5805 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ea as xkb_keysym_t,
            offset: 6957 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028eb as xkb_keysym_t,
            offset: 4648 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ec as xkb_keysym_t,
            offset: 7517 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ed as xkb_keysym_t,
            offset: 5240 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ee as xkb_keysym_t,
            offset: 6392 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ef as xkb_keysym_t,
            offset: 4052 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f0 as xkb_keysym_t,
            offset: 8183 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f1 as xkb_keysym_t,
            offset: 5945 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f2 as xkb_keysym_t,
            offset: 7097 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f3 as xkb_keysym_t,
            offset: 4796 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f4 as xkb_keysym_t,
            offset: 7657 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f5 as xkb_keysym_t,
            offset: 5388 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f6 as xkb_keysym_t,
            offset: 6540 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f7 as xkb_keysym_t,
            offset: 4208 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f8 as xkb_keysym_t,
            offset: 7929 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028f9 as xkb_keysym_t,
            offset: 5676 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fa as xkb_keysym_t,
            offset: 6828 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fb as xkb_keysym_t,
            offset: 4512 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fc as xkb_keysym_t,
            offset: 7388 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fd as xkb_keysym_t,
            offset: 5104 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028fe as xkb_keysym_t,
            offset: 6256 as uint16_t,
        },
        name_keysym {
            keysym: 0x10028ff as xkb_keysym_t,
            offset: 3909 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a8 as xkb_keysym_t,
            offset: 17070 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000a9 as xkb_keysym_t,
            offset: 17137 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000aa as xkb_keysym_t,
            offset: 17083 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ab as xkb_keysym_t,
            offset: 17120 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ac as xkb_keysym_t,
            offset: 17102 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000af as xkb_keysym_t,
            offset: 17027 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000be as xkb_keysym_t,
            offset: 16973 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000ee as xkb_keysym_t,
            offset: 17174 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000f6 as xkb_keysym_t,
            offset: 17034 as uint16_t,
        },
        name_keysym {
            keysym: 0x100000fc as xkb_keysym_t,
            offset: 16927 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe22 as xkb_keysym_t,
            offset: 10496 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe27 as xkb_keysym_t,
            offset: 10426 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe2c as xkb_keysym_t,
            offset: 10461 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe5e as xkb_keysym_t,
            offset: 10477 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe60 as xkb_keysym_t,
            offset: 11320 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000fe7e as xkb_keysym_t,
            offset: 11548 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000feb0 as xkb_keysym_t,
            offset: 11519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff00 as xkb_keysym_t,
            offset: 11511 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff48 as xkb_keysym_t,
            offset: 17046 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff49 as xkb_keysym_t,
            offset: 17058 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6c as xkb_keysym_t,
            offset: 17150 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6d as xkb_keysym_t,
            offset: 17158 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6e as xkb_keysym_t,
            offset: 17167 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff6f as xkb_keysym_t,
            offset: 16935 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff70 as xkb_keysym_t,
            offset: 16996 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff71 as xkb_keysym_t,
            offset: 16960 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff72 as xkb_keysym_t,
            offset: 16983 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff73 as xkb_keysym_t,
            offset: 16947 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff74 as xkb_keysym_t,
            offset: 16917 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff75 as xkb_keysym_t,
            offset: 17014 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff76 as xkb_keysym_t,
            offset: 12256 as uint16_t,
        },
        name_keysym {
            keysym: 0x1000ff77 as xkb_keysym_t,
            offset: 12267 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff02 as xkb_keysym_t,
            offset: 21154 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff03 as xkb_keysym_t,
            offset: 21162 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff04 as xkb_keysym_t,
            offset: 21361 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff07 as xkb_keysym_t,
            offset: 21098 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff08 as xkb_keysym_t,
            offset: 21085 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff0b as xkb_keysym_t,
            offset: 21145 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff1b as xkb_keysym_t,
            offset: 21224 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff31 as xkb_keysym_t,
            offset: 21074 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff32 as xkb_keysym_t,
            offset: 21395 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff33 as xkb_keysym_t,
            offset: 21411 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff40 as xkb_keysym_t,
            offset: 21326 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff41 as xkb_keysym_t,
            offset: 21351 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff42 as xkb_keysym_t,
            offset: 21314 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff43 as xkb_keysym_t,
            offset: 21338 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff44 as xkb_keysym_t,
            offset: 21062 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff45 as xkb_keysym_t,
            offset: 21278 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff51 as xkb_keysym_t,
            offset: 21262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff52 as xkb_keysym_t,
            offset: 21488 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff53 as xkb_keysym_t,
            offset: 21448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff54 as xkb_keysym_t,
            offset: 21194 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff57 as xkb_keysym_t,
            offset: 21213 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff58 as xkb_keysym_t,
            offset: 21122 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff59 as xkb_keysym_t,
            offset: 21202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5a as xkb_keysym_t,
            offset: 21109 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5b as xkb_keysym_t,
            offset: 21383 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5c as xkb_keysym_t,
            offset: 21302 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5d as xkb_keysym_t,
            offset: 21370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff5e as xkb_keysym_t,
            offset: 21289 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff60 as xkb_keysym_t,
            offset: 21457 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff63 as xkb_keysym_t,
            offset: 21252 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff65 as xkb_keysym_t,
            offset: 21480 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff67 as xkb_keysym_t,
            offset: 21270 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff69 as xkb_keysym_t,
            offset: 21135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff6a as xkb_keysym_t,
            offset: 21244 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff71 as xkb_keysym_t,
            offset: 21467 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff72 as xkb_keysym_t,
            offset: 21179 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff73 as xkb_keysym_t,
            offset: 21425 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff74 as xkb_keysym_t,
            offset: 21234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ff78 as xkb_keysym_t,
            offset: 21437 as uint16_t,
        },
        name_keysym {
            keysym: 0x1004ffff as xkb_keysym_t,
            offset: 21169 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff00 as xkb_keysym_t,
            offset: 24263 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff01 as xkb_keysym_t,
            offset: 24234 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff02 as xkb_keysym_t,
            offset: 24275 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff03 as xkb_keysym_t,
            offset: 24208 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff04 as xkb_keysym_t,
            offset: 24247 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff05 as xkb_keysym_t,
            offset: 24220 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff10 as xkb_keysym_t,
            offset: 24194 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff11 as xkb_keysym_t,
            offset: 24201 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff60 as xkb_keysym_t,
            offset: 24411 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff70 as xkb_keysym_t,
            offset: 24394 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff71 as xkb_keysym_t,
            offset: 24295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff72 as xkb_keysym_t,
            offset: 24179 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff73 as xkb_keysym_t,
            offset: 24304 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff74 as xkb_keysym_t,
            offset: 24334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff75 as xkb_keysym_t,
            offset: 24187 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff76 as xkb_keysym_t,
            offset: 24343 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff77 as xkb_keysym_t,
            offset: 24115 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff78 as xkb_keysym_t,
            offset: 24135 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff79 as xkb_keysym_t,
            offset: 24148 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7a as xkb_keysym_t,
            offset: 24430 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7b as xkb_keysym_t,
            offset: 24446 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7c as xkb_keysym_t,
            offset: 24470 as uint16_t,
        },
        name_keysym {
            keysym: 0x1005ff7d as xkb_keysym_t,
            offset: 24358 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810a4 as xkb_keysym_t,
            offset: 30039 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810ae as xkb_keysym_t,
            offset: 28348 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810d1 as xkb_keysym_t,
            offset: 27145 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810dc as xkb_keysym_t,
            offset: 31793 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810f4 as xkb_keysym_t,
            offset: 27614 as uint16_t,
        },
        name_keysym {
            keysym: 0x100810f5 as xkb_keysym_t,
            offset: 28141 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081160 as xkb_keysym_t,
            offset: 31033 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081162 as xkb_keysym_t,
            offset: 28724 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081166 as xkb_keysym_t,
            offset: 28841 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081168 as xkb_keysym_t,
            offset: 32329 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008116a as xkb_keysym_t,
            offset: 30213 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008116e as xkb_keysym_t,
            offset: 30175 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081170 as xkb_keysym_t,
            offset: 30001 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081171 as xkb_keysym_t,
            offset: 30412 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081175 as xkb_keysym_t,
            offset: 27164 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081177 as xkb_keysym_t,
            offset: 27052 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081178 as xkb_keysym_t,
            offset: 30195 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081179 as xkb_keysym_t,
            offset: 30352 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117a as xkb_keysym_t,
            offset: 30117 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117b as xkb_keysym_t,
            offset: 30370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117c as xkb_keysym_t,
            offset: 30389 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008117d as xkb_keysym_t,
            offset: 30262 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081180 as xkb_keysym_t,
            offset: 30287 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081181 as xkb_keysym_t,
            offset: 30241 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081182 as xkb_keysym_t,
            offset: 30331 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081183 as xkb_keysym_t,
            offset: 30023 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081184 as xkb_keysym_t,
            offset: 30307 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081185 as xkb_keysym_t,
            offset: 28232 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081186 as xkb_keysym_t,
            offset: 30092 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081188 as xkb_keysym_t,
            offset: 27135 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081192 as xkb_keysym_t,
            offset: 27932 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081193 as xkb_keysym_t,
            offset: 27916 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081199 as xkb_keysym_t,
            offset: 30058 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008119b as xkb_keysym_t,
            offset: 27583 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008119d as xkb_keysym_t,
            offset: 30755 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a0 as xkb_keysym_t,
            offset: 32354 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a4 as xkb_keysym_t,
            offset: 32547 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a6 as xkb_keysym_t,
            offset: 28240 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a8 as xkb_keysym_t,
            offset: 28733 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811a9 as xkb_keysym_t,
            offset: 31205 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811aa as xkb_keysym_t,
            offset: 28104 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ac as xkb_keysym_t,
            offset: 32403 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ad as xkb_keysym_t,
            offset: 26967 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811af as xkb_keysym_t,
            offset: 28156 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b0 as xkb_keysym_t,
            offset: 31762 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b6 as xkb_keysym_t,
            offset: 28011 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b7 as xkb_keysym_t,
            offset: 30076 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b8 as xkb_keysym_t,
            offset: 26905 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811b9 as xkb_keysym_t,
            offset: 26886 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811ba as xkb_keysym_t,
            offset: 28830 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bc as xkb_keysym_t,
            offset: 30732 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bd as xkb_keysym_t,
            offset: 31149 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811be as xkb_keysym_t,
            offset: 28762 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811bf as xkb_keysym_t,
            offset: 29425 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d0 as xkb_keysym_t,
            offset: 28429 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d1 as xkb_keysym_t,
            offset: 28481 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d2 as xkb_keysym_t,
            offset: 28501 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d3 as xkb_keysym_t,
            offset: 28544 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d4 as xkb_keysym_t,
            offset: 28554 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d5 as xkb_keysym_t,
            offset: 28564 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d6 as xkb_keysym_t,
            offset: 28574 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d7 as xkb_keysym_t,
            offset: 28584 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d8 as xkb_keysym_t,
            offset: 28594 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811d9 as xkb_keysym_t,
            offset: 28604 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811da as xkb_keysym_t,
            offset: 28614 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811db as xkb_keysym_t,
            offset: 28511 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811dc as xkb_keysym_t,
            offset: 28522 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811dd as xkb_keysym_t,
            offset: 28533 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811de as xkb_keysym_t,
            offset: 28436 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811df as xkb_keysym_t,
            offset: 28445 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e0 as xkb_keysym_t,
            offset: 28463 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e1 as xkb_keysym_t,
            offset: 28472 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e2 as xkb_keysym_t,
            offset: 28492 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e3 as xkb_keysym_t,
            offset: 28624 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e4 as xkb_keysym_t,
            offset: 28454 as uint16_t,
        },
        name_keysym {
            keysym: 0x100811e5 as xkb_keysym_t,
            offset: 28633 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081200 as xkb_keysym_t,
            offset: 30775 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081201 as xkb_keysym_t,
            offset: 30788 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081202 as xkb_keysym_t,
            offset: 30829 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081203 as xkb_keysym_t,
            offset: 30842 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081204 as xkb_keysym_t,
            offset: 30855 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081205 as xkb_keysym_t,
            offset: 30868 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081206 as xkb_keysym_t,
            offset: 30881 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081207 as xkb_keysym_t,
            offset: 30894 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081208 as xkb_keysym_t,
            offset: 30907 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081209 as xkb_keysym_t,
            offset: 30920 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120a as xkb_keysym_t,
            offset: 31002 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120b as xkb_keysym_t,
            offset: 30985 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120c as xkb_keysym_t,
            offset: 30933 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120d as xkb_keysym_t,
            offset: 30946 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120e as xkb_keysym_t,
            offset: 30959 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008120f as xkb_keysym_t,
            offset: 30972 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081210 as xkb_keysym_t,
            offset: 27814 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081211 as xkb_keysym_t,
            offset: 32473 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081215 as xkb_keysym_t,
            offset: 27874 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081216 as xkb_keysym_t,
            offset: 27891 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081217 as xkb_keysym_t,
            offset: 27861 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081218 as xkb_keysym_t,
            offset: 27799 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081219 as xkb_keysym_t,
            offset: 27830 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121a as xkb_keysym_t,
            offset: 27845 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121b as xkb_keysym_t,
            offset: 27099 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121c as xkb_keysym_t,
            offset: 27082 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121d as xkb_keysym_t,
            offset: 27115 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008121e as xkb_keysym_t,
            offset: 29408 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081230 as xkb_keysym_t,
            offset: 26983 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081232 as xkb_keysym_t,
            offset: 31324 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081240 as xkb_keysym_t,
            offset: 27669 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081241 as xkb_keysym_t,
            offset: 32081 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081242 as xkb_keysym_t,
            offset: 28861 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081243 as xkb_keysym_t,
            offset: 28046 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081244 as xkb_keysym_t,
            offset: 27038 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081245 as xkb_keysym_t,
            offset: 31557 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081246 as xkb_keysym_t,
            offset: 32386 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081247 as xkb_keysym_t,
            offset: 27068 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081249 as xkb_keysym_t,
            offset: 28322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124a as xkb_keysym_t,
            offset: 28117 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124b as xkb_keysym_t,
            offset: 27753 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124c as xkb_keysym_t,
            offset: 27729 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124d as xkb_keysym_t,
            offset: 27776 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124e as xkb_keysym_t,
            offset: 26933 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008124f as xkb_keysym_t,
            offset: 28188 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081250 as xkb_keysym_t,
            offset: 27651 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081251 as xkb_keysym_t,
            offset: 27633 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081252 as xkb_keysym_t,
            offset: 28292 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081253 as xkb_keysym_t,
            offset: 28261 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081260 as xkb_keysym_t,
            offset: 29016 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081261 as xkb_keysym_t,
            offset: 28965 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081262 as xkb_keysym_t,
            offset: 29039 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081263 as xkb_keysym_t,
            offset: 28988 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081264 as xkb_keysym_t,
            offset: 28915 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081265 as xkb_keysym_t,
            offset: 28940 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081266 as xkb_keysym_t,
            offset: 31392 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081267 as xkb_keysym_t,
            offset: 31378 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081268 as xkb_keysym_t,
            offset: 29383 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081269 as xkb_keysym_t,
            offset: 29370 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126a as xkb_keysym_t,
            offset: 31448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126b as xkb_keysym_t,
            offset: 30431 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126c as xkb_keysym_t,
            offset: 30801 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126d as xkb_keysym_t,
            offset: 30815 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126e as xkb_keysym_t,
            offset: 27205 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008126f as xkb_keysym_t,
            offset: 26922 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081270 as xkb_keysym_t,
            offset: 30715 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081271 as xkb_keysym_t,
            offset: 31834 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081272 as xkb_keysym_t,
            offset: 31103 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081273 as xkb_keysym_t,
            offset: 32378 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081274 as xkb_keysym_t,
            offset: 32260 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081275 as xkb_keysym_t,
            offset: 28370 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081276 as xkb_keysym_t,
            offset: 31728 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081277 as xkb_keysym_t,
            offset: 28095 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081278 as xkb_keysym_t,
            offset: 31040 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081279 as xkb_keysym_t,
            offset: 31257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127a as xkb_keysym_t,
            offset: 31639 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127b as xkb_keysym_t,
            offset: 30699 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127c as xkb_keysym_t,
            offset: 31237 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127d as xkb_keysym_t,
            offset: 27479 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127e as xkb_keysym_t,
            offset: 29984 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008127f as xkb_keysym_t,
            offset: 31744 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081280 as xkb_keysym_t,
            offset: 30642 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081281 as xkb_keysym_t,
            offset: 28412 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081282 as xkb_keysym_t,
            offset: 31697 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081283 as xkb_keysym_t,
            offset: 28213 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081284 as xkb_keysym_t,
            offset: 31287 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081285 as xkb_keysym_t,
            offset: 32217 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081286 as xkb_keysym_t,
            offset: 27970 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081287 as xkb_keysym_t,
            offset: 31681 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081288 as xkb_keysym_t,
            offset: 30655 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081290 as xkb_keysym_t,
            offset: 29484 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081291 as xkb_keysym_t,
            offset: 29615 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081292 as xkb_keysym_t,
            offset: 29746 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081293 as xkb_keysym_t,
            offset: 29769 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081294 as xkb_keysym_t,
            offset: 29780 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081295 as xkb_keysym_t,
            offset: 29791 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081296 as xkb_keysym_t,
            offset: 29802 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081297 as xkb_keysym_t,
            offset: 29813 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081298 as xkb_keysym_t,
            offset: 29824 as uint16_t,
        },
        name_keysym {
            keysym: 0x10081299 as xkb_keysym_t,
            offset: 29495 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129a as xkb_keysym_t,
            offset: 29507 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129b as xkb_keysym_t,
            offset: 29519 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129c as xkb_keysym_t,
            offset: 29531 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129d as xkb_keysym_t,
            offset: 29543 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129e as xkb_keysym_t,
            offset: 29555 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008129f as xkb_keysym_t,
            offset: 29567 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a0 as xkb_keysym_t,
            offset: 29579 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a1 as xkb_keysym_t,
            offset: 29591 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a2 as xkb_keysym_t,
            offset: 29603 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a3 as xkb_keysym_t,
            offset: 29626 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a4 as xkb_keysym_t,
            offset: 29638 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a5 as xkb_keysym_t,
            offset: 29650 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a6 as xkb_keysym_t,
            offset: 29662 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a7 as xkb_keysym_t,
            offset: 29674 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a8 as xkb_keysym_t,
            offset: 29686 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812a9 as xkb_keysym_t,
            offset: 29698 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812aa as xkb_keysym_t,
            offset: 29710 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ab as xkb_keysym_t,
            offset: 29722 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ac as xkb_keysym_t,
            offset: 29734 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ad as xkb_keysym_t,
            offset: 29757 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b0 as xkb_keysym_t,
            offset: 29907 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b1 as xkb_keysym_t,
            offset: 29928 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b2 as xkb_keysym_t,
            offset: 29886 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b3 as xkb_keysym_t,
            offset: 29835 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b4 as xkb_keysym_t,
            offset: 29852 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b5 as xkb_keysym_t,
            offset: 29869 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b8 as xkb_keysym_t,
            offset: 29067 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812b9 as xkb_keysym_t,
            offset: 29083 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812ba as xkb_keysym_t,
            offset: 29099 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bb as xkb_keysym_t,
            offset: 29115 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bc as xkb_keysym_t,
            offset: 29131 as uint16_t,
        },
        name_keysym {
            keysym: 0x100812bd as xkb_keysym_t,
            offset: 31119 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe01 as xkb_keysym_t,
            offset: 31886 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe02 as xkb_keysym_t,
            offset: 31953 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe03 as xkb_keysym_t,
            offset: 31969 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe04 as xkb_keysym_t,
            offset: 31985 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe05 as xkb_keysym_t,
            offset: 32001 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe06 as xkb_keysym_t,
            offset: 32017 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe07 as xkb_keysym_t,
            offset: 32033 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe08 as xkb_keysym_t,
            offset: 32049 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe09 as xkb_keysym_t,
            offset: 32065 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0a as xkb_keysym_t,
            offset: 31902 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0b as xkb_keysym_t,
            offset: 31919 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe0c as xkb_keysym_t,
            offset: 31936 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe20 as xkb_keysym_t,
            offset: 32249 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe21 as xkb_keysym_t,
            offset: 27956 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe22 as xkb_keysym_t,
            offset: 30684 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe23 as xkb_keysym_t,
            offset: 31222 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe24 as xkb_keysym_t,
            offset: 29466 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008fe25 as xkb_keysym_t,
            offset: 29439 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff01 as xkb_keysym_t,
            offset: 30505 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff02 as xkb_keysym_t,
            offset: 30585 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff03 as xkb_keysym_t,
            offset: 30563 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff04 as xkb_keysym_t,
            offset: 29147 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff05 as xkb_keysym_t,
            offset: 28895 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff06 as xkb_keysym_t,
            offset: 28873 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff07 as xkb_keysym_t,
            offset: 30540 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff10 as xkb_keysym_t,
            offset: 31803 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff11 as xkb_keysym_t,
            offset: 27236 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff12 as xkb_keysym_t,
            offset: 27289 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff13 as xkb_keysym_t,
            offset: 27376 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff14 as xkb_keysym_t,
            offset: 27332 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff15 as xkb_keysym_t,
            offset: 27465 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff16 as xkb_keysym_t,
            offset: 27362 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff17 as xkb_keysym_t,
            offset: 27303 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff18 as xkb_keysym_t,
            offset: 28804 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff19 as xkb_keysym_t,
            offset: 29948 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1a as xkb_keysym_t,
            offset: 31815 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1b as xkb_keysym_t,
            offset: 31617 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1c as xkb_keysym_t,
            offset: 27417 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1d as xkb_keysym_t,
            offset: 27701 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1e as xkb_keysym_t,
            offset: 30460 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff1f as xkb_keysym_t,
            offset: 32132 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff20 as xkb_keysym_t,
            offset: 27716 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff21 as xkb_keysym_t,
            offset: 31178 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff22 as xkb_keysym_t,
            offset: 28027 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff23 as xkb_keysym_t,
            offset: 31435 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff24 as xkb_keysym_t,
            offset: 31404 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff25 as xkb_keysym_t,
            offset: 31419 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff26 as xkb_keysym_t,
            offset: 27514 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff27 as xkb_keysym_t,
            offset: 28650 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff28 as xkb_keysym_t,
            offset: 31825 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff29 as xkb_keysym_t,
            offset: 31312 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2a as xkb_keysym_t,
            offset: 31192 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2b as xkb_keysym_t,
            offset: 32417 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2c as xkb_keysym_t,
            offset: 28251 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2d as xkb_keysym_t,
            offset: 31541 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2e as xkb_keysym_t,
            offset: 32496 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff2f as xkb_keysym_t,
            offset: 31718 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff30 as xkb_keysym_t,
            offset: 28386 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff31 as xkb_keysym_t,
            offset: 27317 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff32 as xkb_keysym_t,
            offset: 27257 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff33 as xkb_keysym_t,
            offset: 30615 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff34 as xkb_keysym_t,
            offset: 32314 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff35 as xkb_keysym_t,
            offset: 29394 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff36 as xkb_keysym_t,
            offset: 31672 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff37 as xkb_keysym_t,
            offset: 28792 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff38 as xkb_keysym_t,
            offset: 31070 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff39 as xkb_keysym_t,
            offset: 26951 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3a as xkb_keysym_t,
            offset: 28817 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3b as xkb_keysym_t,
            offset: 27593 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3c as xkb_keysym_t,
            offset: 28400 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3d as xkb_keysym_t,
            offset: 27997 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3e as xkb_keysym_t,
            offset: 27449 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff3f as xkb_keysym_t,
            offset: 27523 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff40 as xkb_keysym_t,
            offset: 29178 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff41 as xkb_keysym_t,
            offset: 29190 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff42 as xkb_keysym_t,
            offset: 29202 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff43 as xkb_keysym_t,
            offset: 29214 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff44 as xkb_keysym_t,
            offset: 29226 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff45 as xkb_keysym_t,
            offset: 29238 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff46 as xkb_keysym_t,
            offset: 29250 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff47 as xkb_keysym_t,
            offset: 29262 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff48 as xkb_keysym_t,
            offset: 29274 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff49 as xkb_keysym_t,
            offset: 29286 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4a as xkb_keysym_t,
            offset: 29298 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4b as xkb_keysym_t,
            offset: 29310 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4c as xkb_keysym_t,
            offset: 29322 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4d as xkb_keysym_t,
            offset: 29334 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4e as xkb_keysym_t,
            offset: 29346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff4f as xkb_keysym_t,
            offset: 29358 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff50 as xkb_keysym_t,
            offset: 26997 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff51 as xkb_keysym_t,
            offset: 27017 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff52 as xkb_keysym_t,
            offset: 27574 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff53 as xkb_keysym_t,
            offset: 27909 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff54 as xkb_keysym_t,
            offset: 27686 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff55 as xkb_keysym_t,
            offset: 27946 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff56 as xkb_keysym_t,
            offset: 27987 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff57 as xkb_keysym_t,
            offset: 28063 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff58 as xkb_keysym_t,
            offset: 28072 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff59 as xkb_keysym_t,
            offset: 28129 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5a as xkb_keysym_t,
            offset: 28205 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5b as xkb_keysym_t,
            offset: 28174 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5c as xkb_keysym_t,
            offset: 28338 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5d as xkb_keysym_t,
            offset: 28357 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5e as xkb_keysym_t,
            offset: 28708 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff5f as xkb_keysym_t,
            offset: 28717 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff60 as xkb_keysym_t,
            offset: 28850 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff61 as xkb_keysym_t,
            offset: 29455 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff62 as xkb_keysym_t,
            offset: 29973 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff63 as xkb_keysym_t,
            offset: 30448 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff65 as xkb_keysym_t,
            offset: 30469 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff66 as xkb_keysym_t,
            offset: 30480 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff67 as xkb_keysym_t,
            offset: 30630 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff68 as xkb_keysym_t,
            offset: 30667 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff69 as xkb_keysym_t,
            offset: 30675 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6a as xkb_keysym_t,
            offset: 31018 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6b as xkb_keysym_t,
            offset: 31061 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6c as xkb_keysym_t,
            offset: 31082 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6d as xkb_keysym_t,
            offset: 31093 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff6e as xkb_keysym_t,
            offset: 31139 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff70 as xkb_keysym_t,
            offset: 31281 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff72 as xkb_keysym_t,
            offset: 31357 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff73 as xkb_keysym_t,
            offset: 31346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff74 as xkb_keysym_t,
            offset: 31461 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff75 as xkb_keysym_t,
            offset: 31517 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff76 as xkb_keysym_t,
            offset: 31479 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff77 as xkb_keysym_t,
            offset: 31532 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff78 as xkb_keysym_t,
            offset: 31604 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff79 as xkb_keysym_t,
            offset: 31589 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7a as xkb_keysym_t,
            offset: 31573 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7b as xkb_keysym_t,
            offset: 31663 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7c as xkb_keysym_t,
            offset: 31752 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7d as xkb_keysym_t,
            offset: 31777 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7e as xkb_keysym_t,
            offset: 31862 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff7f as xkb_keysym_t,
            offset: 32097 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff80 as xkb_keysym_t,
            offset: 32110 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff81 as xkb_keysym_t,
            offset: 32145 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff82 as xkb_keysym_t,
            offset: 32238 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff84 as xkb_keysym_t,
            offset: 32295 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff85 as xkb_keysym_t,
            offset: 32271 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff86 as xkb_keysym_t,
            offset: 32283 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff87 as xkb_keysym_t,
            offset: 32344 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff88 as xkb_keysym_t,
            offset: 32439 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff89 as xkb_keysym_t,
            offset: 32464 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8a as xkb_keysym_t,
            offset: 32504 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8b as xkb_keysym_t,
            offset: 32524 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8c as xkb_keysym_t,
            offset: 32535 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8d as xkb_keysym_t,
            offset: 27505 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8e as xkb_keysym_t,
            offset: 30491 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff8f as xkb_keysym_t,
            offset: 32428 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff90 as xkb_keysym_t,
            offset: 29957 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff91 as xkb_keysym_t,
            offset: 31165 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff92 as xkb_keysym_t,
            offset: 30605 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff93 as xkb_keysym_t,
            offset: 27539 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff94 as xkb_keysym_t,
            offset: 27560 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff95 as xkb_keysym_t,
            offset: 32455 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff96 as xkb_keysym_t,
            offset: 32306 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff97 as xkb_keysym_t,
            offset: 27219 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff98 as xkb_keysym_t,
            offset: 27433 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff99 as xkb_keysym_t,
            offset: 27397 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9a as xkb_keysym_t,
            offset: 31849 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9b as xkb_keysym_t,
            offset: 27185 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9c as xkb_keysym_t,
            offset: 28080 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9d as xkb_keysym_t,
            offset: 28662 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9e as xkb_keysym_t,
            offset: 28676 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ff9f as xkb_keysym_t,
            offset: 32123 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa0 as xkb_keysym_t,
            offset: 31628 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa1 as xkb_keysym_t,
            offset: 32369 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa2 as xkb_keysym_t,
            offset: 32155 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa3 as xkb_keysym_t,
            offset: 31304 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa4 as xkb_keysym_t,
            offset: 28752 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa5 as xkb_keysym_t,
            offset: 32513 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa6 as xkb_keysym_t,
            offset: 27551 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa7 as xkb_keysym_t,
            offset: 31874 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa8 as xkb_keysym_t,
            offset: 28778 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffa9 as xkb_keysym_t,
            offset: 32198 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb0 as xkb_keysym_t,
            offset: 32183 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb1 as xkb_keysym_t,
            offset: 32167 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb2 as xkb_keysym_t,
            offset: 27272 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb3 as xkb_keysym_t,
            offset: 29165 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb4 as xkb_keysym_t,
            offset: 32487 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb5 as xkb_keysym_t,
            offset: 31367 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb6 as xkb_keysym_t,
            offset: 27346 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb7 as xkb_keysym_t,
            offset: 31494 as uint16_t,
        },
        name_keysym {
            keysym: 0x1008ffb8 as xkb_keysym_t,
            offset: 28693 as uint16_t,
        },
    ];
    pub const UNICODE_KEYSYM: i32 = 0xfffe as i32;
    pub const DEPRECATED_KEYSYM: i32 = 0xffff as i32;
    pub static mut deprecated_keysyms: [deprecated_keysym; 157] = [
        deprecated_keysym {
            keysym: 0x27 as xkb_keysym_t,
            offset: 934 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x60 as xkb_keysym_t,
            offset: 13397 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xab as xkb_keysym_t,
            offset: 14480 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xba as xkb_keysym_t,
            offset: 21049 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbb as xkb_keysym_t,
            offset: 14494 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xd0 as xkb_keysym_t,
            offset: 12195 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xd8 as xkb_keysym_t,
            offset: 21494 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xde as xkb_keysym_t,
            offset: 25730 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xf8 as xkb_keysym_t,
            offset: 21501 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x3a2 as xkb_keysym_t,
            offset: 19268 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x4a5 as xkb_keysym_t,
            offset: 18364 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x4af as xkb_keysym_t,
            offset: 18802 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x4c1 as xkb_keysym_t,
            offset: 18324 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x4c2 as xkb_keysym_t,
            offset: 18793 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x4cc as xkb_keysym_t,
            offset: 18395 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x5e7 as xkb_keysym_t,
            offset: 1282 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6a4 as xkb_keysym_t,
            offset: 26443 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6a6 as xkb_keysym_t,
            offset: 26418 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6a7 as xkb_keysym_t,
            offset: 26469 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6a8 as xkb_keysym_t,
            offset: 9574 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6a9 as xkb_keysym_t,
            offset: 9713 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6aa as xkb_keysym_t,
            offset: 9739 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6af as xkb_keysym_t,
            offset: 9068 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6b4 as xkb_keysym_t,
            offset: 26430 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6b6 as xkb_keysym_t,
            offset: 26406 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6b7 as xkb_keysym_t,
            offset: 26456 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6b8 as xkb_keysym_t,
            offset: 9562 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6b9 as xkb_keysym_t,
            offset: 9700 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6ba as xkb_keysym_t,
            offset: 9726 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x6bf as xkb_keysym_t,
            offset: 9054 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x7a5 as xkb_keysym_t,
            offset: 13857 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x8a2 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x8a3 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x8a6 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xaac as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xabc as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xabd as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xabe as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xaca as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xacc as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xacd as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xace as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xacf as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xadb as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xadc as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xadd as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xade as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xadf as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae0 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae1 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae2 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae3 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae4 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae5 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae6 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae7 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae8 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xae9 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xaea as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xaeb as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xba3 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xba6 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xba8 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xba9 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbc0 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbc3 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbc6 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbd6 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbd8 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xbda as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce1 as xkb_keysym_t,
            offset: 16271 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce2 as xkb_keysym_t,
            offset: 16452 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce3 as xkb_keysym_t,
            offset: 16306 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce6 as xkb_keysym_t,
            offset: 16737 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce7 as xkb_keysym_t,
            offset: 16294 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xce8 as xkb_keysym_t,
            offset: 16668 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xcf1 as xkb_keysym_t,
            offset: 16592 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xcf5 as xkb_keysym_t,
            offset: 16418 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xcf6 as xkb_keysym_t,
            offset: 16713 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xcf7 as xkb_keysym_t,
            offset: 16568 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xcfa as xkb_keysym_t,
            offset: 16657 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xdde as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xeff as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xfe8a as xkb_keysym_t,
            offset: 11166 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xfe8b as xkb_keysym_t,
            offset: 11155 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff20 as xkb_keysym_t,
            offset: 19996 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff37 as xkb_keysym_t,
            offset: 8654 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff3c as xkb_keysym_t,
            offset: 23236 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff3d as xkb_keysym_t,
            offset: 20006 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff3e as xkb_keysym_t,
            offset: 22298 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff55 as xkb_keysym_t,
            offset: 22322 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff56 as xkb_keysym_t,
            offset: 20208 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff61 as xkb_keysym_t,
            offset: 22316 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff65 as xkb_keysym_t,
            offset: 26588 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff66 as xkb_keysym_t,
            offset: 22524 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff68 as xkb_keysym_t,
            offset: 12662 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff69 as xkb_keysym_t,
            offset: 8489 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff7e as xkb_keysym_t,
            offset: 17760 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 1 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff9a as xkb_keysym_t,
            offset: 19203 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xff9b as xkb_keysym_t,
            offset: 19171 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffc8 as xkb_keysym_t,
            offset: 12297 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffc9 as xkb_keysym_t,
            offset: 12301 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffca as xkb_keysym_t,
            offset: 12305 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffcb as xkb_keysym_t,
            offset: 12309 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffcc as xkb_keysym_t,
            offset: 12313 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffcd as xkb_keysym_t,
            offset: 12317 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffce as xkb_keysym_t,
            offset: 12321 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffcf as xkb_keysym_t,
            offset: 12325 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd0 as xkb_keysym_t,
            offset: 12329 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd1 as xkb_keysym_t,
            offset: 12336 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd2 as xkb_keysym_t,
            offset: 12340 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd3 as xkb_keysym_t,
            offset: 12344 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd4 as xkb_keysym_t,
            offset: 12348 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd5 as xkb_keysym_t,
            offset: 12352 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd6 as xkb_keysym_t,
            offset: 12356 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd7 as xkb_keysym_t,
            offset: 12360 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd8 as xkb_keysym_t,
            offset: 12364 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffd9 as xkb_keysym_t,
            offset: 12368 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffda as xkb_keysym_t,
            offset: 12372 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffdb as xkb_keysym_t,
            offset: 12379 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffdc as xkb_keysym_t,
            offset: 12383 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffdd as xkb_keysym_t,
            offset: 12387 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffde as xkb_keysym_t,
            offset: 12391 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffdf as xkb_keysym_t,
            offset: 12395 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0xffe0 as xkb_keysym_t,
            offset: 12399 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100055b as xkb_keysym_t,
            offset: 2048 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100055c as xkb_keysym_t,
            offset: 2287 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100055d as xkb_keysym_t,
            offset: 2869 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100055e as xkb_keysym_t,
            offset: 2779 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000589 as xkb_keysym_t,
            offset: 2327 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100058a as xkb_keysym_t,
            offset: 2448 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x10006cc as xkb_keysym_t,
            offset: 12521 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1002247 as xkb_keysym_t,
            offset: UNICODE_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1002248 as xkb_keysym_t,
            offset: UNICODE_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000a8 as xkb_keysym_t,
            offset: 17070 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000a9 as xkb_keysym_t,
            offset: 17137 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000aa as xkb_keysym_t,
            offset: 17083 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000ab as xkb_keysym_t,
            offset: 17120 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000ac as xkb_keysym_t,
            offset: 17102 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000af as xkb_keysym_t,
            offset: 17027 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000be as xkb_keysym_t,
            offset: 16973 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000ee as xkb_keysym_t,
            offset: 17174 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000f6 as xkb_keysym_t,
            offset: 17034 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100000fc as xkb_keysym_t,
            offset: 16927 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff6c as xkb_keysym_t,
            offset: 17150 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff6d as xkb_keysym_t,
            offset: 17158 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff6e as xkb_keysym_t,
            offset: 17167 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff6f as xkb_keysym_t,
            offset: 16935 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff70 as xkb_keysym_t,
            offset: 16996 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff71 as xkb_keysym_t,
            offset: 16960 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff72 as xkb_keysym_t,
            offset: 16983 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff73 as xkb_keysym_t,
            offset: 16947 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff74 as xkb_keysym_t,
            offset: 16917 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff75 as xkb_keysym_t,
            offset: 17014 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff76 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x1000ff77 as xkb_keysym_t,
            offset: DEPRECATED_KEYSYM as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
        deprecated_keysym {
            keysym: 0x100810f4 as xkb_keysym_t,
            offset: 30518 as uint16_t,
            explicit_index: 0 as uint8_t,
            explicit_count: 0 as uint8_t,
        },
    ];
    pub static mut explicit_deprecated_aliases: [u32; 1] = [24103 as i32 as u32];

    use super::stdint_uintn_h::{u32, uint16_t, uint8_t};
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn is_xdigit(mut ch: i8) -> bool {
        return ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32
            || ch as i32 >= 'a' as i32 && ch as i32 <= 'f' as i32
            || ch as i32 >= 'A' as i32 && ch as i32 <= 'F' as i32;
    }

    pub use crate::xkb::utils::{istrcmp, istrncmp};
}
pub mod utils_numbers_h {
    pub static mut digits__: [::core::ffi::c_uchar; 256] = [
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0 as i32 as ::core::ffi::c_uchar,
        1 as i32 as ::core::ffi::c_uchar,
        2 as i32 as ::core::ffi::c_uchar,
        3 as i32 as ::core::ffi::c_uchar,
        4 as i32 as ::core::ffi::c_uchar,
        5 as i32 as ::core::ffi::c_uchar,
        6 as i32 as ::core::ffi::c_uchar,
        7 as i32 as ::core::ffi::c_uchar,
        8 as i32 as ::core::ffi::c_uchar,
        9 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        10 as i32 as ::core::ffi::c_uchar,
        11 as i32 as ::core::ffi::c_uchar,
        12 as i32 as ::core::ffi::c_uchar,
        13 as i32 as ::core::ffi::c_uchar,
        14 as i32 as ::core::ffi::c_uchar,
        15 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        10 as i32 as ::core::ffi::c_uchar,
        11 as i32 as ::core::ffi::c_uchar,
        12 as i32 as ::core::ffi::c_uchar,
        13 as i32 as ::core::ffi::c_uchar,
        14 as i32 as ::core::ffi::c_uchar,
        15 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
    ];
    #[inline]
    pub unsafe fn parse_hex_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> i32 {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32)
                    < 16 as u32
                && result <= 4294967295 as u32 >> 4 as i32
            {
                result = result.wrapping_mul(16 as u32).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as i32
            } else {
                -1 as i32
            };
        }
    }

    use super::stdint_uintn_h::u32;
    use super::utils_h::is_xdigit;
}
pub mod utf8_decoding_h {
    pub const INVALID_UTF8_CODE_POINT: u32 = UINT32_MAX;

    use super::stdint_h::UINT32_MAX;
    use super::stdint_uintn_h::u32;
    pub use crate::xkb::utf8_decoding::utf8_next_code_point;
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: i32 = 0 as i32;
    pub const XKB_KEY_Mode_switch: i32 = 0xff7e as i32;
    pub const XKB_KEY_Num_Lock: i32 = 0xff7f as i32;
    pub const XKB_KEY_KP_Space: i32 = 0xff80 as i32;
    pub const XKB_KEY_KP_Equal: i32 = 0xffbd as i32;
    pub const XKB_KEY_Shift_L: i32 = 0xffe1 as i32;
    pub const XKB_KEY_Hyper_R: i32 = 0xffee as i32;
    pub const XKB_KEY_ISO_Lock: i32 = 0xfe01 as i32;
    pub const XKB_KEY_ISO_Level5_Lock: i32 = 0xfe13 as i32;
}
pub mod stdint_h {
    pub const UINT32_MAX: u32 = 4294967295 as u32;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_MAX_EXPLICIT: i32 = 0x1008ffb8 as i32;
    pub const XKB_KEYSYM_UNICODE_OFFSET: i32 = 0x1000000 as i32;
    pub const XKB_KEYSYM_UNICODE_MIN: i32 = 0x1000100 as i32;
    pub const XKB_KEYSYM_UNICODE_MAX: i32 = 0x110ffff as i32;
}
pub use self::__stddef_null_h::NULL;

pub use self::keysym_h::{
    XKB_KEYSYM_MAX_EXPLICIT, XKB_KEYSYM_UNICODE_MAX, XKB_KEYSYM_UNICODE_MIN,
    XKB_KEYSYM_UNICODE_OFFSET,
};
pub use self::keysym_names_h::{
    deprecated_keysym, deprecated_keysyms, explicit_deprecated_aliases, keysym_name_G,
    keysym_name_perfect_hash, keysym_names, keysym_to_name, name_keysym, name_to_keysym,
    DEPRECATED_KEYSYM, UNICODE_KEYSYM,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::UINT32_MAX;
pub use self::stdint_intn_h::i32;
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::{calloc, free};
pub use self::sys_types_h::ssize_t;
pub use self::types_h::{__int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::utf8_decoding_h::{utf8_next_code_point, INVALID_UTF8_CODE_POINT};
pub use self::utils_h::{is_xdigit, istrcmp, istrncmp};
pub use self::utils_numbers_h::{digits__, parse_hex_to_uint32_t};
pub use self::xkbcommon_h::{
    xkb_keysym_flags, xkb_keysym_t, xkb_utf32_to_keysym, XKB_KEYSYM_CASE_INSENSITIVE,
    XKB_KEYSYM_MAX, XKB_KEYSYM_NO_FLAGS,
};
pub use self::xkbcommon_keysyms_h::{
    XKB_KEY_Hyper_R, XKB_KEY_ISO_Level5_Lock, XKB_KEY_ISO_Lock, XKB_KEY_KP_Equal, XKB_KEY_KP_Space,
    XKB_KEY_Mode_switch, XKB_KEY_NoSymbol, XKB_KEY_Num_Lock, XKB_KEY_Shift_L,
};
use crate::xkb::utils::cstr_dup;
use crate::xkb::utils::{cstr_cmp, cstr_len, cstr_ncmp};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_keysym_iterator {
    pub explicit: bool,
    pub index: i32,
    pub keysym: xkb_keysym_t,
}
unsafe fn find_keysym_index(mut ks: xkb_keysym_t) -> ssize_t {
    unsafe {
        if ks > XKB_KEYSYM_MAX_EXPLICIT as xkb_keysym_t {
            return -1 as i32 as ssize_t;
        }
        let mut lo: ssize_t = 0 as ssize_t;
        let mut hi: ssize_t = (::core::mem::size_of::<[name_keysym; 2502]>() as usize)
            .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
            .wrapping_sub(1 as usize) as ssize_t;
        while hi >= lo {
            let mut mid: ssize_t = (lo + hi) / 2 as ssize_t;
            if ks > keysym_to_name[mid as usize].keysym {
                lo = mid + 1 as ssize_t;
            } else if ks < keysym_to_name[mid as usize].keysym {
                hi = mid - 1 as ssize_t;
            } else {
                return mid;
            }
        }
        return -1 as i32 as ssize_t;
    }
}
#[inline]
unsafe fn get_name(mut entry: *const name_keysym) -> *const i8 {
    unsafe {
        return keysym_names.offset((*entry).offset as i32 as isize);
    }
}
#[inline]
unsafe fn get_unicode_name(mut ks: xkb_keysym_t, mut buffer: *mut i8, mut size: usize) -> i32 {
    unsafe {
        return crate::xkb::utils::snprintf_c(
            buffer,
            size,
            format_args!("U{:04X}", ks & 0xffffff as xkb_keysym_t),
        );
    }
}
pub unsafe fn xkb_keysym_get_name(
    mut ks: xkb_keysym_t,
    mut buffer: *mut i8,
    mut size: usize,
) -> i32 {
    unsafe {
        if ks > XKB_KEYSYM_MAX as xkb_keysym_t {
            crate::xkb::utils::snprintf_args(buffer, size, format_args!("Invalid"));
            return -1 as i32;
        }
        let mut index: isize = find_keysym_index(ks);
        if index != -1 as i32 as isize {
            return crate::xkb::utils::snprintf_c(
                buffer,
                size,
                format_args!(
                    "{}",
                    crate::xkb::utils::CStrDisplay(get_name(
                        (&raw const keysym_to_name as *const name_keysym).offset(index as isize)
                            as *const name_keysym,
                    ))
                ),
            );
        }
        if ks >= XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t
            && ks <= XKB_KEYSYM_UNICODE_MAX as xkb_keysym_t
        {
            return get_unicode_name(ks, buffer, size);
        }
        return crate::xkb::utils::snprintf_c(buffer, size, format_args!("0x{:08x}", ks));
    }
}

pub unsafe fn xkb_keysym_is_assigned(mut ks: xkb_keysym_t) -> bool {
    unsafe {
        return XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks
            && ks <= XKB_KEYSYM_UNICODE_MAX as xkb_keysym_t
            || find_keysym_index(ks) != -1 as i32 as ssize_t;
    }
}

pub unsafe fn xkb_keysym_get_explicit_names(
    mut ks: xkb_keysym_t,
    mut buffer: *mut *const i8,
    mut size: usize,
) -> i32 {
    unsafe {
        if ks > XKB_KEYSYM_MAX as xkb_keysym_t {
            return -1 as i32;
        }
        let index: ssize_t = find_keysym_index(ks) as ssize_t;
        if index < 0 as ssize_t {
            return 0 as i32;
        }
        let canonical: uint16_t = keysym_to_name[index as usize].offset;
        if size > 0 as usize {
            let ref mut c2rust_fresh0 = *buffer.offset(0 as i32 as isize);
            *c2rust_fresh0 = get_name(
                (&raw const keysym_to_name as *const name_keysym).offset(index as isize)
                    as *const name_keysym,
            );
        }
        let mut count: i32 = 1 as i32;
        let mut pos: usize = 0 as usize;
        while pos
            < (::core::mem::size_of::<[name_keysym; 2635]>() as usize)
                .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
        {
            if name_to_keysym[pos as usize].keysym == ks
                && name_to_keysym[pos as usize].offset as i32 != canonical as i32
            {
                if (count as usize) < size {
                    let ref mut c2rust_fresh1 = *buffer.offset(count as isize);
                    *c2rust_fresh1 = get_name(
                        (&raw const name_to_keysym as *const name_keysym).offset(pos as isize)
                            as *const name_keysym,
                    );
                }
                count += 1;
            }
            pos = pos.wrapping_add(1);
        }
        return count;
    }
}

pub unsafe fn xkb_keysym_iterator_new(
    mut iterate_only_explicit_keysyms: bool,
) -> *mut xkb_keysym_iterator {
    unsafe {
        let mut iter: *mut xkb_keysym_iterator = calloc(
            1 as usize,
            ::core::mem::size_of::<xkb_keysym_iterator>() as usize,
        ) as *mut xkb_keysym_iterator;
        (*iter).explicit = iterate_only_explicit_keysyms;
        (*iter).index = -1 as i32 as i32;
        (*iter).keysym = XKB_KEYSYM_UNICODE_MAX as xkb_keysym_t;
        return iter;
    }
}

pub unsafe fn xkb_keysym_iterator_unref(
    mut iter: *mut xkb_keysym_iterator,
) -> *mut xkb_keysym_iterator {
    unsafe {
        if !iter.is_null() {
            free(iter as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<xkb_keysym_iterator>();
    }
}

pub unsafe fn xkb_keysym_iterator_get_keysym(mut iter: *mut xkb_keysym_iterator) -> xkb_keysym_t {
    unsafe {
        return (*iter).keysym;
    }
}

pub unsafe fn xkb_keysym_iterator_is_explicitly_named(mut iter: *mut xkb_keysym_iterator) -> bool {
    unsafe {
        return (*iter).index >= 0 as i32
            && (*iter).index
                < (::core::mem::size_of::<[name_keysym; 2502]>() as usize)
                    .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
                    as i32
            && ((*iter).explicit as i32 != 0
                || (*iter).keysym == keysym_to_name[(*iter).index as usize].keysym);
    }
}

pub unsafe fn xkb_keysym_iterator_get_name(
    mut iter: *mut xkb_keysym_iterator,
    mut buffer: *mut i8,
    mut size: usize,
) -> i32 {
    unsafe {
        if (*iter).index < 0 as i32
            || (*iter).index
                >= (::core::mem::size_of::<[name_keysym; 2502]>() as usize)
                    .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
                    as i32
        {
            return -1 as i32;
        }
        if (*iter).explicit as i32 != 0
            || (*iter).keysym == keysym_to_name[(*iter).index as usize].keysym
        {
            return crate::xkb::utils::snprintf_c(
                buffer,
                size,
                format_args!(
                    "{}",
                    crate::xkb::utils::CStrDisplay(get_name(
                        (&raw const keysym_to_name as *const name_keysym)
                            .offset((*iter).index as isize)
                            as *const name_keysym,
                    ))
                ),
            );
        }
        return get_unicode_name((*iter).keysym, buffer, size);
    }
}

pub unsafe fn xkb_keysym_iterator_next(mut iter: *mut xkb_keysym_iterator) -> bool {
    unsafe {
        if (*iter).index
            >= (::core::mem::size_of::<[name_keysym; 2502]>() as usize)
                .wrapping_div(::core::mem::size_of::<name_keysym>() as usize) as i32
                - 1 as i32
        {
            return false_0 != 0;
        }
        if (*iter).explicit as i32 != 0
            || (*iter).keysym >= XKB_KEYSYM_UNICODE_MAX as xkb_keysym_t
            || keysym_to_name[((*iter).index + 1 as i32) as usize].keysym
                < XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t
        {
            (*iter).index += 1;
            (*iter).keysym = keysym_to_name[(*iter).index as usize].keysym;
        } else {
            if (*iter).keysym >= keysym_to_name[(*iter).index as usize].keysym {
                (*iter).index += 1;
            }
            if (*iter).keysym >= XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t {
                (*iter).keysym = (*iter).keysym.wrapping_add(1);
            } else {
                (*iter).keysym = XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t;
            }
        }
        return true_0 != 0;
    }
}
unsafe fn parse_keysym_hex(mut s: *const i8, mut out: *mut u32) -> bool {
    unsafe {
        let count: i32 = parse_hex_to_uint32_t(s, 8 as usize, out as *mut u32) as i32;
        return count > 0 as i32 && *s.offset(count as isize) as i32 == '\0' as i32;
    }
}
pub unsafe fn xkb_keysym_from_name(
    mut name: *const i8,
    mut flags: xkb_keysym_flags,
) -> xkb_keysym_t {
    unsafe {
        static mut XKB_KEYSYM_FLAGS: xkb_keysym_flags = XKB_KEYSYM_CASE_INSENSITIVE;
        if flags as u32 & !(XKB_KEYSYM_FLAGS as u32) != 0 {
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut entry: *const name_keysym = ::core::ptr::null::<name_keysym>();
        let mut tmp: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut val: u32 = 0;
        let mut icase: bool = flags as u32 & XKB_KEYSYM_CASE_INSENSITIVE as i32 as u32 != 0;
        if !icase {
            let mut pos: usize = keysym_name_perfect_hash(name);
            if pos
                < (::core::mem::size_of::<[name_keysym; 2635]>() as usize)
                    .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
            {
                let mut s: *const i8 = get_name(
                    (&raw const name_to_keysym as *const name_keysym).offset(pos as isize)
                        as *const name_keysym,
                );
                if cstr_cmp(name, s) == 0 as i32 {
                    return name_to_keysym[pos as usize].keysym;
                }
            }
        } else {
            let mut lo: i32 = 0 as i32;
            let mut hi: i32 = (::core::mem::size_of::<[name_keysym; 2635]>() as usize)
                .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
                .wrapping_sub(1 as usize) as i32;
            while hi >= lo {
                let mut mid: i32 = (lo + hi) / 2 as i32;
                let mut cmp: i32 = istrcmp(
                    name,
                    get_name(
                        (&raw const name_to_keysym as *const name_keysym).offset(mid as isize)
                            as *const name_keysym,
                    ),
                );
                if cmp > 0 as i32 {
                    lo = mid + 1 as i32;
                } else if cmp < 0 as i32 {
                    hi = mid - 1 as i32;
                } else {
                    entry = (&raw const name_to_keysym as *const name_keysym).offset(mid as isize)
                        as *const name_keysym;
                    break;
                }
            }
            if !entry.is_null() {
                let mut last: *const name_keysym = ::core::ptr::null::<name_keysym>();
                last = (&raw const name_to_keysym as *const name_keysym)
                    .offset(
                        (::core::mem::size_of::<[name_keysym; 2635]>() as usize)
                            .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
                            as isize,
                    )
                    .offset(-(1 as i32 as isize));
                while entry < last
                    && istrcmp(get_name(entry.offset(1 as i32 as isize)), get_name(entry))
                        == 0 as i32
                {
                    entry = entry.offset(1);
                }
                return (*entry).keysym;
            }
        }
        if *name as i32 == 'U' as i32 || icase as i32 != 0 && *name as i32 == 'u' as i32 {
            if !parse_keysym_hex(name.offset(1 as i32 as isize) as *const i8, &raw mut val) {
                return XKB_KEY_NoSymbol as xkb_keysym_t;
            }
            return if val > 0xff as u32 && val <= 0x10ffff as u32 {
                (XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t).wrapping_add(val as xkb_keysym_t)
            } else {
                xkb_utf32_to_keysym(val)
            };
        } else if *name.offset(0 as i32 as isize) as i32 == '0' as i32
            && (*name.offset(1 as i32 as isize) as i32 == 'x' as i32
                || icase as i32 != 0 && *name.offset(1 as i32 as isize) as i32 == 'X' as i32)
        {
            if !parse_keysym_hex(name.offset(2 as i32 as isize) as *const i8, &raw mut val)
                || val > XKB_KEYSYM_MAX as u32
            {
                return XKB_KEY_NoSymbol as xkb_keysym_t;
            }
            return val;
        }
        if cstr_ncmp(name, b"XF86_\0".as_ptr() as *const i8, 5 as usize) == 0 as i32
            || icase as i32 != 0
                && istrncmp(name, b"XF86_\0".as_ptr() as *const i8, 5 as usize) == 0 as i32
        {
            let mut ret: xkb_keysym_t = 0;
            tmp = cstr_dup(name);
            if tmp.is_null() {
                return XKB_KEY_NoSymbol as xkb_keysym_t;
            }
            std::ptr::copy(
                tmp.offset(5 as i32 as isize),
                tmp.offset(4 as i32 as isize),
                cstr_len(name)
                    .wrapping_sub(5 as usize)
                    .wrapping_add(1 as usize),
            );
            ret = xkb_keysym_from_name(tmp, flags);
            free(tmp as *mut ::core::ffi::c_void);
            return ret;
        }
        return XKB_KEY_NoSymbol as xkb_keysym_t;
    }
}

pub unsafe fn xkb_utf8_to_keysym(mut buffer: *const i8, mut size: usize) -> xkb_keysym_t {
    unsafe {
        if buffer.is_null() || size == 0 {
            return 0 as xkb_keysym_t;
        }
        let mut length: usize = 0 as usize;
        let codepoint: u32 = utf8_next_code_point(buffer, size, &raw mut length) as u32;
        return if codepoint == INVALID_UTF8_CODE_POINT as u32 || length == 0 as usize {
            XKB_KEY_NoSymbol as xkb_keysym_t
        } else {
            xkb_utf32_to_keysym(codepoint)
        };
    }
}
pub unsafe fn xkb_keysym_is_keypad(mut keysym: xkb_keysym_t) -> bool {
    return keysym >= XKB_KEY_KP_Space as xkb_keysym_t
        && keysym <= XKB_KEY_KP_Equal as xkb_keysym_t;
}

pub unsafe fn xkb_keysym_is_modifier(mut keysym: xkb_keysym_t) -> bool {
    return keysym >= XKB_KEY_Shift_L as xkb_keysym_t && keysym <= XKB_KEY_Hyper_R as xkb_keysym_t
        || keysym >= XKB_KEY_ISO_Lock as xkb_keysym_t
            && keysym <= XKB_KEY_ISO_Level5_Lock as xkb_keysym_t
        || keysym == XKB_KEY_Mode_switch as xkb_keysym_t
        || keysym == XKB_KEY_Num_Lock as xkb_keysym_t;
}
pub unsafe fn xkb_keysym_is_deprecated(
    mut _keysym: xkb_keysym_t,
    mut _name: *const i8,
    mut reference_name: *mut *const i8,
) -> bool {
    unsafe {
        // Stub implementation: For Wayland-only usage, we don't need to track deprecated keysym names
        // This function is used by the parser to warn about deprecated keysym names
        *reference_name = ::core::ptr::null::<i8>();
        return false;
    }
}
