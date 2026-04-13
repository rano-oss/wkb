use crate::xkb::keysym_utf::xkb_utf32_to_keysym;
pub const XKB_KEY_Mode_switch: i32 = 0xff7e as i32;
pub const XKB_KEY_Num_Lock: i32 = 0xff7f as i32;
pub const XKB_KEY_KP_Space: i32 = 0xff80 as i32;
pub const XKB_KEY_KP_Equal: i32 = 0xffbd as i32;
pub const XKB_KEY_Shift_L: i32 = 0xffe1 as i32;
pub const XKB_KEY_Hyper_R: i32 = 0xffee as i32;
pub const XKB_KEY_ISO_Lock: i32 = 0xfe01 as i32;
pub const XKB_KEY_ISO_Level5_Lock: i32 = 0xfe13 as i32;
use crate::xkb::shared_types::*;
pub type ssize_t = isize;
pub mod keysym_names_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct name_keysym {
        pub keysym: xkb_keysym_t,
        pub offset: u16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct deprecated_keysym {
        pub keysym: xkb_keysym_t,
        pub offset: u16,
        pub explicit_index: u8,
        pub explicit_count: u8,
    }
    pub static mut keysym_names: *const i8 = b"0\x001\x002\x003\x003270_AltCursor\x003270_Attn\x003270_BackTab\x003270_ChangeScreen\x003270_Copy\x003270_CursorBlink\x003270_CursorSelect\x003270_DeleteWord\x003270_Duplicate\x003270_Enter\x003270_EraseEOF\x003270_EraseInput\x003270_ExSelect\x003270_FieldMark\x003270_Ident\x003270_Jump\x003270_KeyClick\x003270_Left2\x003270_PA1\x003270_PA2\x003270_PA3\x003270_Play\x003270_PrintScreen\x003270_Quit\x003270_Record\x003270_Reset\x003270_Right2\x003270_Rule\x003270_Setup\x003270_Test\x004\x005\x006\x007\08\09\0A\0a\0Aacute\0aacute\0Abelowdot\0abelowdot\0abovedot\0Abreve\0abreve\0Abreveacute\0abreveacute\0Abrevebelowdot\0abrevebelowdot\0Abrevegrave\0abrevegrave\0Abrevehook\0abrevehook\0Abrevetilde\0abrevetilde\0AccessX_Enable\0AccessX_Feedback_Enable\0Acircumflex\0acircumflex\0Acircumflexacute\0acircumflexacute\0Acircumflexbelowdot\0acircumflexbelowdot\0Acircumflexgrave\0acircumflexgrave\0Acircumflexhook\0acircumflexhook\0Acircumflextilde\0acircumflextilde\0acute\0Adiaeresis\0adiaeresis\0AE\0ae\0Agrave\0agrave\0Ahook\0ahook\0Alt_L\0Alt_R\0Amacron\0amacron\0ampersand\0Aogonek\0aogonek\0apostrophe\0approxeq\0approximate\0Arabic_0\0Arabic_1\0Arabic_2\0Arabic_3\0Arabic_4\0Arabic_5\0Arabic_6\0Arabic_7\0Arabic_8\0Arabic_9\0Arabic_ain\0Arabic_alef\0Arabic_alefmaksura\0Arabic_beh\0Arabic_comma\0Arabic_dad\0Arabic_dal\0Arabic_damma\0Arabic_dammatan\0Arabic_ddal\0Arabic_farsi_yeh\0Arabic_fatha\0Arabic_fathatan\0Arabic_feh\0Arabic_fullstop\0Arabic_gaf\0Arabic_ghain\0Arabic_ha\0Arabic_hah\0Arabic_hamza\0Arabic_hamza_above\0Arabic_hamza_below\0Arabic_hamzaonalef\0Arabic_hamzaonwaw\0Arabic_hamzaonyeh\0Arabic_hamzaunderalef\0Arabic_heh\0Arabic_heh_doachashmee\0Arabic_heh_goal\0Arabic_jeem\0Arabic_jeh\0Arabic_kaf\0Arabic_kasra\0Arabic_kasratan\0Arabic_keheh\0Arabic_khah\0Arabic_lam\0Arabic_madda_above\0Arabic_maddaonalef\0Arabic_meem\0Arabic_noon\0Arabic_noon_ghunna\0Arabic_peh\0Arabic_percent\0Arabic_qaf\0Arabic_question_mark\0Arabic_ra\0Arabic_rreh\0Arabic_sad\0Arabic_seen\0Arabic_semicolon\0Arabic_shadda\0Arabic_sheen\0Arabic_sukun\0Arabic_superscript_alef\0Arabic_switch\0Arabic_tah\0Arabic_tatweel\0Arabic_tcheh\0Arabic_teh\0Arabic_tehmarbuta\0Arabic_thal\0Arabic_theh\0Arabic_tteh\0Arabic_veh\0Arabic_waw\0Arabic_yeh\0Arabic_yeh_baree\0Arabic_zah\0Arabic_zain\0Aring\0aring\0Armenian_accent\0Armenian_amanak\0Armenian_apostrophe\0Armenian_AT\0Armenian_at\0Armenian_AYB\0Armenian_ayb\0Armenian_BEN\0Armenian_ben\0Armenian_but\0Armenian_CHA\0Armenian_cha\0Armenian_DA\0Armenian_da\0Armenian_DZA\0Armenian_dza\0Armenian_E\0Armenian_e\0Armenian_exclam\0Armenian_FE\0Armenian_fe\0Armenian_full_stop\0Armenian_GHAT\0Armenian_ghat\0Armenian_GIM\0Armenian_gim\0Armenian_HI\0Armenian_hi\0Armenian_HO\0Armenian_ho\0Armenian_hyphen\0Armenian_INI\0Armenian_ini\0Armenian_JE\0Armenian_je\0Armenian_KE\0Armenian_ke\0Armenian_KEN\0Armenian_ken\0Armenian_KHE\0Armenian_khe\0Armenian_ligature_ew\0Armenian_LYUN\0Armenian_lyun\0Armenian_MEN\0Armenian_men\0Armenian_NU\0Armenian_nu\0Armenian_O\0Armenian_o\0Armenian_paruyk\0Armenian_PE\0Armenian_pe\0Armenian_PYUR\0Armenian_pyur\0Armenian_question\0Armenian_RA\0Armenian_ra\0Armenian_RE\0Armenian_re\0Armenian_SE\0Armenian_se\0Armenian_separation_mark\0Armenian_SHA\0Armenian_sha\0Armenian_shesht\0Armenian_TCHE\0Armenian_tche\0Armenian_TO\0Armenian_to\0Armenian_TSA\0Armenian_tsa\0Armenian_TSO\0Armenian_tso\0Armenian_TYUN\0Armenian_tyun\0Armenian_verjaket\0Armenian_VEV\0Armenian_vev\0Armenian_VO\0Armenian_vo\0Armenian_VYUN\0Armenian_vyun\0Armenian_YECH\0Armenian_yech\0Armenian_yentamna\0Armenian_ZA\0Armenian_za\0Armenian_ZHE\0Armenian_zhe\0asciicircum\0asciitilde\0asterisk\0at\0Atilde\0atilde\0AudibleBell_Enable\0B\0b\0Babovedot\0babovedot\0backslash\0BackSpace\0BackTab\0ballotcross\0bar\0because\0Begin\0blank\0block\0botintegral\0botleftparens\0botleftsqbracket\0botleftsummation\0botrightparens\0botrightsqbracket\0botrightsummation\0bott\0botvertsummationconnector\0BounceKeys_Enable\0braceleft\0braceright\0bracketleft\0bracketright\0braille_blank\0braille_dot_1\0braille_dot_10\0braille_dot_2\0braille_dot_3\0braille_dot_4\0braille_dot_5\0braille_dot_6\0braille_dot_7\0braille_dot_8\0braille_dot_9\0braille_dots_1\0braille_dots_12\0braille_dots_123\0braille_dots_1234\0braille_dots_12345\0braille_dots_123456\0braille_dots_1234567\0braille_dots_12345678\0braille_dots_1234568\0braille_dots_123457\0braille_dots_1234578\0braille_dots_123458\0braille_dots_12346\0braille_dots_123467\0braille_dots_1234678\0braille_dots_123468\0braille_dots_12347\0braille_dots_123478\0braille_dots_12348\0braille_dots_1235\0braille_dots_12356\0braille_dots_123567\0braille_dots_1235678\0braille_dots_123568\0braille_dots_12357\0braille_dots_123578\0braille_dots_12358\0braille_dots_1236\0braille_dots_12367\0braille_dots_123678\0braille_dots_12368\0braille_dots_1237\0braille_dots_12378\0braille_dots_1238\0braille_dots_124\0braille_dots_1245\0braille_dots_12456\0braille_dots_124567\0braille_dots_1245678\0braille_dots_124568\0braille_dots_12457\0braille_dots_124578\0braille_dots_12458\0braille_dots_1246\0braille_dots_12467\0braille_dots_124678\0braille_dots_12468\0braille_dots_1247\0braille_dots_12478\0braille_dots_1248\0braille_dots_125\0braille_dots_1256\0braille_dots_12567\0braille_dots_125678\0braille_dots_12568\0braille_dots_1257\0braille_dots_12578\0braille_dots_1258\0braille_dots_126\0braille_dots_1267\0braille_dots_12678\0braille_dots_1268\0braille_dots_127\0braille_dots_1278\0braille_dots_128\0braille_dots_13\0braille_dots_134\0braille_dots_1345\0braille_dots_13456\0braille_dots_134567\0braille_dots_1345678\0braille_dots_134568\0braille_dots_13457\0braille_dots_134578\0braille_dots_13458\0braille_dots_1346\0braille_dots_13467\0braille_dots_134678\0braille_dots_13468\0braille_dots_1347\0braille_dots_13478\0braille_dots_1348\0braille_dots_135\0braille_dots_1356\0braille_dots_13567\0braille_dots_135678\0braille_dots_13568\0braille_dots_1357\0braille_dots_13578\0braille_dots_1358\0braille_dots_136\0braille_dots_1367\0braille_dots_13678\0braille_dots_1368\0braille_dots_137\0braille_dots_1378\0braille_dots_138\0braille_dots_14\0braille_dots_145\0braille_dots_1456\0braille_dots_14567\0braille_dots_145678\0braille_dots_14568\0braille_dots_1457\0braille_dots_14578\0braille_dots_1458\0braille_dots_146\0braille_dots_1467\0braille_dots_14678\0braille_dots_1468\0braille_dots_147\0braille_dots_1478\0braille_dots_148\0braille_dots_15\0braille_dots_156\0braille_dots_1567\0braille_dots_15678\0braille_dots_1568\0braille_dots_157\0braille_dots_1578\0braille_dots_158\0braille_dots_16\0braille_dots_167\0braille_dots_1678\0braille_dots_168\0braille_dots_17\0braille_dots_178\0braille_dots_18\0braille_dots_2\0braille_dots_23\0braille_dots_234\0braille_dots_2345\0braille_dots_23456\0braille_dots_234567\0braille_dots_2345678\0braille_dots_234568\0braille_dots_23457\0braille_dots_234578\0braille_dots_23458\0braille_dots_2346\0braille_dots_23467\0braille_dots_234678\0braille_dots_23468\0braille_dots_2347\0braille_dots_23478\0braille_dots_2348\0braille_dots_235\0braille_dots_2356\0braille_dots_23567\0braille_dots_235678\0braille_dots_23568\0braille_dots_2357\0braille_dots_23578\0braille_dots_2358\0braille_dots_236\0braille_dots_2367\0braille_dots_23678\0braille_dots_2368\0braille_dots_237\0braille_dots_2378\0braille_dots_238\0braille_dots_24\0braille_dots_245\0braille_dots_2456\0braille_dots_24567\0braille_dots_245678\0braille_dots_24568\0braille_dots_2457\0braille_dots_24578\0braille_dots_2458\0braille_dots_246\0braille_dots_2467\0braille_dots_24678\0braille_dots_2468\0braille_dots_247\0braille_dots_2478\0braille_dots_248\0braille_dots_25\0braille_dots_256\0braille_dots_2567\0braille_dots_25678\0braille_dots_2568\0braille_dots_257\0braille_dots_2578\0braille_dots_258\0braille_dots_26\0braille_dots_267\0braille_dots_2678\0braille_dots_268\0braille_dots_27\0braille_dots_278\0braille_dots_28\0braille_dots_3\0braille_dots_34\0braille_dots_345\0braille_dots_3456\0braille_dots_34567\0braille_dots_345678\0braille_dots_34568\0braille_dots_3457\0braille_dots_34578\0braille_dots_3458\0braille_dots_346\0braille_dots_3467\0braille_dots_34678\0braille_dots_3468\0braille_dots_347\0braille_dots_3478\0braille_dots_348\0braille_dots_35\0braille_dots_356\0braille_dots_3567\0braille_dots_35678\0braille_dots_3568\0braille_dots_357\0braille_dots_3578\0braille_dots_358\0braille_dots_36\0braille_dots_367\0braille_dots_3678\0braille_dots_368\0braille_dots_37\0braille_dots_378\0braille_dots_38\0braille_dots_4\0braille_dots_45\0braille_dots_456\0braille_dots_4567\0braille_dots_45678\0braille_dots_4568\0braille_dots_457\0braille_dots_4578\0braille_dots_458\0braille_dots_46\0braille_dots_467\0braille_dots_4678\0braille_dots_468\0braille_dots_47\0braille_dots_478\0braille_dots_48\0braille_dots_5\0braille_dots_56\0braille_dots_567\0braille_dots_5678\0braille_dots_568\0braille_dots_57\0braille_dots_578\0braille_dots_58\0braille_dots_6\0braille_dots_67\0braille_dots_678\0braille_dots_68\0braille_dots_7\0braille_dots_78\0braille_dots_8\0Break\0breve\0brokenbar\0Byelorussian_SHORTU\0Byelorussian_shortu\0C\0c\0C_H\0C_h\0c_h\0Cabovedot\0cabovedot\0Cacute\0cacute\0Cancel\0Caps_Lock\0careof\0caret\0caron\0Ccaron\0ccaron\0Ccedilla\0ccedilla\0Ccircumflex\0ccircumflex\0cedilla\0cent\0CH\0Ch\0ch\0checkerboard\0checkmark\0circle\0Clear\0ClearLine\0club\0Codeinput\0colon\0ColonSign\0combining_acute\0combining_belowdot\0combining_grave\0combining_hook\0combining_tilde\0comma\0containsas\0Control_L\0Control_R\0copyright\0cr\0crossinglines\0CruzeiroSign\0cuberoot\0currency\0cursor\0Cyrillic_A\0Cyrillic_a\0Cyrillic_BE\0Cyrillic_be\0Cyrillic_CHE\0Cyrillic_che\0Cyrillic_CHE_descender\0Cyrillic_che_descender\0Cyrillic_CHE_vertstroke\0Cyrillic_che_vertstroke\0Cyrillic_DE\0Cyrillic_de\0Cyrillic_DZHE\0Cyrillic_dzhe\0Cyrillic_E\0Cyrillic_e\0Cyrillic_EF\0Cyrillic_ef\0Cyrillic_EL\0Cyrillic_el\0Cyrillic_EM\0Cyrillic_em\0Cyrillic_EN\0Cyrillic_en\0Cyrillic_EN_descender\0Cyrillic_en_descender\0Cyrillic_ER\0Cyrillic_er\0Cyrillic_ES\0Cyrillic_es\0Cyrillic_GHE\0Cyrillic_ghe\0Cyrillic_GHE_bar\0Cyrillic_ghe_bar\0Cyrillic_HA\0Cyrillic_ha\0Cyrillic_HA_descender\0Cyrillic_ha_descender\0Cyrillic_HARDSIGN\0Cyrillic_hardsign\0Cyrillic_I\0Cyrillic_i\0Cyrillic_I_macron\0Cyrillic_i_macron\0Cyrillic_IE\0Cyrillic_ie\0Cyrillic_IO\0Cyrillic_io\0Cyrillic_JE\0Cyrillic_je\0Cyrillic_KA\0Cyrillic_ka\0Cyrillic_KA_descender\0Cyrillic_ka_descender\0Cyrillic_KA_vertstroke\0Cyrillic_ka_vertstroke\0Cyrillic_LJE\0Cyrillic_lje\0Cyrillic_NJE\0Cyrillic_nje\0Cyrillic_O\0Cyrillic_o\0Cyrillic_O_bar\0Cyrillic_o_bar\0Cyrillic_PE\0Cyrillic_pe\0Cyrillic_SCHWA\0Cyrillic_schwa\0Cyrillic_SHA\0Cyrillic_sha\0Cyrillic_SHCHA\0Cyrillic_shcha\0Cyrillic_SHHA\0Cyrillic_shha\0Cyrillic_SHORTI\0Cyrillic_shorti\0Cyrillic_SOFTSIGN\0Cyrillic_softsign\0Cyrillic_TE\0Cyrillic_te\0Cyrillic_TSE\0Cyrillic_tse\0Cyrillic_U\0Cyrillic_u\0Cyrillic_U_macron\0Cyrillic_u_macron\0Cyrillic_U_straight\0Cyrillic_u_straight\0Cyrillic_U_straight_bar\0Cyrillic_u_straight_bar\0Cyrillic_VE\0Cyrillic_ve\0Cyrillic_YA\0Cyrillic_ya\0Cyrillic_YERU\0Cyrillic_yeru\0Cyrillic_YU\0Cyrillic_yu\0Cyrillic_ZE\0Cyrillic_ze\0Cyrillic_ZHE\0Cyrillic_zhe\0Cyrillic_ZHE_descender\0Cyrillic_zhe_descender\0D\0d\0Dabovedot\0dabovedot\0Dacute_accent\0dagger\0Dcaron\0dcaron\0Dcedilla_accent\0Dcircumflex_accent\0Ddiaeresis\0dead_A\0dead_a\0dead_abovecomma\0dead_abovedot\0dead_abovereversedcomma\0dead_abovering\0dead_aboveverticalline\0dead_acute\0dead_apostrophe\0dead_belowbreve\0dead_belowcircumflex\0dead_belowcomma\0dead_belowdiaeresis\0dead_belowdot\0dead_belowmacron\0dead_belowring\0dead_belowtilde\0dead_belowverticalline\0dead_breve\0dead_capital_schwa\0dead_caron\0dead_cedilla\0dead_circumflex\0dead_currency\0dead_dasia\0dead_diaeresis\0dead_doubleacute\0dead_doublegrave\0dead_E\0dead_e\0dead_grave\0dead_greek\0dead_hamza\0dead_hook\0dead_horn\0dead_I\0dead_i\0dead_invertedbreve\0dead_iota\0dead_longsolidusoverlay\0dead_lowline\0dead_macron\0dead_O\0dead_o\0dead_ogonek\0dead_perispomeni\0dead_psili\0dead_SCHWA\0dead_schwa\0dead_semivoiced_sound\0dead_small_schwa\0dead_stroke\0dead_tilde\0dead_U\0dead_u\0dead_voiced_sound\0decimalpoint\0degree\0Delete\0DeleteChar\0DeleteLine\0Dgrave_accent\0diaeresis\0diamond\0digitspace\0dintegral\0division\0dollar\0DongSign\0doubbaselinedot\0doubleacute\0doubledagger\0doublelowquotemark\0Down\0downarrow\0downcaret\0downshoe\0downstile\0downtack\0DRemove\0Dring_accent\0Dstroke\0dstroke\0Dtilde\0E\0e\0Eabovedot\0eabovedot\0Eacute\0eacute\0Ebelowdot\0ebelowdot\0Ecaron\0ecaron\0Ecircumflex\0ecircumflex\0Ecircumflexacute\0ecircumflexacute\0Ecircumflexbelowdot\0ecircumflexbelowdot\0Ecircumflexgrave\0ecircumflexgrave\0Ecircumflexhook\0ecircumflexhook\0Ecircumflextilde\0ecircumflextilde\0EcuSign\0Ediaeresis\0ediaeresis\0Egrave\0egrave\0Ehook\0ehook\0eightsubscript\0eightsuperior\0Eisu_Shift\0Eisu_toggle\0elementof\0ellipsis\0em3space\0em4space\0Emacron\0emacron\0emdash\0emfilledcircle\0emfilledrect\0emopencircle\0emopenrectangle\0emptyset\0emspace\0End\0endash\0enfilledcircbullet\0enfilledsqbullet\0ENG\0eng\0enopencircbullet\0enopensquarebullet\0enspace\0Eogonek\0eogonek\0equal\0Escape\0ETH\0Eth\0eth\0Etilde\0etilde\0EuroSign\0exclam\0exclamdown\0Execute\0Ext16bit_L\0Ext16bit_R\0EZH\0ezh\0F\0f\0F1\0F10\0F11\0F12\0F13\0F14\0F15\0F16\0F17\0F18\0F19\0F2\0F20\0F21\0F22\0F23\0F24\0F25\0F26\0F27\0F28\0F29\0F3\0F30\0F31\0F32\0F33\0F34\0F35\0F4\0F5\0F6\0F7\0F8\0F9\0Fabovedot\0fabovedot\0Farsi_0\0Farsi_1\0Farsi_2\0Farsi_3\0Farsi_4\0Farsi_5\0Farsi_6\0Farsi_7\0Farsi_8\0Farsi_9\0Farsi_yeh\0femalesymbol\0ff\0FFrancSign\0figdash\0filledlefttribullet\0filledrectbullet\0filledrighttribullet\0filledtribulletdown\0filledtribulletup\0Find\0First_Virtual_Screen\0fiveeighths\0fivesixths\0fivesubscript\0fivesuperior\0fourfifths\0foursubscript\0foursuperior\0fourthroot\0function\0G\0g\0Gabovedot\0gabovedot\0Gbreve\0gbreve\0Gcaron\0gcaron\0Gcedilla\0gcedilla\0Gcircumflex\0gcircumflex\0Georgian_an\0Georgian_ban\0Georgian_can\0Georgian_char\0Georgian_chin\0Georgian_cil\0Georgian_don\0Georgian_en\0Georgian_fi\0Georgian_gan\0Georgian_ghan\0Georgian_hae\0Georgian_har\0Georgian_he\0Georgian_hie\0Georgian_hoe\0Georgian_in\0Georgian_jhan\0Georgian_jil\0Georgian_kan\0Georgian_khar\0Georgian_las\0Georgian_man\0Georgian_nar\0Georgian_on\0Georgian_par\0Georgian_phar\0Georgian_qar\0Georgian_rae\0Georgian_san\0Georgian_shin\0Georgian_tan\0Georgian_tar\0Georgian_un\0Georgian_vin\0Georgian_we\0Georgian_xan\0Georgian_zen\0Georgian_zhar\0grave\0greater\0greaterthanequal\0Greek_accentdieresis\0Greek_ALPHA\0Greek_alpha\0Greek_ALPHAaccent\0Greek_alphaaccent\0Greek_BETA\0Greek_beta\0Greek_CHI\0Greek_chi\0Greek_DELTA\0Greek_delta\0Greek_EPSILON\0Greek_epsilon\0Greek_EPSILONaccent\0Greek_epsilonaccent\0Greek_ETA\0Greek_eta\0Greek_ETAaccent\0Greek_etaaccent\0Greek_finalsmallsigma\0Greek_GAMMA\0Greek_gamma\0Greek_horizbar\0Greek_IOTA\0Greek_iota\0Greek_IOTAaccent\0Greek_iotaaccent\0Greek_iotaaccentdieresis\0Greek_IOTAdiaeresis\0Greek_IOTAdieresis\0Greek_iotadieresis\0Greek_KAPPA\0Greek_kappa\0Greek_LAMBDA\0Greek_lambda\0Greek_LAMDA\0Greek_lamda\0Greek_MU\0Greek_mu\0Greek_NU\0Greek_nu\0Greek_OMEGA\0Greek_omega\0Greek_OMEGAaccent\0Greek_omegaaccent\0Greek_OMICRON\0Greek_omicron\0Greek_OMICRONaccent\0Greek_omicronaccent\0Greek_PHI\0Greek_phi\0Greek_PI\0Greek_pi\0Greek_PSI\0Greek_psi\0Greek_RHO\0Greek_rho\0Greek_SIGMA\0Greek_sigma\0Greek_switch\0Greek_TAU\0Greek_tau\0Greek_THETA\0Greek_theta\0Greek_UPSILON\0Greek_upsilon\0Greek_UPSILONaccent\0Greek_upsilonaccent\0Greek_upsilonaccentdieresis\0Greek_UPSILONdieresis\0Greek_upsilondieresis\0Greek_XI\0Greek_xi\0Greek_ZETA\0Greek_zeta\0guilder\0guillemetleft\0guillemetright\0guillemotleft\0guillemotright\0H\0h\0hairspace\0Hangul\0Hangul_A\0Hangul_AE\0Hangul_AraeA\0Hangul_AraeAE\0Hangul_Banja\0Hangul_Cieuc\0Hangul_Codeinput\0Hangul_Dikeud\0Hangul_E\0Hangul_End\0Hangul_EO\0Hangul_EU\0Hangul_Hanja\0Hangul_Hieuh\0Hangul_I\0Hangul_Ieung\0Hangul_J_Cieuc\0Hangul_J_Dikeud\0Hangul_J_Hieuh\0Hangul_J_Ieung\0Hangul_J_Jieuj\0Hangul_J_Khieuq\0Hangul_J_Kiyeog\0Hangul_J_KiyeogSios\0Hangul_J_KkogjiDalrinIeung\0Hangul_J_Mieum\0Hangul_J_Nieun\0Hangul_J_NieunHieuh\0Hangul_J_NieunJieuj\0Hangul_J_PanSios\0Hangul_J_Phieuf\0Hangul_J_Pieub\0Hangul_J_PieubSios\0Hangul_J_Rieul\0Hangul_J_RieulHieuh\0Hangul_J_RieulKiyeog\0Hangul_J_RieulMieum\0Hangul_J_RieulPhieuf\0Hangul_J_RieulPieub\0Hangul_J_RieulSios\0Hangul_J_RieulTieut\0Hangul_J_Sios\0Hangul_J_SsangKiyeog\0Hangul_J_SsangSios\0Hangul_J_Tieut\0Hangul_J_YeorinHieuh\0Hangul_Jamo\0Hangul_Jeonja\0Hangul_Jieuj\0Hangul_Khieuq\0Hangul_Kiyeog\0Hangul_KiyeogSios\0Hangul_KkogjiDalrinIeung\0Hangul_Mieum\0Hangul_MultipleCandidate\0Hangul_Nieun\0Hangul_NieunHieuh\0Hangul_NieunJieuj\0Hangul_O\0Hangul_OE\0Hangul_PanSios\0Hangul_Phieuf\0Hangul_Pieub\0Hangul_PieubSios\0Hangul_PostHanja\0Hangul_PreHanja\0Hangul_PreviousCandidate\0Hangul_Rieul\0Hangul_RieulHieuh\0Hangul_RieulKiyeog\0Hangul_RieulMieum\0Hangul_RieulPhieuf\0Hangul_RieulPieub\0Hangul_RieulSios\0Hangul_RieulTieut\0Hangul_RieulYeorinHieuh\0Hangul_Romaja\0Hangul_SingleCandidate\0Hangul_Sios\0Hangul_Special\0Hangul_SsangDikeud\0Hangul_SsangJieuj\0Hangul_SsangKiyeog\0Hangul_SsangPieub\0Hangul_SsangSios\0Hangul_Start\0Hangul_SunkyeongeumMieum\0Hangul_SunkyeongeumPhieuf\0Hangul_SunkyeongeumPieub\0Hangul_switch\0Hangul_Tieut\0Hangul_U\0Hangul_WA\0Hangul_WAE\0Hangul_WE\0Hangul_WEO\0Hangul_WI\0Hangul_YA\0Hangul_YAE\0Hangul_YE\0Hangul_YEO\0Hangul_YeorinHieuh\0Hangul_YI\0Hangul_YO\0Hangul_YU\0Hankaku\0Hcircumflex\0hcircumflex\0heart\0hebrew_aleph\0hebrew_ayin\0hebrew_bet\0hebrew_beth\0hebrew_chet\0hebrew_dalet\0hebrew_daleth\0hebrew_doublelowline\0hebrew_finalkaph\0hebrew_finalmem\0hebrew_finalnun\0hebrew_finalpe\0hebrew_finalzade\0hebrew_finalzadi\0hebrew_gimel\0hebrew_gimmel\0hebrew_he\0hebrew_het\0hebrew_kaph\0hebrew_kuf\0hebrew_lamed\0hebrew_mem\0hebrew_nun\0hebrew_pe\0hebrew_qoph\0hebrew_resh\0hebrew_samech\0hebrew_samekh\0hebrew_shin\0Hebrew_switch\0hebrew_taf\0hebrew_taw\0hebrew_tet\0hebrew_teth\0hebrew_waw\0hebrew_yod\0hebrew_zade\0hebrew_zadi\0hebrew_zain\0hebrew_zayin\0Help\0Henkan\0Henkan_Mode\0hexagram\0Hiragana\0Hiragana_Katakana\0Home\0horizconnector\0horizlinescan1\0horizlinescan3\0horizlinescan5\0horizlinescan7\0horizlinescan9\0hpBackTab\0hpblock\0hpClearLine\0hpDeleteChar\0hpDeleteLine\0hpguilder\0hpInsertChar\0hpInsertLine\0hpIO\0hpKP_BackTab\0hplira\0hplongminus\0hpModelock1\0hpModelock2\0hpmute_acute\0hpmute_asciicircum\0hpmute_asciitilde\0hpmute_diaeresis\0hpmute_grave\0hpReset\0hpSystem\0hpUser\0hpYdiaeresis\0Hstroke\0hstroke\0ht\0Hyper_L\0Hyper_R\0hyphen\0I\0i\0Iabovedot\0Iacute\0iacute\0Ibelowdot\0ibelowdot\0Ibreve\0ibreve\0Icircumflex\0icircumflex\0identical\0Idiaeresis\0idiaeresis\0idotless\0ifonlyif\0Igrave\0igrave\0Ihook\0ihook\0Imacron\0imacron\0implies\0includedin\0includes\0infinity\0Insert\0InsertChar\0InsertLine\0integral\0intersection\0IO\0Iogonek\0iogonek\0ISO_Center_Object\0ISO_Continuous_Underline\0ISO_Discontinuous_Underline\0ISO_Emphasize\0ISO_Enter\0ISO_Fast_Cursor_Down\0ISO_Fast_Cursor_Left\0ISO_Fast_Cursor_Right\0ISO_Fast_Cursor_Up\0ISO_First_Group\0ISO_First_Group_Lock\0ISO_Group_Latch\0ISO_Group_Lock\0ISO_Group_Shift\0ISO_Last_Group\0ISO_Last_Group_Lock\0ISO_Left_Tab\0ISO_Level2_Latch\0ISO_Level3_Latch\0ISO_Level3_Lock\0ISO_Level3_Shift\0ISO_Level5_Latch\0ISO_Level5_Lock\0ISO_Level5_Shift\0ISO_Lock\0ISO_Move_Line_Down\0ISO_Move_Line_Up\0ISO_Next_Group\0ISO_Next_Group_Lock\0ISO_Partial_Line_Down\0ISO_Partial_Line_Up\0ISO_Partial_Space_Left\0ISO_Partial_Space_Right\0ISO_Prev_Group\0ISO_Prev_Group_Lock\0ISO_Release_Both_Margins\0ISO_Release_Margin_Left\0ISO_Release_Margin_Right\0ISO_Set_Margin_Left\0ISO_Set_Margin_Right\0Itilde\0itilde\0J\0j\0Jcircumflex\0jcircumflex\0jot\0K\0k\0kana_A\0kana_a\0kana_CHI\0kana_closingbracket\0kana_comma\0kana_conjunctive\0kana_E\0kana_e\0kana_FU\0kana_fullstop\0kana_HA\0kana_HE\0kana_HI\0kana_HO\0kana_HU\0kana_I\0kana_i\0kana_KA\0kana_KE\0kana_KI\0kana_KO\0kana_KU\0Kana_Lock\0kana_MA\0kana_ME\0kana_MI\0kana_middledot\0kana_MO\0kana_MU\0kana_N\0kana_NA\0kana_NE\0kana_NI\0kana_NO\0kana_NU\0kana_O\0kana_o\0kana_openingbracket\0kana_RA\0kana_RE\0kana_RI\0kana_RO\0kana_RU\0kana_SA\0kana_SE\0kana_SHI\0Kana_Shift\0kana_SO\0kana_SU\0kana_switch\0kana_TA\0kana_TE\0kana_TI\0kana_TO\0kana_TSU\0kana_tsu\0kana_TU\0kana_tu\0kana_U\0kana_u\0kana_WA\0kana_WO\0kana_YA\0kana_ya\0kana_YO\0kana_yo\0kana_YU\0kana_yu\0Kanji\0Kanji_Bangou\0kappa\0Katakana\0Kcedilla\0kcedilla\0Korean_Won\0KP_0\0KP_1\0KP_2\0KP_3\0KP_4\0KP_5\0KP_6\0KP_7\0KP_8\0KP_9\0KP_Add\0KP_BackTab\0KP_Begin\0KP_Decimal\0KP_Delete\0KP_Divide\0KP_Down\0KP_End\0KP_Enter\0KP_Equal\0KP_F1\0KP_F2\0KP_F3\0KP_F4\0KP_Home\0KP_Insert\0KP_Left\0KP_Multiply\0KP_Next\0KP_Page_Down\0KP_Page_Up\0KP_Prior\0KP_Right\0KP_Separator\0KP_Space\0KP_Subtract\0KP_Tab\0KP_Up\0kra\0L\0l\0L1\0L10\0L2\0L3\0L4\0L5\0L6\0L7\0L8\0L9\0Lacute\0lacute\0Last_Virtual_Screen\0latincross\0Lbelowdot\0lbelowdot\0Lcaron\0lcaron\0Lcedilla\0lcedilla\0Left\0leftanglebracket\0leftarrow\0leftcaret\0leftdoublequotemark\0leftmiddlecurlybrace\0leftopentriangle\0leftpointer\0leftradical\0leftshoe\0leftsingleanglequotemark\0leftsinglequotemark\0leftt\0lefttack\0less\0lessthanequal\0lf\0Linefeed\0lira\0LiraSign\0logicaland\0logicalor\0longminus\0lowleftcorner\0lowrightcorner\0Lstroke\0lstroke\0M\0m\0Mabovedot\0mabovedot\0Macedonia_DSE\0Macedonia_dse\0Macedonia_GJE\0Macedonia_gje\0Macedonia_KJE\0Macedonia_kje\0macron\0Mae_Koho\0malesymbol\0maltesecross\0marker\0masculine\0Massyo\0Menu\0Meta_L\0Meta_R\0MillSign\0minus\0minutes\0Mode_switch\0MouseKeys_Accel_Enable\0MouseKeys_Enable\0mu\0Muhenkan\0Multi_key\0MultipleCandidate\0multiply\0musicalflat\0musicalsharp\0mute_acute\0mute_asciicircum\0mute_asciitilde\0mute_diaeresis\0mute_grave\0N\0n\0nabla\0Nacute\0nacute\0NairaSign\0Ncaron\0ncaron\0Ncedilla\0ncedilla\0NewSheqelSign\0Next\0Next_Virtual_Screen\0ninesubscript\0ninesuperior\0nl\0nobreakspace\0NoSymbol\0notapproxeq\0notelementof\0notequal\0notidentical\0notsign\0Ntilde\0ntilde\0Num_Lock\0numbersign\0numerosign\0O\0o\0Oacute\0oacute\0Obarred\0obarred\0Obelowdot\0obelowdot\0Ocaron\0ocaron\0Ocircumflex\0ocircumflex\0Ocircumflexacute\0ocircumflexacute\0Ocircumflexbelowdot\0ocircumflexbelowdot\0Ocircumflexgrave\0ocircumflexgrave\0Ocircumflexhook\0ocircumflexhook\0Ocircumflextilde\0ocircumflextilde\0Odiaeresis\0odiaeresis\0Odoubleacute\0odoubleacute\0OE\0oe\0ogonek\0Ograve\0ograve\0Ohook\0ohook\0Ohorn\0ohorn\0Ohornacute\0ohornacute\0Ohornbelowdot\0ohornbelowdot\0Ohorngrave\0ohorngrave\0Ohornhook\0ohornhook\0Ohorntilde\0ohorntilde\0Omacron\0omacron\0oneeighth\0onefifth\0onehalf\0onequarter\0onesixth\0onesubscript\0onesuperior\0onethird\0Ooblique\0ooblique\0openrectbullet\0openstar\0opentribulletdown\0opentribulletup\0ordfeminine\0ordmasculine\0osfActivate\0osfAddMode\0osfBackSpace\0osfBackTab\0osfBeginData\0osfBeginLine\0osfCancel\0osfClear\0osfCopy\0osfCut\0osfDelete\0osfDeselectAll\0osfDown\0osfEndData\0osfEndLine\0osfEscape\0osfExtend\0osfHelp\0osfInsert\0osfLeft\0osfMenu\0osfMenuBar\0osfNextField\0osfNextMenu\0osfPageDown\0osfPageLeft\0osfPageRight\0osfPageUp\0osfPaste\0osfPrevField\0osfPrevMenu\0osfPrimaryPaste\0osfQuickPaste\0osfReselect\0osfRestore\0osfRight\0osfSelect\0osfSelectAll\0osfUndo\0osfUp\0Oslash\0oslash\0Otilde\0otilde\0overbar\0Overlay1_Enable\0Overlay2_Enable\0overline\0P\0p\0Pabovedot\0pabovedot\0Page_Down\0Page_Up\0paragraph\0parenleft\0parenright\0partdifferential\0partialderivative\0Pause\0percent\0period\0periodcentered\0permille\0PesetaSign\0phonographcopyright\0plus\0plusminus\0Pointer_Accelerate\0Pointer_Button1\0Pointer_Button2\0Pointer_Button3\0Pointer_Button4\0Pointer_Button5\0Pointer_Button_Dflt\0Pointer_DblClick1\0Pointer_DblClick2\0Pointer_DblClick3\0Pointer_DblClick4\0Pointer_DblClick5\0Pointer_DblClick_Dflt\0Pointer_DfltBtnNext\0Pointer_DfltBtnPrev\0Pointer_Down\0Pointer_DownLeft\0Pointer_DownRight\0Pointer_Drag1\0Pointer_Drag2\0Pointer_Drag3\0Pointer_Drag4\0Pointer_Drag5\0Pointer_Drag_Dflt\0Pointer_EnableKeys\0Pointer_Left\0Pointer_Right\0Pointer_Up\0Pointer_UpLeft\0Pointer_UpRight\0prescription\0Prev_Virtual_Screen\0PreviousCandidate\0Print\0Prior\0prolongedsound\0punctspace\0Q\0q\0quad\0question\0questiondown\0quotedbl\0quoteleft\0quoteright\0R\0r\0R1\0R10\0R11\0R12\0R13\0R14\0R15\0R2\0R3\0R4\0R5\0R6\0R7\0R8\0R9\0Racute\0racute\0radical\0Rcaron\0rcaron\0Rcedilla\0rcedilla\0Redo\0registered\0RepeatKeys_Enable\0Reset\0Return\0Right\0rightanglebracket\0rightarrow\0rightcaret\0rightdoublequotemark\0rightmiddlecurlybrace\0rightmiddlesummation\0rightopentriangle\0rightpointer\0rightshoe\0rightsingleanglequotemark\0rightsinglequotemark\0rightt\0righttack\0Romaji\0RupeeSign\0S\0s\0Sabovedot\0sabovedot\0Sacute\0sacute\0Scaron\0scaron\0Scedilla\0scedilla\0SCHWA\0schwa\0Scircumflex\0scircumflex\0script_switch\0Scroll_Lock\0seconds\0section\0Select\0semicolon\0semivoicedsound\0Serbian_DJE\0Serbian_dje\0Serbian_DZE\0Serbian_dze\0Serbian_JE\0Serbian_je\0Serbian_LJE\0Serbian_lje\0Serbian_NJE\0Serbian_nje\0Serbian_TSHE\0Serbian_tshe\0seveneighths\0sevensubscript\0sevensuperior\0Shift_L\0Shift_Lock\0Shift_R\0signaturemark\0signifblank\0similarequal\0SingleCandidate\0singlelowquotemark\0Sinh_a\0Sinh_aa\0Sinh_aa2\0Sinh_ae\0Sinh_ae2\0Sinh_aee\0Sinh_aee2\0Sinh_ai\0Sinh_ai2\0Sinh_al\0Sinh_au\0Sinh_au2\0Sinh_ba\0Sinh_bha\0Sinh_ca\0Sinh_cha\0Sinh_dda\0Sinh_ddha\0Sinh_dha\0Sinh_dhha\0Sinh_e\0Sinh_e2\0Sinh_ee\0Sinh_ee2\0Sinh_fa\0Sinh_ga\0Sinh_gha\0Sinh_h2\0Sinh_ha\0Sinh_i\0Sinh_i2\0Sinh_ii\0Sinh_ii2\0Sinh_ja\0Sinh_jha\0Sinh_jnya\0Sinh_ka\0Sinh_kha\0Sinh_kunddaliya\0Sinh_la\0Sinh_lla\0Sinh_lu\0Sinh_lu2\0Sinh_luu\0Sinh_luu2\0Sinh_ma\0Sinh_mba\0Sinh_na\0Sinh_ndda\0Sinh_ndha\0Sinh_ng\0Sinh_ng2\0Sinh_nga\0Sinh_nja\0Sinh_nna\0Sinh_nya\0Sinh_o\0Sinh_o2\0Sinh_oo\0Sinh_oo2\0Sinh_pa\0Sinh_pha\0Sinh_ra\0Sinh_ri\0Sinh_rii\0Sinh_ru2\0Sinh_ruu2\0Sinh_sa\0Sinh_sha\0Sinh_ssha\0Sinh_tha\0Sinh_thha\0Sinh_tta\0Sinh_ttha\0Sinh_u\0Sinh_u2\0Sinh_uu\0Sinh_uu2\0Sinh_va\0Sinh_ya\0sixsubscript\0sixsuperior\0slash\0SlowKeys_Enable\0soliddiamond\0space\0squareroot\0SSHARP\0ssharp\0sterling\0StickyKeys_Enable\0stricteq\0SunAgain\0SunAltGraph\0SunAudioLowerVolume\0SunAudioMute\0SunAudioRaiseVolume\0SunCompose\0SunCopy\0SunCut\0SunF36\0SunF37\0SunFA_Acute\0SunFA_Cedilla\0SunFA_Circum\0SunFA_Diaeresis\0SunFA_Grave\0SunFA_Tilde\0SunFind\0SunFront\0SunOpen\0SunPageDown\0SunPageUp\0SunPaste\0SunPowerSwitch\0SunPowerSwitchShift\0SunPrint_Screen\0SunProps\0SunStop\0SunSys_Req\0SunUndo\0SunVideoDegauss\0SunVideoLowerBrightness\0SunVideoRaiseBrightness\0Super_L\0Super_R\0Sys_Req\0System\0T\0t\0Tab\0Tabovedot\0tabovedot\0Tcaron\0tcaron\0Tcedilla\0tcedilla\0telephone\0telephonerecorder\0Terminate_Server\0Thai_baht\0Thai_bobaimai\0Thai_chochan\0Thai_chochang\0Thai_choching\0Thai_chochoe\0Thai_dochada\0Thai_dodek\0Thai_fofa\0Thai_fofan\0Thai_hohip\0Thai_honokhuk\0Thai_khokhai\0Thai_khokhon\0Thai_khokhuat\0Thai_khokhwai\0Thai_khorakhang\0Thai_kokai\0Thai_lakkhangyao\0Thai_lekchet\0Thai_lekha\0Thai_lekhok\0Thai_lekkao\0Thai_leknung\0Thai_lekpaet\0Thai_leksam\0Thai_leksi\0Thai_leksong\0Thai_leksun\0Thai_lochula\0Thai_loling\0Thai_lu\0Thai_maichattawa\0Thai_maiek\0Thai_maihanakat\0Thai_maihanakat_maitho\0Thai_maitaikhu\0Thai_maitho\0Thai_maitri\0Thai_maiyamok\0Thai_moma\0Thai_ngongu\0Thai_nikhahit\0Thai_nonen\0Thai_nonu\0Thai_oang\0Thai_paiyannoi\0Thai_phinthu\0Thai_phophan\0Thai_phophung\0Thai_phosamphao\0Thai_popla\0Thai_rorua\0Thai_ru\0Thai_saraa\0Thai_saraaa\0Thai_saraae\0Thai_saraaimaimalai\0Thai_saraaimaimuan\0Thai_saraam\0Thai_sarae\0Thai_sarai\0Thai_saraii\0Thai_sarao\0Thai_sarau\0Thai_saraue\0Thai_sarauee\0Thai_sarauu\0Thai_sorusi\0Thai_sosala\0Thai_soso\0Thai_sosua\0Thai_thanthakhat\0Thai_thonangmontho\0Thai_thophuthao\0Thai_thothahan\0Thai_thothan\0Thai_thothong\0Thai_thothung\0Thai_topatak\0Thai_totao\0Thai_wowaen\0Thai_yoyak\0Thai_yoying\0therefore\0thinspace\0THORN\0Thorn\0thorn\0threeeighths\0threefifths\0threequarters\0threesubscript\0threesuperior\0tintegral\0topintegral\0topleftparens\0topleftradical\0topleftsqbracket\0topleftsummation\0toprightparens\0toprightsqbracket\0toprightsummation\0topt\0topvertsummationconnector\0Touroku\0trademark\0trademarkincircle\0Tslash\0tslash\0twofifths\0twosubscript\0twosuperior\0twothirds\0U\0u\0Uacute\0uacute\0Ubelowdot\0ubelowdot\0Ubreve\0ubreve\0Ucircumflex\0ucircumflex\0Udiaeresis\0udiaeresis\0Udoubleacute\0udoubleacute\0Ugrave\0ugrave\0Uhook\0uhook\0Uhorn\0uhorn\0Uhornacute\0uhornacute\0Uhornbelowdot\0uhornbelowdot\0Uhorngrave\0uhorngrave\0Uhornhook\0uhornhook\0Uhorntilde\0uhorntilde\0Ukrainian_GHE_WITH_UPTURN\0Ukrainian_ghe_with_upturn\0Ukrainian_I\0Ukrainian_i\0Ukrainian_IE\0Ukrainian_ie\0Ukrainian_YI\0Ukrainian_yi\0Ukranian_I\0Ukranian_i\0Ukranian_JE\0Ukranian_je\0Ukranian_YI\0Ukranian_yi\0Umacron\0umacron\0underbar\0underscore\0Undo\0union\0Uogonek\0uogonek\0Up\0uparrow\0upcaret\0upleftcorner\0uprightcorner\0upshoe\0upstile\0uptack\0Uring\0uring\0User\0Utilde\0utilde\0V\0v\0variation\0vertbar\0vertconnector\0voicedsound\0VoidSymbol\0vt\0W\0w\0Wacute\0wacute\0Wcircumflex\0wcircumflex\0Wdiaeresis\0wdiaeresis\0Wgrave\0wgrave\0WonSign\0X\0x\0Xabovedot\0xabovedot\0XF8610ChannelsDown\0XF8610ChannelsUp\0XF863DMode\0XF86Accessibility\0XF86AddFavorite\0XF86Addressbook\0XF86ALSToggle\0XF86ApplicationLeft\0XF86ApplicationRight\0XF86AppSelect\0XF86AspectRatio\0XF86Assistant\0XF86AttendantOff\0XF86AttendantOn\0XF86AttendantToggle\0XF86Audio\0XF86AudioBassBoost\0XF86AudioChannelMode\0XF86AudioCycleTrack\0XF86AudioDesc\0XF86AudioForward\0XF86AudioLowerVolume\0XF86AudioMedia\0XF86AudioMicMute\0XF86AudioMute\0XF86AudioNext\0XF86AudioPause\0XF86AudioPlay\0XF86AudioPreset\0XF86AudioPrev\0XF86AudioRaiseVolume\0XF86AudioRandomPlay\0XF86AudioRecord\0XF86AudioRepeat\0XF86AudioRewind\0XF86AudioStop\0XF86AutopilotEngageToggle\0XF86Away\0XF86Back\0XF86BackForward\0XF86Battery\0XF86Blue\0XF86Bluetooth\0XF86Book\0XF86Break\0XF86BrightnessAdjust\0XF86BrightnessAuto\0XF86BrightnessMax\0XF86BrightnessMin\0XF86Buttonconfig\0XF86Calculater\0XF86Calculator\0XF86Calendar\0XF86CameraAccessDisable\0XF86CameraAccessEnable\0XF86CameraAccessToggle\0XF86CameraDown\0XF86CameraFocus\0XF86CameraLeft\0XF86CameraRight\0XF86CameraUp\0XF86CameraZoomIn\0XF86CameraZoomOut\0XF86CD\0XF86ChannelDown\0XF86ChannelUp\0XF86Clear\0XF86ClearGrab\0XF86ClearvuSonar\0XF86Close\0XF86Community\0XF86ContextMenu\0XF86ContrastAdjust\0XF86ControlPanel\0XF86Copy\0XF86Cut\0XF86CycleAngle\0XF86Data\0XF86Database\0XF86Dictate\0XF86Display\0XF86DisplayOff\0XF86DisplayToggle\0XF86Documents\0XF86DoNotDisturb\0XF86DOS\0XF86DualRangeRadar\0XF86DVD\0XF86Editor\0XF86Eject\0XF86ElectronicPrivacyScreenOff\0XF86ElectronicPrivacyScreenOn\0XF86EmojiPicker\0XF86Excel\0XF86Exit\0XF86Explorer\0XF86FastReverse\0XF86Favorites\0XF86Finance\0XF86FishingChart\0XF86Fn\0XF86Fn_1\0XF86Fn_2\0XF86Fn_B\0XF86Fn_D\0XF86Fn_E\0XF86Fn_Esc\0XF86Fn_F\0XF86Fn_F1\0XF86Fn_F10\0XF86Fn_F11\0XF86Fn_F12\0XF86Fn_F2\0XF86Fn_F3\0XF86Fn_F4\0XF86Fn_F5\0XF86Fn_F6\0XF86Fn_F7\0XF86Fn_F8\0XF86Fn_F9\0XF86Fn_S\0XF86FnRightShift\0XF86Forward\0XF86FrameBack\0XF86FrameForward\0XF86FullScreen\0XF86Game\0XF86Go\0XF86GoTo\0XF86GraphicsEditor\0XF86Green\0XF86HangupPhone\0XF86Hibernate\0XF86History\0XF86HomePage\0XF86HotLinks\0XF86Images\0XF86Info\0XF86iTouch\0XF86Journal\0XF86KbdBrightnessDown\0XF86KbdBrightnessUp\0XF86KbdInputAssistAccept\0XF86KbdInputAssistCancel\0XF86KbdInputAssistNext\0XF86KbdInputAssistNextgroup\0XF86KbdInputAssistPrev\0XF86KbdInputAssistPrevgroup\0XF86KbdLcdMenu1\0XF86KbdLcdMenu2\0XF86KbdLcdMenu3\0XF86KbdLcdMenu4\0XF86KbdLcdMenu5\0XF86KbdLightOnOff\0XF86Keyboard\0XF86Launch0\0XF86Launch1\0XF86Launch2\0XF86Launch3\0XF86Launch4\0XF86Launch5\0XF86Launch6\0XF86Launch7\0XF86Launch8\0XF86Launch9\0XF86LaunchA\0XF86LaunchB\0XF86LaunchC\0XF86LaunchD\0XF86LaunchE\0XF86LaunchF\0XF86LeftDown\0XF86LeftUp\0XF86LightBulb\0XF86LightsToggle\0XF86LinkPhone\0XF86LogGrabInfo\0XF86LogOff\0XF86LogWindowTree\0XF86Macro1\0XF86Macro10\0XF86Macro11\0XF86Macro12\0XF86Macro13\0XF86Macro14\0XF86Macro15\0XF86Macro16\0XF86Macro17\0XF86Macro18\0XF86Macro19\0XF86Macro2\0XF86Macro20\0XF86Macro21\0XF86Macro22\0XF86Macro23\0XF86Macro24\0XF86Macro25\0XF86Macro26\0XF86Macro27\0XF86Macro28\0XF86Macro29\0XF86Macro3\0XF86Macro30\0XF86Macro4\0XF86Macro5\0XF86Macro6\0XF86Macro7\0XF86Macro8\0XF86Macro9\0XF86MacroPreset1\0XF86MacroPreset2\0XF86MacroPreset3\0XF86MacroPresetCycle\0XF86MacroRecordStart\0XF86MacroRecordStop\0XF86Mail\0XF86MailForward\0XF86Market\0XF86MarkWaypoint\0XF86MediaLanguageMenu\0XF86MediaPlayer\0XF86MediaPlayPause\0XF86MediaPlaySlow\0XF86MediaRepeat\0XF86MediaSelectAuxiliary\0XF86MediaSelectCable\0XF86MediaSelectCD\0XF86MediaSelectDVD\0XF86MediaSelectHome\0XF86MediaSelectPC\0XF86MediaSelectProgramGuide\0XF86MediaSelectRadio\0XF86MediaSelectSatellite\0XF86MediaSelectTape\0XF86MediaSelectTeletext\0XF86MediaSelectTuner\0XF86MediaSelectTV\0XF86MediaSelectVCR\0XF86MediaSelectVCRPlus\0XF86MediaTitleMenu\0XF86MediaTopMenu\0XF86Meeting\0XF86Memo\0XF86MenuKB\0XF86MenuPB\0XF86Messenger\0XF86ModeLock\0XF86MonBrightnessAuto\0XF86MonBrightnessCycle\0XF86MonBrightnessDown\0XF86MonBrightnessUp\0XF86Music\0XF86MyComputer\0XF86MySites\0XF86NavChart\0XF86NavInfo\0XF86New\0XF86News\0XF86Next_VMode\0XF86NextElement\0XF86NextFavorite\0XF86NotificationCenter\0XF86NumberEntryMode\0XF86Numeric0\0XF86Numeric1\0XF86Numeric11\0XF86Numeric12\0XF86Numeric2\0XF86Numeric3\0XF86Numeric4\0XF86Numeric5\0XF86Numeric6\0XF86Numeric7\0XF86Numeric8\0XF86Numeric9\0XF86NumericA\0XF86NumericB\0XF86NumericC\0XF86NumericD\0XF86NumericPound\0XF86NumericStar\0XF86OfficeHome\0XF86OK\0XF86OnScreenKeyboard\0XF86Open\0XF86OpenURL\0XF86Option\0XF86Paste\0XF86PauseRecord\0XF86PerformanceMode\0XF86Phone\0XF86PickupPhone\0XF86Pictures\0XF86PowerDown\0XF86PowerOff\0XF86Presentation\0XF86Prev_VMode\0XF86PreviousElement\0XF86PrivacyScreenToggle\0XF86Q\0XF86RadarOverlay\0XF86Red\0XF86Refresh\0XF86RefreshRateToggle\0XF86Reload\0XF86Reply\0XF86RFKill\0XF86RightDown\0XF86RightUp\0XF86RockerDown\0XF86RockerEnter\0XF86RockerUp\0XF86RootMenu\0XF86RotateWindows\0XF86RotationKB\0XF86RotationLockToggle\0XF86RotationPB\0XF86Save\0XF86ScreenSaver\0XF86Screensaver\0XF86ScrollClick\0XF86ScrollDown\0XF86ScrollUp\0XF86Search\0XF86Select\0XF86SelectiveScreenshot\0XF86Send\0XF86Shop\0XF86SidevuSonar\0XF86SingleRangeRadar\0XF86Sleep\0XF86SlowReverse\0XF86Sos\0XF86Spell\0XF86SpellCheck\0XF86SplitScreen\0XF86Sport\0XF86Standby\0XF86Start\0XF86Stop\0XF86StopRecord\0XF86Subtitle\0XF86Support\0XF86Suspend\0XF86Switch_VT_1\0XF86Switch_VT_10\0XF86Switch_VT_11\0XF86Switch_VT_12\0XF86Switch_VT_2\0XF86Switch_VT_3\0XF86Switch_VT_4\0XF86Switch_VT_5\0XF86Switch_VT_6\0XF86Switch_VT_7\0XF86Switch_VT_8\0XF86Switch_VT_9\0XF86Taskmanager\0XF86TaskPane\0XF86Terminal\0XF86Time\0XF86ToDoList\0XF86Tools\0XF86TopMenu\0XF86TouchpadOff\0XF86TouchpadOn\0XF86TouchpadToggle\0XF86TraditionalSonar\0XF86Travel\0XF86Ungrab\0XF86Unmute\0XF86User1KB\0XF86User2KB\0XF86UserPB\0XF86UWB\0XF86VendorHome\0XF86VendorLogo\0XF86Video\0XF86VideoPhone\0XF86View\0XF86VOD\0XF86VoiceCommand\0XF86Voicemail\0XF86WakeUp\0XF86WebCam\0XF86WheelButton\0XF86WLAN\0XF86Word\0XF86WPSButton\0XF86WWAN\0XF86WWW\0XF86Xfer\0XF86Yellow\0XF86ZoomIn\0XF86ZoomOut\0XF86ZoomReset\0Y\0y\0Yacute\0yacute\0Ybelowdot\0ybelowdot\0Ycircumflex\0ycircumflex\0Ydiaeresis\0ydiaeresis\0yen\0Ygrave\0ygrave\0Yhook\0yhook\0Ytilde\0ytilde\0Z\0z\0Zabovedot\0zabovedot\0Zacute\0zacute\0Zcaron\0zcaron\0Zen_Koho\0Zenkaku\0Zenkaku_Hankaku\0zerosubscript\0zerosuperior\0Zstroke\0zstroke\0\0"
        .as_ptr() as *const i8;
    pub static mut keysym_name_G: [u16; 4728] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2700, 0, 0, 0, 0, 493, 0, 177, 0, 0, 4536, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2918, 0, 0, 0, 0, 0, 0, 3763, 0, 0, 1499, 0, 0, 0, 0, 0, 3907, 0, 0, 0, 3178, 0, 0,
        0, 517, 0, 0, 0, 3994, 0, 1572, 0, 707, 0, 0, 0, 0, 0, 3717, 0, 0, 0, 0, 0, 3803, 0, 0,
        2063, 0, 1010, 0, 0, 0, 0, 0, 4131, 0, 221, 0, 0, 0, 0, 0, 1616, 0, 4288, 387, 0, 0, 0, 0,
        0, 0, 0, 4135, 713, 0, 0, 0, 3214, 0, 0, 0, 0, 0, 0, 0, 0, 4364, 4099, 0, 3448, 227, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3832, 0, 0, 0, 815, 0,
        415, 0, 0, 0, 0, 1701, 2344, 0, 0, 0, 0, 0, 0, 3193, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 436, 147, 0, 0, 3646, 2899, 0, 0, 0, 0, 0, 0, 0, 1857, 0, 0, 0, 0, 0, 0,
        0, 2348, 0, 0, 0, 643, 0, 0, 0, 2257, 0, 0, 0, 0, 0, 0, 871, 2710, 630, 2243, 0, 0, 0, 0,
        1106, 0, 0, 0, 1263, 0, 0, 0, 0, 0, 0, 3366, 0, 0, 3111, 0, 1218, 0, 3547, 0, 939, 0, 0,
        1112, 0, 0, 2144, 0, 3737, 4308, 3320, 1700, 594, 0, 799, 0, 0, 0, 4057, 0, 1908, 4331, 0,
        1361, 0, 0, 1942, 0, 61, 0, 0, 0, 1660, 0, 0, 2422, 1514, 0, 0, 0, 1067, 0, 2968, 0, 0,
        1989, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1974, 4592, 3523, 0, 0, 0, 0, 222, 0, 0, 0, 0, 3722, 0,
        3867, 0, 0, 0, 806, 0, 0, 0, 2233, 0, 4050, 0, 0, 698, 2784, 0, 1092, 0, 2819, 0, 0, 2433,
        4664, 2245, 901, 0, 0, 2816, 0, 0, 0, 4693, 0, 4125, 0, 0, 0, 0, 0, 0, 0, 0, 2133, 0, 1956,
        0, 0, 0, 0, 1616, 1716, 0, 0, 0, 52, 0, 0, 7, 0, 4358, 0, 0, 4043, 2366, 1320, 0, 2050, 0,
        0, 0, 1781, 3818, 0, 0, 0, 4019, 0, 0, 0, 0, 0, 2746, 0, 0, 2980, 200, 3598, 1551, 4176, 0,
        0, 4318, 0, 0, 0, 2592, 0, 42, 4429, 3736, 2741, 0, 0, 0, 0, 0, 3567, 2557, 1537, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 717, 1274, 0, 0, 0, 0, 0, 0, 818, 2129, 648, 0, 0, 0, 0, 0, 888, 510, 0,
        0, 0, 0, 0, 208, 1522, 3794, 1374, 0, 0, 1550, 0, 0, 2843, 0, 3657, 1248, 0, 4497, 4142,
        52, 0, 0, 4490, 4517, 0, 0, 4136, 0, 0, 1278, 0, 0, 0, 0, 0, 0, 1540, 2216, 0, 0, 4297,
        844, 0, 2326, 2293, 2797, 342, 0, 0, 0, 0, 3796, 0, 0, 0, 0, 3135, 0, 957, 540, 0, 0, 2849,
        0, 854, 0, 0, 0, 1747, 0, 0, 0, 0, 178, 0, 0, 4641, 509, 4462, 1693, 0, 0, 4526, 1062, 0,
        0, 0, 0, 0, 576, 2654, 44, 0, 2527, 0, 0, 0, 0, 0, 0, 0, 2197, 0, 4376, 3290, 0, 4662, 0,
        0, 2176, 1845, 1647, 0, 0, 0, 0, 0, 0, 0, 3717, 0, 404, 0, 0, 0, 997, 0, 0, 3022, 0, 1965,
        0, 4691, 0, 0, 0, 0, 1568, 2224, 0, 1731, 850, 2880, 4405, 1383, 0, 174, 2562, 0, 0, 0,
        2239, 0, 0, 0, 0, 1863, 0, 0, 4560, 0, 0, 0, 0, 0, 1714, 3211, 0, 880, 1873, 654, 190, 91,
        0, 897, 4701, 2704, 1806, 0, 623, 0, 989, 0, 4349, 3114, 3887, 0, 0, 0, 0, 2769, 0, 448, 0,
        0, 1249, 0, 0, 0, 1191, 1865, 3089, 0, 1567, 0, 0, 0, 0, 0, 4350, 144, 3244, 0, 3023, 3625,
        1802, 3930, 0, 3441, 1756, 435, 0, 2437, 0, 0, 0, 0, 0, 0, 0, 0, 587, 0, 0, 0, 241, 462, 0,
        2053, 0, 3567, 0, 0, 3830, 0, 4161, 0, 2009, 1812, 0, 0, 2749, 0, 2260, 0, 1764, 0, 583, 0,
        2260, 1711, 1269, 826, 0, 0, 928, 0, 0, 2574, 4161, 0, 0, 3829, 0, 0, 1767, 3178, 0, 0,
        2579, 2014, 1398, 2734, 0, 1366, 2076, 3799, 1830, 1719, 0, 4658, 0, 0, 0, 2048, 1840, 0,
        1296, 0, 0, 2315, 0, 0, 1427, 0, 2382, 337, 0, 0, 0, 4236, 4678, 0, 0, 4629, 0, 4722, 3215,
        673, 2242, 0, 0, 1918, 0, 0, 588, 4379, 2428, 0, 4553, 2933, 0, 0, 836, 0, 1226, 854, 0, 0,
        0, 3645, 0, 0, 0, 2769, 0, 0, 0, 3258, 0, 0, 824, 42, 906, 1456, 0, 0, 0, 3681, 3990, 944,
        441, 1891, 0, 4012, 0, 3325, 1185, 1823, 387, 2357, 0, 0, 176, 0, 4059, 4099, 940, 0, 449,
        0, 0, 0, 1436, 2567, 0, 0, 1800, 0, 0, 562, 1692, 0, 4417, 0, 3954, 0, 0, 0, 3290, 0, 3651,
        1591, 3235, 3517, 1340, 0, 0, 0, 0, 0, 0, 0, 0, 714, 0, 0, 1657, 0, 0, 0, 0, 0, 0, 0, 1997,
        0, 2466, 1852, 2623, 0, 0, 2171, 0, 717, 0, 3173, 2358, 864, 1525, 0, 3822, 0, 2009, 0,
        371, 0, 2030, 0, 0, 0, 0, 3795, 0, 3034, 0, 0, 0, 2199, 0, 4490, 0, 3342, 2098, 0, 2235, 0,
        0, 579, 0, 0, 295, 0, 1341, 0, 4648, 1233, 0, 1425, 4363, 2928, 4149, 4050, 232, 3137, 501,
        0, 0, 0, 2762, 1217, 3238, 3318, 0, 3825, 0, 0, 392, 0, 0, 4376, 0, 0, 0, 0, 0, 0, 3720, 0,
        0, 875, 4366, 0, 0, 47, 0, 2137, 887, 4030, 4238, 4341, 1333, 58, 1091, 4534, 0, 0, 4413,
        1245, 904, 0, 1319, 1088, 2230, 0, 0, 0, 0, 0, 2559, 0, 2965, 0, 0, 3306, 0, 4024, 4422, 0,
        0, 0, 0, 4396, 245, 1609, 0, 2753, 945, 191, 575, 3515, 861, 0, 0, 3355, 901, 493, 4400,
        3129, 4577, 1707, 77, 0, 0, 0, 2460, 1539, 3399, 1834, 0, 0, 3033, 0, 905, 0, 2185, 0, 0,
        0, 0, 2231, 898, 328, 70, 2954, 2800, 0, 0, 2305, 2822, 1892, 0, 0, 2087, 2249, 0, 0, 0,
        220, 4504, 0, 580, 0, 0, 3132, 2321, 0, 3868, 1278, 286, 0, 2514, 1870, 3984, 0, 896, 0, 0,
        1743, 0, 3849, 0, 3432, 2231, 0, 0, 0, 906, 2577, 0, 0, 0, 3384, 4259, 0, 797, 0, 0, 2803,
        916, 4226, 699, 3817, 4126, 0, 0, 3789, 0, 4176, 416, 1589, 0, 0, 1923, 4313, 3929, 1560,
        0, 0, 0, 0, 1229, 0, 2776, 0, 0, 732, 0, 3179, 897, 0, 1595, 0, 0, 1453, 2206, 318, 0,
        2069, 0, 0, 3411, 0, 0, 2304, 0, 950, 2103, 1532, 2671, 4251, 2050, 0, 917, 0, 0, 0, 2648,
        2755, 2028, 226, 1588, 0, 2249, 0, 0, 0, 1596, 514, 0, 2035, 0, 0, 793, 0, 0, 1498, 0,
        2709, 0, 1008, 0, 0, 4377, 0, 0, 2689, 4556, 3784, 1285, 1632, 0, 0, 0, 0, 908, 4222, 1764,
        0, 519, 0, 0, 0, 883, 3725, 4430, 1754, 3890, 3872, 0, 0, 0, 0, 2180, 0, 463, 0, 0, 0,
        3998, 4604, 0, 0, 0, 0, 0, 3511, 0, 0, 0, 4387, 3512, 248, 31, 4625, 0, 1009, 899, 0, 0, 0,
        2577, 4642, 0, 0, 96, 1663, 1509, 0, 2459, 0, 85, 0, 1920, 303, 0, 709, 0, 0, 0, 0, 3928,
        638, 0, 0, 3440, 3219, 0, 1297, 0, 1706, 217, 0, 1471, 4178, 0, 0, 0, 0, 2847, 0, 2857,
        2440, 0, 1014, 0, 13, 2835, 0, 593, 0, 64, 1019, 0, 842, 2947, 3902, 0, 0, 0, 3582, 0,
        1326, 910, 0, 2903, 0, 0, 2444, 0, 1381, 2556, 4364, 3576, 3939, 920, 0, 0, 3990, 1142, 0,
        4414, 0, 2113, 0, 0, 4090, 991, 0, 0, 0, 0, 0, 3608, 1049, 0, 0, 2669, 1854, 3910, 0, 394,
        0, 196, 3192, 4038, 2225, 4259, 0, 0, 0, 0, 857, 0, 3495, 0, 806, 197, 0, 1223, 0, 0, 1807,
        3168, 0, 0, 0, 0, 0, 0, 3614, 2477, 581, 1429, 0, 0, 0, 0, 0, 989, 2565, 3705, 1273, 1891,
        721, 3885, 139, 0, 0, 0, 3525, 354, 0, 2302, 3242, 0, 0, 4326, 0, 607, 539, 0, 2140, 4632,
        3858, 0, 2502, 1692, 0, 0, 2511, 680, 172, 0, 3316, 1228, 4480, 1126, 3542, 0, 0, 2126,
        3884, 1730, 1366, 0, 0, 2597, 0, 0, 0, 940, 55, 3928, 311, 2323, 2334, 0, 0, 1556, 0, 2213,
        3691, 1780, 0, 2894, 2037, 0, 4286, 0, 2123, 1269, 886, 0, 0, 0, 0, 0, 0, 0, 1282, 0, 480,
        0, 0, 1245, 0, 3407, 0, 1223, 0, 0, 0, 1078, 0, 0, 442, 2479, 3294, 1338, 0, 0, 338, 0,
        739, 461, 3503, 0, 0, 2573, 0, 0, 3439, 0, 0, 4083, 110, 19, 0, 0, 0, 0, 1014, 0, 3885,
        187, 0, 3213, 0, 3954, 1520, 0, 0, 2072, 4384, 287, 0, 1840, 807, 0, 2469, 0, 0, 2178, 0,
        0, 1683, 1175, 2249, 0, 2480, 3441, 2283, 0, 522, 3654, 0, 2717, 0, 0, 1202, 0, 0, 0, 3428,
        748, 4552, 0, 4612, 0, 0, 2499, 1034, 4667, 0, 0, 1005, 0, 0, 3225, 876, 4140, 4429, 0, 0,
        3456, 0, 0, 809, 0, 307, 2324, 4558, 0, 0, 780, 1910, 1392, 0, 0, 913, 375, 2343, 2481,
        1661, 0, 0, 280, 0, 0, 4516, 812, 0, 0, 0, 0, 0, 4640, 1163, 2988, 0, 0, 1009, 0, 0, 3745,
        0, 2542, 818, 492, 2909, 214, 1052, 1626, 2580, 0, 1867, 0, 0, 2480, 0, 4147, 0, 0, 3,
        2372, 0, 1098, 0, 2786, 1447, 0, 1607, 2251, 3102, 4011, 1420, 41, 605, 2129, 3461, 4435,
        0, 2807, 0, 1865, 2284, 0, 0, 983, 1941, 0, 0, 142, 523, 0, 3981, 0, 0, 4020, 1247, 4109,
        0, 1630, 2720, 0, 0, 0, 2093, 1923, 3645, 0, 3088, 0, 0, 0, 3183, 0, 0, 2253, 0, 2422, 0,
        4064, 0, 0, 4102, 1454, 0, 1762, 0, 40, 0, 1780, 0, 0, 1310, 0, 0, 0, 3932, 1278, 0, 1096,
        240, 0, 2077, 2895, 0, 3771, 0, 0, 0, 2219, 0, 0, 1874, 436, 4139, 310, 2398, 0, 2770,
        1364, 2341, 2537, 0, 0, 407, 0, 3473, 3883, 1766, 0, 0, 2326, 0, 2630, 3338, 2513, 4383,
        2538, 2427, 3728, 0, 3814, 759, 2764, 4600, 0, 0, 0, 0, 0, 1890, 0, 3867, 0, 3283, 0, 0, 0,
        4646, 0, 3995, 3652, 2620, 1375, 3902, 2595, 0, 2688, 0, 168, 373, 3692, 0, 0, 2363, 140,
        1327, 0, 185, 0, 0, 3078, 0, 4609, 0, 0, 0, 856, 2244, 0, 2138, 0, 415, 384, 0, 1413, 0,
        1782, 0, 874, 0, 0, 1261, 672, 0, 27, 4296, 0, 2826, 1668, 0, 927, 1765, 36, 0, 702, 2913,
        0, 1418, 3523, 1891, 0, 0, 2319, 0, 0, 4330, 0, 0, 1912, 0, 1800, 4539, 0, 0, 0, 0, 4434,
        2600, 2945, 0, 539, 0, 2280, 4576, 1611, 1598, 2148, 0, 2364, 0, 2783, 3103, 0, 911, 0,
        2450, 2534, 6, 0, 1889, 1558, 1853, 548, 2787, 4088, 0, 0, 2563, 0, 3277, 381, 0, 4638,
        4474, 2135, 994, 0, 120, 0, 0, 828, 0, 0, 4114, 0, 0, 1089, 0, 3223, 521, 821, 0, 1826,
        1419, 0, 3740, 0, 0, 2924, 991, 0, 503, 4165, 0, 3686, 0, 0, 297, 0, 0, 0, 0, 1491, 0, 0,
        0, 0, 4123, 0, 0, 2139, 0, 0, 108, 77, 2635, 0, 2115, 63, 4688, 0, 0, 2885, 0, 0, 764,
        2507, 3345, 2956, 0, 0, 1736, 0, 0, 0, 0, 0, 0, 3040, 0, 3713, 0, 1668, 0, 1099, 11, 0, 0,
        1785, 2042, 2002, 3947, 0, 1983, 1972, 1973, 0, 0, 1322, 511, 0, 0, 3060, 0, 4657, 910,
        1150, 0, 4231, 4682, 0, 0, 0, 219, 0, 0, 1218, 3797, 2883, 0, 1696, 566, 1425, 0, 1962, 0,
        0, 1940, 2366, 133, 0, 2923, 0, 0, 1655, 1797, 1802, 1057, 0, 1443, 0, 348, 1688, 2004, 0,
        0, 485, 2030, 0, 0, 3364, 0, 868, 3605, 4341, 0, 0, 491, 1316, 2072, 0, 4283, 4000, 0, 0,
        4379, 1928, 3106, 0, 4558, 118, 130, 4398, 488, 3291, 2944, 1275, 0, 906, 0, 661, 0, 0,
        1917, 2855, 2067, 4131, 3001, 27, 0, 1598, 1401, 0, 4567, 1079, 94, 0, 3113, 3045, 0, 1310,
        1641, 0, 2367, 0, 0, 29, 3620, 0, 608, 0, 4281, 113, 2180, 998, 433, 839, 0, 0, 190, 0, 0,
        0, 0, 0, 2418, 2557, 2767, 3079, 0, 2518, 0, 2625, 0, 1030, 3375, 1406, 0, 2484, 4113, 0,
        230, 0, 0, 0, 3523, 523, 0, 0, 2516, 0, 3506, 520, 0, 4567, 2736, 4388, 2085, 0, 0, 1322,
        2152, 707, 1035, 4476, 0, 0, 3014, 0, 1818, 2863, 4292, 4409, 3406, 0, 2492, 0, 0, 2368,
        3568, 1366, 3525, 1853, 0, 4308, 0, 653, 0, 1924, 0, 0, 565, 1805, 451, 538, 102, 0, 0,
        1143, 4618, 2885, 1699, 0, 270, 1353, 788, 3640, 0, 0, 0, 1618, 4606, 0, 0, 525, 0, 0, 0,
        903, 2081, 1828, 1963, 4618, 0, 0, 2566, 0, 2086, 3544, 0, 4284, 3274, 0, 1621, 0, 313,
        536, 4167, 0, 2236, 663, 317, 0, 0, 2486, 1371, 3855, 3209, 4160, 2540, 3172, 0, 0, 2191,
        0, 516, 2846, 0, 0, 3187, 0, 0, 4264, 398, 0, 0, 0, 2201, 1408, 0, 4368, 1448, 0, 3811,
        2315, 637, 1023, 784, 2113, 0, 0, 492, 4007, 1237, 2663, 0, 1834, 869, 3212, 339, 1641,
        3705, 0, 4238, 4615, 3200, 0, 2543, 0, 128, 0, 0, 0, 2050, 3956, 2978, 1473, 1253, 2303, 0,
        2106, 0, 0, 0, 3565, 4646, 0, 1082, 0, 4298, 0, 657, 0, 0, 0, 236, 2455, 255, 2370, 0, 0,
        0, 2377, 1875, 2195, 858, 0, 58, 1557, 0, 2654, 3623, 0, 2073, 2112, 2303, 0, 3134, 0, 157,
        1111, 862, 0, 3426, 3668, 923, 0, 696, 4110, 4586, 0, 4210, 1894, 3126, 4238, 524, 1801,
        1875, 4637, 3424, 2033, 0, 3119, 3169, 181, 2269, 0, 4172, 3585, 0, 0, 0, 545, 1821, 385,
        0, 1906, 352, 2541, 182, 2585, 4105, 0, 281, 726, 3905, 2257, 1211, 0, 895, 0, 229, 0,
        1784, 3867, 0, 2416, 0, 2633, 0, 3616, 558, 0, 0, 0, 0, 0, 3087, 0, 195, 4108, 880, 0, 0,
        548, 363, 0, 0, 0, 0, 0, 0, 0, 0, 1541, 0, 387, 4440, 0, 0, 1026, 4472, 0, 73, 0, 4552,
        851, 2810, 0, 4005, 0, 4232, 1953, 4432, 1226, 2079, 0, 522, 0, 1585, 1919, 1772, 0, 0, 0,
        0, 0, 336, 0, 471, 0, 3999, 582, 55, 3313, 4626, 447, 2467, 2864, 0, 2414, 2750, 3824, 0,
        0, 389, 0, 0, 2173, 370, 0, 0, 4708, 0, 0, 3794, 0, 4639, 0, 0, 88, 4320, 0, 2525, 3086,
        4357, 601, 2523, 758, 1698, 3380, 1450, 231, 1648, 3628, 4356, 0, 4273, 0, 0, 0, 1210, 0,
        0, 0, 3813, 0, 0, 0, 1990, 0, 4059, 3926, 1570, 1927, 162, 1096, 50, 3637, 1952, 633, 457,
        369, 0, 1809, 2297, 0, 0, 971, 952, 248, 1156, 0, 458, 0, 749, 3498, 0, 0, 322, 0, 2146, 0,
        4669, 1446, 1898, 0, 2666, 910, 1685, 3396, 0, 0, 149, 1740, 4238, 124, 4214, 2840, 356, 0,
        150, 0, 4134, 0, 0, 1549, 0, 0, 0, 1016, 553, 0, 2707, 0, 359, 0, 0, 832, 0, 810, 1504, 0,
        3541, 3515, 4352, 3460, 2363, 3899, 0, 1024, 0, 0, 0, 582, 0, 0, 1408, 1654, 0, 473, 1109,
        0, 1741, 1094, 0, 3705, 3273, 0, 4086, 2553, 4417, 3511, 0, 533, 71, 785, 0, 0, 36, 0,
        4364, 420, 552, 3311, 2425, 449, 0, 0, 0, 0, 0, 0, 314, 1196, 0, 4714, 0, 0, 0, 0, 0, 0,
        3702, 0, 0, 0, 0, 0, 0, 0, 3336, 1742, 347, 4152, 0, 3518, 0, 4608, 0, 0, 0, 0, 1417, 1508,
        1777, 1242, 0, 0, 0, 0, 0, 1110, 0, 0, 2166, 2079, 1208, 1047, 3945, 2550, 276, 0, 2316,
        142, 0, 0, 266, 3376, 798, 1465, 1227, 2339, 0, 3671, 0, 1035, 0, 2292, 272, 903, 2441, 0,
        0, 1920, 229, 0, 0, 0, 3992, 2326, 3732, 0, 0, 1507, 0, 519, 0, 0, 0, 2132, 983, 2472,
        1459, 0, 1962, 0, 0, 3948, 0, 0, 1452, 318, 0, 208, 3787, 1446, 0, 0, 0, 0, 2894, 0, 4333,
        89, 1823, 1661, 0, 2563, 0, 0, 0, 408, 2624, 513, 0, 800, 1036, 2046, 3018, 0, 0, 0, 1630,
        1819, 0, 0, 0, 0, 2845, 1360, 3400, 3039, 2061, 0, 0, 1337, 0, 1455, 4122, 0, 0, 3048, 479,
        742, 0, 3767, 0, 0, 1289, 509, 0, 1814, 166, 2766, 3733, 0, 0, 1561, 2099, 0, 607, 210,
        466, 4362, 2424, 0, 0, 0, 0, 3343, 0, 1181, 1426, 2135, 0, 0, 1063, 2041, 0, 894, 3038, 0,
        4229, 0, 4001, 725, 1539, 916, 2282, 2199, 3936, 3175, 330, 0, 3466, 238, 0, 0, 160, 2619,
        0, 1518, 985, 0, 4140, 0, 0, 3432, 0, 1324, 4623, 0, 1903, 0, 0, 3481, 3456, 2356, 0, 0, 0,
        1703, 3799, 0, 175, 1558, 2457, 1705, 2152, 0, 0, 2286, 10, 4290, 2121, 0, 1030, 1862, 0,
        2626, 0, 1282, 3960, 1203, 0, 298, 251, 0, 0, 735, 0, 1365, 0, 2445, 0, 3056, 656, 1638,
        4358, 0, 1417, 3323, 0, 2633, 2460, 4680, 599, 1759, 4349, 0, 4291, 1591, 4136, 0, 0, 0,
        3258, 3969, 158, 126, 2296, 0, 0, 0, 0, 1936, 0, 2215, 0, 0, 1121, 332, 1010, 0, 1318,
        2982, 851, 2617, 3810, 1151, 4701, 0, 0, 1258, 0, 4314, 1304, 0, 2014, 0, 0, 0, 1871, 4057,
        652, 2043, 2440, 1072, 4177, 2949, 2086, 4522, 0, 4658, 3277, 942, 144, 1430, 2565, 4329,
        3142, 0, 4085, 0, 1682, 2852, 1419, 231, 0, 378, 757, 0, 225, 3315, 3304, 3179, 668, 0,
        3093, 0, 4363, 2521, 3601, 4103, 1370, 3670, 0, 4542, 1378, 304, 0, 4699, 3784, 0, 95, 0,
        785, 0, 1969, 0, 1244, 2044, 901, 3046, 0, 3301, 0, 1823, 0, 0, 806, 1470, 0, 4208, 4301,
        877, 0, 0, 0, 0, 3983, 1416, 365, 0, 3269, 2611, 0, 376, 0, 1316, 0, 20, 0, 0, 142, 3840,
        0, 0, 752, 2088, 0, 4181, 0, 3746, 2689, 434, 0, 2373, 0, 0, 1435, 2060, 2335, 1242, 2341,
        0, 0, 4100, 442, 3610, 2169, 0, 1149, 2122, 0, 236, 0, 1837, 2019, 0, 0, 939, 4458, 322,
        638, 1378, 1666, 0, 0, 1031, 4255, 4090, 4042, 434, 0, 0, 0, 1486, 0, 4622, 3684, 0, 2861,
        494, 0, 2326, 2505, 0, 1307, 3623, 0, 2629, 0, 305, 1280, 0, 0, 0, 4658, 519, 4078, 0, 568,
        0, 2363, 2244, 4710, 2364, 269, 4511, 3744, 3311, 2374, 2530, 2509, 733, 0, 2824, 747, 0,
        0, 3101, 3928, 0, 0, 652, 0, 0, 1860, 854, 2328, 0, 1151, 0, 0, 4397, 0, 3895, 4667, 0, 0,
        0, 0, 705, 0, 604, 4303, 3096, 3290, 0, 1559, 0, 436, 754, 2386, 0, 669, 0, 2533, 3422,
        2462, 4716, 862, 712, 485, 1168, 0, 1037, 2311, 1093, 2395, 0, 526, 765, 0, 2077, 0, 0,
        4699, 2621, 1474, 0, 534, 229, 2294, 282, 1534, 2375, 2683, 0, 1617, 306, 0, 183, 996, 0,
        0, 852, 0, 3351, 1837, 0, 0, 756, 1970, 0, 0, 1462, 1505, 0, 2513, 2870, 0, 0, 0, 0, 973,
        1436, 0, 1803, 2535, 0, 2601, 1465, 0, 0, 178, 2, 1351, 0, 0, 4550, 4396, 0, 0, 0, 2399, 0,
        0, 1700, 0, 500, 0, 2891, 0, 0, 0, 2730, 530, 1545, 3881, 530, 0, 4468, 738, 0, 853, 1749,
        0, 0, 1181, 542, 4318, 0, 2550, 965, 2714, 1032, 211, 2423, 0, 1153, 0, 3711, 0, 0, 0,
        2360, 0, 4024, 2453, 0, 0, 4056, 974, 2955, 3875, 0, 0, 357, 3611, 0, 2984, 2813, 0, 1337,
        2334, 1271, 0, 0, 4003, 2127, 0, 1665, 2410, 281, 410, 0, 3073, 0, 3249, 1483, 300, 0,
        3667, 3322, 2153, 0, 2390, 1060, 4324, 0, 0, 1143, 4589, 1340, 0, 3960, 0, 2097, 532, 0,
        1720, 1484, 567, 1158, 4349, 2377, 4051, 0, 3760, 2252, 0, 0, 2651, 0, 2630, 0, 0, 0, 1958,
        1928, 301, 0, 1123, 516, 0, 4572, 622, 1962, 443, 0, 2739, 400, 794, 1194, 0, 4598, 0,
        1079, 4441, 4574, 0, 130, 218, 1138, 2355, 2898, 4471, 31, 4641, 0, 1938, 1604, 879, 0,
        2153, 3213, 0, 0, 1059, 1260, 0, 2391, 586, 1791, 1540, 1384, 3100, 0, 845, 935, 1850,
        2616, 4323, 587, 0, 1985, 2636, 1921, 0, 2385, 2497, 173, 3971, 0, 2502, 1549, 0, 548, 0,
        0, 35, 1630, 1776, 0, 0, 3696, 3169, 3566, 0, 0, 0, 1929, 1037, 0, 0, 0, 3344, 101, 0, 0,
        2548, 0, 0, 0, 4166, 3962, 1221, 3928, 1014, 0, 0, 3624, 3461, 0, 2535, 0, 2350, 0, 942,
        4574, 166, 0, 1509, 2067, 3966, 2717, 4451, 4047, 0, 0, 0, 0, 3925, 0, 2505, 961, 0, 3514,
        0, 3902, 0, 0, 506, 0, 100, 3824, 254, 464, 0, 4262, 358, 0, 4625, 3350, 4556, 1193, 1473,
        0, 0, 2732, 0, 0, 0, 0, 0, 2103, 794, 1165, 1612, 3616, 1988, 3737, 3924, 2342, 0, 164,
        4398, 0, 1955, 0, 0, 0, 4611, 1612, 0, 0, 0, 1994, 2208, 0, 2425, 937, 4317, 0, 3606, 0,
        2842, 671, 0, 0, 0, 2641, 0, 437, 3751, 0, 0, 0, 4107, 4238, 1837, 2404, 1260, 281, 981,
        2499, 693, 1802, 413, 744, 2163, 0, 0, 3845, 0, 2540, 3519, 1235, 4136, 0, 2107, 0, 0, 832,
        731, 2436, 79, 249, 0, 2366, 0, 0, 2940, 1542, 0, 0, 0, 812, 1426, 0, 1769, 1069, 0, 0,
        787, 626, 4653, 2225, 1061, 873, 0, 0, 0, 0, 4481, 1170, 1662, 0, 1223, 1181, 0, 0, 0, 0,
        0, 3861, 3039, 0, 3556, 1492, 0, 0, 4597, 400, 0, 1653, 1396, 0, 677, 3848, 0, 3121, 2289,
        1198, 2014, 2634, 2812, 2255, 1182, 0, 0, 579, 0, 0, 629, 4452, 0, 0, 0, 1551, 2626, 2362,
        0, 3958, 1725, 1357, 0, 0, 0, 1673, 0, 2728, 0, 0, 2609, 0, 2977, 1350, 3003, 943, 0, 2261,
        4303, 0, 2359, 0, 0, 0, 0, 0, 4449, 100, 0, 0, 2521, 76, 0, 0, 2851, 3597, 0, 2395, 0,
        1777, 0, 2090, 859, 0, 1167, 4662, 0, 0, 0, 4581, 0, 0, 0, 0, 0, 0, 0, 1707, 2877, 587, 0,
        0, 0, 2381, 38, 607, 1408, 2284, 2449, 1556, 0, 4238, 2669, 273, 1578, 1403, 527, 0, 2859,
        310, 2528, 1775, 4043, 41, 0, 0, 313, 1912, 3434, 1584, 0, 180, 832, 25, 562, 1880, 1055,
        1405, 0, 2974, 1042, 0, 0, 0, 1757, 1308, 0, 1944, 0, 0, 1740, 0, 2254, 0, 0, 0, 2159,
        2123, 0, 1635, 1602, 0, 977, 0, 0, 0, 0, 1276, 0, 1386, 584, 1114, 0, 3483, 3749, 0, 0,
        1166, 1702, 516, 0, 3480, 1678, 1832, 3840, 217, 0, 3749, 1025, 0, 969, 2568, 0, 526, 867,
        1813, 4222, 4067, 0, 0, 0, 955, 0, 2222, 509, 2402, 1094, 1200, 4192, 4471, 0, 129, 147,
        363, 3249, 0, 3767, 4592, 751, 0, 330, 1540, 0, 3096, 5, 0, 2420, 631, 0, 0, 1060, 733, 0,
        4132, 0, 3809, 0, 0, 4001, 2593, 1891, 4277, 199, 0, 0, 1811, 1816, 0, 0, 3734, 0, 0, 0,
        1382, 1537, 0, 205, 0, 258, 3305, 1566, 1966, 0, 1410, 0, 2309, 0, 2984, 695, 1544, 4425,
        0, 2073, 1466, 0, 0, 1223, 588, 0, 1199, 4493, 0, 1825, 717, 0, 227, 3467, 4458, 0, 3412,
        90, 1208, 0, 0, 0, 4058, 4236, 0, 1771, 4672, 0, 0, 0, 0, 0, 763, 2812, 601, 0, 0, 0, 3605,
        967, 0, 0, 0, 1364, 2176, 2820, 0, 1913, 421, 0, 111, 0, 2262, 2052, 0, 2150, 1857, 0,
        1893, 3413, 0, 0, 829, 1520, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2132, 238, 3160, 4447, 312, 2310,
        4329, 0, 3651, 3034, 0, 1843, 0, 0, 3999, 0, 127, 0, 2233, 2913, 1415, 4129, 0, 2437, 4696,
        902, 0, 2473, 561, 0, 827, 0, 0, 1971, 3875, 0, 3285, 0, 0, 3777, 1901, 0, 3203, 4581, 0,
        3210, 4715, 0, 1130, 3895, 1444, 118, 0, 328, 0, 1188, 1646, 2104, 863, 1055, 0, 0, 0, 0,
        2140, 4672, 4079, 1650, 1961, 993, 0, 375, 3954, 0, 0, 2196, 963, 0, 0, 0, 0, 2727, 0,
        4349, 2474, 0, 0, 2352, 3781, 2116, 3509, 1144, 246, 0, 3334, 0, 1038, 0, 0, 0, 70, 0,
        1339, 0, 2162, 180, 0, 710, 103, 861, 4521, 0, 3550, 0, 3970, 608, 0, 2088, 1693, 1607,
        2331, 4189, 773, 2025, 872, 31, 0, 0, 0, 0, 1919, 2265, 962, 0, 2497, 0, 4664, 2501, 249,
        1695, 817, 3380, 3386, 1188, 3559, 845, 0, 3553, 1614, 2032, 4660, 2512, 3175, 0, 0, 868,
        847, 727, 1218, 0, 0, 0, 0, 1002, 0, 2821, 0, 0, 0, 2978, 3699, 0, 0, 1940, 0, 69, 0, 1113,
        1694, 0, 750, 4266, 643, 0, 4647, 2975, 4556, 3997, 1197, 0, 0, 0, 811, 2419, 1849, 2695,
        0, 0, 0, 2687, 1528, 1158, 0, 3487, 3584, 2061, 2591, 4274, 176, 0, 4138, 4401, 1112, 2610,
        167, 2658, 2591, 1768, 2699, 3958, 0, 0, 0, 221, 0, 0, 38, 0, 0, 1237, 0, 2439, 877, 1858,
        0, 2070, 3062, 2181, 608, 0, 29, 0, 0, 880, 1679, 72, 0, 1756, 2193, 384, 0, 0, 4181, 0, 0,
        0, 3608, 1326, 0, 3699, 0, 0, 0, 3503, 0, 1735, 0, 0, 358, 0, 0, 0, 972, 1063, 2668, 3960,
        2786, 1862, 2093, 1845, 0, 1931, 995, 0, 3570, 364, 1054, 2914, 1957, 842, 0, 1903, 4485,
        1076, 1656, 0, 2265, 811, 3941, 957, 757, 0, 3404, 3369, 0, 244, 553, 1362, 0, 958, 0,
        1755, 1901, 1792, 3757, 789, 0, 3437, 0, 3932, 0, 407, 3403, 0, 0, 1177, 1399, 0, 1726,
        1234, 0, 189, 0, 518, 719, 4596, 3647, 0, 1291, 25, 0, 0, 4187, 94, 3004, 2954, 0, 2016,
        3532, 0, 0, 2204, 1180, 0, 0, 2310, 0, 0, 0, 3341, 4423, 0, 0, 844, 0, 0, 2873, 3035, 0,
        2831, 4, 948, 0, 0, 566, 1199, 1194, 0, 2456, 3581, 0, 0, 0, 0, 0, 0, 1850, 238, 1484,
        4036, 0, 0, 0, 1949, 4581, 2960, 3716, 0, 1885, 0, 0, 0, 601, 2318, 370, 0, 2275, 3911,
        983, 4570, 3494, 15, 4610, 1993, 0, 743, 3104, 0, 2771, 2214, 2709, 14, 2365, 702, 0, 803,
        0, 627, 3921, 1501, 0, 0, 984, 1249, 2615, 836, 1043, 0, 1374, 843, 2195, 0, 0, 228, 137,
        1412, 1678, 0, 1149, 0, 2071, 3584, 84, 1292, 711, 24, 0, 646, 79, 1083, 0, 2266, 2305,
        1212, 0, 1363, 0, 4428, 3377, 3576, 2382, 3798, 2500, 0, 0, 2645, 2628, 978, 1701, 0, 0, 0,
        2000, 402, 0, 2802, 999, 1674, 8, 0, 0, 1907, 2781, 1564, 3544, 1228, 839, 0, 0, 0, 4335,
        3413, 0, 0, 3299, 2507, 0, 2262, 0, 790, 0, 3672, 0, 3533, 670, 0, 1393, 746, 2492, 713,
        853, 0, 2241, 1669, 0, 114, 366, 1320, 0, 0, 884, 2181, 2489, 0, 2885, 2508, 0, 317, 2181,
        2890, 3096, 1270, 0, 4718, 922, 0, 0, 1374, 238, 1630, 0, 404, 4567, 940, 0, 2164, 2168,
        3419, 0, 876, 1069, 946, 418, 1307, 0, 802, 1110, 1090, 4617, 0, 1912, 582, 2559, 1671,
        504, 3516, 1076, 3092, 3788, 0, 4113, 1496, 0, 518, 284, 4067, 454, 2210, 89, 0, 2354,
        1694, 4053, 0, 0, 1055, 0, 0, 2794, 2685, 1623, 1683, 417, 1791, 0, 4294, 1874, 0, 3481,
        1579, 466, 2244, 0, 4007, 1961, 0, 0, 0, 0, 0, 460, 1946, 490, 508, 0, 1619, 0, 0, 0, 3757,
        2147, 1059, 0, 0, 1201, 443, 0, 723, 872, 873, 913, 3753, 0, 24, 739, 0, 0, 0, 0, 2216,
        3475, 831, 4259, 0, 4037, 0, 0, 526, 4702, 0, 1204, 3819, 2230, 0, 914, 0, 430, 2408, 3673,
        0, 0, 0, 3234, 3541, 3932, 0, 1355, 0, 3061, 2170, 0, 488, 0, 3964, 1286, 0, 0, 0, 2193,
        333, 295, 3552, 4081, 3204, 0, 1094, 3640, 1553, 0, 1305, 3915, 100, 0, 0, 4189, 2095,
        1923, 2472, 0, 428, 2977, 0, 665, 2493, 255, 927, 778, 1614, 1723, 920, 0, 26, 343, 199,
        213, 1928, 78, 3113, 1156, 4255, 2726, 1137, 2272, 0, 0, 1379, 1631, 2260, 0, 0, 2726, 0,
        2414,
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
            keysym: 0x30,
            offset: 0,
        },
        name_keysym {
            keysym: 0x31,
            offset: 2,
        },
        name_keysym {
            keysym: 0x32,
            offset: 4,
        },
        name_keysym {
            keysym: 0x33,
            offset: 6,
        },
        name_keysym {
            keysym: 0xfd10,
            offset: 8,
        },
        name_keysym {
            keysym: 0xfd0e,
            offset: 23,
        },
        name_keysym {
            keysym: 0xfd05,
            offset: 33,
        },
        name_keysym {
            keysym: 0xfd19,
            offset: 46,
        },
        name_keysym {
            keysym: 0xfd15,
            offset: 64,
        },
        name_keysym {
            keysym: 0xfd0f,
            offset: 74,
        },
        name_keysym {
            keysym: 0xfd1c,
            offset: 91,
        },
        name_keysym {
            keysym: 0xfd1a,
            offset: 109,
        },
        name_keysym {
            keysym: 0xfd01,
            offset: 125,
        },
        name_keysym {
            keysym: 0xfd1e,
            offset: 140,
        },
        name_keysym {
            keysym: 0xfd06,
            offset: 151,
        },
        name_keysym {
            keysym: 0xfd07,
            offset: 165,
        },
        name_keysym {
            keysym: 0xfd1b,
            offset: 181,
        },
        name_keysym {
            keysym: 0xfd02,
            offset: 195,
        },
        name_keysym {
            keysym: 0xfd13,
            offset: 210,
        },
        name_keysym {
            keysym: 0xfd12,
            offset: 221,
        },
        name_keysym {
            keysym: 0xfd11,
            offset: 231,
        },
        name_keysym {
            keysym: 0xfd04,
            offset: 245,
        },
        name_keysym {
            keysym: 0xfd0a,
            offset: 256,
        },
        name_keysym {
            keysym: 0xfd0b,
            offset: 265,
        },
        name_keysym {
            keysym: 0xfd0c,
            offset: 274,
        },
        name_keysym {
            keysym: 0xfd16,
            offset: 283,
        },
        name_keysym {
            keysym: 0xfd1d,
            offset: 293,
        },
        name_keysym {
            keysym: 0xfd09,
            offset: 310,
        },
        name_keysym {
            keysym: 0xfd18,
            offset: 320,
        },
        name_keysym {
            keysym: 0xfd08,
            offset: 332,
        },
        name_keysym {
            keysym: 0xfd03,
            offset: 343,
        },
        name_keysym {
            keysym: 0xfd14,
            offset: 355,
        },
        name_keysym {
            keysym: 0xfd17,
            offset: 365,
        },
        name_keysym {
            keysym: 0xfd0d,
            offset: 376,
        },
        name_keysym {
            keysym: 0x34,
            offset: 386,
        },
        name_keysym {
            keysym: 0x35,
            offset: 388,
        },
        name_keysym {
            keysym: 0x36,
            offset: 390,
        },
        name_keysym {
            keysym: 0x37,
            offset: 392,
        },
        name_keysym {
            keysym: 0x38,
            offset: 394,
        },
        name_keysym {
            keysym: 0x39,
            offset: 396,
        },
        name_keysym {
            keysym: 0x41,
            offset: 398,
        },
        name_keysym {
            keysym: 0x61,
            offset: 400,
        },
        name_keysym {
            keysym: 0xc1,
            offset: 402,
        },
        name_keysym {
            keysym: 0xe1,
            offset: 409,
        },
        name_keysym {
            keysym: 0x1001ea0,
            offset: 416,
        },
        name_keysym {
            keysym: 0x1001ea1,
            offset: 426,
        },
        name_keysym {
            keysym: 0x1ff,
            offset: 436,
        },
        name_keysym {
            keysym: 0x1c3,
            offset: 445,
        },
        name_keysym {
            keysym: 0x1e3,
            offset: 452,
        },
        name_keysym {
            keysym: 0x1001eae,
            offset: 459,
        },
        name_keysym {
            keysym: 0x1001eaf,
            offset: 471,
        },
        name_keysym {
            keysym: 0x1001eb6,
            offset: 483,
        },
        name_keysym {
            keysym: 0x1001eb7,
            offset: 498,
        },
        name_keysym {
            keysym: 0x1001eb0,
            offset: 513,
        },
        name_keysym {
            keysym: 0x1001eb1,
            offset: 525,
        },
        name_keysym {
            keysym: 0x1001eb2,
            offset: 537,
        },
        name_keysym {
            keysym: 0x1001eb3,
            offset: 548,
        },
        name_keysym {
            keysym: 0x1001eb4,
            offset: 559,
        },
        name_keysym {
            keysym: 0x1001eb5,
            offset: 571,
        },
        name_keysym {
            keysym: 0xfe70,
            offset: 583,
        },
        name_keysym {
            keysym: 0xfe71,
            offset: 598,
        },
        name_keysym {
            keysym: 0xc2,
            offset: 622,
        },
        name_keysym {
            keysym: 0xe2,
            offset: 634,
        },
        name_keysym {
            keysym: 0x1001ea4,
            offset: 646,
        },
        name_keysym {
            keysym: 0x1001ea5,
            offset: 663,
        },
        name_keysym {
            keysym: 0x1001eac,
            offset: 680,
        },
        name_keysym {
            keysym: 0x1001ead,
            offset: 700,
        },
        name_keysym {
            keysym: 0x1001ea6,
            offset: 720,
        },
        name_keysym {
            keysym: 0x1001ea7,
            offset: 737,
        },
        name_keysym {
            keysym: 0x1001ea8,
            offset: 754,
        },
        name_keysym {
            keysym: 0x1001ea9,
            offset: 770,
        },
        name_keysym {
            keysym: 0x1001eaa,
            offset: 786,
        },
        name_keysym {
            keysym: 0x1001eab,
            offset: 803,
        },
        name_keysym {
            keysym: 0xb4,
            offset: 820,
        },
        name_keysym {
            keysym: 0xc4,
            offset: 826,
        },
        name_keysym {
            keysym: 0xe4,
            offset: 837,
        },
        name_keysym {
            keysym: 0xc6,
            offset: 848,
        },
        name_keysym {
            keysym: 0xe6,
            offset: 851,
        },
        name_keysym {
            keysym: 0xc0,
            offset: 854,
        },
        name_keysym {
            keysym: 0xe0,
            offset: 861,
        },
        name_keysym {
            keysym: 0x1001ea2,
            offset: 868,
        },
        name_keysym {
            keysym: 0x1001ea3,
            offset: 874,
        },
        name_keysym {
            keysym: 0xffe9,
            offset: 880,
        },
        name_keysym {
            keysym: 0xffea,
            offset: 886,
        },
        name_keysym {
            keysym: 0x3c0,
            offset: 892,
        },
        name_keysym {
            keysym: 0x3e0,
            offset: 900,
        },
        name_keysym {
            keysym: 0x26,
            offset: 908,
        },
        name_keysym {
            keysym: 0x1a1,
            offset: 918,
        },
        name_keysym {
            keysym: 0x1b1,
            offset: 926,
        },
        name_keysym {
            keysym: 0x27,
            offset: 934,
        },
        name_keysym {
            keysym: 0x1002248,
            offset: 945,
        },
        name_keysym {
            keysym: 0x8c8,
            offset: 954,
        },
        name_keysym {
            keysym: 0x1000660,
            offset: 966,
        },
        name_keysym {
            keysym: 0x1000661,
            offset: 975,
        },
        name_keysym {
            keysym: 0x1000662,
            offset: 984,
        },
        name_keysym {
            keysym: 0x1000663,
            offset: 993,
        },
        name_keysym {
            keysym: 0x1000664,
            offset: 1002,
        },
        name_keysym {
            keysym: 0x1000665,
            offset: 1011,
        },
        name_keysym {
            keysym: 0x1000666,
            offset: 1020,
        },
        name_keysym {
            keysym: 0x1000667,
            offset: 1029,
        },
        name_keysym {
            keysym: 0x1000668,
            offset: 1038,
        },
        name_keysym {
            keysym: 0x1000669,
            offset: 1047,
        },
        name_keysym {
            keysym: 0x5d9,
            offset: 1056,
        },
        name_keysym {
            keysym: 0x5c7,
            offset: 1067,
        },
        name_keysym {
            keysym: 0x5e9,
            offset: 1079,
        },
        name_keysym {
            keysym: 0x5c8,
            offset: 1098,
        },
        name_keysym {
            keysym: 0x5ac,
            offset: 1109,
        },
        name_keysym {
            keysym: 0x5d6,
            offset: 1122,
        },
        name_keysym {
            keysym: 0x5cf,
            offset: 1133,
        },
        name_keysym {
            keysym: 0x5ef,
            offset: 1144,
        },
        name_keysym {
            keysym: 0x5ec,
            offset: 1157,
        },
        name_keysym {
            keysym: 0x1000688,
            offset: 1173,
        },
        name_keysym {
            keysym: 0x10006cc,
            offset: 1185,
        },
        name_keysym {
            keysym: 0x5ee,
            offset: 1202,
        },
        name_keysym {
            keysym: 0x5eb,
            offset: 1215,
        },
        name_keysym {
            keysym: 0x5e1,
            offset: 1231,
        },
        name_keysym {
            keysym: 0x10006d4,
            offset: 1242,
        },
        name_keysym {
            keysym: 0x10006af,
            offset: 1258,
        },
        name_keysym {
            keysym: 0x5da,
            offset: 1269,
        },
        name_keysym {
            keysym: 0x5e7,
            offset: 1282,
        },
        name_keysym {
            keysym: 0x5cd,
            offset: 1292,
        },
        name_keysym {
            keysym: 0x5c1,
            offset: 1303,
        },
        name_keysym {
            keysym: 0x1000654,
            offset: 1316,
        },
        name_keysym {
            keysym: 0x1000655,
            offset: 1335,
        },
        name_keysym {
            keysym: 0x5c3,
            offset: 1354,
        },
        name_keysym {
            keysym: 0x5c4,
            offset: 1373,
        },
        name_keysym {
            keysym: 0x5c6,
            offset: 1391,
        },
        name_keysym {
            keysym: 0x5c5,
            offset: 1409,
        },
        name_keysym {
            keysym: 0x5e7,
            offset: 1431,
        },
        name_keysym {
            keysym: 0x10006be,
            offset: 1442,
        },
        name_keysym {
            keysym: 0x10006c1,
            offset: 1465,
        },
        name_keysym {
            keysym: 0x5cc,
            offset: 1481,
        },
        name_keysym {
            keysym: 0x1000698,
            offset: 1493,
        },
        name_keysym {
            keysym: 0x5e3,
            offset: 1504,
        },
        name_keysym {
            keysym: 0x5f0,
            offset: 1515,
        },
        name_keysym {
            keysym: 0x5ed,
            offset: 1528,
        },
        name_keysym {
            keysym: 0x10006a9,
            offset: 1544,
        },
        name_keysym {
            keysym: 0x5ce,
            offset: 1557,
        },
        name_keysym {
            keysym: 0x5e4,
            offset: 1569,
        },
        name_keysym {
            keysym: 0x1000653,
            offset: 1580,
        },
        name_keysym {
            keysym: 0x5c2,
            offset: 1599,
        },
        name_keysym {
            keysym: 0x5e5,
            offset: 1618,
        },
        name_keysym {
            keysym: 0x5e6,
            offset: 1630,
        },
        name_keysym {
            keysym: 0x10006ba,
            offset: 1642,
        },
        name_keysym {
            keysym: 0x100067e,
            offset: 1661,
        },
        name_keysym {
            keysym: 0x100066a,
            offset: 1672,
        },
        name_keysym {
            keysym: 0x5e2,
            offset: 1687,
        },
        name_keysym {
            keysym: 0x5bf,
            offset: 1698,
        },
        name_keysym {
            keysym: 0x5d1,
            offset: 1719,
        },
        name_keysym {
            keysym: 0x1000691,
            offset: 1729,
        },
        name_keysym {
            keysym: 0x5d5,
            offset: 1741,
        },
        name_keysym {
            keysym: 0x5d3,
            offset: 1752,
        },
        name_keysym {
            keysym: 0x5bb,
            offset: 1764,
        },
        name_keysym {
            keysym: 0x5f1,
            offset: 1781,
        },
        name_keysym {
            keysym: 0x5d4,
            offset: 1795,
        },
        name_keysym {
            keysym: 0x5f2,
            offset: 1808,
        },
        name_keysym {
            keysym: 0x1000670,
            offset: 1821,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 1845,
        },
        name_keysym {
            keysym: 0x5d7,
            offset: 1859,
        },
        name_keysym {
            keysym: 0x5e0,
            offset: 1870,
        },
        name_keysym {
            keysym: 0x1000686,
            offset: 1885,
        },
        name_keysym {
            keysym: 0x5ca,
            offset: 1898,
        },
        name_keysym {
            keysym: 0x5c9,
            offset: 1909,
        },
        name_keysym {
            keysym: 0x5d0,
            offset: 1927,
        },
        name_keysym {
            keysym: 0x5cb,
            offset: 1939,
        },
        name_keysym {
            keysym: 0x1000679,
            offset: 1951,
        },
        name_keysym {
            keysym: 0x10006a4,
            offset: 1963,
        },
        name_keysym {
            keysym: 0x5e8,
            offset: 1974,
        },
        name_keysym {
            keysym: 0x5ea,
            offset: 1985,
        },
        name_keysym {
            keysym: 0x10006d2,
            offset: 1996,
        },
        name_keysym {
            keysym: 0x5d8,
            offset: 2013,
        },
        name_keysym {
            keysym: 0x5d2,
            offset: 2024,
        },
        name_keysym {
            keysym: 0xc5,
            offset: 2036,
        },
        name_keysym {
            keysym: 0xe5,
            offset: 2042,
        },
        name_keysym {
            keysym: 0x100055b,
            offset: 2048,
        },
        name_keysym {
            keysym: 0x100055c,
            offset: 2064,
        },
        name_keysym {
            keysym: 0x100055a,
            offset: 2080,
        },
        name_keysym {
            keysym: 0x1000538,
            offset: 2100,
        },
        name_keysym {
            keysym: 0x1000568,
            offset: 2112,
        },
        name_keysym {
            keysym: 0x1000531,
            offset: 2124,
        },
        name_keysym {
            keysym: 0x1000561,
            offset: 2137,
        },
        name_keysym {
            keysym: 0x1000532,
            offset: 2150,
        },
        name_keysym {
            keysym: 0x1000562,
            offset: 2163,
        },
        name_keysym {
            keysym: 0x100055d,
            offset: 2176,
        },
        name_keysym {
            keysym: 0x1000549,
            offset: 2189,
        },
        name_keysym {
            keysym: 0x1000579,
            offset: 2202,
        },
        name_keysym {
            keysym: 0x1000534,
            offset: 2215,
        },
        name_keysym {
            keysym: 0x1000564,
            offset: 2227,
        },
        name_keysym {
            keysym: 0x1000541,
            offset: 2239,
        },
        name_keysym {
            keysym: 0x1000571,
            offset: 2252,
        },
        name_keysym {
            keysym: 0x1000537,
            offset: 2265,
        },
        name_keysym {
            keysym: 0x1000567,
            offset: 2276,
        },
        name_keysym {
            keysym: 0x100055c,
            offset: 2287,
        },
        name_keysym {
            keysym: 0x1000556,
            offset: 2303,
        },
        name_keysym {
            keysym: 0x1000586,
            offset: 2315,
        },
        name_keysym {
            keysym: 0x1000589,
            offset: 2327,
        },
        name_keysym {
            keysym: 0x1000542,
            offset: 2346,
        },
        name_keysym {
            keysym: 0x1000572,
            offset: 2360,
        },
        name_keysym {
            keysym: 0x1000533,
            offset: 2374,
        },
        name_keysym {
            keysym: 0x1000563,
            offset: 2387,
        },
        name_keysym {
            keysym: 0x1000545,
            offset: 2400,
        },
        name_keysym {
            keysym: 0x1000575,
            offset: 2412,
        },
        name_keysym {
            keysym: 0x1000540,
            offset: 2424,
        },
        name_keysym {
            keysym: 0x1000570,
            offset: 2436,
        },
        name_keysym {
            keysym: 0x100058a,
            offset: 2448,
        },
        name_keysym {
            keysym: 0x100053b,
            offset: 2464,
        },
        name_keysym {
            keysym: 0x100056b,
            offset: 2477,
        },
        name_keysym {
            keysym: 0x100054b,
            offset: 2490,
        },
        name_keysym {
            keysym: 0x100057b,
            offset: 2502,
        },
        name_keysym {
            keysym: 0x1000554,
            offset: 2514,
        },
        name_keysym {
            keysym: 0x1000584,
            offset: 2526,
        },
        name_keysym {
            keysym: 0x100053f,
            offset: 2538,
        },
        name_keysym {
            keysym: 0x100056f,
            offset: 2551,
        },
        name_keysym {
            keysym: 0x100053d,
            offset: 2564,
        },
        name_keysym {
            keysym: 0x100056d,
            offset: 2577,
        },
        name_keysym {
            keysym: 0x1000587,
            offset: 2590,
        },
        name_keysym {
            keysym: 0x100053c,
            offset: 2611,
        },
        name_keysym {
            keysym: 0x100056c,
            offset: 2625,
        },
        name_keysym {
            keysym: 0x1000544,
            offset: 2639,
        },
        name_keysym {
            keysym: 0x1000574,
            offset: 2652,
        },
        name_keysym {
            keysym: 0x1000546,
            offset: 2665,
        },
        name_keysym {
            keysym: 0x1000576,
            offset: 2677,
        },
        name_keysym {
            keysym: 0x1000555,
            offset: 2689,
        },
        name_keysym {
            keysym: 0x1000585,
            offset: 2700,
        },
        name_keysym {
            keysym: 0x100055e,
            offset: 2711,
        },
        name_keysym {
            keysym: 0x100054a,
            offset: 2727,
        },
        name_keysym {
            keysym: 0x100057a,
            offset: 2739,
        },
        name_keysym {
            keysym: 0x1000553,
            offset: 2751,
        },
        name_keysym {
            keysym: 0x1000583,
            offset: 2765,
        },
        name_keysym {
            keysym: 0x100055e,
            offset: 2779,
        },
        name_keysym {
            keysym: 0x100054c,
            offset: 2797,
        },
        name_keysym {
            keysym: 0x100057c,
            offset: 2809,
        },
        name_keysym {
            keysym: 0x1000550,
            offset: 2821,
        },
        name_keysym {
            keysym: 0x1000580,
            offset: 2833,
        },
        name_keysym {
            keysym: 0x100054d,
            offset: 2845,
        },
        name_keysym {
            keysym: 0x100057d,
            offset: 2857,
        },
        name_keysym {
            keysym: 0x100055d,
            offset: 2869,
        },
        name_keysym {
            keysym: 0x1000547,
            offset: 2894,
        },
        name_keysym {
            keysym: 0x1000577,
            offset: 2907,
        },
        name_keysym {
            keysym: 0x100055b,
            offset: 2920,
        },
        name_keysym {
            keysym: 0x1000543,
            offset: 2936,
        },
        name_keysym {
            keysym: 0x1000573,
            offset: 2950,
        },
        name_keysym {
            keysym: 0x1000539,
            offset: 2964,
        },
        name_keysym {
            keysym: 0x1000569,
            offset: 2976,
        },
        name_keysym {
            keysym: 0x100053e,
            offset: 2988,
        },
        name_keysym {
            keysym: 0x100056e,
            offset: 3001,
        },
        name_keysym {
            keysym: 0x1000551,
            offset: 3014,
        },
        name_keysym {
            keysym: 0x1000581,
            offset: 3027,
        },
        name_keysym {
            keysym: 0x100054f,
            offset: 3040,
        },
        name_keysym {
            keysym: 0x100057f,
            offset: 3054,
        },
        name_keysym {
            keysym: 0x1000589,
            offset: 3068,
        },
        name_keysym {
            keysym: 0x100054e,
            offset: 3086,
        },
        name_keysym {
            keysym: 0x100057e,
            offset: 3099,
        },
        name_keysym {
            keysym: 0x1000548,
            offset: 3112,
        },
        name_keysym {
            keysym: 0x1000578,
            offset: 3124,
        },
        name_keysym {
            keysym: 0x1000552,
            offset: 3136,
        },
        name_keysym {
            keysym: 0x1000582,
            offset: 3150,
        },
        name_keysym {
            keysym: 0x1000535,
            offset: 3164,
        },
        name_keysym {
            keysym: 0x1000565,
            offset: 3178,
        },
        name_keysym {
            keysym: 0x100058a,
            offset: 3192,
        },
        name_keysym {
            keysym: 0x1000536,
            offset: 3210,
        },
        name_keysym {
            keysym: 0x1000566,
            offset: 3222,
        },
        name_keysym {
            keysym: 0x100053a,
            offset: 3234,
        },
        name_keysym {
            keysym: 0x100056a,
            offset: 3247,
        },
        name_keysym {
            keysym: 0x5e,
            offset: 3260,
        },
        name_keysym {
            keysym: 0x7e,
            offset: 3272,
        },
        name_keysym {
            keysym: 0x2a,
            offset: 3283,
        },
        name_keysym {
            keysym: 0x40,
            offset: 3292,
        },
        name_keysym {
            keysym: 0xc3,
            offset: 3295,
        },
        name_keysym {
            keysym: 0xe3,
            offset: 3302,
        },
        name_keysym {
            keysym: 0xfe7a,
            offset: 3309,
        },
        name_keysym {
            keysym: 0x42,
            offset: 3328,
        },
        name_keysym {
            keysym: 0x62,
            offset: 3330,
        },
        name_keysym {
            keysym: 0x1001e02,
            offset: 3332,
        },
        name_keysym {
            keysym: 0x1001e03,
            offset: 3342,
        },
        name_keysym {
            keysym: 0x5c,
            offset: 3352,
        },
        name_keysym {
            keysym: 0xff08,
            offset: 3362,
        },
        name_keysym {
            keysym: 0x1000ff74,
            offset: 3372,
        },
        name_keysym {
            keysym: 0xaf4,
            offset: 3380,
        },
        name_keysym {
            keysym: 0x7c,
            offset: 3392,
        },
        name_keysym {
            keysym: 0x1002235,
            offset: 3396,
        },
        name_keysym {
            keysym: 0xff58,
            offset: 3404,
        },
        name_keysym {
            keysym: 0x9df,
            offset: 3410,
        },
        name_keysym {
            keysym: 0x100000fc,
            offset: 3416,
        },
        name_keysym {
            keysym: 0x8a5,
            offset: 3422,
        },
        name_keysym {
            keysym: 0x8ac,
            offset: 3434,
        },
        name_keysym {
            keysym: 0x8a8,
            offset: 3448,
        },
        name_keysym {
            keysym: 0x8b2,
            offset: 3465,
        },
        name_keysym {
            keysym: 0x8ae,
            offset: 3482,
        },
        name_keysym {
            keysym: 0x8aa,
            offset: 3497,
        },
        name_keysym {
            keysym: 0x8b6,
            offset: 3515,
        },
        name_keysym {
            keysym: 0x9f6,
            offset: 3533,
        },
        name_keysym {
            keysym: 0x8b4,
            offset: 3538,
        },
        name_keysym {
            keysym: 0xfe74,
            offset: 3564,
        },
        name_keysym {
            keysym: 0x7b,
            offset: 3582,
        },
        name_keysym {
            keysym: 0x7d,
            offset: 3592,
        },
        name_keysym {
            keysym: 0x5b,
            offset: 3603,
        },
        name_keysym {
            keysym: 0x5d,
            offset: 3615,
        },
        name_keysym {
            keysym: 0x1002800,
            offset: 3628,
        },
        name_keysym {
            keysym: 0xfff1,
            offset: 3642,
        },
        name_keysym {
            keysym: 0xfffa,
            offset: 3656,
        },
        name_keysym {
            keysym: 0xfff2,
            offset: 3671,
        },
        name_keysym {
            keysym: 0xfff3,
            offset: 3685,
        },
        name_keysym {
            keysym: 0xfff4,
            offset: 3699,
        },
        name_keysym {
            keysym: 0xfff5,
            offset: 3713,
        },
        name_keysym {
            keysym: 0xfff6,
            offset: 3727,
        },
        name_keysym {
            keysym: 0xfff7,
            offset: 3741,
        },
        name_keysym {
            keysym: 0xfff8,
            offset: 3755,
        },
        name_keysym {
            keysym: 0xfff9,
            offset: 3769,
        },
        name_keysym {
            keysym: 0x1002801,
            offset: 3783,
        },
        name_keysym {
            keysym: 0x1002803,
            offset: 3798,
        },
        name_keysym {
            keysym: 0x1002807,
            offset: 3814,
        },
        name_keysym {
            keysym: 0x100280f,
            offset: 3831,
        },
        name_keysym {
            keysym: 0x100281f,
            offset: 3849,
        },
        name_keysym {
            keysym: 0x100283f,
            offset: 3868,
        },
        name_keysym {
            keysym: 0x100287f,
            offset: 3888,
        },
        name_keysym {
            keysym: 0x10028ff,
            offset: 3909,
        },
        name_keysym {
            keysym: 0x10028bf,
            offset: 3931,
        },
        name_keysym {
            keysym: 0x100285f,
            offset: 3952,
        },
        name_keysym {
            keysym: 0x10028df,
            offset: 3972,
        },
        name_keysym {
            keysym: 0x100289f,
            offset: 3993,
        },
        name_keysym {
            keysym: 0x100282f,
            offset: 4013,
        },
        name_keysym {
            keysym: 0x100286f,
            offset: 4032,
        },
        name_keysym {
            keysym: 0x10028ef,
            offset: 4052,
        },
        name_keysym {
            keysym: 0x10028af,
            offset: 4073,
        },
        name_keysym {
            keysym: 0x100284f,
            offset: 4093,
        },
        name_keysym {
            keysym: 0x10028cf,
            offset: 4112,
        },
        name_keysym {
            keysym: 0x100288f,
            offset: 4132,
        },
        name_keysym {
            keysym: 0x1002817,
            offset: 4151,
        },
        name_keysym {
            keysym: 0x1002837,
            offset: 4169,
        },
        name_keysym {
            keysym: 0x1002877,
            offset: 4188,
        },
        name_keysym {
            keysym: 0x10028f7,
            offset: 4208,
        },
        name_keysym {
            keysym: 0x10028b7,
            offset: 4229,
        },
        name_keysym {
            keysym: 0x1002857,
            offset: 4249,
        },
        name_keysym {
            keysym: 0x10028d7,
            offset: 4268,
        },
        name_keysym {
            keysym: 0x1002897,
            offset: 4288,
        },
        name_keysym {
            keysym: 0x1002827,
            offset: 4307,
        },
        name_keysym {
            keysym: 0x1002867,
            offset: 4325,
        },
        name_keysym {
            keysym: 0x10028e7,
            offset: 4344,
        },
        name_keysym {
            keysym: 0x10028a7,
            offset: 4364,
        },
        name_keysym {
            keysym: 0x1002847,
            offset: 4383,
        },
        name_keysym {
            keysym: 0x10028c7,
            offset: 4401,
        },
        name_keysym {
            keysym: 0x1002887,
            offset: 4420,
        },
        name_keysym {
            keysym: 0x100280b,
            offset: 4438,
        },
        name_keysym {
            keysym: 0x100281b,
            offset: 4455,
        },
        name_keysym {
            keysym: 0x100283b,
            offset: 4473,
        },
        name_keysym {
            keysym: 0x100287b,
            offset: 4492,
        },
        name_keysym {
            keysym: 0x10028fb,
            offset: 4512,
        },
        name_keysym {
            keysym: 0x10028bb,
            offset: 4533,
        },
        name_keysym {
            keysym: 0x100285b,
            offset: 4553,
        },
        name_keysym {
            keysym: 0x10028db,
            offset: 4572,
        },
        name_keysym {
            keysym: 0x100289b,
            offset: 4592,
        },
        name_keysym {
            keysym: 0x100282b,
            offset: 4611,
        },
        name_keysym {
            keysym: 0x100286b,
            offset: 4629,
        },
        name_keysym {
            keysym: 0x10028eb,
            offset: 4648,
        },
        name_keysym {
            keysym: 0x10028ab,
            offset: 4668,
        },
        name_keysym {
            keysym: 0x100284b,
            offset: 4687,
        },
        name_keysym {
            keysym: 0x10028cb,
            offset: 4705,
        },
        name_keysym {
            keysym: 0x100288b,
            offset: 4724,
        },
        name_keysym {
            keysym: 0x1002813,
            offset: 4742,
        },
        name_keysym {
            keysym: 0x1002833,
            offset: 4759,
        },
        name_keysym {
            keysym: 0x1002873,
            offset: 4777,
        },
        name_keysym {
            keysym: 0x10028f3,
            offset: 4796,
        },
        name_keysym {
            keysym: 0x10028b3,
            offset: 4816,
        },
        name_keysym {
            keysym: 0x1002853,
            offset: 4835,
        },
        name_keysym {
            keysym: 0x10028d3,
            offset: 4853,
        },
        name_keysym {
            keysym: 0x1002893,
            offset: 4872,
        },
        name_keysym {
            keysym: 0x1002823,
            offset: 4890,
        },
        name_keysym {
            keysym: 0x1002863,
            offset: 4907,
        },
        name_keysym {
            keysym: 0x10028e3,
            offset: 4925,
        },
        name_keysym {
            keysym: 0x10028a3,
            offset: 4944,
        },
        name_keysym {
            keysym: 0x1002843,
            offset: 4962,
        },
        name_keysym {
            keysym: 0x10028c3,
            offset: 4979,
        },
        name_keysym {
            keysym: 0x1002883,
            offset: 4997,
        },
        name_keysym {
            keysym: 0x1002805,
            offset: 5014,
        },
        name_keysym {
            keysym: 0x100280d,
            offset: 5030,
        },
        name_keysym {
            keysym: 0x100281d,
            offset: 5047,
        },
        name_keysym {
            keysym: 0x100283d,
            offset: 5065,
        },
        name_keysym {
            keysym: 0x100287d,
            offset: 5084,
        },
        name_keysym {
            keysym: 0x10028fd,
            offset: 5104,
        },
        name_keysym {
            keysym: 0x10028bd,
            offset: 5125,
        },
        name_keysym {
            keysym: 0x100285d,
            offset: 5145,
        },
        name_keysym {
            keysym: 0x10028dd,
            offset: 5164,
        },
        name_keysym {
            keysym: 0x100289d,
            offset: 5184,
        },
        name_keysym {
            keysym: 0x100282d,
            offset: 5203,
        },
        name_keysym {
            keysym: 0x100286d,
            offset: 5221,
        },
        name_keysym {
            keysym: 0x10028ed,
            offset: 5240,
        },
        name_keysym {
            keysym: 0x10028ad,
            offset: 5260,
        },
        name_keysym {
            keysym: 0x100284d,
            offset: 5279,
        },
        name_keysym {
            keysym: 0x10028cd,
            offset: 5297,
        },
        name_keysym {
            keysym: 0x100288d,
            offset: 5316,
        },
        name_keysym {
            keysym: 0x1002815,
            offset: 5334,
        },
        name_keysym {
            keysym: 0x1002835,
            offset: 5351,
        },
        name_keysym {
            keysym: 0x1002875,
            offset: 5369,
        },
        name_keysym {
            keysym: 0x10028f5,
            offset: 5388,
        },
        name_keysym {
            keysym: 0x10028b5,
            offset: 5408,
        },
        name_keysym {
            keysym: 0x1002855,
            offset: 5427,
        },
        name_keysym {
            keysym: 0x10028d5,
            offset: 5445,
        },
        name_keysym {
            keysym: 0x1002895,
            offset: 5464,
        },
        name_keysym {
            keysym: 0x1002825,
            offset: 5482,
        },
        name_keysym {
            keysym: 0x1002865,
            offset: 5499,
        },
        name_keysym {
            keysym: 0x10028e5,
            offset: 5517,
        },
        name_keysym {
            keysym: 0x10028a5,
            offset: 5536,
        },
        name_keysym {
            keysym: 0x1002845,
            offset: 5554,
        },
        name_keysym {
            keysym: 0x10028c5,
            offset: 5571,
        },
        name_keysym {
            keysym: 0x1002885,
            offset: 5589,
        },
        name_keysym {
            keysym: 0x1002809,
            offset: 5606,
        },
        name_keysym {
            keysym: 0x1002819,
            offset: 5622,
        },
        name_keysym {
            keysym: 0x1002839,
            offset: 5639,
        },
        name_keysym {
            keysym: 0x1002879,
            offset: 5657,
        },
        name_keysym {
            keysym: 0x10028f9,
            offset: 5676,
        },
        name_keysym {
            keysym: 0x10028b9,
            offset: 5696,
        },
        name_keysym {
            keysym: 0x1002859,
            offset: 5715,
        },
        name_keysym {
            keysym: 0x10028d9,
            offset: 5733,
        },
        name_keysym {
            keysym: 0x1002899,
            offset: 5752,
        },
        name_keysym {
            keysym: 0x1002829,
            offset: 5770,
        },
        name_keysym {
            keysym: 0x1002869,
            offset: 5787,
        },
        name_keysym {
            keysym: 0x10028e9,
            offset: 5805,
        },
        name_keysym {
            keysym: 0x10028a9,
            offset: 5824,
        },
        name_keysym {
            keysym: 0x1002849,
            offset: 5842,
        },
        name_keysym {
            keysym: 0x10028c9,
            offset: 5859,
        },
        name_keysym {
            keysym: 0x1002889,
            offset: 5877,
        },
        name_keysym {
            keysym: 0x1002811,
            offset: 5894,
        },
        name_keysym {
            keysym: 0x1002831,
            offset: 5910,
        },
        name_keysym {
            keysym: 0x1002871,
            offset: 5927,
        },
        name_keysym {
            keysym: 0x10028f1,
            offset: 5945,
        },
        name_keysym {
            keysym: 0x10028b1,
            offset: 5964,
        },
        name_keysym {
            keysym: 0x1002851,
            offset: 5982,
        },
        name_keysym {
            keysym: 0x10028d1,
            offset: 5999,
        },
        name_keysym {
            keysym: 0x1002891,
            offset: 6017,
        },
        name_keysym {
            keysym: 0x1002821,
            offset: 6034,
        },
        name_keysym {
            keysym: 0x1002861,
            offset: 6050,
        },
        name_keysym {
            keysym: 0x10028e1,
            offset: 6067,
        },
        name_keysym {
            keysym: 0x10028a1,
            offset: 6085,
        },
        name_keysym {
            keysym: 0x1002841,
            offset: 6102,
        },
        name_keysym {
            keysym: 0x10028c1,
            offset: 6118,
        },
        name_keysym {
            keysym: 0x1002881,
            offset: 6135,
        },
        name_keysym {
            keysym: 0x1002802,
            offset: 6151,
        },
        name_keysym {
            keysym: 0x1002806,
            offset: 6166,
        },
        name_keysym {
            keysym: 0x100280e,
            offset: 6182,
        },
        name_keysym {
            keysym: 0x100281e,
            offset: 6199,
        },
        name_keysym {
            keysym: 0x100283e,
            offset: 6217,
        },
        name_keysym {
            keysym: 0x100287e,
            offset: 6236,
        },
        name_keysym {
            keysym: 0x10028fe,
            offset: 6256,
        },
        name_keysym {
            keysym: 0x10028be,
            offset: 6277,
        },
        name_keysym {
            keysym: 0x100285e,
            offset: 6297,
        },
        name_keysym {
            keysym: 0x10028de,
            offset: 6316,
        },
        name_keysym {
            keysym: 0x100289e,
            offset: 6336,
        },
        name_keysym {
            keysym: 0x100282e,
            offset: 6355,
        },
        name_keysym {
            keysym: 0x100286e,
            offset: 6373,
        },
        name_keysym {
            keysym: 0x10028ee,
            offset: 6392,
        },
        name_keysym {
            keysym: 0x10028ae,
            offset: 6412,
        },
        name_keysym {
            keysym: 0x100284e,
            offset: 6431,
        },
        name_keysym {
            keysym: 0x10028ce,
            offset: 6449,
        },
        name_keysym {
            keysym: 0x100288e,
            offset: 6468,
        },
        name_keysym {
            keysym: 0x1002816,
            offset: 6486,
        },
        name_keysym {
            keysym: 0x1002836,
            offset: 6503,
        },
        name_keysym {
            keysym: 0x1002876,
            offset: 6521,
        },
        name_keysym {
            keysym: 0x10028f6,
            offset: 6540,
        },
        name_keysym {
            keysym: 0x10028b6,
            offset: 6560,
        },
        name_keysym {
            keysym: 0x1002856,
            offset: 6579,
        },
        name_keysym {
            keysym: 0x10028d6,
            offset: 6597,
        },
        name_keysym {
            keysym: 0x1002896,
            offset: 6616,
        },
        name_keysym {
            keysym: 0x1002826,
            offset: 6634,
        },
        name_keysym {
            keysym: 0x1002866,
            offset: 6651,
        },
        name_keysym {
            keysym: 0x10028e6,
            offset: 6669,
        },
        name_keysym {
            keysym: 0x10028a6,
            offset: 6688,
        },
        name_keysym {
            keysym: 0x1002846,
            offset: 6706,
        },
        name_keysym {
            keysym: 0x10028c6,
            offset: 6723,
        },
        name_keysym {
            keysym: 0x1002886,
            offset: 6741,
        },
        name_keysym {
            keysym: 0x100280a,
            offset: 6758,
        },
        name_keysym {
            keysym: 0x100281a,
            offset: 6774,
        },
        name_keysym {
            keysym: 0x100283a,
            offset: 6791,
        },
        name_keysym {
            keysym: 0x100287a,
            offset: 6809,
        },
        name_keysym {
            keysym: 0x10028fa,
            offset: 6828,
        },
        name_keysym {
            keysym: 0x10028ba,
            offset: 6848,
        },
        name_keysym {
            keysym: 0x100285a,
            offset: 6867,
        },
        name_keysym {
            keysym: 0x10028da,
            offset: 6885,
        },
        name_keysym {
            keysym: 0x100289a,
            offset: 6904,
        },
        name_keysym {
            keysym: 0x100282a,
            offset: 6922,
        },
        name_keysym {
            keysym: 0x100286a,
            offset: 6939,
        },
        name_keysym {
            keysym: 0x10028ea,
            offset: 6957,
        },
        name_keysym {
            keysym: 0x10028aa,
            offset: 6976,
        },
        name_keysym {
            keysym: 0x100284a,
            offset: 6994,
        },
        name_keysym {
            keysym: 0x10028ca,
            offset: 7011,
        },
        name_keysym {
            keysym: 0x100288a,
            offset: 7029,
        },
        name_keysym {
            keysym: 0x1002812,
            offset: 7046,
        },
        name_keysym {
            keysym: 0x1002832,
            offset: 7062,
        },
        name_keysym {
            keysym: 0x1002872,
            offset: 7079,
        },
        name_keysym {
            keysym: 0x10028f2,
            offset: 7097,
        },
        name_keysym {
            keysym: 0x10028b2,
            offset: 7116,
        },
        name_keysym {
            keysym: 0x1002852,
            offset: 7134,
        },
        name_keysym {
            keysym: 0x10028d2,
            offset: 7151,
        },
        name_keysym {
            keysym: 0x1002892,
            offset: 7169,
        },
        name_keysym {
            keysym: 0x1002822,
            offset: 7186,
        },
        name_keysym {
            keysym: 0x1002862,
            offset: 7202,
        },
        name_keysym {
            keysym: 0x10028e2,
            offset: 7219,
        },
        name_keysym {
            keysym: 0x10028a2,
            offset: 7237,
        },
        name_keysym {
            keysym: 0x1002842,
            offset: 7254,
        },
        name_keysym {
            keysym: 0x10028c2,
            offset: 7270,
        },
        name_keysym {
            keysym: 0x1002882,
            offset: 7287,
        },
        name_keysym {
            keysym: 0x1002804,
            offset: 7303,
        },
        name_keysym {
            keysym: 0x100280c,
            offset: 7318,
        },
        name_keysym {
            keysym: 0x100281c,
            offset: 7334,
        },
        name_keysym {
            keysym: 0x100283c,
            offset: 7351,
        },
        name_keysym {
            keysym: 0x100287c,
            offset: 7369,
        },
        name_keysym {
            keysym: 0x10028fc,
            offset: 7388,
        },
        name_keysym {
            keysym: 0x10028bc,
            offset: 7408,
        },
        name_keysym {
            keysym: 0x100285c,
            offset: 7427,
        },
        name_keysym {
            keysym: 0x10028dc,
            offset: 7445,
        },
        name_keysym {
            keysym: 0x100289c,
            offset: 7464,
        },
        name_keysym {
            keysym: 0x100282c,
            offset: 7482,
        },
        name_keysym {
            keysym: 0x100286c,
            offset: 7499,
        },
        name_keysym {
            keysym: 0x10028ec,
            offset: 7517,
        },
        name_keysym {
            keysym: 0x10028ac,
            offset: 7536,
        },
        name_keysym {
            keysym: 0x100284c,
            offset: 7554,
        },
        name_keysym {
            keysym: 0x10028cc,
            offset: 7571,
        },
        name_keysym {
            keysym: 0x100288c,
            offset: 7589,
        },
        name_keysym {
            keysym: 0x1002814,
            offset: 7606,
        },
        name_keysym {
            keysym: 0x1002834,
            offset: 7622,
        },
        name_keysym {
            keysym: 0x1002874,
            offset: 7639,
        },
        name_keysym {
            keysym: 0x10028f4,
            offset: 7657,
        },
        name_keysym {
            keysym: 0x10028b4,
            offset: 7676,
        },
        name_keysym {
            keysym: 0x1002854,
            offset: 7694,
        },
        name_keysym {
            keysym: 0x10028d4,
            offset: 7711,
        },
        name_keysym {
            keysym: 0x1002894,
            offset: 7729,
        },
        name_keysym {
            keysym: 0x1002824,
            offset: 7746,
        },
        name_keysym {
            keysym: 0x1002864,
            offset: 7762,
        },
        name_keysym {
            keysym: 0x10028e4,
            offset: 7779,
        },
        name_keysym {
            keysym: 0x10028a4,
            offset: 7797,
        },
        name_keysym {
            keysym: 0x1002844,
            offset: 7814,
        },
        name_keysym {
            keysym: 0x10028c4,
            offset: 7830,
        },
        name_keysym {
            keysym: 0x1002884,
            offset: 7847,
        },
        name_keysym {
            keysym: 0x1002808,
            offset: 7863,
        },
        name_keysym {
            keysym: 0x1002818,
            offset: 7878,
        },
        name_keysym {
            keysym: 0x1002838,
            offset: 7894,
        },
        name_keysym {
            keysym: 0x1002878,
            offset: 7911,
        },
        name_keysym {
            keysym: 0x10028f8,
            offset: 7929,
        },
        name_keysym {
            keysym: 0x10028b8,
            offset: 7948,
        },
        name_keysym {
            keysym: 0x1002858,
            offset: 7966,
        },
        name_keysym {
            keysym: 0x10028d8,
            offset: 7983,
        },
        name_keysym {
            keysym: 0x1002898,
            offset: 8001,
        },
        name_keysym {
            keysym: 0x1002828,
            offset: 8018,
        },
        name_keysym {
            keysym: 0x1002868,
            offset: 8034,
        },
        name_keysym {
            keysym: 0x10028e8,
            offset: 8051,
        },
        name_keysym {
            keysym: 0x10028a8,
            offset: 8069,
        },
        name_keysym {
            keysym: 0x1002848,
            offset: 8086,
        },
        name_keysym {
            keysym: 0x10028c8,
            offset: 8102,
        },
        name_keysym {
            keysym: 0x1002888,
            offset: 8119,
        },
        name_keysym {
            keysym: 0x1002810,
            offset: 8135,
        },
        name_keysym {
            keysym: 0x1002830,
            offset: 8150,
        },
        name_keysym {
            keysym: 0x1002870,
            offset: 8166,
        },
        name_keysym {
            keysym: 0x10028f0,
            offset: 8183,
        },
        name_keysym {
            keysym: 0x10028b0,
            offset: 8201,
        },
        name_keysym {
            keysym: 0x1002850,
            offset: 8218,
        },
        name_keysym {
            keysym: 0x10028d0,
            offset: 8234,
        },
        name_keysym {
            keysym: 0x1002890,
            offset: 8251,
        },
        name_keysym {
            keysym: 0x1002820,
            offset: 8267,
        },
        name_keysym {
            keysym: 0x1002860,
            offset: 8282,
        },
        name_keysym {
            keysym: 0x10028e0,
            offset: 8298,
        },
        name_keysym {
            keysym: 0x10028a0,
            offset: 8315,
        },
        name_keysym {
            keysym: 0x1002840,
            offset: 8331,
        },
        name_keysym {
            keysym: 0x10028c0,
            offset: 8346,
        },
        name_keysym {
            keysym: 0x1002880,
            offset: 8362,
        },
        name_keysym {
            keysym: 0xff6b,
            offset: 8377,
        },
        name_keysym {
            keysym: 0x1a2,
            offset: 8383,
        },
        name_keysym {
            keysym: 0xa6,
            offset: 8389,
        },
        name_keysym {
            keysym: 0x6be,
            offset: 8399,
        },
        name_keysym {
            keysym: 0x6ae,
            offset: 8419,
        },
        name_keysym {
            keysym: 0x43,
            offset: 8439,
        },
        name_keysym {
            keysym: 0x63,
            offset: 8441,
        },
        name_keysym {
            keysym: 0xfea5,
            offset: 8443,
        },
        name_keysym {
            keysym: 0xfea4,
            offset: 8447,
        },
        name_keysym {
            keysym: 0xfea3,
            offset: 8451,
        },
        name_keysym {
            keysym: 0x2c5,
            offset: 8455,
        },
        name_keysym {
            keysym: 0x2e5,
            offset: 8465,
        },
        name_keysym {
            keysym: 0x1c6,
            offset: 8475,
        },
        name_keysym {
            keysym: 0x1e6,
            offset: 8482,
        },
        name_keysym {
            keysym: 0xff69,
            offset: 8489,
        },
        name_keysym {
            keysym: 0xffe5,
            offset: 8496,
        },
        name_keysym {
            keysym: 0xab8,
            offset: 8506,
        },
        name_keysym {
            keysym: 0xafc,
            offset: 8513,
        },
        name_keysym {
            keysym: 0x1b7,
            offset: 8519,
        },
        name_keysym {
            keysym: 0x1c8,
            offset: 8525,
        },
        name_keysym {
            keysym: 0x1e8,
            offset: 8532,
        },
        name_keysym {
            keysym: 0xc7,
            offset: 8539,
        },
        name_keysym {
            keysym: 0xe7,
            offset: 8548,
        },
        name_keysym {
            keysym: 0x2c6,
            offset: 8557,
        },
        name_keysym {
            keysym: 0x2e6,
            offset: 8569,
        },
        name_keysym {
            keysym: 0xb8,
            offset: 8581,
        },
        name_keysym {
            keysym: 0xa2,
            offset: 8589,
        },
        name_keysym {
            keysym: 0xfea2,
            offset: 8594,
        },
        name_keysym {
            keysym: 0xfea1,
            offset: 8597,
        },
        name_keysym {
            keysym: 0xfea0,
            offset: 8600,
        },
        name_keysym {
            keysym: 0x9e1,
            offset: 8603,
        },
        name_keysym {
            keysym: 0xaf3,
            offset: 8616,
        },
        name_keysym {
            keysym: 0xbcf,
            offset: 8626,
        },
        name_keysym {
            keysym: 0xff0b,
            offset: 8633,
        },
        name_keysym {
            keysym: 0x1000ff6f,
            offset: 8639,
        },
        name_keysym {
            keysym: 0xaec,
            offset: 8649,
        },
        name_keysym {
            keysym: 0xff37,
            offset: 8654,
        },
        name_keysym {
            keysym: 0x3a,
            offset: 8664,
        },
        name_keysym {
            keysym: 0x10020a1,
            offset: 8670,
        },
        name_keysym {
            keysym: 0x1000301,
            offset: 8680,
        },
        name_keysym {
            keysym: 0x1000323,
            offset: 8696,
        },
        name_keysym {
            keysym: 0x1000300,
            offset: 8715,
        },
        name_keysym {
            keysym: 0x1000309,
            offset: 8731,
        },
        name_keysym {
            keysym: 0x1000303,
            offset: 8746,
        },
        name_keysym {
            keysym: 0x2c,
            offset: 8762,
        },
        name_keysym {
            keysym: 0x100220b,
            offset: 8768,
        },
        name_keysym {
            keysym: 0xffe3,
            offset: 8779,
        },
        name_keysym {
            keysym: 0xffe4,
            offset: 8789,
        },
        name_keysym {
            keysym: 0xa9,
            offset: 8799,
        },
        name_keysym {
            keysym: 0x9e4,
            offset: 8809,
        },
        name_keysym {
            keysym: 0x9ee,
            offset: 8812,
        },
        name_keysym {
            keysym: 0x10020a2,
            offset: 8826,
        },
        name_keysym {
            keysym: 0x100221b,
            offset: 8839,
        },
        name_keysym {
            keysym: 0xa4,
            offset: 8848,
        },
        name_keysym {
            keysym: 0xaff,
            offset: 8857,
        },
        name_keysym {
            keysym: 0x6e1,
            offset: 8864,
        },
        name_keysym {
            keysym: 0x6c1,
            offset: 8875,
        },
        name_keysym {
            keysym: 0x6e2,
            offset: 8886,
        },
        name_keysym {
            keysym: 0x6c2,
            offset: 8898,
        },
        name_keysym {
            keysym: 0x6fe,
            offset: 8910,
        },
        name_keysym {
            keysym: 0x6de,
            offset: 8923,
        },
        name_keysym {
            keysym: 0x10004b6,
            offset: 8936,
        },
        name_keysym {
            keysym: 0x10004b7,
            offset: 8959,
        },
        name_keysym {
            keysym: 0x10004b8,
            offset: 8982,
        },
        name_keysym {
            keysym: 0x10004b9,
            offset: 9006,
        },
        name_keysym {
            keysym: 0x6e4,
            offset: 9030,
        },
        name_keysym {
            keysym: 0x6c4,
            offset: 9042,
        },
        name_keysym {
            keysym: 0x6bf,
            offset: 9054,
        },
        name_keysym {
            keysym: 0x6af,
            offset: 9068,
        },
        name_keysym {
            keysym: 0x6fc,
            offset: 9082,
        },
        name_keysym {
            keysym: 0x6dc,
            offset: 9093,
        },
        name_keysym {
            keysym: 0x6e6,
            offset: 9104,
        },
        name_keysym {
            keysym: 0x6c6,
            offset: 9116,
        },
        name_keysym {
            keysym: 0x6ec,
            offset: 9128,
        },
        name_keysym {
            keysym: 0x6cc,
            offset: 9140,
        },
        name_keysym {
            keysym: 0x6ed,
            offset: 9152,
        },
        name_keysym {
            keysym: 0x6cd,
            offset: 9164,
        },
        name_keysym {
            keysym: 0x6ee,
            offset: 9176,
        },
        name_keysym {
            keysym: 0x6ce,
            offset: 9188,
        },
        name_keysym {
            keysym: 0x10004a2,
            offset: 9200,
        },
        name_keysym {
            keysym: 0x10004a3,
            offset: 9222,
        },
        name_keysym {
            keysym: 0x6f2,
            offset: 9244,
        },
        name_keysym {
            keysym: 0x6d2,
            offset: 9256,
        },
        name_keysym {
            keysym: 0x6f3,
            offset: 9268,
        },
        name_keysym {
            keysym: 0x6d3,
            offset: 9280,
        },
        name_keysym {
            keysym: 0x6e7,
            offset: 9292,
        },
        name_keysym {
            keysym: 0x6c7,
            offset: 9305,
        },
        name_keysym {
            keysym: 0x1000492,
            offset: 9318,
        },
        name_keysym {
            keysym: 0x1000493,
            offset: 9335,
        },
        name_keysym {
            keysym: 0x6e8,
            offset: 9352,
        },
        name_keysym {
            keysym: 0x6c8,
            offset: 9364,
        },
        name_keysym {
            keysym: 0x10004b2,
            offset: 9376,
        },
        name_keysym {
            keysym: 0x10004b3,
            offset: 9398,
        },
        name_keysym {
            keysym: 0x6ff,
            offset: 9420,
        },
        name_keysym {
            keysym: 0x6df,
            offset: 9438,
        },
        name_keysym {
            keysym: 0x6e9,
            offset: 9456,
        },
        name_keysym {
            keysym: 0x6c9,
            offset: 9467,
        },
        name_keysym {
            keysym: 0x10004e2,
            offset: 9478,
        },
        name_keysym {
            keysym: 0x10004e3,
            offset: 9496,
        },
        name_keysym {
            keysym: 0x6e5,
            offset: 9514,
        },
        name_keysym {
            keysym: 0x6c5,
            offset: 9526,
        },
        name_keysym {
            keysym: 0x6b3,
            offset: 9538,
        },
        name_keysym {
            keysym: 0x6a3,
            offset: 9550,
        },
        name_keysym {
            keysym: 0x6b8,
            offset: 9562,
        },
        name_keysym {
            keysym: 0x6a8,
            offset: 9574,
        },
        name_keysym {
            keysym: 0x6eb,
            offset: 9586,
        },
        name_keysym {
            keysym: 0x6cb,
            offset: 9598,
        },
        name_keysym {
            keysym: 0x100049a,
            offset: 9610,
        },
        name_keysym {
            keysym: 0x100049b,
            offset: 9632,
        },
        name_keysym {
            keysym: 0x100049c,
            offset: 9654,
        },
        name_keysym {
            keysym: 0x100049d,
            offset: 9677,
        },
        name_keysym {
            keysym: 0x6b9,
            offset: 9700,
        },
        name_keysym {
            keysym: 0x6a9,
            offset: 9713,
        },
        name_keysym {
            keysym: 0x6ba,
            offset: 9726,
        },
        name_keysym {
            keysym: 0x6aa,
            offset: 9739,
        },
        name_keysym {
            keysym: 0x6ef,
            offset: 9752,
        },
        name_keysym {
            keysym: 0x6cf,
            offset: 9763,
        },
        name_keysym {
            keysym: 0x10004e8,
            offset: 9774,
        },
        name_keysym {
            keysym: 0x10004e9,
            offset: 9789,
        },
        name_keysym {
            keysym: 0x6f0,
            offset: 9804,
        },
        name_keysym {
            keysym: 0x6d0,
            offset: 9816,
        },
        name_keysym {
            keysym: 0x10004d8,
            offset: 9828,
        },
        name_keysym {
            keysym: 0x10004d9,
            offset: 9843,
        },
        name_keysym {
            keysym: 0x6fb,
            offset: 9858,
        },
        name_keysym {
            keysym: 0x6db,
            offset: 9871,
        },
        name_keysym {
            keysym: 0x6fd,
            offset: 9884,
        },
        name_keysym {
            keysym: 0x6dd,
            offset: 9899,
        },
        name_keysym {
            keysym: 0x10004ba,
            offset: 9914,
        },
        name_keysym {
            keysym: 0x10004bb,
            offset: 9928,
        },
        name_keysym {
            keysym: 0x6ea,
            offset: 9942,
        },
        name_keysym {
            keysym: 0x6ca,
            offset: 9958,
        },
        name_keysym {
            keysym: 0x6f8,
            offset: 9974,
        },
        name_keysym {
            keysym: 0x6d8,
            offset: 9992,
        },
        name_keysym {
            keysym: 0x6f4,
            offset: 10010,
        },
        name_keysym {
            keysym: 0x6d4,
            offset: 10022,
        },
        name_keysym {
            keysym: 0x6e3,
            offset: 10034,
        },
        name_keysym {
            keysym: 0x6c3,
            offset: 10047,
        },
        name_keysym {
            keysym: 0x6f5,
            offset: 10060,
        },
        name_keysym {
            keysym: 0x6d5,
            offset: 10071,
        },
        name_keysym {
            keysym: 0x10004ee,
            offset: 10082,
        },
        name_keysym {
            keysym: 0x10004ef,
            offset: 10100,
        },
        name_keysym {
            keysym: 0x10004ae,
            offset: 10118,
        },
        name_keysym {
            keysym: 0x10004af,
            offset: 10138,
        },
        name_keysym {
            keysym: 0x10004b0,
            offset: 10158,
        },
        name_keysym {
            keysym: 0x10004b1,
            offset: 10182,
        },
        name_keysym {
            keysym: 0x6f7,
            offset: 10206,
        },
        name_keysym {
            keysym: 0x6d7,
            offset: 10218,
        },
        name_keysym {
            keysym: 0x6f1,
            offset: 10230,
        },
        name_keysym {
            keysym: 0x6d1,
            offset: 10242,
        },
        name_keysym {
            keysym: 0x6f9,
            offset: 10254,
        },
        name_keysym {
            keysym: 0x6d9,
            offset: 10268,
        },
        name_keysym {
            keysym: 0x6e0,
            offset: 10282,
        },
        name_keysym {
            keysym: 0x6c0,
            offset: 10294,
        },
        name_keysym {
            keysym: 0x6fa,
            offset: 10306,
        },
        name_keysym {
            keysym: 0x6da,
            offset: 10318,
        },
        name_keysym {
            keysym: 0x6f6,
            offset: 10330,
        },
        name_keysym {
            keysym: 0x6d6,
            offset: 10343,
        },
        name_keysym {
            keysym: 0x1000496,
            offset: 10356,
        },
        name_keysym {
            keysym: 0x1000497,
            offset: 10379,
        },
        name_keysym {
            keysym: 0x44,
            offset: 10402,
        },
        name_keysym {
            keysym: 0x64,
            offset: 10404,
        },
        name_keysym {
            keysym: 0x1001e0a,
            offset: 10406,
        },
        name_keysym {
            keysym: 0x1001e0b,
            offset: 10416,
        },
        name_keysym {
            keysym: 0x1000fe27,
            offset: 10426,
        },
        name_keysym {
            keysym: 0xaf1,
            offset: 10440,
        },
        name_keysym {
            keysym: 0x1cf,
            offset: 10447,
        },
        name_keysym {
            keysym: 0x1ef,
            offset: 10454,
        },
        name_keysym {
            keysym: 0x1000fe2c,
            offset: 10461,
        },
        name_keysym {
            keysym: 0x1000fe5e,
            offset: 10477,
        },
        name_keysym {
            keysym: 0x1000fe22,
            offset: 10496,
        },
        name_keysym {
            keysym: 0xfe81,
            offset: 10507,
        },
        name_keysym {
            keysym: 0xfe80,
            offset: 10514,
        },
        name_keysym {
            keysym: 0xfe64,
            offset: 10521,
        },
        name_keysym {
            keysym: 0xfe56,
            offset: 10537,
        },
        name_keysym {
            keysym: 0xfe65,
            offset: 10551,
        },
        name_keysym {
            keysym: 0xfe58,
            offset: 10575,
        },
        name_keysym {
            keysym: 0xfe91,
            offset: 10590,
        },
        name_keysym {
            keysym: 0xfe51,
            offset: 10613,
        },
        name_keysym {
            keysym: 0xfe8e,
            offset: 10624,
        },
        name_keysym {
            keysym: 0xfe6b,
            offset: 10640,
        },
        name_keysym {
            keysym: 0xfe69,
            offset: 10656,
        },
        name_keysym {
            keysym: 0xfe6e,
            offset: 10677,
        },
        name_keysym {
            keysym: 0xfe6c,
            offset: 10693,
        },
        name_keysym {
            keysym: 0xfe60,
            offset: 10713,
        },
        name_keysym {
            keysym: 0xfe68,
            offset: 10727,
        },
        name_keysym {
            keysym: 0xfe67,
            offset: 10744,
        },
        name_keysym {
            keysym: 0xfe6a,
            offset: 10759,
        },
        name_keysym {
            keysym: 0xfe92,
            offset: 10775,
        },
        name_keysym {
            keysym: 0xfe55,
            offset: 10798,
        },
        name_keysym {
            keysym: 0xfe8b,
            offset: 10809,
        },
        name_keysym {
            keysym: 0xfe5a,
            offset: 10828,
        },
        name_keysym {
            keysym: 0xfe5b,
            offset: 10839,
        },
        name_keysym {
            keysym: 0xfe52,
            offset: 10852,
        },
        name_keysym {
            keysym: 0xfe6f,
            offset: 10868,
        },
        name_keysym {
            keysym: 0xfe65,
            offset: 10882,
        },
        name_keysym {
            keysym: 0xfe57,
            offset: 10893,
        },
        name_keysym {
            keysym: 0xfe59,
            offset: 10908,
        },
        name_keysym {
            keysym: 0xfe66,
            offset: 10925,
        },
        name_keysym {
            keysym: 0xfe83,
            offset: 10942,
        },
        name_keysym {
            keysym: 0xfe82,
            offset: 10949,
        },
        name_keysym {
            keysym: 0xfe50,
            offset: 10956,
        },
        name_keysym {
            keysym: 0xfe8c,
            offset: 10967,
        },
        name_keysym {
            keysym: 0xfe8d,
            offset: 10978,
        },
        name_keysym {
            keysym: 0xfe61,
            offset: 10989,
        },
        name_keysym {
            keysym: 0xfe62,
            offset: 10999,
        },
        name_keysym {
            keysym: 0xfe85,
            offset: 11009,
        },
        name_keysym {
            keysym: 0xfe84,
            offset: 11016,
        },
        name_keysym {
            keysym: 0xfe6d,
            offset: 11023,
        },
        name_keysym {
            keysym: 0xfe5d,
            offset: 11042,
        },
        name_keysym {
            keysym: 0xfe93,
            offset: 11052,
        },
        name_keysym {
            keysym: 0xfe90,
            offset: 11076,
        },
        name_keysym {
            keysym: 0xfe54,
            offset: 11089,
        },
        name_keysym {
            keysym: 0xfe87,
            offset: 11101,
        },
        name_keysym {
            keysym: 0xfe86,
            offset: 11108,
        },
        name_keysym {
            keysym: 0xfe5c,
            offset: 11115,
        },
        name_keysym {
            keysym: 0xfe53,
            offset: 11127,
        },
        name_keysym {
            keysym: 0xfe64,
            offset: 11144,
        },
        name_keysym {
            keysym: 0xfe8b,
            offset: 11155,
        },
        name_keysym {
            keysym: 0xfe8a,
            offset: 11166,
        },
        name_keysym {
            keysym: 0xfe5f,
            offset: 11177,
        },
        name_keysym {
            keysym: 0xfe8a,
            offset: 11199,
        },
        name_keysym {
            keysym: 0xfe63,
            offset: 11216,
        },
        name_keysym {
            keysym: 0xfe53,
            offset: 11228,
        },
        name_keysym {
            keysym: 0xfe89,
            offset: 11239,
        },
        name_keysym {
            keysym: 0xfe88,
            offset: 11246,
        },
        name_keysym {
            keysym: 0xfe5e,
            offset: 11253,
        },
        name_keysym {
            keysym: 0xabd,
            offset: 11271,
        },
        name_keysym {
            keysym: 0xb0,
            offset: 11284,
        },
        name_keysym {
            keysym: 0xffff,
            offset: 11291,
        },
        name_keysym {
            keysym: 0x1000ff73,
            offset: 11298,
        },
        name_keysym {
            keysym: 0x1000ff71,
            offset: 11309,
        },
        name_keysym {
            keysym: 0x1000fe60,
            offset: 11320,
        },
        name_keysym {
            keysym: 0xa8,
            offset: 11334,
        },
        name_keysym {
            keysym: 0xaed,
            offset: 11344,
        },
        name_keysym {
            keysym: 0xaa5,
            offset: 11352,
        },
        name_keysym {
            keysym: 0x100222c,
            offset: 11363,
        },
        name_keysym {
            keysym: 0xf7,
            offset: 11373,
        },
        name_keysym {
            keysym: 0x24,
            offset: 11382,
        },
        name_keysym {
            keysym: 0x10020ab,
            offset: 11389,
        },
        name_keysym {
            keysym: 0xaaf,
            offset: 11398,
        },
        name_keysym {
            keysym: 0x1bd,
            offset: 11414,
        },
        name_keysym {
            keysym: 0xaf2,
            offset: 11426,
        },
        name_keysym {
            keysym: 0xafe,
            offset: 11439,
        },
        name_keysym {
            keysym: 0xff54,
            offset: 11458,
        },
        name_keysym {
            keysym: 0x8fe,
            offset: 11463,
        },
        name_keysym {
            keysym: 0xba8,
            offset: 11473,
        },
        name_keysym {
            keysym: 0xbd6,
            offset: 11483,
        },
        name_keysym {
            keysym: 0xbc4,
            offset: 11492,
        },
        name_keysym {
            keysym: 0xbc2,
            offset: 11502,
        },
        name_keysym {
            keysym: 0x1000ff00,
            offset: 11511,
        },
        name_keysym {
            keysym: 0x1000feb0,
            offset: 11519,
        },
        name_keysym {
            keysym: 0x1d0,
            offset: 11532,
        },
        name_keysym {
            keysym: 0x1f0,
            offset: 11540,
        },
        name_keysym {
            keysym: 0x1000fe7e,
            offset: 11548,
        },
        name_keysym {
            keysym: 0x45,
            offset: 11555,
        },
        name_keysym {
            keysym: 0x65,
            offset: 11557,
        },
        name_keysym {
            keysym: 0x3cc,
            offset: 11559,
        },
        name_keysym {
            keysym: 0x3ec,
            offset: 11569,
        },
        name_keysym {
            keysym: 0xc9,
            offset: 11579,
        },
        name_keysym {
            keysym: 0xe9,
            offset: 11586,
        },
        name_keysym {
            keysym: 0x1001eb8,
            offset: 11593,
        },
        name_keysym {
            keysym: 0x1001eb9,
            offset: 11603,
        },
        name_keysym {
            keysym: 0x1cc,
            offset: 11613,
        },
        name_keysym {
            keysym: 0x1ec,
            offset: 11620,
        },
        name_keysym {
            keysym: 0xca,
            offset: 11627,
        },
        name_keysym {
            keysym: 0xea,
            offset: 11639,
        },
        name_keysym {
            keysym: 0x1001ebe,
            offset: 11651,
        },
        name_keysym {
            keysym: 0x1001ebf,
            offset: 11668,
        },
        name_keysym {
            keysym: 0x1001ec6,
            offset: 11685,
        },
        name_keysym {
            keysym: 0x1001ec7,
            offset: 11705,
        },
        name_keysym {
            keysym: 0x1001ec0,
            offset: 11725,
        },
        name_keysym {
            keysym: 0x1001ec1,
            offset: 11742,
        },
        name_keysym {
            keysym: 0x1001ec2,
            offset: 11759,
        },
        name_keysym {
            keysym: 0x1001ec3,
            offset: 11775,
        },
        name_keysym {
            keysym: 0x1001ec4,
            offset: 11791,
        },
        name_keysym {
            keysym: 0x1001ec5,
            offset: 11808,
        },
        name_keysym {
            keysym: 0x10020a0,
            offset: 11825,
        },
        name_keysym {
            keysym: 0xcb,
            offset: 11833,
        },
        name_keysym {
            keysym: 0xeb,
            offset: 11844,
        },
        name_keysym {
            keysym: 0xc8,
            offset: 11855,
        },
        name_keysym {
            keysym: 0xe8,
            offset: 11862,
        },
        name_keysym {
            keysym: 0x1001eba,
            offset: 11869,
        },
        name_keysym {
            keysym: 0x1001ebb,
            offset: 11875,
        },
        name_keysym {
            keysym: 0x1002088,
            offset: 11881,
        },
        name_keysym {
            keysym: 0x1002078,
            offset: 11896,
        },
        name_keysym {
            keysym: 0xff2f,
            offset: 11910,
        },
        name_keysym {
            keysym: 0xff30,
            offset: 11921,
        },
        name_keysym {
            keysym: 0x1002208,
            offset: 11933,
        },
        name_keysym {
            keysym: 0xaae,
            offset: 11943,
        },
        name_keysym {
            keysym: 0xaa3,
            offset: 11952,
        },
        name_keysym {
            keysym: 0xaa4,
            offset: 11961,
        },
        name_keysym {
            keysym: 0x3aa,
            offset: 11970,
        },
        name_keysym {
            keysym: 0x3ba,
            offset: 11978,
        },
        name_keysym {
            keysym: 0xaa9,
            offset: 11986,
        },
        name_keysym {
            keysym: 0xade,
            offset: 11993,
        },
        name_keysym {
            keysym: 0xadf,
            offset: 12008,
        },
        name_keysym {
            keysym: 0xace,
            offset: 12021,
        },
        name_keysym {
            keysym: 0xacf,
            offset: 12034,
        },
        name_keysym {
            keysym: 0x1002205,
            offset: 12050,
        },
        name_keysym {
            keysym: 0xaa1,
            offset: 12059,
        },
        name_keysym {
            keysym: 0xff57,
            offset: 12067,
        },
        name_keysym {
            keysym: 0xaaa,
            offset: 12071,
        },
        name_keysym {
            keysym: 0xae6,
            offset: 12078,
        },
        name_keysym {
            keysym: 0xae7,
            offset: 12097,
        },
        name_keysym {
            keysym: 0x3bd,
            offset: 12114,
        },
        name_keysym {
            keysym: 0x3bf,
            offset: 12118,
        },
        name_keysym {
            keysym: 0xae0,
            offset: 12122,
        },
        name_keysym {
            keysym: 0xae1,
            offset: 12139,
        },
        name_keysym {
            keysym: 0xaa2,
            offset: 12158,
        },
        name_keysym {
            keysym: 0x1ca,
            offset: 12166,
        },
        name_keysym {
            keysym: 0x1ea,
            offset: 12174,
        },
        name_keysym {
            keysym: 0x3d,
            offset: 12182,
        },
        name_keysym {
            keysym: 0xff1b,
            offset: 12188,
        },
        name_keysym {
            keysym: 0xd0,
            offset: 12195,
        },
        name_keysym {
            keysym: 0xd0,
            offset: 12199,
        },
        name_keysym {
            keysym: 0xf0,
            offset: 12203,
        },
        name_keysym {
            keysym: 0x1001ebc,
            offset: 12207,
        },
        name_keysym {
            keysym: 0x1001ebd,
            offset: 12214,
        },
        name_keysym {
            keysym: 0x20ac,
            offset: 12221,
        },
        name_keysym {
            keysym: 0x21,
            offset: 12230,
        },
        name_keysym {
            keysym: 0xa1,
            offset: 12237,
        },
        name_keysym {
            keysym: 0xff62,
            offset: 12248,
        },
        name_keysym {
            keysym: 0x1000ff76,
            offset: 12256,
        },
        name_keysym {
            keysym: 0x1000ff77,
            offset: 12267,
        },
        name_keysym {
            keysym: 0x10001b7,
            offset: 12278,
        },
        name_keysym {
            keysym: 0x1000292,
            offset: 12282,
        },
        name_keysym {
            keysym: 0x46,
            offset: 12286,
        },
        name_keysym {
            keysym: 0x66,
            offset: 12288,
        },
        name_keysym {
            keysym: 0xffbe,
            offset: 12290,
        },
        name_keysym {
            keysym: 0xffc7,
            offset: 12293,
        },
        name_keysym {
            keysym: 0xffc8,
            offset: 12297,
        },
        name_keysym {
            keysym: 0xffc9,
            offset: 12301,
        },
        name_keysym {
            keysym: 0xffca,
            offset: 12305,
        },
        name_keysym {
            keysym: 0xffcb,
            offset: 12309,
        },
        name_keysym {
            keysym: 0xffcc,
            offset: 12313,
        },
        name_keysym {
            keysym: 0xffcd,
            offset: 12317,
        },
        name_keysym {
            keysym: 0xffce,
            offset: 12321,
        },
        name_keysym {
            keysym: 0xffcf,
            offset: 12325,
        },
        name_keysym {
            keysym: 0xffd0,
            offset: 12329,
        },
        name_keysym {
            keysym: 0xffbf,
            offset: 12333,
        },
        name_keysym {
            keysym: 0xffd1,
            offset: 12336,
        },
        name_keysym {
            keysym: 0xffd2,
            offset: 12340,
        },
        name_keysym {
            keysym: 0xffd3,
            offset: 12344,
        },
        name_keysym {
            keysym: 0xffd4,
            offset: 12348,
        },
        name_keysym {
            keysym: 0xffd5,
            offset: 12352,
        },
        name_keysym {
            keysym: 0xffd6,
            offset: 12356,
        },
        name_keysym {
            keysym: 0xffd7,
            offset: 12360,
        },
        name_keysym {
            keysym: 0xffd8,
            offset: 12364,
        },
        name_keysym {
            keysym: 0xffd9,
            offset: 12368,
        },
        name_keysym {
            keysym: 0xffda,
            offset: 12372,
        },
        name_keysym {
            keysym: 0xffc0,
            offset: 12376,
        },
        name_keysym {
            keysym: 0xffdb,
            offset: 12379,
        },
        name_keysym {
            keysym: 0xffdc,
            offset: 12383,
        },
        name_keysym {
            keysym: 0xffdd,
            offset: 12387,
        },
        name_keysym {
            keysym: 0xffde,
            offset: 12391,
        },
        name_keysym {
            keysym: 0xffdf,
            offset: 12395,
        },
        name_keysym {
            keysym: 0xffe0,
            offset: 12399,
        },
        name_keysym {
            keysym: 0xffc1,
            offset: 12403,
        },
        name_keysym {
            keysym: 0xffc2,
            offset: 12406,
        },
        name_keysym {
            keysym: 0xffc3,
            offset: 12409,
        },
        name_keysym {
            keysym: 0xffc4,
            offset: 12412,
        },
        name_keysym {
            keysym: 0xffc5,
            offset: 12415,
        },
        name_keysym {
            keysym: 0xffc6,
            offset: 12418,
        },
        name_keysym {
            keysym: 0x1001e1e,
            offset: 12421,
        },
        name_keysym {
            keysym: 0x1001e1f,
            offset: 12431,
        },
        name_keysym {
            keysym: 0x10006f0,
            offset: 12441,
        },
        name_keysym {
            keysym: 0x10006f1,
            offset: 12449,
        },
        name_keysym {
            keysym: 0x10006f2,
            offset: 12457,
        },
        name_keysym {
            keysym: 0x10006f3,
            offset: 12465,
        },
        name_keysym {
            keysym: 0x10006f4,
            offset: 12473,
        },
        name_keysym {
            keysym: 0x10006f5,
            offset: 12481,
        },
        name_keysym {
            keysym: 0x10006f6,
            offset: 12489,
        },
        name_keysym {
            keysym: 0x10006f7,
            offset: 12497,
        },
        name_keysym {
            keysym: 0x10006f8,
            offset: 12505,
        },
        name_keysym {
            keysym: 0x10006f9,
            offset: 12513,
        },
        name_keysym {
            keysym: 0x10006cc,
            offset: 12521,
        },
        name_keysym {
            keysym: 0xaf8,
            offset: 12531,
        },
        name_keysym {
            keysym: 0x9e3,
            offset: 12544,
        },
        name_keysym {
            keysym: 0x10020a3,
            offset: 12547,
        },
        name_keysym {
            keysym: 0xabb,
            offset: 12558,
        },
        name_keysym {
            keysym: 0xadc,
            offset: 12566,
        },
        name_keysym {
            keysym: 0xadb,
            offset: 12586,
        },
        name_keysym {
            keysym: 0xadd,
            offset: 12603,
        },
        name_keysym {
            keysym: 0xae9,
            offset: 12624,
        },
        name_keysym {
            keysym: 0xae8,
            offset: 12644,
        },
        name_keysym {
            keysym: 0xff68,
            offset: 12662,
        },
        name_keysym {
            keysym: 0xfed0,
            offset: 12667,
        },
        name_keysym {
            keysym: 0xac5,
            offset: 12688,
        },
        name_keysym {
            keysym: 0xab7,
            offset: 12700,
        },
        name_keysym {
            keysym: 0x1002085,
            offset: 12711,
        },
        name_keysym {
            keysym: 0x1002075,
            offset: 12725,
        },
        name_keysym {
            keysym: 0xab5,
            offset: 12738,
        },
        name_keysym {
            keysym: 0x1002084,
            offset: 12749,
        },
        name_keysym {
            keysym: 0x1002074,
            offset: 12763,
        },
        name_keysym {
            keysym: 0x100221c,
            offset: 12776,
        },
        name_keysym {
            keysym: 0x8f6,
            offset: 12787,
        },
        name_keysym {
            keysym: 0x47,
            offset: 12796,
        },
        name_keysym {
            keysym: 0x67,
            offset: 12798,
        },
        name_keysym {
            keysym: 0x2d5,
            offset: 12800,
        },
        name_keysym {
            keysym: 0x2f5,
            offset: 12810,
        },
        name_keysym {
            keysym: 0x2ab,
            offset: 12820,
        },
        name_keysym {
            keysym: 0x2bb,
            offset: 12827,
        },
        name_keysym {
            keysym: 0x10001e6,
            offset: 12834,
        },
        name_keysym {
            keysym: 0x10001e7,
            offset: 12841,
        },
        name_keysym {
            keysym: 0x3ab,
            offset: 12848,
        },
        name_keysym {
            keysym: 0x3bb,
            offset: 12857,
        },
        name_keysym {
            keysym: 0x2d8,
            offset: 12866,
        },
        name_keysym {
            keysym: 0x2f8,
            offset: 12878,
        },
        name_keysym {
            keysym: 0x10010d0,
            offset: 12890,
        },
        name_keysym {
            keysym: 0x10010d1,
            offset: 12902,
        },
        name_keysym {
            keysym: 0x10010ea,
            offset: 12915,
        },
        name_keysym {
            keysym: 0x10010ed,
            offset: 12928,
        },
        name_keysym {
            keysym: 0x10010e9,
            offset: 12942,
        },
        name_keysym {
            keysym: 0x10010ec,
            offset: 12956,
        },
        name_keysym {
            keysym: 0x10010d3,
            offset: 12969,
        },
        name_keysym {
            keysym: 0x10010d4,
            offset: 12982,
        },
        name_keysym {
            keysym: 0x10010f6,
            offset: 12994,
        },
        name_keysym {
            keysym: 0x10010d2,
            offset: 13006,
        },
        name_keysym {
            keysym: 0x10010e6,
            offset: 13019,
        },
        name_keysym {
            keysym: 0x10010f0,
            offset: 13033,
        },
        name_keysym {
            keysym: 0x10010f4,
            offset: 13046,
        },
        name_keysym {
            keysym: 0x10010f1,
            offset: 13059,
        },
        name_keysym {
            keysym: 0x10010f2,
            offset: 13071,
        },
        name_keysym {
            keysym: 0x10010f5,
            offset: 13084,
        },
        name_keysym {
            keysym: 0x10010d8,
            offset: 13097,
        },
        name_keysym {
            keysym: 0x10010ef,
            offset: 13109,
        },
        name_keysym {
            keysym: 0x10010eb,
            offset: 13123,
        },
        name_keysym {
            keysym: 0x10010d9,
            offset: 13136,
        },
        name_keysym {
            keysym: 0x10010e5,
            offset: 13149,
        },
        name_keysym {
            keysym: 0x10010da,
            offset: 13163,
        },
        name_keysym {
            keysym: 0x10010db,
            offset: 13176,
        },
        name_keysym {
            keysym: 0x10010dc,
            offset: 13189,
        },
        name_keysym {
            keysym: 0x10010dd,
            offset: 13202,
        },
        name_keysym {
            keysym: 0x10010de,
            offset: 13214,
        },
        name_keysym {
            keysym: 0x10010e4,
            offset: 13227,
        },
        name_keysym {
            keysym: 0x10010e7,
            offset: 13241,
        },
        name_keysym {
            keysym: 0x10010e0,
            offset: 13254,
        },
        name_keysym {
            keysym: 0x10010e1,
            offset: 13267,
        },
        name_keysym {
            keysym: 0x10010e8,
            offset: 13280,
        },
        name_keysym {
            keysym: 0x10010d7,
            offset: 13294,
        },
        name_keysym {
            keysym: 0x10010e2,
            offset: 13307,
        },
        name_keysym {
            keysym: 0x10010e3,
            offset: 13320,
        },
        name_keysym {
            keysym: 0x10010d5,
            offset: 13332,
        },
        name_keysym {
            keysym: 0x10010f3,
            offset: 13345,
        },
        name_keysym {
            keysym: 0x10010ee,
            offset: 13357,
        },
        name_keysym {
            keysym: 0x10010d6,
            offset: 13370,
        },
        name_keysym {
            keysym: 0x10010df,
            offset: 13383,
        },
        name_keysym {
            keysym: 0x60,
            offset: 13397,
        },
        name_keysym {
            keysym: 0x3e,
            offset: 13403,
        },
        name_keysym {
            keysym: 0x8be,
            offset: 13411,
        },
        name_keysym {
            keysym: 0x7ae,
            offset: 13428,
        },
        name_keysym {
            keysym: 0x7c1,
            offset: 13449,
        },
        name_keysym {
            keysym: 0x7e1,
            offset: 13461,
        },
        name_keysym {
            keysym: 0x7a1,
            offset: 13473,
        },
        name_keysym {
            keysym: 0x7b1,
            offset: 13491,
        },
        name_keysym {
            keysym: 0x7c2,
            offset: 13509,
        },
        name_keysym {
            keysym: 0x7e2,
            offset: 13520,
        },
        name_keysym {
            keysym: 0x7d7,
            offset: 13531,
        },
        name_keysym {
            keysym: 0x7f7,
            offset: 13541,
        },
        name_keysym {
            keysym: 0x7c4,
            offset: 13551,
        },
        name_keysym {
            keysym: 0x7e4,
            offset: 13563,
        },
        name_keysym {
            keysym: 0x7c5,
            offset: 13575,
        },
        name_keysym {
            keysym: 0x7e5,
            offset: 13589,
        },
        name_keysym {
            keysym: 0x7a2,
            offset: 13603,
        },
        name_keysym {
            keysym: 0x7b2,
            offset: 13623,
        },
        name_keysym {
            keysym: 0x7c7,
            offset: 13643,
        },
        name_keysym {
            keysym: 0x7e7,
            offset: 13653,
        },
        name_keysym {
            keysym: 0x7a3,
            offset: 13663,
        },
        name_keysym {
            keysym: 0x7b3,
            offset: 13679,
        },
        name_keysym {
            keysym: 0x7f3,
            offset: 13695,
        },
        name_keysym {
            keysym: 0x7c3,
            offset: 13717,
        },
        name_keysym {
            keysym: 0x7e3,
            offset: 13729,
        },
        name_keysym {
            keysym: 0x7af,
            offset: 13741,
        },
        name_keysym {
            keysym: 0x7c9,
            offset: 13756,
        },
        name_keysym {
            keysym: 0x7e9,
            offset: 13767,
        },
        name_keysym {
            keysym: 0x7a4,
            offset: 13778,
        },
        name_keysym {
            keysym: 0x7b4,
            offset: 13795,
        },
        name_keysym {
            keysym: 0x7b6,
            offset: 13812,
        },
        name_keysym {
            keysym: 0x7a5,
            offset: 13837,
        },
        name_keysym {
            keysym: 0x7a5,
            offset: 13857,
        },
        name_keysym {
            keysym: 0x7b5,
            offset: 13876,
        },
        name_keysym {
            keysym: 0x7ca,
            offset: 13895,
        },
        name_keysym {
            keysym: 0x7ea,
            offset: 13907,
        },
        name_keysym {
            keysym: 0x7cb,
            offset: 13919,
        },
        name_keysym {
            keysym: 0x7eb,
            offset: 13932,
        },
        name_keysym {
            keysym: 0x7cb,
            offset: 13945,
        },
        name_keysym {
            keysym: 0x7eb,
            offset: 13957,
        },
        name_keysym {
            keysym: 0x7cc,
            offset: 13969,
        },
        name_keysym {
            keysym: 0x7ec,
            offset: 13978,
        },
        name_keysym {
            keysym: 0x7cd,
            offset: 13987,
        },
        name_keysym {
            keysym: 0x7ed,
            offset: 13996,
        },
        name_keysym {
            keysym: 0x7d9,
            offset: 14005,
        },
        name_keysym {
            keysym: 0x7f9,
            offset: 14017,
        },
        name_keysym {
            keysym: 0x7ab,
            offset: 14029,
        },
        name_keysym {
            keysym: 0x7bb,
            offset: 14047,
        },
        name_keysym {
            keysym: 0x7cf,
            offset: 14065,
        },
        name_keysym {
            keysym: 0x7ef,
            offset: 14079,
        },
        name_keysym {
            keysym: 0x7a7,
            offset: 14093,
        },
        name_keysym {
            keysym: 0x7b7,
            offset: 14113,
        },
        name_keysym {
            keysym: 0x7d6,
            offset: 14133,
        },
        name_keysym {
            keysym: 0x7f6,
            offset: 14143,
        },
        name_keysym {
            keysym: 0x7d0,
            offset: 14153,
        },
        name_keysym {
            keysym: 0x7f0,
            offset: 14162,
        },
        name_keysym {
            keysym: 0x7d8,
            offset: 14171,
        },
        name_keysym {
            keysym: 0x7f8,
            offset: 14181,
        },
        name_keysym {
            keysym: 0x7d1,
            offset: 14191,
        },
        name_keysym {
            keysym: 0x7f1,
            offset: 14201,
        },
        name_keysym {
            keysym: 0x7d2,
            offset: 14211,
        },
        name_keysym {
            keysym: 0x7f2,
            offset: 14223,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 14235,
        },
        name_keysym {
            keysym: 0x7d4,
            offset: 14248,
        },
        name_keysym {
            keysym: 0x7f4,
            offset: 14258,
        },
        name_keysym {
            keysym: 0x7c8,
            offset: 14268,
        },
        name_keysym {
            keysym: 0x7e8,
            offset: 14280,
        },
        name_keysym {
            keysym: 0x7d5,
            offset: 14292,
        },
        name_keysym {
            keysym: 0x7f5,
            offset: 14306,
        },
        name_keysym {
            keysym: 0x7a8,
            offset: 14320,
        },
        name_keysym {
            keysym: 0x7b8,
            offset: 14340,
        },
        name_keysym {
            keysym: 0x7ba,
            offset: 14360,
        },
        name_keysym {
            keysym: 0x7a9,
            offset: 14388,
        },
        name_keysym {
            keysym: 0x7b9,
            offset: 14410,
        },
        name_keysym {
            keysym: 0x7ce,
            offset: 14432,
        },
        name_keysym {
            keysym: 0x7ee,
            offset: 14441,
        },
        name_keysym {
            keysym: 0x7c6,
            offset: 14450,
        },
        name_keysym {
            keysym: 0x7e6,
            offset: 14461,
        },
        name_keysym {
            keysym: 0x100000be,
            offset: 14472,
        },
        name_keysym {
            keysym: 0xab,
            offset: 14480,
        },
        name_keysym {
            keysym: 0xbb,
            offset: 14494,
        },
        name_keysym {
            keysym: 0xab,
            offset: 14509,
        },
        name_keysym {
            keysym: 0xbb,
            offset: 14523,
        },
        name_keysym {
            keysym: 0x48,
            offset: 14538,
        },
        name_keysym {
            keysym: 0x68,
            offset: 14540,
        },
        name_keysym {
            keysym: 0xaa8,
            offset: 14542,
        },
        name_keysym {
            keysym: 0xff31,
            offset: 14552,
        },
        name_keysym {
            keysym: 0xebf,
            offset: 14559,
        },
        name_keysym {
            keysym: 0xec0,
            offset: 14568,
        },
        name_keysym {
            keysym: 0xef6,
            offset: 14578,
        },
        name_keysym {
            keysym: 0xef7,
            offset: 14591,
        },
        name_keysym {
            keysym: 0xff39,
            offset: 14605,
        },
        name_keysym {
            keysym: 0xeba,
            offset: 14618,
        },
        name_keysym {
            keysym: 0xff37,
            offset: 14631,
        },
        name_keysym {
            keysym: 0xea7,
            offset: 14648,
        },
        name_keysym {
            keysym: 0xec4,
            offset: 14662,
        },
        name_keysym {
            keysym: 0xff33,
            offset: 14671,
        },
        name_keysym {
            keysym: 0xec3,
            offset: 14682,
        },
        name_keysym {
            keysym: 0xed1,
            offset: 14692,
        },
        name_keysym {
            keysym: 0xff34,
            offset: 14702,
        },
        name_keysym {
            keysym: 0xebe,
            offset: 14715,
        },
        name_keysym {
            keysym: 0xed3,
            offset: 14728,
        },
        name_keysym {
            keysym: 0xeb7,
            offset: 14737,
        },
        name_keysym {
            keysym: 0xeea,
            offset: 14750,
        },
        name_keysym {
            keysym: 0xeda,
            offset: 14765,
        },
        name_keysym {
            keysym: 0xeee,
            offset: 14781,
        },
        name_keysym {
            keysym: 0xee8,
            offset: 14796,
        },
        name_keysym {
            keysym: 0xee9,
            offset: 14811,
        },
        name_keysym {
            keysym: 0xeeb,
            offset: 14826,
        },
        name_keysym {
            keysym: 0xed4,
            offset: 14842,
        },
        name_keysym {
            keysym: 0xed6,
            offset: 14858,
        },
        name_keysym {
            keysym: 0xef9,
            offset: 14878,
        },
        name_keysym {
            keysym: 0xee3,
            offset: 14905,
        },
        name_keysym {
            keysym: 0xed7,
            offset: 14920,
        },
        name_keysym {
            keysym: 0xed9,
            offset: 14935,
        },
        name_keysym {
            keysym: 0xed8,
            offset: 14955,
        },
        name_keysym {
            keysym: 0xef8,
            offset: 14975,
        },
        name_keysym {
            keysym: 0xeed,
            offset: 14992,
        },
        name_keysym {
            keysym: 0xee4,
            offset: 15008,
        },
        name_keysym {
            keysym: 0xee5,
            offset: 15023,
        },
        name_keysym {
            keysym: 0xedb,
            offset: 15042,
        },
        name_keysym {
            keysym: 0xee2,
            offset: 15057,
        },
        name_keysym {
            keysym: 0xedc,
            offset: 15077,
        },
        name_keysym {
            keysym: 0xedd,
            offset: 15098,
        },
        name_keysym {
            keysym: 0xee1,
            offset: 15118,
        },
        name_keysym {
            keysym: 0xede,
            offset: 15139,
        },
        name_keysym {
            keysym: 0xedf,
            offset: 15159,
        },
        name_keysym {
            keysym: 0xee0,
            offset: 15178,
        },
        name_keysym {
            keysym: 0xee6,
            offset: 15198,
        },
        name_keysym {
            keysym: 0xed5,
            offset: 15212,
        },
        name_keysym {
            keysym: 0xee7,
            offset: 15233,
        },
        name_keysym {
            keysym: 0xeec,
            offset: 15252,
        },
        name_keysym {
            keysym: 0xefa,
            offset: 15267,
        },
        name_keysym {
            keysym: 0xff35,
            offset: 15288,
        },
        name_keysym {
            keysym: 0xff38,
            offset: 15300,
        },
        name_keysym {
            keysym: 0xeb8,
            offset: 15314,
        },
        name_keysym {
            keysym: 0xebb,
            offset: 15327,
        },
        name_keysym {
            keysym: 0xea1,
            offset: 15341,
        },
        name_keysym {
            keysym: 0xea3,
            offset: 15355,
        },
        name_keysym {
            keysym: 0xef3,
            offset: 15373,
        },
        name_keysym {
            keysym: 0xeb1,
            offset: 15398,
        },
        name_keysym {
            keysym: 0xff3d,
            offset: 15411,
        },
        name_keysym {
            keysym: 0xea4,
            offset: 15436,
        },
        name_keysym {
            keysym: 0xea6,
            offset: 15449,
        },
        name_keysym {
            keysym: 0xea5,
            offset: 15467,
        },
        name_keysym {
            keysym: 0xec7,
            offset: 15485,
        },
        name_keysym {
            keysym: 0xeca,
            offset: 15494,
        },
        name_keysym {
            keysym: 0xef2,
            offset: 15504,
        },
        name_keysym {
            keysym: 0xebd,
            offset: 15519,
        },
        name_keysym {
            keysym: 0xeb2,
            offset: 15533,
        },
        name_keysym {
            keysym: 0xeb4,
            offset: 15546,
        },
        name_keysym {
            keysym: 0xff3b,
            offset: 15563,
        },
        name_keysym {
            keysym: 0xff3a,
            offset: 15580,
        },
        name_keysym {
            keysym: 0xff3e,
            offset: 15596,
        },
        name_keysym {
            keysym: 0xea9,
            offset: 15621,
        },
        name_keysym {
            keysym: 0xeb0,
            offset: 15634,
        },
        name_keysym {
            keysym: 0xeaa,
            offset: 15652,
        },
        name_keysym {
            keysym: 0xeab,
            offset: 15671,
        },
        name_keysym {
            keysym: 0xeaf,
            offset: 15689,
        },
        name_keysym {
            keysym: 0xeac,
            offset: 15708,
        },
        name_keysym {
            keysym: 0xead,
            offset: 15726,
        },
        name_keysym {
            keysym: 0xeae,
            offset: 15743,
        },
        name_keysym {
            keysym: 0xeef,
            offset: 15761,
        },
        name_keysym {
            keysym: 0xff36,
            offset: 15785,
        },
        name_keysym {
            keysym: 0xff3c,
            offset: 15799,
        },
        name_keysym {
            keysym: 0xeb5,
            offset: 15822,
        },
        name_keysym {
            keysym: 0xff3f,
            offset: 15834,
        },
        name_keysym {
            keysym: 0xea8,
            offset: 15849,
        },
        name_keysym {
            keysym: 0xeb9,
            offset: 15868,
        },
        name_keysym {
            keysym: 0xea2,
            offset: 15886,
        },
        name_keysym {
            keysym: 0xeb3,
            offset: 15905,
        },
        name_keysym {
            keysym: 0xeb6,
            offset: 15923,
        },
        name_keysym {
            keysym: 0xff32,
            offset: 15940,
        },
        name_keysym {
            keysym: 0xef0,
            offset: 15953,
        },
        name_keysym {
            keysym: 0xef4,
            offset: 15978,
        },
        name_keysym {
            keysym: 0xef1,
            offset: 16004,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 16029,
        },
        name_keysym {
            keysym: 0xebc,
            offset: 16043,
        },
        name_keysym {
            keysym: 0xecc,
            offset: 16056,
        },
        name_keysym {
            keysym: 0xec8,
            offset: 16065,
        },
        name_keysym {
            keysym: 0xec9,
            offset: 16075,
        },
        name_keysym {
            keysym: 0xece,
            offset: 16086,
        },
        name_keysym {
            keysym: 0xecd,
            offset: 16096,
        },
        name_keysym {
            keysym: 0xecf,
            offset: 16107,
        },
        name_keysym {
            keysym: 0xec1,
            offset: 16117,
        },
        name_keysym {
            keysym: 0xec2,
            offset: 16127,
        },
        name_keysym {
            keysym: 0xec6,
            offset: 16138,
        },
        name_keysym {
            keysym: 0xec5,
            offset: 16148,
        },
        name_keysym {
            keysym: 0xef5,
            offset: 16159,
        },
        name_keysym {
            keysym: 0xed2,
            offset: 16178,
        },
        name_keysym {
            keysym: 0xecb,
            offset: 16188,
        },
        name_keysym {
            keysym: 0xed0,
            offset: 16198,
        },
        name_keysym {
            keysym: 0xff29,
            offset: 16208,
        },
        name_keysym {
            keysym: 0x2a6,
            offset: 16216,
        },
        name_keysym {
            keysym: 0x2b6,
            offset: 16228,
        },
        name_keysym {
            keysym: 0xaee,
            offset: 16240,
        },
        name_keysym {
            keysym: 0xce0,
            offset: 16246,
        },
        name_keysym {
            keysym: 0xcf2,
            offset: 16259,
        },
        name_keysym {
            keysym: 0xce1,
            offset: 16271,
        },
        name_keysym {
            keysym: 0xce1,
            offset: 16282,
        },
        name_keysym {
            keysym: 0xce7,
            offset: 16294,
        },
        name_keysym {
            keysym: 0xce3,
            offset: 16306,
        },
        name_keysym {
            keysym: 0xce3,
            offset: 16319,
        },
        name_keysym {
            keysym: 0xcdf,
            offset: 16333,
        },
        name_keysym {
            keysym: 0xcea,
            offset: 16354,
        },
        name_keysym {
            keysym: 0xced,
            offset: 16371,
        },
        name_keysym {
            keysym: 0xcef,
            offset: 16387,
        },
        name_keysym {
            keysym: 0xcf3,
            offset: 16403,
        },
        name_keysym {
            keysym: 0xcf5,
            offset: 16418,
        },
        name_keysym {
            keysym: 0xcf5,
            offset: 16435,
        },
        name_keysym {
            keysym: 0xce2,
            offset: 16452,
        },
        name_keysym {
            keysym: 0xce2,
            offset: 16465,
        },
        name_keysym {
            keysym: 0xce4,
            offset: 16479,
        },
        name_keysym {
            keysym: 0xce7,
            offset: 16489,
        },
        name_keysym {
            keysym: 0xceb,
            offset: 16500,
        },
        name_keysym {
            keysym: 0xcf7,
            offset: 16512,
        },
        name_keysym {
            keysym: 0xcec,
            offset: 16523,
        },
        name_keysym {
            keysym: 0xcee,
            offset: 16536,
        },
        name_keysym {
            keysym: 0xcf0,
            offset: 16547,
        },
        name_keysym {
            keysym: 0xcf4,
            offset: 16558,
        },
        name_keysym {
            keysym: 0xcf7,
            offset: 16568,
        },
        name_keysym {
            keysym: 0xcf8,
            offset: 16580,
        },
        name_keysym {
            keysym: 0xcf1,
            offset: 16592,
        },
        name_keysym {
            keysym: 0xcf1,
            offset: 16606,
        },
        name_keysym {
            keysym: 0xcf9,
            offset: 16620,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 16632,
        },
        name_keysym {
            keysym: 0xcfa,
            offset: 16646,
        },
        name_keysym {
            keysym: 0xcfa,
            offset: 16657,
        },
        name_keysym {
            keysym: 0xce8,
            offset: 16668,
        },
        name_keysym {
            keysym: 0xce8,
            offset: 16679,
        },
        name_keysym {
            keysym: 0xce5,
            offset: 16691,
        },
        name_keysym {
            keysym: 0xce9,
            offset: 16702,
        },
        name_keysym {
            keysym: 0xcf6,
            offset: 16713,
        },
        name_keysym {
            keysym: 0xcf6,
            offset: 16725,
        },
        name_keysym {
            keysym: 0xce6,
            offset: 16737,
        },
        name_keysym {
            keysym: 0xce6,
            offset: 16749,
        },
        name_keysym {
            keysym: 0xff6a,
            offset: 16762,
        },
        name_keysym {
            keysym: 0xff23,
            offset: 16767,
        },
        name_keysym {
            keysym: 0xff23,
            offset: 16774,
        },
        name_keysym {
            keysym: 0xada,
            offset: 16786,
        },
        name_keysym {
            keysym: 0xff25,
            offset: 16795,
        },
        name_keysym {
            keysym: 0xff27,
            offset: 16804,
        },
        name_keysym {
            keysym: 0xff50,
            offset: 16822,
        },
        name_keysym {
            keysym: 0x8a3,
            offset: 16827,
        },
        name_keysym {
            keysym: 0x9ef,
            offset: 16842,
        },
        name_keysym {
            keysym: 0x9f0,
            offset: 16857,
        },
        name_keysym {
            keysym: 0x9f1,
            offset: 16872,
        },
        name_keysym {
            keysym: 0x9f2,
            offset: 16887,
        },
        name_keysym {
            keysym: 0x9f3,
            offset: 16902,
        },
        name_keysym {
            keysym: 0x1000ff74,
            offset: 16917,
        },
        name_keysym {
            keysym: 0x100000fc,
            offset: 16927,
        },
        name_keysym {
            keysym: 0x1000ff6f,
            offset: 16935,
        },
        name_keysym {
            keysym: 0x1000ff73,
            offset: 16947,
        },
        name_keysym {
            keysym: 0x1000ff71,
            offset: 16960,
        },
        name_keysym {
            keysym: 0x100000be,
            offset: 16973,
        },
        name_keysym {
            keysym: 0x1000ff72,
            offset: 16983,
        },
        name_keysym {
            keysym: 0x1000ff70,
            offset: 16996,
        },
        name_keysym {
            keysym: 0x100000ee,
            offset: 17009,
        },
        name_keysym {
            keysym: 0x1000ff75,
            offset: 17014,
        },
        name_keysym {
            keysym: 0x100000af,
            offset: 17027,
        },
        name_keysym {
            keysym: 0x100000f6,
            offset: 17034,
        },
        name_keysym {
            keysym: 0x1000ff48,
            offset: 17046,
        },
        name_keysym {
            keysym: 0x1000ff49,
            offset: 17058,
        },
        name_keysym {
            keysym: 0x100000a8,
            offset: 17070,
        },
        name_keysym {
            keysym: 0x100000aa,
            offset: 17083,
        },
        name_keysym {
            keysym: 0x100000ac,
            offset: 17102,
        },
        name_keysym {
            keysym: 0x100000ab,
            offset: 17120,
        },
        name_keysym {
            keysym: 0x100000a9,
            offset: 17137,
        },
        name_keysym {
            keysym: 0x1000ff6c,
            offset: 17150,
        },
        name_keysym {
            keysym: 0x1000ff6d,
            offset: 17158,
        },
        name_keysym {
            keysym: 0x1000ff6e,
            offset: 17167,
        },
        name_keysym {
            keysym: 0x100000ee,
            offset: 17174,
        },
        name_keysym {
            keysym: 0x2a1,
            offset: 17187,
        },
        name_keysym {
            keysym: 0x2b1,
            offset: 17195,
        },
        name_keysym {
            keysym: 0x9e2,
            offset: 17203,
        },
        name_keysym {
            keysym: 0xffed,
            offset: 17206,
        },
        name_keysym {
            keysym: 0xffee,
            offset: 17214,
        },
        name_keysym {
            keysym: 0xad,
            offset: 17222,
        },
        name_keysym {
            keysym: 0x49,
            offset: 17229,
        },
        name_keysym {
            keysym: 0x69,
            offset: 17231,
        },
        name_keysym {
            keysym: 0x2a9,
            offset: 17233,
        },
        name_keysym {
            keysym: 0xcd,
            offset: 17243,
        },
        name_keysym {
            keysym: 0xed,
            offset: 17250,
        },
        name_keysym {
            keysym: 0x1001eca,
            offset: 17257,
        },
        name_keysym {
            keysym: 0x1001ecb,
            offset: 17267,
        },
        name_keysym {
            keysym: 0x100012c,
            offset: 17277,
        },
        name_keysym {
            keysym: 0x100012d,
            offset: 17284,
        },
        name_keysym {
            keysym: 0xce,
            offset: 17291,
        },
        name_keysym {
            keysym: 0xee,
            offset: 17303,
        },
        name_keysym {
            keysym: 0x8cf,
            offset: 17315,
        },
        name_keysym {
            keysym: 0xcf,
            offset: 17325,
        },
        name_keysym {
            keysym: 0xef,
            offset: 17336,
        },
        name_keysym {
            keysym: 0x2b9,
            offset: 17347,
        },
        name_keysym {
            keysym: 0x8cd,
            offset: 17356,
        },
        name_keysym {
            keysym: 0xcc,
            offset: 17365,
        },
        name_keysym {
            keysym: 0xec,
            offset: 17372,
        },
        name_keysym {
            keysym: 0x1001ec8,
            offset: 17379,
        },
        name_keysym {
            keysym: 0x1001ec9,
            offset: 17385,
        },
        name_keysym {
            keysym: 0x3cf,
            offset: 17391,
        },
        name_keysym {
            keysym: 0x3ef,
            offset: 17399,
        },
        name_keysym {
            keysym: 0x8ce,
            offset: 17407,
        },
        name_keysym {
            keysym: 0x8da,
            offset: 17415,
        },
        name_keysym {
            keysym: 0x8db,
            offset: 17426,
        },
        name_keysym {
            keysym: 0x8c2,
            offset: 17435,
        },
        name_keysym {
            keysym: 0xff63,
            offset: 17444,
        },
        name_keysym {
            keysym: 0x1000ff72,
            offset: 17451,
        },
        name_keysym {
            keysym: 0x1000ff70,
            offset: 17462,
        },
        name_keysym {
            keysym: 0x8bf,
            offset: 17473,
        },
        name_keysym {
            keysym: 0x8dc,
            offset: 17482,
        },
        name_keysym {
            keysym: 0x100000ee,
            offset: 17495,
        },
        name_keysym {
            keysym: 0x3c7,
            offset: 17498,
        },
        name_keysym {
            keysym: 0x3e7,
            offset: 17506,
        },
        name_keysym {
            keysym: 0xfe33,
            offset: 17514,
        },
        name_keysym {
            keysym: 0xfe30,
            offset: 17532,
        },
        name_keysym {
            keysym: 0xfe31,
            offset: 17557,
        },
        name_keysym {
            keysym: 0xfe32,
            offset: 17585,
        },
        name_keysym {
            keysym: 0xfe34,
            offset: 17599,
        },
        name_keysym {
            keysym: 0xfe2f,
            offset: 17609,
        },
        name_keysym {
            keysym: 0xfe2c,
            offset: 17630,
        },
        name_keysym {
            keysym: 0xfe2d,
            offset: 17651,
        },
        name_keysym {
            keysym: 0xfe2e,
            offset: 17673,
        },
        name_keysym {
            keysym: 0xfe0c,
            offset: 17692,
        },
        name_keysym {
            keysym: 0xfe0d,
            offset: 17708,
        },
        name_keysym {
            keysym: 0xfe06,
            offset: 17729,
        },
        name_keysym {
            keysym: 0xfe07,
            offset: 17745,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 17760,
        },
        name_keysym {
            keysym: 0xfe0e,
            offset: 17776,
        },
        name_keysym {
            keysym: 0xfe0f,
            offset: 17791,
        },
        name_keysym {
            keysym: 0xfe20,
            offset: 17811,
        },
        name_keysym {
            keysym: 0xfe02,
            offset: 17824,
        },
        name_keysym {
            keysym: 0xfe04,
            offset: 17841,
        },
        name_keysym {
            keysym: 0xfe05,
            offset: 17858,
        },
        name_keysym {
            keysym: 0xfe03,
            offset: 17874,
        },
        name_keysym {
            keysym: 0xfe12,
            offset: 17891,
        },
        name_keysym {
            keysym: 0xfe13,
            offset: 17908,
        },
        name_keysym {
            keysym: 0xfe11,
            offset: 17924,
        },
        name_keysym {
            keysym: 0xfe01,
            offset: 17941,
        },
        name_keysym {
            keysym: 0xfe22,
            offset: 17950,
        },
        name_keysym {
            keysym: 0xfe21,
            offset: 17969,
        },
        name_keysym {
            keysym: 0xfe08,
            offset: 17986,
        },
        name_keysym {
            keysym: 0xfe09,
            offset: 18001,
        },
        name_keysym {
            keysym: 0xfe24,
            offset: 18021,
        },
        name_keysym {
            keysym: 0xfe23,
            offset: 18043,
        },
        name_keysym {
            keysym: 0xfe25,
            offset: 18063,
        },
        name_keysym {
            keysym: 0xfe26,
            offset: 18086,
        },
        name_keysym {
            keysym: 0xfe0a,
            offset: 18110,
        },
        name_keysym {
            keysym: 0xfe0b,
            offset: 18125,
        },
        name_keysym {
            keysym: 0xfe2b,
            offset: 18145,
        },
        name_keysym {
            keysym: 0xfe29,
            offset: 18170,
        },
        name_keysym {
            keysym: 0xfe2a,
            offset: 18194,
        },
        name_keysym {
            keysym: 0xfe27,
            offset: 18219,
        },
        name_keysym {
            keysym: 0xfe28,
            offset: 18239,
        },
        name_keysym {
            keysym: 0x3a5,
            offset: 18260,
        },
        name_keysym {
            keysym: 0x3b5,
            offset: 18267,
        },
        name_keysym {
            keysym: 0x4a,
            offset: 18274,
        },
        name_keysym {
            keysym: 0x6a,
            offset: 18276,
        },
        name_keysym {
            keysym: 0x2ac,
            offset: 18278,
        },
        name_keysym {
            keysym: 0x2bc,
            offset: 18290,
        },
        name_keysym {
            keysym: 0xbca,
            offset: 18302,
        },
        name_keysym {
            keysym: 0x4b,
            offset: 18306,
        },
        name_keysym {
            keysym: 0x6b,
            offset: 18308,
        },
        name_keysym {
            keysym: 0x4b1,
            offset: 18310,
        },
        name_keysym {
            keysym: 0x4a7,
            offset: 18317,
        },
        name_keysym {
            keysym: 0x4c1,
            offset: 18324,
        },
        name_keysym {
            keysym: 0x4a3,
            offset: 18333,
        },
        name_keysym {
            keysym: 0x4a4,
            offset: 18353,
        },
        name_keysym {
            keysym: 0x4a5,
            offset: 18364,
        },
        name_keysym {
            keysym: 0x4b4,
            offset: 18381,
        },
        name_keysym {
            keysym: 0x4aa,
            offset: 18388,
        },
        name_keysym {
            keysym: 0x4cc,
            offset: 18395,
        },
        name_keysym {
            keysym: 0x4a1,
            offset: 18403,
        },
        name_keysym {
            keysym: 0x4ca,
            offset: 18417,
        },
        name_keysym {
            keysym: 0x4cd,
            offset: 18425,
        },
        name_keysym {
            keysym: 0x4cb,
            offset: 18433,
        },
        name_keysym {
            keysym: 0x4ce,
            offset: 18441,
        },
        name_keysym {
            keysym: 0x4cc,
            offset: 18449,
        },
        name_keysym {
            keysym: 0x4b2,
            offset: 18457,
        },
        name_keysym {
            keysym: 0x4a8,
            offset: 18464,
        },
        name_keysym {
            keysym: 0x4b6,
            offset: 18471,
        },
        name_keysym {
            keysym: 0x4b9,
            offset: 18479,
        },
        name_keysym {
            keysym: 0x4b7,
            offset: 18487,
        },
        name_keysym {
            keysym: 0x4ba,
            offset: 18495,
        },
        name_keysym {
            keysym: 0x4b8,
            offset: 18503,
        },
        name_keysym {
            keysym: 0xff2d,
            offset: 18511,
        },
        name_keysym {
            keysym: 0x4cf,
            offset: 18521,
        },
        name_keysym {
            keysym: 0x4d2,
            offset: 18529,
        },
        name_keysym {
            keysym: 0x4d0,
            offset: 18537,
        },
        name_keysym {
            keysym: 0x4a5,
            offset: 18545,
        },
        name_keysym {
            keysym: 0x4d3,
            offset: 18560,
        },
        name_keysym {
            keysym: 0x4d1,
            offset: 18568,
        },
        name_keysym {
            keysym: 0x4dd,
            offset: 18576,
        },
        name_keysym {
            keysym: 0x4c5,
            offset: 18583,
        },
        name_keysym {
            keysym: 0x4c8,
            offset: 18591,
        },
        name_keysym {
            keysym: 0x4c6,
            offset: 18599,
        },
        name_keysym {
            keysym: 0x4c9,
            offset: 18607,
        },
        name_keysym {
            keysym: 0x4c7,
            offset: 18615,
        },
        name_keysym {
            keysym: 0x4b5,
            offset: 18623,
        },
        name_keysym {
            keysym: 0x4ab,
            offset: 18630,
        },
        name_keysym {
            keysym: 0x4a2,
            offset: 18637,
        },
        name_keysym {
            keysym: 0x4d7,
            offset: 18657,
        },
        name_keysym {
            keysym: 0x4da,
            offset: 18665,
        },
        name_keysym {
            keysym: 0x4d8,
            offset: 18673,
        },
        name_keysym {
            keysym: 0x4db,
            offset: 18681,
        },
        name_keysym {
            keysym: 0x4d9,
            offset: 18689,
        },
        name_keysym {
            keysym: 0x4bb,
            offset: 18697,
        },
        name_keysym {
            keysym: 0x4be,
            offset: 18705,
        },
        name_keysym {
            keysym: 0x4bc,
            offset: 18713,
        },
        name_keysym {
            keysym: 0xff2e,
            offset: 18722,
        },
        name_keysym {
            keysym: 0x4bf,
            offset: 18733,
        },
        name_keysym {
            keysym: 0x4bd,
            offset: 18741,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 18749,
        },
        name_keysym {
            keysym: 0x4c0,
            offset: 18761,
        },
        name_keysym {
            keysym: 0x4c3,
            offset: 18769,
        },
        name_keysym {
            keysym: 0x4c1,
            offset: 18777,
        },
        name_keysym {
            keysym: 0x4c4,
            offset: 18785,
        },
        name_keysym {
            keysym: 0x4c2,
            offset: 18793,
        },
        name_keysym {
            keysym: 0x4af,
            offset: 18802,
        },
        name_keysym {
            keysym: 0x4c2,
            offset: 18811,
        },
        name_keysym {
            keysym: 0x4af,
            offset: 18819,
        },
        name_keysym {
            keysym: 0x4b3,
            offset: 18827,
        },
        name_keysym {
            keysym: 0x4a9,
            offset: 18834,
        },
        name_keysym {
            keysym: 0x4dc,
            offset: 18841,
        },
        name_keysym {
            keysym: 0x4a6,
            offset: 18849,
        },
        name_keysym {
            keysym: 0x4d4,
            offset: 18857,
        },
        name_keysym {
            keysym: 0x4ac,
            offset: 18865,
        },
        name_keysym {
            keysym: 0x4d6,
            offset: 18873,
        },
        name_keysym {
            keysym: 0x4ae,
            offset: 18881,
        },
        name_keysym {
            keysym: 0x4d5,
            offset: 18889,
        },
        name_keysym {
            keysym: 0x4ad,
            offset: 18897,
        },
        name_keysym {
            keysym: 0xff21,
            offset: 18905,
        },
        name_keysym {
            keysym: 0xff37,
            offset: 18911,
        },
        name_keysym {
            keysym: 0x3a2,
            offset: 18924,
        },
        name_keysym {
            keysym: 0xff26,
            offset: 18930,
        },
        name_keysym {
            keysym: 0x3d3,
            offset: 18939,
        },
        name_keysym {
            keysym: 0x3f3,
            offset: 18948,
        },
        name_keysym {
            keysym: 0xeff,
            offset: 18957,
        },
        name_keysym {
            keysym: 0xffb0,
            offset: 18968,
        },
        name_keysym {
            keysym: 0xffb1,
            offset: 18973,
        },
        name_keysym {
            keysym: 0xffb2,
            offset: 18978,
        },
        name_keysym {
            keysym: 0xffb3,
            offset: 18983,
        },
        name_keysym {
            keysym: 0xffb4,
            offset: 18988,
        },
        name_keysym {
            keysym: 0xffb5,
            offset: 18993,
        },
        name_keysym {
            keysym: 0xffb6,
            offset: 18998,
        },
        name_keysym {
            keysym: 0xffb7,
            offset: 19003,
        },
        name_keysym {
            keysym: 0xffb8,
            offset: 19008,
        },
        name_keysym {
            keysym: 0xffb9,
            offset: 19013,
        },
        name_keysym {
            keysym: 0xffab,
            offset: 19018,
        },
        name_keysym {
            keysym: 0x1000ff75,
            offset: 19025,
        },
        name_keysym {
            keysym: 0xff9d,
            offset: 19036,
        },
        name_keysym {
            keysym: 0xffae,
            offset: 19045,
        },
        name_keysym {
            keysym: 0xff9f,
            offset: 19056,
        },
        name_keysym {
            keysym: 0xffaf,
            offset: 19066,
        },
        name_keysym {
            keysym: 0xff99,
            offset: 19076,
        },
        name_keysym {
            keysym: 0xff9c,
            offset: 19084,
        },
        name_keysym {
            keysym: 0xff8d,
            offset: 19091,
        },
        name_keysym {
            keysym: 0xffbd,
            offset: 19100,
        },
        name_keysym {
            keysym: 0xff91,
            offset: 19109,
        },
        name_keysym {
            keysym: 0xff92,
            offset: 19115,
        },
        name_keysym {
            keysym: 0xff93,
            offset: 19121,
        },
        name_keysym {
            keysym: 0xff94,
            offset: 19127,
        },
        name_keysym {
            keysym: 0xff95,
            offset: 19133,
        },
        name_keysym {
            keysym: 0xff9e,
            offset: 19141,
        },
        name_keysym {
            keysym: 0xff96,
            offset: 19151,
        },
        name_keysym {
            keysym: 0xffaa,
            offset: 19159,
        },
        name_keysym {
            keysym: 0xff9b,
            offset: 19171,
        },
        name_keysym {
            keysym: 0xff9b,
            offset: 19179,
        },
        name_keysym {
            keysym: 0xff9a,
            offset: 19192,
        },
        name_keysym {
            keysym: 0xff9a,
            offset: 19203,
        },
        name_keysym {
            keysym: 0xff98,
            offset: 19212,
        },
        name_keysym {
            keysym: 0xffac,
            offset: 19221,
        },
        name_keysym {
            keysym: 0xff80,
            offset: 19234,
        },
        name_keysym {
            keysym: 0xffad,
            offset: 19243,
        },
        name_keysym {
            keysym: 0xff89,
            offset: 19255,
        },
        name_keysym {
            keysym: 0xff97,
            offset: 19262,
        },
        name_keysym {
            keysym: 0x3a2,
            offset: 19268,
        },
        name_keysym {
            keysym: 0x4c,
            offset: 19272,
        },
        name_keysym {
            keysym: 0x6c,
            offset: 19274,
        },
        name_keysym {
            keysym: 0xffc8,
            offset: 19276,
        },
        name_keysym {
            keysym: 0xffd1,
            offset: 19279,
        },
        name_keysym {
            keysym: 0xffc9,
            offset: 19283,
        },
        name_keysym {
            keysym: 0xffca,
            offset: 19286,
        },
        name_keysym {
            keysym: 0xffcb,
            offset: 19289,
        },
        name_keysym {
            keysym: 0xffcc,
            offset: 19292,
        },
        name_keysym {
            keysym: 0xffcd,
            offset: 19295,
        },
        name_keysym {
            keysym: 0xffce,
            offset: 19298,
        },
        name_keysym {
            keysym: 0xffcf,
            offset: 19301,
        },
        name_keysym {
            keysym: 0xffd0,
            offset: 19304,
        },
        name_keysym {
            keysym: 0x1c5,
            offset: 19307,
        },
        name_keysym {
            keysym: 0x1e5,
            offset: 19314,
        },
        name_keysym {
            keysym: 0xfed4,
            offset: 19321,
        },
        name_keysym {
            keysym: 0xad9,
            offset: 19341,
        },
        name_keysym {
            keysym: 0x1001e36,
            offset: 19352,
        },
        name_keysym {
            keysym: 0x1001e37,
            offset: 19362,
        },
        name_keysym {
            keysym: 0x1a5,
            offset: 19372,
        },
        name_keysym {
            keysym: 0x1b5,
            offset: 19379,
        },
        name_keysym {
            keysym: 0x3a6,
            offset: 19386,
        },
        name_keysym {
            keysym: 0x3b6,
            offset: 19395,
        },
        name_keysym {
            keysym: 0xff51,
            offset: 19404,
        },
        name_keysym {
            keysym: 0xabc,
            offset: 19409,
        },
        name_keysym {
            keysym: 0x8fb,
            offset: 19426,
        },
        name_keysym {
            keysym: 0xba3,
            offset: 19436,
        },
        name_keysym {
            keysym: 0xad2,
            offset: 19446,
        },
        name_keysym {
            keysym: 0x8af,
            offset: 19466,
        },
        name_keysym {
            keysym: 0xacc,
            offset: 19487,
        },
        name_keysym {
            keysym: 0xaea,
            offset: 19504,
        },
        name_keysym {
            keysym: 0x8a1,
            offset: 19516,
        },
        name_keysym {
            keysym: 0xbda,
            offset: 19528,
        },
        name_keysym {
            keysym: 0x1002039,
            offset: 19537,
        },
        name_keysym {
            keysym: 0xad0,
            offset: 19562,
        },
        name_keysym {
            keysym: 0x9f4,
            offset: 19582,
        },
        name_keysym {
            keysym: 0xbdc,
            offset: 19588,
        },
        name_keysym {
            keysym: 0x3c,
            offset: 19597,
        },
        name_keysym {
            keysym: 0x8bc,
            offset: 19602,
        },
        name_keysym {
            keysym: 0x9e5,
            offset: 19616,
        },
        name_keysym {
            keysym: 0xff0a,
            offset: 19619,
        },
        name_keysym {
            keysym: 0x100000af,
            offset: 19628,
        },
        name_keysym {
            keysym: 0x10020a4,
            offset: 19633,
        },
        name_keysym {
            keysym: 0x8de,
            offset: 19642,
        },
        name_keysym {
            keysym: 0x8df,
            offset: 19653,
        },
        name_keysym {
            keysym: 0x100000f6,
            offset: 19663,
        },
        name_keysym {
            keysym: 0x9ed,
            offset: 19673,
        },
        name_keysym {
            keysym: 0x9ea,
            offset: 19687,
        },
        name_keysym {
            keysym: 0x1a3,
            offset: 19702,
        },
        name_keysym {
            keysym: 0x1b3,
            offset: 19710,
        },
        name_keysym {
            keysym: 0x4d,
            offset: 19718,
        },
        name_keysym {
            keysym: 0x6d,
            offset: 19720,
        },
        name_keysym {
            keysym: 0x1001e40,
            offset: 19722,
        },
        name_keysym {
            keysym: 0x1001e41,
            offset: 19732,
        },
        name_keysym {
            keysym: 0x6b5,
            offset: 19742,
        },
        name_keysym {
            keysym: 0x6a5,
            offset: 19756,
        },
        name_keysym {
            keysym: 0x6b2,
            offset: 19770,
        },
        name_keysym {
            keysym: 0x6a2,
            offset: 19784,
        },
        name_keysym {
            keysym: 0x6bc,
            offset: 19798,
        },
        name_keysym {
            keysym: 0x6ac,
            offset: 19812,
        },
        name_keysym {
            keysym: 0xaf,
            offset: 19826,
        },
        name_keysym {
            keysym: 0xff3e,
            offset: 19833,
        },
        name_keysym {
            keysym: 0xaf7,
            offset: 19842,
        },
        name_keysym {
            keysym: 0xaf0,
            offset: 19853,
        },
        name_keysym {
            keysym: 0xabf,
            offset: 19866,
        },
        name_keysym {
            keysym: 0xba,
            offset: 19873,
        },
        name_keysym {
            keysym: 0xff2c,
            offset: 19883,
        },
        name_keysym {
            keysym: 0xff67,
            offset: 19890,
        },
        name_keysym {
            keysym: 0xffe7,
            offset: 19895,
        },
        name_keysym {
            keysym: 0xffe8,
            offset: 19902,
        },
        name_keysym {
            keysym: 0x10020a5,
            offset: 19909,
        },
        name_keysym {
            keysym: 0x2d,
            offset: 19918,
        },
        name_keysym {
            keysym: 0xad6,
            offset: 19924,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 19932,
        },
        name_keysym {
            keysym: 0xfe77,
            offset: 19944,
        },
        name_keysym {
            keysym: 0xfe76,
            offset: 19967,
        },
        name_keysym {
            keysym: 0xb5,
            offset: 19984,
        },
        name_keysym {
            keysym: 0xff22,
            offset: 19987,
        },
        name_keysym {
            keysym: 0xff20,
            offset: 19996,
        },
        name_keysym {
            keysym: 0xff3d,
            offset: 20006,
        },
        name_keysym {
            keysym: 0xd7,
            offset: 20024,
        },
        name_keysym {
            keysym: 0xaf6,
            offset: 20033,
        },
        name_keysym {
            keysym: 0xaf5,
            offset: 20045,
        },
        name_keysym {
            keysym: 0x100000a8,
            offset: 20058,
        },
        name_keysym {
            keysym: 0x100000aa,
            offset: 20069,
        },
        name_keysym {
            keysym: 0x100000ac,
            offset: 20086,
        },
        name_keysym {
            keysym: 0x100000ab,
            offset: 20102,
        },
        name_keysym {
            keysym: 0x100000a9,
            offset: 20117,
        },
        name_keysym {
            keysym: 0x4e,
            offset: 20128,
        },
        name_keysym {
            keysym: 0x6e,
            offset: 20130,
        },
        name_keysym {
            keysym: 0x8c5,
            offset: 20132,
        },
        name_keysym {
            keysym: 0x1d1,
            offset: 20138,
        },
        name_keysym {
            keysym: 0x1f1,
            offset: 20145,
        },
        name_keysym {
            keysym: 0x10020a6,
            offset: 20152,
        },
        name_keysym {
            keysym: 0x1d2,
            offset: 20162,
        },
        name_keysym {
            keysym: 0x1f2,
            offset: 20169,
        },
        name_keysym {
            keysym: 0x3d1,
            offset: 20176,
        },
        name_keysym {
            keysym: 0x3f1,
            offset: 20185,
        },
        name_keysym {
            keysym: 0x10020aa,
            offset: 20194,
        },
        name_keysym {
            keysym: 0xff56,
            offset: 20208,
        },
        name_keysym {
            keysym: 0xfed2,
            offset: 20213,
        },
        name_keysym {
            keysym: 0x1002089,
            offset: 20233,
        },
        name_keysym {
            keysym: 0x1002079,
            offset: 20247,
        },
        name_keysym {
            keysym: 0x9e8,
            offset: 20260,
        },
        name_keysym {
            keysym: 0xa0,
            offset: 20263,
        },
        name_keysym {
            keysym: 0,
            offset: 20276,
        },
        name_keysym {
            keysym: 0x1002247,
            offset: 20285,
        },
        name_keysym {
            keysym: 0x1002209,
            offset: 20297,
        },
        name_keysym {
            keysym: 0x8bd,
            offset: 20310,
        },
        name_keysym {
            keysym: 0x1002262,
            offset: 20319,
        },
        name_keysym {
            keysym: 0xac,
            offset: 20332,
        },
        name_keysym {
            keysym: 0xd1,
            offset: 20340,
        },
        name_keysym {
            keysym: 0xf1,
            offset: 20347,
        },
        name_keysym {
            keysym: 0xff7f,
            offset: 20354,
        },
        name_keysym {
            keysym: 0x23,
            offset: 20363,
        },
        name_keysym {
            keysym: 0x6b0,
            offset: 20374,
        },
        name_keysym {
            keysym: 0x4f,
            offset: 20385,
        },
        name_keysym {
            keysym: 0x6f,
            offset: 20387,
        },
        name_keysym {
            keysym: 0xd3,
            offset: 20389,
        },
        name_keysym {
            keysym: 0xf3,
            offset: 20396,
        },
        name_keysym {
            keysym: 0x100019f,
            offset: 20403,
        },
        name_keysym {
            keysym: 0x1000275,
            offset: 20411,
        },
        name_keysym {
            keysym: 0x1001ecc,
            offset: 20419,
        },
        name_keysym {
            keysym: 0x1001ecd,
            offset: 20429,
        },
        name_keysym {
            keysym: 0x10001d1,
            offset: 20439,
        },
        name_keysym {
            keysym: 0x10001d2,
            offset: 20446,
        },
        name_keysym {
            keysym: 0xd4,
            offset: 20453,
        },
        name_keysym {
            keysym: 0xf4,
            offset: 20465,
        },
        name_keysym {
            keysym: 0x1001ed0,
            offset: 20477,
        },
        name_keysym {
            keysym: 0x1001ed1,
            offset: 20494,
        },
        name_keysym {
            keysym: 0x1001ed8,
            offset: 20511,
        },
        name_keysym {
            keysym: 0x1001ed9,
            offset: 20531,
        },
        name_keysym {
            keysym: 0x1001ed2,
            offset: 20551,
        },
        name_keysym {
            keysym: 0x1001ed3,
            offset: 20568,
        },
        name_keysym {
            keysym: 0x1001ed4,
            offset: 20585,
        },
        name_keysym {
            keysym: 0x1001ed5,
            offset: 20601,
        },
        name_keysym {
            keysym: 0x1001ed6,
            offset: 20617,
        },
        name_keysym {
            keysym: 0x1001ed7,
            offset: 20634,
        },
        name_keysym {
            keysym: 0xd6,
            offset: 20651,
        },
        name_keysym {
            keysym: 0xf6,
            offset: 20662,
        },
        name_keysym {
            keysym: 0x1d5,
            offset: 20673,
        },
        name_keysym {
            keysym: 0x1f5,
            offset: 20686,
        },
        name_keysym {
            keysym: 0x13bc,
            offset: 20699,
        },
        name_keysym {
            keysym: 0x13bd,
            offset: 20702,
        },
        name_keysym {
            keysym: 0x1b2,
            offset: 20705,
        },
        name_keysym {
            keysym: 0xd2,
            offset: 20712,
        },
        name_keysym {
            keysym: 0xf2,
            offset: 20719,
        },
        name_keysym {
            keysym: 0x1001ece,
            offset: 20726,
        },
        name_keysym {
            keysym: 0x1001ecf,
            offset: 20732,
        },
        name_keysym {
            keysym: 0x10001a0,
            offset: 20738,
        },
        name_keysym {
            keysym: 0x10001a1,
            offset: 20744,
        },
        name_keysym {
            keysym: 0x1001eda,
            offset: 20750,
        },
        name_keysym {
            keysym: 0x1001edb,
            offset: 20761,
        },
        name_keysym {
            keysym: 0x1001ee2,
            offset: 20772,
        },
        name_keysym {
            keysym: 0x1001ee3,
            offset: 20786,
        },
        name_keysym {
            keysym: 0x1001edc,
            offset: 20800,
        },
        name_keysym {
            keysym: 0x1001edd,
            offset: 20811,
        },
        name_keysym {
            keysym: 0x1001ede,
            offset: 20822,
        },
        name_keysym {
            keysym: 0x1001edf,
            offset: 20832,
        },
        name_keysym {
            keysym: 0x1001ee0,
            offset: 20842,
        },
        name_keysym {
            keysym: 0x1001ee1,
            offset: 20853,
        },
        name_keysym {
            keysym: 0x3d2,
            offset: 20864,
        },
        name_keysym {
            keysym: 0x3f2,
            offset: 20872,
        },
        name_keysym {
            keysym: 0xac3,
            offset: 20880,
        },
        name_keysym {
            keysym: 0xab2,
            offset: 20890,
        },
        name_keysym {
            keysym: 0xbd,
            offset: 20899,
        },
        name_keysym {
            keysym: 0xbc,
            offset: 20907,
        },
        name_keysym {
            keysym: 0xab6,
            offset: 20918,
        },
        name_keysym {
            keysym: 0x1002081,
            offset: 20927,
        },
        name_keysym {
            keysym: 0xb9,
            offset: 20940,
        },
        name_keysym {
            keysym: 0xab0,
            offset: 20952,
        },
        name_keysym {
            keysym: 0xd8,
            offset: 20961,
        },
        name_keysym {
            keysym: 0xf8,
            offset: 20970,
        },
        name_keysym {
            keysym: 0xae2,
            offset: 20979,
        },
        name_keysym {
            keysym: 0xae5,
            offset: 20994,
        },
        name_keysym {
            keysym: 0xae4,
            offset: 21003,
        },
        name_keysym {
            keysym: 0xae3,
            offset: 21021,
        },
        name_keysym {
            keysym: 0xaa,
            offset: 21037,
        },
        name_keysym {
            keysym: 0xba,
            offset: 21049,
        },
        name_keysym {
            keysym: 0x1004ff44,
            offset: 21062,
        },
        name_keysym {
            keysym: 0x1004ff31,
            offset: 21074,
        },
        name_keysym {
            keysym: 0x1004ff08,
            offset: 21085,
        },
        name_keysym {
            keysym: 0x1004ff07,
            offset: 21098,
        },
        name_keysym {
            keysym: 0x1004ff5a,
            offset: 21109,
        },
        name_keysym {
            keysym: 0x1004ff58,
            offset: 21122,
        },
        name_keysym {
            keysym: 0x1004ff69,
            offset: 21135,
        },
        name_keysym {
            keysym: 0x1004ff0b,
            offset: 21145,
        },
        name_keysym {
            keysym: 0x1004ff02,
            offset: 21154,
        },
        name_keysym {
            keysym: 0x1004ff03,
            offset: 21162,
        },
        name_keysym {
            keysym: 0x1004ffff,
            offset: 21169,
        },
        name_keysym {
            keysym: 0x1004ff72,
            offset: 21179,
        },
        name_keysym {
            keysym: 0x1004ff54,
            offset: 21194,
        },
        name_keysym {
            keysym: 0x1004ff59,
            offset: 21202,
        },
        name_keysym {
            keysym: 0x1004ff57,
            offset: 21213,
        },
        name_keysym {
            keysym: 0x1004ff1b,
            offset: 21224,
        },
        name_keysym {
            keysym: 0x1004ff74,
            offset: 21234,
        },
        name_keysym {
            keysym: 0x1004ff6a,
            offset: 21244,
        },
        name_keysym {
            keysym: 0x1004ff63,
            offset: 21252,
        },
        name_keysym {
            keysym: 0x1004ff51,
            offset: 21262,
        },
        name_keysym {
            keysym: 0x1004ff67,
            offset: 21270,
        },
        name_keysym {
            keysym: 0x1004ff45,
            offset: 21278,
        },
        name_keysym {
            keysym: 0x1004ff5e,
            offset: 21289,
        },
        name_keysym {
            keysym: 0x1004ff5c,
            offset: 21302,
        },
        name_keysym {
            keysym: 0x1004ff42,
            offset: 21314,
        },
        name_keysym {
            keysym: 0x1004ff40,
            offset: 21326,
        },
        name_keysym {
            keysym: 0x1004ff43,
            offset: 21338,
        },
        name_keysym {
            keysym: 0x1004ff41,
            offset: 21351,
        },
        name_keysym {
            keysym: 0x1004ff04,
            offset: 21361,
        },
        name_keysym {
            keysym: 0x1004ff5d,
            offset: 21370,
        },
        name_keysym {
            keysym: 0x1004ff5b,
            offset: 21383,
        },
        name_keysym {
            keysym: 0x1004ff32,
            offset: 21395,
        },
        name_keysym {
            keysym: 0x1004ff33,
            offset: 21411,
        },
        name_keysym {
            keysym: 0x1004ff73,
            offset: 21425,
        },
        name_keysym {
            keysym: 0x1004ff78,
            offset: 21437,
        },
        name_keysym {
            keysym: 0x1004ff53,
            offset: 21448,
        },
        name_keysym {
            keysym: 0x1004ff60,
            offset: 21457,
        },
        name_keysym {
            keysym: 0x1004ff71,
            offset: 21467,
        },
        name_keysym {
            keysym: 0x1004ff65,
            offset: 21480,
        },
        name_keysym {
            keysym: 0x1004ff52,
            offset: 21488,
        },
        name_keysym {
            keysym: 0xd8,
            offset: 21494,
        },
        name_keysym {
            keysym: 0xf8,
            offset: 21501,
        },
        name_keysym {
            keysym: 0xd5,
            offset: 21508,
        },
        name_keysym {
            keysym: 0xf5,
            offset: 21515,
        },
        name_keysym {
            keysym: 0xbc0,
            offset: 21522,
        },
        name_keysym {
            keysym: 0xfe78,
            offset: 21530,
        },
        name_keysym {
            keysym: 0xfe79,
            offset: 21546,
        },
        name_keysym {
            keysym: 0x47e,
            offset: 21562,
        },
        name_keysym {
            keysym: 0x50,
            offset: 21571,
        },
        name_keysym {
            keysym: 0x70,
            offset: 21573,
        },
        name_keysym {
            keysym: 0x1001e56,
            offset: 21575,
        },
        name_keysym {
            keysym: 0x1001e57,
            offset: 21585,
        },
        name_keysym {
            keysym: 0xff56,
            offset: 21595,
        },
        name_keysym {
            keysym: 0xff55,
            offset: 21605,
        },
        name_keysym {
            keysym: 0xb6,
            offset: 21613,
        },
        name_keysym {
            keysym: 0x28,
            offset: 21623,
        },
        name_keysym {
            keysym: 0x29,
            offset: 21633,
        },
        name_keysym {
            keysym: 0x1002202,
            offset: 21644,
        },
        name_keysym {
            keysym: 0x8ef,
            offset: 21661,
        },
        name_keysym {
            keysym: 0xff13,
            offset: 21679,
        },
        name_keysym {
            keysym: 0x25,
            offset: 21685,
        },
        name_keysym {
            keysym: 0x2e,
            offset: 21693,
        },
        name_keysym {
            keysym: 0xb7,
            offset: 21700,
        },
        name_keysym {
            keysym: 0xad5,
            offset: 21715,
        },
        name_keysym {
            keysym: 0x10020a7,
            offset: 21724,
        },
        name_keysym {
            keysym: 0xafb,
            offset: 21735,
        },
        name_keysym {
            keysym: 0x2b,
            offset: 21755,
        },
        name_keysym {
            keysym: 0xb1,
            offset: 21760,
        },
        name_keysym {
            keysym: 0xfefa,
            offset: 21770,
        },
        name_keysym {
            keysym: 0xfee9,
            offset: 21789,
        },
        name_keysym {
            keysym: 0xfeea,
            offset: 21805,
        },
        name_keysym {
            keysym: 0xfeeb,
            offset: 21821,
        },
        name_keysym {
            keysym: 0xfeec,
            offset: 21837,
        },
        name_keysym {
            keysym: 0xfeed,
            offset: 21853,
        },
        name_keysym {
            keysym: 0xfee8,
            offset: 21869,
        },
        name_keysym {
            keysym: 0xfeef,
            offset: 21889,
        },
        name_keysym {
            keysym: 0xfef0,
            offset: 21907,
        },
        name_keysym {
            keysym: 0xfef1,
            offset: 21925,
        },
        name_keysym {
            keysym: 0xfef2,
            offset: 21943,
        },
        name_keysym {
            keysym: 0xfef3,
            offset: 21961,
        },
        name_keysym {
            keysym: 0xfeee,
            offset: 21979,
        },
        name_keysym {
            keysym: 0xfefb,
            offset: 22001,
        },
        name_keysym {
            keysym: 0xfefc,
            offset: 22021,
        },
        name_keysym {
            keysym: 0xfee3,
            offset: 22041,
        },
        name_keysym {
            keysym: 0xfee6,
            offset: 22054,
        },
        name_keysym {
            keysym: 0xfee7,
            offset: 22071,
        },
        name_keysym {
            keysym: 0xfef5,
            offset: 22089,
        },
        name_keysym {
            keysym: 0xfef6,
            offset: 22103,
        },
        name_keysym {
            keysym: 0xfef7,
            offset: 22117,
        },
        name_keysym {
            keysym: 0xfef8,
            offset: 22131,
        },
        name_keysym {
            keysym: 0xfefd,
            offset: 22145,
        },
        name_keysym {
            keysym: 0xfef4,
            offset: 22159,
        },
        name_keysym {
            keysym: 0xfef9,
            offset: 22177,
        },
        name_keysym {
            keysym: 0xfee0,
            offset: 22196,
        },
        name_keysym {
            keysym: 0xfee1,
            offset: 22209,
        },
        name_keysym {
            keysym: 0xfee2,
            offset: 22223,
        },
        name_keysym {
            keysym: 0xfee4,
            offset: 22234,
        },
        name_keysym {
            keysym: 0xfee5,
            offset: 22249,
        },
        name_keysym {
            keysym: 0xad4,
            offset: 22265,
        },
        name_keysym {
            keysym: 0xfed1,
            offset: 22278,
        },
        name_keysym {
            keysym: 0xff3e,
            offset: 22298,
        },
        name_keysym {
            keysym: 0xff61,
            offset: 22316,
        },
        name_keysym {
            keysym: 0xff55,
            offset: 22322,
        },
        name_keysym {
            keysym: 0x4b0,
            offset: 22328,
        },
        name_keysym {
            keysym: 0xaa6,
            offset: 22343,
        },
        name_keysym {
            keysym: 0x51,
            offset: 22354,
        },
        name_keysym {
            keysym: 0x71,
            offset: 22356,
        },
        name_keysym {
            keysym: 0xbcc,
            offset: 22358,
        },
        name_keysym {
            keysym: 0x3f,
            offset: 22363,
        },
        name_keysym {
            keysym: 0xbf,
            offset: 22372,
        },
        name_keysym {
            keysym: 0x22,
            offset: 22385,
        },
        name_keysym {
            keysym: 0x60,
            offset: 22394,
        },
        name_keysym {
            keysym: 0x27,
            offset: 22404,
        },
        name_keysym {
            keysym: 0x52,
            offset: 22415,
        },
        name_keysym {
            keysym: 0x72,
            offset: 22417,
        },
        name_keysym {
            keysym: 0xffd2,
            offset: 22419,
        },
        name_keysym {
            keysym: 0xffdb,
            offset: 22422,
        },
        name_keysym {
            keysym: 0xffdc,
            offset: 22426,
        },
        name_keysym {
            keysym: 0xffdd,
            offset: 22430,
        },
        name_keysym {
            keysym: 0xffde,
            offset: 22434,
        },
        name_keysym {
            keysym: 0xffdf,
            offset: 22438,
        },
        name_keysym {
            keysym: 0xffe0,
            offset: 22442,
        },
        name_keysym {
            keysym: 0xffd3,
            offset: 22446,
        },
        name_keysym {
            keysym: 0xffd4,
            offset: 22449,
        },
        name_keysym {
            keysym: 0xffd5,
            offset: 22452,
        },
        name_keysym {
            keysym: 0xffd6,
            offset: 22455,
        },
        name_keysym {
            keysym: 0xffd7,
            offset: 22458,
        },
        name_keysym {
            keysym: 0xffd8,
            offset: 22461,
        },
        name_keysym {
            keysym: 0xffd9,
            offset: 22464,
        },
        name_keysym {
            keysym: 0xffda,
            offset: 22467,
        },
        name_keysym {
            keysym: 0x1c0,
            offset: 22470,
        },
        name_keysym {
            keysym: 0x1e0,
            offset: 22477,
        },
        name_keysym {
            keysym: 0x8d6,
            offset: 22484,
        },
        name_keysym {
            keysym: 0x1d8,
            offset: 22492,
        },
        name_keysym {
            keysym: 0x1f8,
            offset: 22499,
        },
        name_keysym {
            keysym: 0x3a3,
            offset: 22506,
        },
        name_keysym {
            keysym: 0x3b3,
            offset: 22515,
        },
        name_keysym {
            keysym: 0xff66,
            offset: 22524,
        },
        name_keysym {
            keysym: 0xae,
            offset: 22529,
        },
        name_keysym {
            keysym: 0xfe72,
            offset: 22540,
        },
        name_keysym {
            keysym: 0x1000ff6c,
            offset: 22558,
        },
        name_keysym {
            keysym: 0xff0d,
            offset: 22564,
        },
        name_keysym {
            keysym: 0xff53,
            offset: 22571,
        },
        name_keysym {
            keysym: 0xabe,
            offset: 22577,
        },
        name_keysym {
            keysym: 0x8fd,
            offset: 22595,
        },
        name_keysym {
            keysym: 0xba6,
            offset: 22606,
        },
        name_keysym {
            keysym: 0xad3,
            offset: 22617,
        },
        name_keysym {
            keysym: 0x8b0,
            offset: 22638,
        },
        name_keysym {
            keysym: 0x8b7,
            offset: 22660,
        },
        name_keysym {
            keysym: 0xacd,
            offset: 22681,
        },
        name_keysym {
            keysym: 0xaeb,
            offset: 22699,
        },
        name_keysym {
            keysym: 0xbd8,
            offset: 22712,
        },
        name_keysym {
            keysym: 0x100203a,
            offset: 22722,
        },
        name_keysym {
            keysym: 0xad1,
            offset: 22748,
        },
        name_keysym {
            keysym: 0x9f5,
            offset: 22769,
        },
        name_keysym {
            keysym: 0xbfc,
            offset: 22776,
        },
        name_keysym {
            keysym: 0xff24,
            offset: 22786,
        },
        name_keysym {
            keysym: 0x10020a8,
            offset: 22793,
        },
        name_keysym {
            keysym: 0x53,
            offset: 22803,
        },
        name_keysym {
            keysym: 0x73,
            offset: 22805,
        },
        name_keysym {
            keysym: 0x1001e60,
            offset: 22807,
        },
        name_keysym {
            keysym: 0x1001e61,
            offset: 22817,
        },
        name_keysym {
            keysym: 0x1a6,
            offset: 22827,
        },
        name_keysym {
            keysym: 0x1b6,
            offset: 22834,
        },
        name_keysym {
            keysym: 0x1a9,
            offset: 22841,
        },
        name_keysym {
            keysym: 0x1b9,
            offset: 22848,
        },
        name_keysym {
            keysym: 0x1aa,
            offset: 22855,
        },
        name_keysym {
            keysym: 0x1ba,
            offset: 22864,
        },
        name_keysym {
            keysym: 0x100018f,
            offset: 22873,
        },
        name_keysym {
            keysym: 0x1000259,
            offset: 22879,
        },
        name_keysym {
            keysym: 0x2de,
            offset: 22885,
        },
        name_keysym {
            keysym: 0x2fe,
            offset: 22897,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 22909,
        },
        name_keysym {
            keysym: 0xff14,
            offset: 22923,
        },
        name_keysym {
            keysym: 0xad7,
            offset: 22935,
        },
        name_keysym {
            keysym: 0xa7,
            offset: 22943,
        },
        name_keysym {
            keysym: 0xff60,
            offset: 22951,
        },
        name_keysym {
            keysym: 0x3b,
            offset: 22958,
        },
        name_keysym {
            keysym: 0x4df,
            offset: 22968,
        },
        name_keysym {
            keysym: 0x6b1,
            offset: 22984,
        },
        name_keysym {
            keysym: 0x6a1,
            offset: 22996,
        },
        name_keysym {
            keysym: 0x6bf,
            offset: 23008,
        },
        name_keysym {
            keysym: 0x6af,
            offset: 23020,
        },
        name_keysym {
            keysym: 0x6b8,
            offset: 23032,
        },
        name_keysym {
            keysym: 0x6a8,
            offset: 23043,
        },
        name_keysym {
            keysym: 0x6b9,
            offset: 23054,
        },
        name_keysym {
            keysym: 0x6a9,
            offset: 23066,
        },
        name_keysym {
            keysym: 0x6ba,
            offset: 23078,
        },
        name_keysym {
            keysym: 0x6aa,
            offset: 23090,
        },
        name_keysym {
            keysym: 0x6bb,
            offset: 23102,
        },
        name_keysym {
            keysym: 0x6ab,
            offset: 23115,
        },
        name_keysym {
            keysym: 0xac6,
            offset: 23128,
        },
        name_keysym {
            keysym: 0x1002087,
            offset: 23141,
        },
        name_keysym {
            keysym: 0x1002077,
            offset: 23156,
        },
        name_keysym {
            keysym: 0xffe1,
            offset: 23170,
        },
        name_keysym {
            keysym: 0xffe6,
            offset: 23178,
        },
        name_keysym {
            keysym: 0xffe2,
            offset: 23189,
        },
        name_keysym {
            keysym: 0xaca,
            offset: 23197,
        },
        name_keysym {
            keysym: 0xaac,
            offset: 23211,
        },
        name_keysym {
            keysym: 0x8c9,
            offset: 23223,
        },
        name_keysym {
            keysym: 0xff3c,
            offset: 23236,
        },
        name_keysym {
            keysym: 0xafd,
            offset: 23252,
        },
        name_keysym {
            keysym: 0x1000d85,
            offset: 23271,
        },
        name_keysym {
            keysym: 0x1000d86,
            offset: 23278,
        },
        name_keysym {
            keysym: 0x1000dcf,
            offset: 23286,
        },
        name_keysym {
            keysym: 0x1000d87,
            offset: 23295,
        },
        name_keysym {
            keysym: 0x1000dd0,
            offset: 23303,
        },
        name_keysym {
            keysym: 0x1000d88,
            offset: 23312,
        },
        name_keysym {
            keysym: 0x1000dd1,
            offset: 23321,
        },
        name_keysym {
            keysym: 0x1000d93,
            offset: 23331,
        },
        name_keysym {
            keysym: 0x1000ddb,
            offset: 23339,
        },
        name_keysym {
            keysym: 0x1000dca,
            offset: 23348,
        },
        name_keysym {
            keysym: 0x1000d96,
            offset: 23356,
        },
        name_keysym {
            keysym: 0x1000dde,
            offset: 23364,
        },
        name_keysym {
            keysym: 0x1000db6,
            offset: 23373,
        },
        name_keysym {
            keysym: 0x1000db7,
            offset: 23381,
        },
        name_keysym {
            keysym: 0x1000da0,
            offset: 23390,
        },
        name_keysym {
            keysym: 0x1000da1,
            offset: 23398,
        },
        name_keysym {
            keysym: 0x1000da9,
            offset: 23407,
        },
        name_keysym {
            keysym: 0x1000daa,
            offset: 23416,
        },
        name_keysym {
            keysym: 0x1000daf,
            offset: 23426,
        },
        name_keysym {
            keysym: 0x1000db0,
            offset: 23435,
        },
        name_keysym {
            keysym: 0x1000d91,
            offset: 23445,
        },
        name_keysym {
            keysym: 0x1000dd9,
            offset: 23452,
        },
        name_keysym {
            keysym: 0x1000d92,
            offset: 23460,
        },
        name_keysym {
            keysym: 0x1000dda,
            offset: 23468,
        },
        name_keysym {
            keysym: 0x1000dc6,
            offset: 23477,
        },
        name_keysym {
            keysym: 0x1000d9c,
            offset: 23485,
        },
        name_keysym {
            keysym: 0x1000d9d,
            offset: 23493,
        },
        name_keysym {
            keysym: 0x1000d83,
            offset: 23502,
        },
        name_keysym {
            keysym: 0x1000dc4,
            offset: 23510,
        },
        name_keysym {
            keysym: 0x1000d89,
            offset: 23518,
        },
        name_keysym {
            keysym: 0x1000dd2,
            offset: 23525,
        },
        name_keysym {
            keysym: 0x1000d8a,
            offset: 23533,
        },
        name_keysym {
            keysym: 0x1000dd3,
            offset: 23541,
        },
        name_keysym {
            keysym: 0x1000da2,
            offset: 23550,
        },
        name_keysym {
            keysym: 0x1000da3,
            offset: 23558,
        },
        name_keysym {
            keysym: 0x1000da5,
            offset: 23567,
        },
        name_keysym {
            keysym: 0x1000d9a,
            offset: 23577,
        },
        name_keysym {
            keysym: 0x1000d9b,
            offset: 23585,
        },
        name_keysym {
            keysym: 0x1000df4,
            offset: 23594,
        },
        name_keysym {
            keysym: 0x1000dbd,
            offset: 23610,
        },
        name_keysym {
            keysym: 0x1000dc5,
            offset: 23618,
        },
        name_keysym {
            keysym: 0x1000d8f,
            offset: 23627,
        },
        name_keysym {
            keysym: 0x1000ddf,
            offset: 23635,
        },
        name_keysym {
            keysym: 0x1000d90,
            offset: 23644,
        },
        name_keysym {
            keysym: 0x1000df3,
            offset: 23653,
        },
        name_keysym {
            keysym: 0x1000db8,
            offset: 23663,
        },
        name_keysym {
            keysym: 0x1000db9,
            offset: 23671,
        },
        name_keysym {
            keysym: 0x1000db1,
            offset: 23680,
        },
        name_keysym {
            keysym: 0x1000dac,
            offset: 23688,
        },
        name_keysym {
            keysym: 0x1000db3,
            offset: 23698,
        },
        name_keysym {
            keysym: 0x1000d82,
            offset: 23708,
        },
        name_keysym {
            keysym: 0x1000d9e,
            offset: 23716,
        },
        name_keysym {
            keysym: 0x1000d9f,
            offset: 23725,
        },
        name_keysym {
            keysym: 0x1000da6,
            offset: 23734,
        },
        name_keysym {
            keysym: 0x1000dab,
            offset: 23743,
        },
        name_keysym {
            keysym: 0x1000da4,
            offset: 23752,
        },
        name_keysym {
            keysym: 0x1000d94,
            offset: 23761,
        },
        name_keysym {
            keysym: 0x1000ddc,
            offset: 23768,
        },
        name_keysym {
            keysym: 0x1000d95,
            offset: 23776,
        },
        name_keysym {
            keysym: 0x1000ddd,
            offset: 23784,
        },
        name_keysym {
            keysym: 0x1000db4,
            offset: 23793,
        },
        name_keysym {
            keysym: 0x1000db5,
            offset: 23801,
        },
        name_keysym {
            keysym: 0x1000dbb,
            offset: 23810,
        },
        name_keysym {
            keysym: 0x1000d8d,
            offset: 23818,
        },
        name_keysym {
            keysym: 0x1000d8e,
            offset: 23826,
        },
        name_keysym {
            keysym: 0x1000dd8,
            offset: 23835,
        },
        name_keysym {
            keysym: 0x1000df2,
            offset: 23844,
        },
        name_keysym {
            keysym: 0x1000dc3,
            offset: 23854,
        },
        name_keysym {
            keysym: 0x1000dc1,
            offset: 23862,
        },
        name_keysym {
            keysym: 0x1000dc2,
            offset: 23871,
        },
        name_keysym {
            keysym: 0x1000dad,
            offset: 23881,
        },
        name_keysym {
            keysym: 0x1000dae,
            offset: 23890,
        },
        name_keysym {
            keysym: 0x1000da7,
            offset: 23900,
        },
        name_keysym {
            keysym: 0x1000da8,
            offset: 23909,
        },
        name_keysym {
            keysym: 0x1000d8b,
            offset: 23919,
        },
        name_keysym {
            keysym: 0x1000dd4,
            offset: 23926,
        },
        name_keysym {
            keysym: 0x1000d8c,
            offset: 23934,
        },
        name_keysym {
            keysym: 0x1000dd6,
            offset: 23942,
        },
        name_keysym {
            keysym: 0x1000dc0,
            offset: 23951,
        },
        name_keysym {
            keysym: 0x1000dba,
            offset: 23959,
        },
        name_keysym {
            keysym: 0x1002086,
            offset: 23967,
        },
        name_keysym {
            keysym: 0x1002076,
            offset: 23980,
        },
        name_keysym {
            keysym: 0x2f,
            offset: 23992,
        },
        name_keysym {
            keysym: 0xfe73,
            offset: 23998,
        },
        name_keysym {
            keysym: 0x9e0,
            offset: 24014,
        },
        name_keysym {
            keysym: 0x20,
            offset: 24027,
        },
        name_keysym {
            keysym: 0x100221a,
            offset: 24033,
        },
        name_keysym {
            keysym: 0x1001e9e,
            offset: 24044,
        },
        name_keysym {
            keysym: 0xdf,
            offset: 24051,
        },
        name_keysym {
            keysym: 0xa3,
            offset: 24058,
        },
        name_keysym {
            keysym: 0xfe75,
            offset: 24067,
        },
        name_keysym {
            keysym: 0x1002263,
            offset: 24085,
        },
        name_keysym {
            keysym: 0xff66,
            offset: 24094,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 24103,
        },
        name_keysym {
            keysym: 0x1005ff77,
            offset: 24115,
        },
        name_keysym {
            keysym: 0x1005ff78,
            offset: 24135,
        },
        name_keysym {
            keysym: 0x1005ff79,
            offset: 24148,
        },
        name_keysym {
            keysym: 0xff20,
            offset: 24168,
        },
        name_keysym {
            keysym: 0x1005ff72,
            offset: 24179,
        },
        name_keysym {
            keysym: 0x1005ff75,
            offset: 24187,
        },
        name_keysym {
            keysym: 0x1005ff10,
            offset: 24194,
        },
        name_keysym {
            keysym: 0x1005ff11,
            offset: 24201,
        },
        name_keysym {
            keysym: 0x1005ff03,
            offset: 24208,
        },
        name_keysym {
            keysym: 0x1005ff05,
            offset: 24220,
        },
        name_keysym {
            keysym: 0x1005ff01,
            offset: 24234,
        },
        name_keysym {
            keysym: 0x1005ff04,
            offset: 24247,
        },
        name_keysym {
            keysym: 0x1005ff00,
            offset: 24263,
        },
        name_keysym {
            keysym: 0x1005ff02,
            offset: 24275,
        },
        name_keysym {
            keysym: 0xff68,
            offset: 24287,
        },
        name_keysym {
            keysym: 0x1005ff71,
            offset: 24295,
        },
        name_keysym {
            keysym: 0x1005ff73,
            offset: 24304,
        },
        name_keysym {
            keysym: 0xff56,
            offset: 24312,
        },
        name_keysym {
            keysym: 0xff55,
            offset: 24324,
        },
        name_keysym {
            keysym: 0x1005ff74,
            offset: 24334,
        },
        name_keysym {
            keysym: 0x1005ff76,
            offset: 24343,
        },
        name_keysym {
            keysym: 0x1005ff7d,
            offset: 24358,
        },
        name_keysym {
            keysym: 0xff61,
            offset: 24378,
        },
        name_keysym {
            keysym: 0x1005ff70,
            offset: 24394,
        },
        name_keysym {
            keysym: 0xff69,
            offset: 24403,
        },
        name_keysym {
            keysym: 0x1005ff60,
            offset: 24411,
        },
        name_keysym {
            keysym: 0xff65,
            offset: 24422,
        },
        name_keysym {
            keysym: 0x1005ff7a,
            offset: 24430,
        },
        name_keysym {
            keysym: 0x1005ff7b,
            offset: 24446,
        },
        name_keysym {
            keysym: 0x1005ff7c,
            offset: 24470,
        },
        name_keysym {
            keysym: 0xffeb,
            offset: 24494,
        },
        name_keysym {
            keysym: 0xffec,
            offset: 24502,
        },
        name_keysym {
            keysym: 0xff15,
            offset: 24510,
        },
        name_keysym {
            keysym: 0x1000ff6d,
            offset: 24518,
        },
        name_keysym {
            keysym: 0x54,
            offset: 24525,
        },
        name_keysym {
            keysym: 0x74,
            offset: 24527,
        },
        name_keysym {
            keysym: 0xff09,
            offset: 24529,
        },
        name_keysym {
            keysym: 0x1001e6a,
            offset: 24533,
        },
        name_keysym {
            keysym: 0x1001e6b,
            offset: 24543,
        },
        name_keysym {
            keysym: 0x1ab,
            offset: 24553,
        },
        name_keysym {
            keysym: 0x1bb,
            offset: 24560,
        },
        name_keysym {
            keysym: 0x1de,
            offset: 24567,
        },
        name_keysym {
            keysym: 0x1fe,
            offset: 24576,
        },
        name_keysym {
            keysym: 0xaf9,
            offset: 24585,
        },
        name_keysym {
            keysym: 0xafa,
            offset: 24595,
        },
        name_keysym {
            keysym: 0xfed5,
            offset: 24613,
        },
        name_keysym {
            keysym: 0xddf,
            offset: 24630,
        },
        name_keysym {
            keysym: 0xdba,
            offset: 24640,
        },
        name_keysym {
            keysym: 0xda8,
            offset: 24654,
        },
        name_keysym {
            keysym: 0xdaa,
            offset: 24667,
        },
        name_keysym {
            keysym: 0xda9,
            offset: 24681,
        },
        name_keysym {
            keysym: 0xdac,
            offset: 24695,
        },
        name_keysym {
            keysym: 0xdae,
            offset: 24708,
        },
        name_keysym {
            keysym: 0xdb4,
            offset: 24721,
        },
        name_keysym {
            keysym: 0xdbd,
            offset: 24732,
        },
        name_keysym {
            keysym: 0xdbf,
            offset: 24742,
        },
        name_keysym {
            keysym: 0xdcb,
            offset: 24753,
        },
        name_keysym {
            keysym: 0xdce,
            offset: 24764,
        },
        name_keysym {
            keysym: 0xda2,
            offset: 24778,
        },
        name_keysym {
            keysym: 0xda5,
            offset: 24791,
        },
        name_keysym {
            keysym: 0xda3,
            offset: 24804,
        },
        name_keysym {
            keysym: 0xda4,
            offset: 24818,
        },
        name_keysym {
            keysym: 0xda6,
            offset: 24832,
        },
        name_keysym {
            keysym: 0xda1,
            offset: 24848,
        },
        name_keysym {
            keysym: 0xde5,
            offset: 24859,
        },
        name_keysym {
            keysym: 0xdf7,
            offset: 24876,
        },
        name_keysym {
            keysym: 0xdf5,
            offset: 24889,
        },
        name_keysym {
            keysym: 0xdf6,
            offset: 24900,
        },
        name_keysym {
            keysym: 0xdf9,
            offset: 24912,
        },
        name_keysym {
            keysym: 0xdf1,
            offset: 24924,
        },
        name_keysym {
            keysym: 0xdf8,
            offset: 24937,
        },
        name_keysym {
            keysym: 0xdf3,
            offset: 24950,
        },
        name_keysym {
            keysym: 0xdf4,
            offset: 24962,
        },
        name_keysym {
            keysym: 0xdf2,
            offset: 24973,
        },
        name_keysym {
            keysym: 0xdf0,
            offset: 24986,
        },
        name_keysym {
            keysym: 0xdcc,
            offset: 24998,
        },
        name_keysym {
            keysym: 0xdc5,
            offset: 25011,
        },
        name_keysym {
            keysym: 0xdc6,
            offset: 25023,
        },
        name_keysym {
            keysym: 0xdeb,
            offset: 25031,
        },
        name_keysym {
            keysym: 0xde8,
            offset: 25048,
        },
        name_keysym {
            keysym: 0xdd1,
            offset: 25059,
        },
        name_keysym {
            keysym: 0xdde,
            offset: 25075,
        },
        name_keysym {
            keysym: 0xde7,
            offset: 25098,
        },
        name_keysym {
            keysym: 0xde9,
            offset: 25113,
        },
        name_keysym {
            keysym: 0xdea,
            offset: 25125,
        },
        name_keysym {
            keysym: 0xde6,
            offset: 25137,
        },
        name_keysym {
            keysym: 0xdc1,
            offset: 25151,
        },
        name_keysym {
            keysym: 0xda7,
            offset: 25161,
        },
        name_keysym {
            keysym: 0xded,
            offset: 25173,
        },
        name_keysym {
            keysym: 0xdb3,
            offset: 25187,
        },
        name_keysym {
            keysym: 0xdb9,
            offset: 25198,
        },
        name_keysym {
            keysym: 0xdcd,
            offset: 25208,
        },
        name_keysym {
            keysym: 0xdcf,
            offset: 25218,
        },
        name_keysym {
            keysym: 0xdda,
            offset: 25233,
        },
        name_keysym {
            keysym: 0xdbe,
            offset: 25246,
        },
        name_keysym {
            keysym: 0xdbc,
            offset: 25259,
        },
        name_keysym {
            keysym: 0xdc0,
            offset: 25273,
        },
        name_keysym {
            keysym: 0xdbb,
            offset: 25289,
        },
        name_keysym {
            keysym: 0xdc3,
            offset: 25300,
        },
        name_keysym {
            keysym: 0xdc4,
            offset: 25311,
        },
        name_keysym {
            keysym: 0xdd0,
            offset: 25319,
        },
        name_keysym {
            keysym: 0xdd2,
            offset: 25330,
        },
        name_keysym {
            keysym: 0xde1,
            offset: 25342,
        },
        name_keysym {
            keysym: 0xde4,
            offset: 25354,
        },
        name_keysym {
            keysym: 0xde3,
            offset: 25374,
        },
        name_keysym {
            keysym: 0xdd3,
            offset: 25393,
        },
        name_keysym {
            keysym: 0xde0,
            offset: 25405,
        },
        name_keysym {
            keysym: 0xdd4,
            offset: 25416,
        },
        name_keysym {
            keysym: 0xdd5,
            offset: 25427,
        },
        name_keysym {
            keysym: 0xde2,
            offset: 25439,
        },
        name_keysym {
            keysym: 0xdd8,
            offset: 25450,
        },
        name_keysym {
            keysym: 0xdd6,
            offset: 25461,
        },
        name_keysym {
            keysym: 0xdd7,
            offset: 25473,
        },
        name_keysym {
            keysym: 0xdd9,
            offset: 25486,
        },
        name_keysym {
            keysym: 0xdc9,
            offset: 25498,
        },
        name_keysym {
            keysym: 0xdc8,
            offset: 25510,
        },
        name_keysym {
            keysym: 0xdab,
            offset: 25522,
        },
        name_keysym {
            keysym: 0xdca,
            offset: 25532,
        },
        name_keysym {
            keysym: 0xdec,
            offset: 25543,
        },
        name_keysym {
            keysym: 0xdb1,
            offset: 25560,
        },
        name_keysym {
            keysym: 0xdb2,
            offset: 25579,
        },
        name_keysym {
            keysym: 0xdb7,
            offset: 25595,
        },
        name_keysym {
            keysym: 0xdb0,
            offset: 25610,
        },
        name_keysym {
            keysym: 0xdb8,
            offset: 25623,
        },
        name_keysym {
            keysym: 0xdb6,
            offset: 25637,
        },
        name_keysym {
            keysym: 0xdaf,
            offset: 25651,
        },
        name_keysym {
            keysym: 0xdb5,
            offset: 25664,
        },
        name_keysym {
            keysym: 0xdc7,
            offset: 25675,
        },
        name_keysym {
            keysym: 0xdc2,
            offset: 25687,
        },
        name_keysym {
            keysym: 0xdad,
            offset: 25698,
        },
        name_keysym {
            keysym: 0x8c0,
            offset: 25710,
        },
        name_keysym {
            keysym: 0xaa7,
            offset: 25720,
        },
        name_keysym {
            keysym: 0xde,
            offset: 25730,
        },
        name_keysym {
            keysym: 0xde,
            offset: 25736,
        },
        name_keysym {
            keysym: 0xfe,
            offset: 25742,
        },
        name_keysym {
            keysym: 0xac4,
            offset: 25748,
        },
        name_keysym {
            keysym: 0xab4,
            offset: 25761,
        },
        name_keysym {
            keysym: 0xbe,
            offset: 25773,
        },
        name_keysym {
            keysym: 0x1002083,
            offset: 25787,
        },
        name_keysym {
            keysym: 0xb3,
            offset: 25802,
        },
        name_keysym {
            keysym: 0x100222d,
            offset: 25816,
        },
        name_keysym {
            keysym: 0x8a4,
            offset: 25826,
        },
        name_keysym {
            keysym: 0x8ab,
            offset: 25838,
        },
        name_keysym {
            keysym: 0x8a2,
            offset: 25852,
        },
        name_keysym {
            keysym: 0x8a7,
            offset: 25867,
        },
        name_keysym {
            keysym: 0x8b1,
            offset: 25884,
        },
        name_keysym {
            keysym: 0x8ad,
            offset: 25901,
        },
        name_keysym {
            keysym: 0x8a9,
            offset: 25916,
        },
        name_keysym {
            keysym: 0x8b5,
            offset: 25934,
        },
        name_keysym {
            keysym: 0x9f7,
            offset: 25952,
        },
        name_keysym {
            keysym: 0x8b3,
            offset: 25957,
        },
        name_keysym {
            keysym: 0xff2b,
            offset: 25983,
        },
        name_keysym {
            keysym: 0xac9,
            offset: 25991,
        },
        name_keysym {
            keysym: 0xacb,
            offset: 26001,
        },
        name_keysym {
            keysym: 0x3ac,
            offset: 26019,
        },
        name_keysym {
            keysym: 0x3bc,
            offset: 26026,
        },
        name_keysym {
            keysym: 0xab3,
            offset: 26033,
        },
        name_keysym {
            keysym: 0x1002082,
            offset: 26043,
        },
        name_keysym {
            keysym: 0xb2,
            offset: 26056,
        },
        name_keysym {
            keysym: 0xab1,
            offset: 26068,
        },
        name_keysym {
            keysym: 0x55,
            offset: 26078,
        },
        name_keysym {
            keysym: 0x75,
            offset: 26080,
        },
        name_keysym {
            keysym: 0xda,
            offset: 26082,
        },
        name_keysym {
            keysym: 0xfa,
            offset: 26089,
        },
        name_keysym {
            keysym: 0x1001ee4,
            offset: 26096,
        },
        name_keysym {
            keysym: 0x1001ee5,
            offset: 26106,
        },
        name_keysym {
            keysym: 0x2dd,
            offset: 26116,
        },
        name_keysym {
            keysym: 0x2fd,
            offset: 26123,
        },
        name_keysym {
            keysym: 0xdb,
            offset: 26130,
        },
        name_keysym {
            keysym: 0xfb,
            offset: 26142,
        },
        name_keysym {
            keysym: 0xdc,
            offset: 26154,
        },
        name_keysym {
            keysym: 0xfc,
            offset: 26165,
        },
        name_keysym {
            keysym: 0x1db,
            offset: 26176,
        },
        name_keysym {
            keysym: 0x1fb,
            offset: 26189,
        },
        name_keysym {
            keysym: 0xd9,
            offset: 26202,
        },
        name_keysym {
            keysym: 0xf9,
            offset: 26209,
        },
        name_keysym {
            keysym: 0x1001ee6,
            offset: 26216,
        },
        name_keysym {
            keysym: 0x1001ee7,
            offset: 26222,
        },
        name_keysym {
            keysym: 0x10001af,
            offset: 26228,
        },
        name_keysym {
            keysym: 0x10001b0,
            offset: 26234,
        },
        name_keysym {
            keysym: 0x1001ee8,
            offset: 26240,
        },
        name_keysym {
            keysym: 0x1001ee9,
            offset: 26251,
        },
        name_keysym {
            keysym: 0x1001ef0,
            offset: 26262,
        },
        name_keysym {
            keysym: 0x1001ef1,
            offset: 26276,
        },
        name_keysym {
            keysym: 0x1001eea,
            offset: 26290,
        },
        name_keysym {
            keysym: 0x1001eeb,
            offset: 26301,
        },
        name_keysym {
            keysym: 0x1001eec,
            offset: 26312,
        },
        name_keysym {
            keysym: 0x1001eed,
            offset: 26322,
        },
        name_keysym {
            keysym: 0x1001eee,
            offset: 26332,
        },
        name_keysym {
            keysym: 0x1001eef,
            offset: 26343,
        },
        name_keysym {
            keysym: 0x6bd,
            offset: 26354,
        },
        name_keysym {
            keysym: 0x6ad,
            offset: 26380,
        },
        name_keysym {
            keysym: 0x6b6,
            offset: 26406,
        },
        name_keysym {
            keysym: 0x6a6,
            offset: 26418,
        },
        name_keysym {
            keysym: 0x6b4,
            offset: 26430,
        },
        name_keysym {
            keysym: 0x6a4,
            offset: 26443,
        },
        name_keysym {
            keysym: 0x6b7,
            offset: 26456,
        },
        name_keysym {
            keysym: 0x6a7,
            offset: 26469,
        },
        name_keysym {
            keysym: 0x6b6,
            offset: 26482,
        },
        name_keysym {
            keysym: 0x6a6,
            offset: 26493,
        },
        name_keysym {
            keysym: 0x6b4,
            offset: 26504,
        },
        name_keysym {
            keysym: 0x6a4,
            offset: 26516,
        },
        name_keysym {
            keysym: 0x6b7,
            offset: 26528,
        },
        name_keysym {
            keysym: 0x6a7,
            offset: 26540,
        },
        name_keysym {
            keysym: 0x3de,
            offset: 26552,
        },
        name_keysym {
            keysym: 0x3fe,
            offset: 26560,
        },
        name_keysym {
            keysym: 0xbc6,
            offset: 26568,
        },
        name_keysym {
            keysym: 0x5f,
            offset: 26577,
        },
        name_keysym {
            keysym: 0xff65,
            offset: 26588,
        },
        name_keysym {
            keysym: 0x8dd,
            offset: 26593,
        },
        name_keysym {
            keysym: 0x3d9,
            offset: 26599,
        },
        name_keysym {
            keysym: 0x3f9,
            offset: 26607,
        },
        name_keysym {
            keysym: 0xff52,
            offset: 26615,
        },
        name_keysym {
            keysym: 0x8fc,
            offset: 26618,
        },
        name_keysym {
            keysym: 0xba9,
            offset: 26626,
        },
        name_keysym {
            keysym: 0x9ec,
            offset: 26634,
        },
        name_keysym {
            keysym: 0x9eb,
            offset: 26647,
        },
        name_keysym {
            keysym: 0xbc3,
            offset: 26661,
        },
        name_keysym {
            keysym: 0xbd3,
            offset: 26668,
        },
        name_keysym {
            keysym: 0xbce,
            offset: 26676,
        },
        name_keysym {
            keysym: 0x1d9,
            offset: 26683,
        },
        name_keysym {
            keysym: 0x1f9,
            offset: 26689,
        },
        name_keysym {
            keysym: 0x1000ff6e,
            offset: 26695,
        },
        name_keysym {
            keysym: 0x3dd,
            offset: 26700,
        },
        name_keysym {
            keysym: 0x3fd,
            offset: 26707,
        },
        name_keysym {
            keysym: 0x56,
            offset: 26714,
        },
        name_keysym {
            keysym: 0x76,
            offset: 26716,
        },
        name_keysym {
            keysym: 0x8c1,
            offset: 26718,
        },
        name_keysym {
            keysym: 0x9f8,
            offset: 26728,
        },
        name_keysym {
            keysym: 0x8a6,
            offset: 26736,
        },
        name_keysym {
            keysym: 0x4de,
            offset: 26750,
        },
        name_keysym {
            keysym: 0xffffff,
            offset: 26762,
        },
        name_keysym {
            keysym: 0x9e9,
            offset: 26773,
        },
        name_keysym {
            keysym: 0x57,
            offset: 26776,
        },
        name_keysym {
            keysym: 0x77,
            offset: 26778,
        },
        name_keysym {
            keysym: 0x1001e82,
            offset: 26780,
        },
        name_keysym {
            keysym: 0x1001e83,
            offset: 26787,
        },
        name_keysym {
            keysym: 0x1000174,
            offset: 26794,
        },
        name_keysym {
            keysym: 0x1000175,
            offset: 26806,
        },
        name_keysym {
            keysym: 0x1001e84,
            offset: 26818,
        },
        name_keysym {
            keysym: 0x1001e85,
            offset: 26829,
        },
        name_keysym {
            keysym: 0x1001e80,
            offset: 26840,
        },
        name_keysym {
            keysym: 0x1001e81,
            offset: 26847,
        },
        name_keysym {
            keysym: 0x10020a9,
            offset: 26854,
        },
        name_keysym {
            keysym: 0x58,
            offset: 26862,
        },
        name_keysym {
            keysym: 0x78,
            offset: 26864,
        },
        name_keysym {
            keysym: 0x1001e8a,
            offset: 26866,
        },
        name_keysym {
            keysym: 0x1001e8b,
            offset: 26876,
        },
        name_keysym {
            keysym: 0x100811b9,
            offset: 26886,
        },
        name_keysym {
            keysym: 0x100811b8,
            offset: 26905,
        },
        name_keysym {
            keysym: 0x1008126f,
            offset: 26922,
        },
        name_keysym {
            keysym: 0x1008124e,
            offset: 26933,
        },
        name_keysym {
            keysym: 0x1008ff39,
            offset: 26951,
        },
        name_keysym {
            keysym: 0x100811ad,
            offset: 26967,
        },
        name_keysym {
            keysym: 0x10081230,
            offset: 26983,
        },
        name_keysym {
            keysym: 0x1008ff50,
            offset: 26997,
        },
        name_keysym {
            keysym: 0x1008ff51,
            offset: 27017,
        },
        name_keysym {
            keysym: 0x10081244,
            offset: 27038,
        },
        name_keysym {
            keysym: 0x10081177,
            offset: 27052,
        },
        name_keysym {
            keysym: 0x10081247,
            offset: 27068,
        },
        name_keysym {
            keysym: 0x1008121c,
            offset: 27082,
        },
        name_keysym {
            keysym: 0x1008121b,
            offset: 27099,
        },
        name_keysym {
            keysym: 0x1008121d,
            offset: 27115,
        },
        name_keysym {
            keysym: 0x10081188,
            offset: 27135,
        },
        name_keysym {
            keysym: 0x100810d1,
            offset: 27145,
        },
        name_keysym {
            keysym: 0x10081175,
            offset: 27164,
        },
        name_keysym {
            keysym: 0x1008ff9b,
            offset: 27185,
        },
        name_keysym {
            keysym: 0x1008126e,
            offset: 27205,
        },
        name_keysym {
            keysym: 0x1008ff97,
            offset: 27219,
        },
        name_keysym {
            keysym: 0x1008ff11,
            offset: 27236,
        },
        name_keysym {
            keysym: 0x1008ff32,
            offset: 27257,
        },
        name_keysym {
            keysym: 0x1008ffb2,
            offset: 27272,
        },
        name_keysym {
            keysym: 0x1008ff12,
            offset: 27289,
        },
        name_keysym {
            keysym: 0x1008ff17,
            offset: 27303,
        },
        name_keysym {
            keysym: 0x1008ff31,
            offset: 27317,
        },
        name_keysym {
            keysym: 0x1008ff14,
            offset: 27332,
        },
        name_keysym {
            keysym: 0x1008ffb6,
            offset: 27346,
        },
        name_keysym {
            keysym: 0x1008ff16,
            offset: 27362,
        },
        name_keysym {
            keysym: 0x1008ff13,
            offset: 27376,
        },
        name_keysym {
            keysym: 0x1008ff99,
            offset: 27397,
        },
        name_keysym {
            keysym: 0x1008ff1c,
            offset: 27417,
        },
        name_keysym {
            keysym: 0x1008ff98,
            offset: 27433,
        },
        name_keysym {
            keysym: 0x1008ff3e,
            offset: 27449,
        },
        name_keysym {
            keysym: 0x1008ff15,
            offset: 27465,
        },
        name_keysym {
            keysym: 0x1008127d,
            offset: 27479,
        },
        name_keysym {
            keysym: 0x1008ff8d,
            offset: 27505,
        },
        name_keysym {
            keysym: 0x1008ff26,
            offset: 27514,
        },
        name_keysym {
            keysym: 0x1008ff3f,
            offset: 27523,
        },
        name_keysym {
            keysym: 0x1008ff93,
            offset: 27539,
        },
        name_keysym {
            keysym: 0x1008ffa6,
            offset: 27551,
        },
        name_keysym {
            keysym: 0x1008ff94,
            offset: 27560,
        },
        name_keysym {
            keysym: 0x1008ff52,
            offset: 27574,
        },
        name_keysym {
            keysym: 0x1008119b,
            offset: 27583,
        },
        name_keysym {
            keysym: 0x1008ff3b,
            offset: 27593,
        },
        name_keysym {
            keysym: 0x100810f4,
            offset: 27614,
        },
        name_keysym {
            keysym: 0x10081251,
            offset: 27633,
        },
        name_keysym {
            keysym: 0x10081250,
            offset: 27651,
        },
        name_keysym {
            keysym: 0x10081240,
            offset: 27669,
        },
        name_keysym {
            keysym: 0x1008ff54,
            offset: 27686,
        },
        name_keysym {
            keysym: 0x1008ff1d,
            offset: 27701,
        },
        name_keysym {
            keysym: 0x1008ff20,
            offset: 27716,
        },
        name_keysym {
            keysym: 0x1008124c,
            offset: 27729,
        },
        name_keysym {
            keysym: 0x1008124b,
            offset: 27753,
        },
        name_keysym {
            keysym: 0x1008124d,
            offset: 27776,
        },
        name_keysym {
            keysym: 0x10081218,
            offset: 27799,
        },
        name_keysym {
            keysym: 0x10081210,
            offset: 27814,
        },
        name_keysym {
            keysym: 0x10081219,
            offset: 27830,
        },
        name_keysym {
            keysym: 0x1008121a,
            offset: 27845,
        },
        name_keysym {
            keysym: 0x10081217,
            offset: 27861,
        },
        name_keysym {
            keysym: 0x10081215,
            offset: 27874,
        },
        name_keysym {
            keysym: 0x10081216,
            offset: 27891,
        },
        name_keysym {
            keysym: 0x1008ff53,
            offset: 27909,
        },
        name_keysym {
            keysym: 0x10081193,
            offset: 27916,
        },
        name_keysym {
            keysym: 0x10081192,
            offset: 27932,
        },
        name_keysym {
            keysym: 0x1008ff55,
            offset: 27946,
        },
        name_keysym {
            keysym: 0x1008fe21,
            offset: 27956,
        },
        name_keysym {
            keysym: 0x10081286,
            offset: 27970,
        },
        name_keysym {
            keysym: 0x1008ff56,
            offset: 27987,
        },
        name_keysym {
            keysym: 0x1008ff3d,
            offset: 27997,
        },
        name_keysym {
            keysym: 0x100811b6,
            offset: 28011,
        },
        name_keysym {
            keysym: 0x1008ff22,
            offset: 28027,
        },
        name_keysym {
            keysym: 0x10081243,
            offset: 28046,
        },
        name_keysym {
            keysym: 0x1008ff57,
            offset: 28063,
        },
        name_keysym {
            keysym: 0x1008ff58,
            offset: 28072,
        },
        name_keysym {
            keysym: 0x1008ff9c,
            offset: 28080,
        },
        name_keysym {
            keysym: 0x10081277,
            offset: 28095,
        },
        name_keysym {
            keysym: 0x100811aa,
            offset: 28104,
        },
        name_keysym {
            keysym: 0x1008124a,
            offset: 28117,
        },
        name_keysym {
            keysym: 0x1008ff59,
            offset: 28129,
        },
        name_keysym {
            keysym: 0x100810f5,
            offset: 28141,
        },
        name_keysym {
            keysym: 0x100811af,
            offset: 28156,
        },
        name_keysym {
            keysym: 0x1008ff5b,
            offset: 28174,
        },
        name_keysym {
            keysym: 0x1008124f,
            offset: 28188,
        },
        name_keysym {
            keysym: 0x1008ff5a,
            offset: 28205,
        },
        name_keysym {
            keysym: 0x10081283,
            offset: 28213,
        },
        name_keysym {
            keysym: 0x10081185,
            offset: 28232,
        },
        name_keysym {
            keysym: 0x100811a6,
            offset: 28240,
        },
        name_keysym {
            keysym: 0x1008ff2c,
            offset: 28251,
        },
        name_keysym {
            keysym: 0x10081253,
            offset: 28261,
        },
        name_keysym {
            keysym: 0x10081252,
            offset: 28292,
        },
        name_keysym {
            keysym: 0x10081249,
            offset: 28322,
        },
        name_keysym {
            keysym: 0x1008ff5c,
            offset: 28338,
        },
        name_keysym {
            keysym: 0x100810ae,
            offset: 28348,
        },
        name_keysym {
            keysym: 0x1008ff5d,
            offset: 28357,
        },
        name_keysym {
            keysym: 0x10081275,
            offset: 28370,
        },
        name_keysym {
            keysym: 0x1008ff30,
            offset: 28386,
        },
        name_keysym {
            keysym: 0x1008ff3c,
            offset: 28400,
        },
        name_keysym {
            keysym: 0x10081281,
            offset: 28412,
        },
        name_keysym {
            keysym: 0x100811d0,
            offset: 28429,
        },
        name_keysym {
            keysym: 0x100811de,
            offset: 28436,
        },
        name_keysym {
            keysym: 0x100811df,
            offset: 28445,
        },
        name_keysym {
            keysym: 0x100811e4,
            offset: 28454,
        },
        name_keysym {
            keysym: 0x100811e0,
            offset: 28463,
        },
        name_keysym {
            keysym: 0x100811e1,
            offset: 28472,
        },
        name_keysym {
            keysym: 0x100811d1,
            offset: 28481,
        },
        name_keysym {
            keysym: 0x100811e2,
            offset: 28492,
        },
        name_keysym {
            keysym: 0x100811d2,
            offset: 28501,
        },
        name_keysym {
            keysym: 0x100811db,
            offset: 28511,
        },
        name_keysym {
            keysym: 0x100811dc,
            offset: 28522,
        },
        name_keysym {
            keysym: 0x100811dd,
            offset: 28533,
        },
        name_keysym {
            keysym: 0x100811d3,
            offset: 28544,
        },
        name_keysym {
            keysym: 0x100811d4,
            offset: 28554,
        },
        name_keysym {
            keysym: 0x100811d5,
            offset: 28564,
        },
        name_keysym {
            keysym: 0x100811d6,
            offset: 28574,
        },
        name_keysym {
            keysym: 0x100811d7,
            offset: 28584,
        },
        name_keysym {
            keysym: 0x100811d8,
            offset: 28594,
        },
        name_keysym {
            keysym: 0x100811d9,
            offset: 28604,
        },
        name_keysym {
            keysym: 0x100811da,
            offset: 28614,
        },
        name_keysym {
            keysym: 0x100811e3,
            offset: 28624,
        },
        name_keysym {
            keysym: 0x100811e5,
            offset: 28633,
        },
        name_keysym {
            keysym: 0x1008ff27,
            offset: 28650,
        },
        name_keysym {
            keysym: 0x1008ff9d,
            offset: 28662,
        },
        name_keysym {
            keysym: 0x1008ff9e,
            offset: 28676,
        },
        name_keysym {
            keysym: 0x1008ffb8,
            offset: 28693,
        },
        name_keysym {
            keysym: 0x1008ff5e,
            offset: 28708,
        },
        name_keysym {
            keysym: 0x1008ff5f,
            offset: 28717,
        },
        name_keysym {
            keysym: 0x10081162,
            offset: 28724,
        },
        name_keysym {
            keysym: 0x100811a8,
            offset: 28733,
        },
        name_keysym {
            keysym: 0x1008ffa4,
            offset: 28752,
        },
        name_keysym {
            keysym: 0x100811be,
            offset: 28762,
        },
        name_keysym {
            keysym: 0x1008ffa8,
            offset: 28778,
        },
        name_keysym {
            keysym: 0x1008ff37,
            offset: 28792,
        },
        name_keysym {
            keysym: 0x1008ff18,
            offset: 28804,
        },
        name_keysym {
            keysym: 0x1008ff3a,
            offset: 28817,
        },
        name_keysym {
            keysym: 0x100811ba,
            offset: 28830,
        },
        name_keysym {
            keysym: 0x10081166,
            offset: 28841,
        },
        name_keysym {
            keysym: 0x1008ff60,
            offset: 28850,
        },
        name_keysym {
            keysym: 0x10081242,
            offset: 28861,
        },
        name_keysym {
            keysym: 0x1008ff06,
            offset: 28873,
        },
        name_keysym {
            keysym: 0x1008ff05,
            offset: 28895,
        },
        name_keysym {
            keysym: 0x10081264,
            offset: 28915,
        },
        name_keysym {
            keysym: 0x10081265,
            offset: 28940,
        },
        name_keysym {
            keysym: 0x10081261,
            offset: 28965,
        },
        name_keysym {
            keysym: 0x10081263,
            offset: 28988,
        },
        name_keysym {
            keysym: 0x10081260,
            offset: 29016,
        },
        name_keysym {
            keysym: 0x10081262,
            offset: 29039,
        },
        name_keysym {
            keysym: 0x100812b8,
            offset: 29067,
        },
        name_keysym {
            keysym: 0x100812b9,
            offset: 29083,
        },
        name_keysym {
            keysym: 0x100812ba,
            offset: 29099,
        },
        name_keysym {
            keysym: 0x100812bb,
            offset: 29115,
        },
        name_keysym {
            keysym: 0x100812bc,
            offset: 29131,
        },
        name_keysym {
            keysym: 0x1008ff04,
            offset: 29147,
        },
        name_keysym {
            keysym: 0x1008ffb3,
            offset: 29165,
        },
        name_keysym {
            keysym: 0x1008ff40,
            offset: 29178,
        },
        name_keysym {
            keysym: 0x1008ff41,
            offset: 29190,
        },
        name_keysym {
            keysym: 0x1008ff42,
            offset: 29202,
        },
        name_keysym {
            keysym: 0x1008ff43,
            offset: 29214,
        },
        name_keysym {
            keysym: 0x1008ff44,
            offset: 29226,
        },
        name_keysym {
            keysym: 0x1008ff45,
            offset: 29238,
        },
        name_keysym {
            keysym: 0x1008ff46,
            offset: 29250,
        },
        name_keysym {
            keysym: 0x1008ff47,
            offset: 29262,
        },
        name_keysym {
            keysym: 0x1008ff48,
            offset: 29274,
        },
        name_keysym {
            keysym: 0x1008ff49,
            offset: 29286,
        },
        name_keysym {
            keysym: 0x1008ff4a,
            offset: 29298,
        },
        name_keysym {
            keysym: 0x1008ff4b,
            offset: 29310,
        },
        name_keysym {
            keysym: 0x1008ff4c,
            offset: 29322,
        },
        name_keysym {
            keysym: 0x1008ff4d,
            offset: 29334,
        },
        name_keysym {
            keysym: 0x1008ff4e,
            offset: 29346,
        },
        name_keysym {
            keysym: 0x1008ff4f,
            offset: 29358,
        },
        name_keysym {
            keysym: 0x10081269,
            offset: 29370,
        },
        name_keysym {
            keysym: 0x10081268,
            offset: 29383,
        },
        name_keysym {
            keysym: 0x1008ff35,
            offset: 29394,
        },
        name_keysym {
            keysym: 0x1008121e,
            offset: 29408,
        },
        name_keysym {
            keysym: 0x100811bf,
            offset: 29425,
        },
        name_keysym {
            keysym: 0x1008fe25,
            offset: 29439,
        },
        name_keysym {
            keysym: 0x1008ff61,
            offset: 29455,
        },
        name_keysym {
            keysym: 0x1008fe24,
            offset: 29466,
        },
        name_keysym {
            keysym: 0x10081290,
            offset: 29484,
        },
        name_keysym {
            keysym: 0x10081299,
            offset: 29495,
        },
        name_keysym {
            keysym: 0x1008129a,
            offset: 29507,
        },
        name_keysym {
            keysym: 0x1008129b,
            offset: 29519,
        },
        name_keysym {
            keysym: 0x1008129c,
            offset: 29531,
        },
        name_keysym {
            keysym: 0x1008129d,
            offset: 29543,
        },
        name_keysym {
            keysym: 0x1008129e,
            offset: 29555,
        },
        name_keysym {
            keysym: 0x1008129f,
            offset: 29567,
        },
        name_keysym {
            keysym: 0x100812a0,
            offset: 29579,
        },
        name_keysym {
            keysym: 0x100812a1,
            offset: 29591,
        },
        name_keysym {
            keysym: 0x100812a2,
            offset: 29603,
        },
        name_keysym {
            keysym: 0x10081291,
            offset: 29615,
        },
        name_keysym {
            keysym: 0x100812a3,
            offset: 29626,
        },
        name_keysym {
            keysym: 0x100812a4,
            offset: 29638,
        },
        name_keysym {
            keysym: 0x100812a5,
            offset: 29650,
        },
        name_keysym {
            keysym: 0x100812a6,
            offset: 29662,
        },
        name_keysym {
            keysym: 0x100812a7,
            offset: 29674,
        },
        name_keysym {
            keysym: 0x100812a8,
            offset: 29686,
        },
        name_keysym {
            keysym: 0x100812a9,
            offset: 29698,
        },
        name_keysym {
            keysym: 0x100812aa,
            offset: 29710,
        },
        name_keysym {
            keysym: 0x100812ab,
            offset: 29722,
        },
        name_keysym {
            keysym: 0x100812ac,
            offset: 29734,
        },
        name_keysym {
            keysym: 0x10081292,
            offset: 29746,
        },
        name_keysym {
            keysym: 0x100812ad,
            offset: 29757,
        },
        name_keysym {
            keysym: 0x10081293,
            offset: 29769,
        },
        name_keysym {
            keysym: 0x10081294,
            offset: 29780,
        },
        name_keysym {
            keysym: 0x10081295,
            offset: 29791,
        },
        name_keysym {
            keysym: 0x10081296,
            offset: 29802,
        },
        name_keysym {
            keysym: 0x10081297,
            offset: 29813,
        },
        name_keysym {
            keysym: 0x10081298,
            offset: 29824,
        },
        name_keysym {
            keysym: 0x100812b3,
            offset: 29835,
        },
        name_keysym {
            keysym: 0x100812b4,
            offset: 29852,
        },
        name_keysym {
            keysym: 0x100812b5,
            offset: 29869,
        },
        name_keysym {
            keysym: 0x100812b2,
            offset: 29886,
        },
        name_keysym {
            keysym: 0x100812b0,
            offset: 29907,
        },
        name_keysym {
            keysym: 0x100812b1,
            offset: 29928,
        },
        name_keysym {
            keysym: 0x1008ff19,
            offset: 29948,
        },
        name_keysym {
            keysym: 0x1008ff90,
            offset: 29957,
        },
        name_keysym {
            keysym: 0x1008ff62,
            offset: 29973,
        },
        name_keysym {
            keysym: 0x1008127e,
            offset: 29984,
        },
        name_keysym {
            keysym: 0x10081170,
            offset: 30001,
        },
        name_keysym {
            keysym: 0x10081183,
            offset: 30023,
        },
        name_keysym {
            keysym: 0x100810a4,
            offset: 30039,
        },
        name_keysym {
            keysym: 0x10081199,
            offset: 30058,
        },
        name_keysym {
            keysym: 0x100811b7,
            offset: 30076,
        },
        name_keysym {
            keysym: 0x10081186,
            offset: 30092,
        },
        name_keysym {
            keysym: 0x1008117a,
            offset: 30117,
        },
        name_keysym {
            keysym: 0x1008ff53,
            offset: 30138,
        },
        name_keysym {
            keysym: 0x10081185,
            offset: 30156,
        },
        name_keysym {
            keysym: 0x1008116e,
            offset: 30175,
        },
        name_keysym {
            keysym: 0x10081178,
            offset: 30195,
        },
        name_keysym {
            keysym: 0x1008116a,
            offset: 30213,
        },
        name_keysym {
            keysym: 0x10081181,
            offset: 30241,
        },
        name_keysym {
            keysym: 0x1008117d,
            offset: 30262,
        },
        name_keysym {
            keysym: 0x10081180,
            offset: 30287,
        },
        name_keysym {
            keysym: 0x10081184,
            offset: 30307,
        },
        name_keysym {
            keysym: 0x10081182,
            offset: 30331,
        },
        name_keysym {
            keysym: 0x10081179,
            offset: 30352,
        },
        name_keysym {
            keysym: 0x1008117b,
            offset: 30370,
        },
        name_keysym {
            keysym: 0x1008117c,
            offset: 30389,
        },
        name_keysym {
            keysym: 0x10081171,
            offset: 30412,
        },
        name_keysym {
            keysym: 0x1008126b,
            offset: 30431,
        },
        name_keysym {
            keysym: 0x1008ff63,
            offset: 30448,
        },
        name_keysym {
            keysym: 0x1008ff1e,
            offset: 30460,
        },
        name_keysym {
            keysym: 0x1008ff65,
            offset: 30469,
        },
        name_keysym {
            keysym: 0x1008ff66,
            offset: 30480,
        },
        name_keysym {
            keysym: 0x1008ff8e,
            offset: 30491,
        },
        name_keysym {
            keysym: 0x1008ff01,
            offset: 30505,
        },
        name_keysym {
            keysym: 0x100810f4,
            offset: 30518,
        },
        name_keysym {
            keysym: 0x1008ff07,
            offset: 30540,
        },
        name_keysym {
            keysym: 0x1008ff03,
            offset: 30563,
        },
        name_keysym {
            keysym: 0x1008ff02,
            offset: 30585,
        },
        name_keysym {
            keysym: 0x1008ff92,
            offset: 30605,
        },
        name_keysym {
            keysym: 0x1008ff33,
            offset: 30615,
        },
        name_keysym {
            keysym: 0x1008ff67,
            offset: 30630,
        },
        name_keysym {
            keysym: 0x10081280,
            offset: 30642,
        },
        name_keysym {
            keysym: 0x10081288,
            offset: 30655,
        },
        name_keysym {
            keysym: 0x1008ff68,
            offset: 30667,
        },
        name_keysym {
            keysym: 0x1008ff69,
            offset: 30675,
        },
        name_keysym {
            keysym: 0x1008fe22,
            offset: 30684,
        },
        name_keysym {
            keysym: 0x1008127b,
            offset: 30699,
        },
        name_keysym {
            keysym: 0x10081270,
            offset: 30715,
        },
        name_keysym {
            keysym: 0x100811bc,
            offset: 30732,
        },
        name_keysym {
            keysym: 0x1008119d,
            offset: 30755,
        },
        name_keysym {
            keysym: 0x10081200,
            offset: 30775,
        },
        name_keysym {
            keysym: 0x10081201,
            offset: 30788,
        },
        name_keysym {
            keysym: 0x1008126c,
            offset: 30801,
        },
        name_keysym {
            keysym: 0x1008126d,
            offset: 30815,
        },
        name_keysym {
            keysym: 0x10081202,
            offset: 30829,
        },
        name_keysym {
            keysym: 0x10081203,
            offset: 30842,
        },
        name_keysym {
            keysym: 0x10081204,
            offset: 30855,
        },
        name_keysym {
            keysym: 0x10081205,
            offset: 30868,
        },
        name_keysym {
            keysym: 0x10081206,
            offset: 30881,
        },
        name_keysym {
            keysym: 0x10081207,
            offset: 30894,
        },
        name_keysym {
            keysym: 0x10081208,
            offset: 30907,
        },
        name_keysym {
            keysym: 0x10081209,
            offset: 30920,
        },
        name_keysym {
            keysym: 0x1008120c,
            offset: 30933,
        },
        name_keysym {
            keysym: 0x1008120d,
            offset: 30946,
        },
        name_keysym {
            keysym: 0x1008120e,
            offset: 30959,
        },
        name_keysym {
            keysym: 0x1008120f,
            offset: 30972,
        },
        name_keysym {
            keysym: 0x1008120b,
            offset: 30985,
        },
        name_keysym {
            keysym: 0x1008120a,
            offset: 31002,
        },
        name_keysym {
            keysym: 0x1008ff6a,
            offset: 31018,
        },
        name_keysym {
            keysym: 0x10081160,
            offset: 31033,
        },
        name_keysym {
            keysym: 0x10081278,
            offset: 31040,
        },
        name_keysym {
            keysym: 0x1008ff6b,
            offset: 31061,
        },
        name_keysym {
            keysym: 0x1008ff38,
            offset: 31070,
        },
        name_keysym {
            keysym: 0x1008ff6c,
            offset: 31082,
        },
        name_keysym {
            keysym: 0x1008ff6d,
            offset: 31093,
        },
        name_keysym {
            keysym: 0x10081272,
            offset: 31103,
        },
        name_keysym {
            keysym: 0x100812bd,
            offset: 31119,
        },
        name_keysym {
            keysym: 0x1008ff6e,
            offset: 31139,
        },
        name_keysym {
            keysym: 0x100811bd,
            offset: 31149,
        },
        name_keysym {
            keysym: 0x1008ff91,
            offset: 31165,
        },
        name_keysym {
            keysym: 0x1008ff21,
            offset: 31178,
        },
        name_keysym {
            keysym: 0x1008ff2a,
            offset: 31192,
        },
        name_keysym {
            keysym: 0x100811a9,
            offset: 31205,
        },
        name_keysym {
            keysym: 0x1008fe23,
            offset: 31222,
        },
        name_keysym {
            keysym: 0x1008127c,
            offset: 31237,
        },
        name_keysym {
            keysym: 0x10081279,
            offset: 31257,
        },
        name_keysym {
            keysym: 0x1008ff70,
            offset: 31281,
        },
        name_keysym {
            keysym: 0x10081284,
            offset: 31287,
        },
        name_keysym {
            keysym: 0x1008ffa3,
            offset: 31304,
        },
        name_keysym {
            keysym: 0x1008ff29,
            offset: 31312,
        },
        name_keysym {
            keysym: 0x10081232,
            offset: 31324,
        },
        name_keysym {
            keysym: 0x1008ff73,
            offset: 31346,
        },
        name_keysym {
            keysym: 0x1008ff72,
            offset: 31357,
        },
        name_keysym {
            keysym: 0x1008ffb5,
            offset: 31367,
        },
        name_keysym {
            keysym: 0x10081267,
            offset: 31378,
        },
        name_keysym {
            keysym: 0x10081266,
            offset: 31392,
        },
        name_keysym {
            keysym: 0x1008ff24,
            offset: 31404,
        },
        name_keysym {
            keysym: 0x1008ff25,
            offset: 31419,
        },
        name_keysym {
            keysym: 0x1008ff23,
            offset: 31435,
        },
        name_keysym {
            keysym: 0x1008126a,
            offset: 31448,
        },
        name_keysym {
            keysym: 0x1008ff74,
            offset: 31461,
        },
        name_keysym {
            keysym: 0x1008ff76,
            offset: 31479,
        },
        name_keysym {
            keysym: 0x1008ffb7,
            offset: 31494,
        },
        name_keysym {
            keysym: 0x1008ff75,
            offset: 31517,
        },
        name_keysym {
            keysym: 0x1008ff77,
            offset: 31532,
        },
        name_keysym {
            keysym: 0x1008ff2d,
            offset: 31541,
        },
        name_keysym {
            keysym: 0x10081245,
            offset: 31557,
        },
        name_keysym {
            keysym: 0x1008ff7a,
            offset: 31573,
        },
        name_keysym {
            keysym: 0x1008ff79,
            offset: 31589,
        },
        name_keysym {
            keysym: 0x1008ff78,
            offset: 31604,
        },
        name_keysym {
            keysym: 0x1008ff1b,
            offset: 31617,
        },
        name_keysym {
            keysym: 0x1008ffa0,
            offset: 31628,
        },
        name_keysym {
            keysym: 0x1008127a,
            offset: 31639,
        },
        name_keysym {
            keysym: 0x1008ff7b,
            offset: 31663,
        },
        name_keysym {
            keysym: 0x1008ff36,
            offset: 31672,
        },
        name_keysym {
            keysym: 0x10081287,
            offset: 31681,
        },
        name_keysym {
            keysym: 0x10081282,
            offset: 31697,
        },
        name_keysym {
            keysym: 0x1008ff2f,
            offset: 31718,
        },
        name_keysym {
            keysym: 0x10081276,
            offset: 31728,
        },
        name_keysym {
            keysym: 0x1008127f,
            offset: 31744,
        },
        name_keysym {
            keysym: 0x1008ff7c,
            offset: 31752,
        },
        name_keysym {
            keysym: 0x100811b0,
            offset: 31762,
        },
        name_keysym {
            keysym: 0x1008ff7d,
            offset: 31777,
        },
        name_keysym {
            keysym: 0x100810dc,
            offset: 31793,
        },
        name_keysym {
            keysym: 0x1008ff10,
            offset: 31803,
        },
        name_keysym {
            keysym: 0x1008ff1a,
            offset: 31815,
        },
        name_keysym {
            keysym: 0x1008ff28,
            offset: 31825,
        },
        name_keysym {
            keysym: 0x10081271,
            offset: 31834,
        },
        name_keysym {
            keysym: 0x1008ff9a,
            offset: 31849,
        },
        name_keysym {
            keysym: 0x1008ff7e,
            offset: 31862,
        },
        name_keysym {
            keysym: 0x1008ffa7,
            offset: 31874,
        },
        name_keysym {
            keysym: 0x1008fe01,
            offset: 31886,
        },
        name_keysym {
            keysym: 0x1008fe0a,
            offset: 31902,
        },
        name_keysym {
            keysym: 0x1008fe0b,
            offset: 31919,
        },
        name_keysym {
            keysym: 0x1008fe0c,
            offset: 31936,
        },
        name_keysym {
            keysym: 0x1008fe02,
            offset: 31953,
        },
        name_keysym {
            keysym: 0x1008fe03,
            offset: 31969,
        },
        name_keysym {
            keysym: 0x1008fe04,
            offset: 31985,
        },
        name_keysym {
            keysym: 0x1008fe05,
            offset: 32001,
        },
        name_keysym {
            keysym: 0x1008fe06,
            offset: 32017,
        },
        name_keysym {
            keysym: 0x1008fe07,
            offset: 32033,
        },
        name_keysym {
            keysym: 0x1008fe08,
            offset: 32049,
        },
        name_keysym {
            keysym: 0x1008fe09,
            offset: 32065,
        },
        name_keysym {
            keysym: 0x10081241,
            offset: 32081,
        },
        name_keysym {
            keysym: 0x1008ff7f,
            offset: 32097,
        },
        name_keysym {
            keysym: 0x1008ff80,
            offset: 32110,
        },
        name_keysym {
            keysym: 0x1008ff9f,
            offset: 32123,
        },
        name_keysym {
            keysym: 0x1008ff1f,
            offset: 32132,
        },
        name_keysym {
            keysym: 0x1008ff81,
            offset: 32145,
        },
        name_keysym {
            keysym: 0x1008ffa2,
            offset: 32155,
        },
        name_keysym {
            keysym: 0x1008ffb1,
            offset: 32167,
        },
        name_keysym {
            keysym: 0x1008ffb0,
            offset: 32183,
        },
        name_keysym {
            keysym: 0x1008ffa9,
            offset: 32198,
        },
        name_keysym {
            keysym: 0x10081285,
            offset: 32217,
        },
        name_keysym {
            keysym: 0x1008ff82,
            offset: 32238,
        },
        name_keysym {
            keysym: 0x1008fe20,
            offset: 32249,
        },
        name_keysym {
            keysym: 0x10081274,
            offset: 32260,
        },
        name_keysym {
            keysym: 0x1008ff85,
            offset: 32271,
        },
        name_keysym {
            keysym: 0x1008ff86,
            offset: 32283,
        },
        name_keysym {
            keysym: 0x1008ff84,
            offset: 32295,
        },
        name_keysym {
            keysym: 0x1008ff96,
            offset: 32306,
        },
        name_keysym {
            keysym: 0x1008ff34,
            offset: 32314,
        },
        name_keysym {
            keysym: 0x10081168,
            offset: 32329,
        },
        name_keysym {
            keysym: 0x1008ff87,
            offset: 32344,
        },
        name_keysym {
            keysym: 0x100811a0,
            offset: 32354,
        },
        name_keysym {
            keysym: 0x1008ffa1,
            offset: 32369,
        },
        name_keysym {
            keysym: 0x10081273,
            offset: 32378,
        },
        name_keysym {
            keysym: 0x10081246,
            offset: 32386,
        },
        name_keysym {
            keysym: 0x100811ac,
            offset: 32403,
        },
        name_keysym {
            keysym: 0x1008ff2b,
            offset: 32417,
        },
        name_keysym {
            keysym: 0x1008ff8f,
            offset: 32428,
        },
        name_keysym {
            keysym: 0x1008ff88,
            offset: 32439,
        },
        name_keysym {
            keysym: 0x1008ff95,
            offset: 32455,
        },
        name_keysym {
            keysym: 0x1008ff89,
            offset: 32464,
        },
        name_keysym {
            keysym: 0x10081211,
            offset: 32473,
        },
        name_keysym {
            keysym: 0x1008ffb4,
            offset: 32487,
        },
        name_keysym {
            keysym: 0x1008ff2e,
            offset: 32496,
        },
        name_keysym {
            keysym: 0x1008ff8a,
            offset: 32504,
        },
        name_keysym {
            keysym: 0x1008ffa5,
            offset: 32513,
        },
        name_keysym {
            keysym: 0x1008ff8b,
            offset: 32524,
        },
        name_keysym {
            keysym: 0x1008ff8c,
            offset: 32535,
        },
        name_keysym {
            keysym: 0x100811a4,
            offset: 32547,
        },
        name_keysym {
            keysym: 0x59,
            offset: 32561,
        },
        name_keysym {
            keysym: 0x79,
            offset: 32563,
        },
        name_keysym {
            keysym: 0xdd,
            offset: 32565,
        },
        name_keysym {
            keysym: 0xfd,
            offset: 32572,
        },
        name_keysym {
            keysym: 0x1001ef4,
            offset: 32579,
        },
        name_keysym {
            keysym: 0x1001ef5,
            offset: 32589,
        },
        name_keysym {
            keysym: 0x1000176,
            offset: 32599,
        },
        name_keysym {
            keysym: 0x1000177,
            offset: 32611,
        },
        name_keysym {
            keysym: 0x13be,
            offset: 32623,
        },
        name_keysym {
            keysym: 0xff,
            offset: 32634,
        },
        name_keysym {
            keysym: 0xa5,
            offset: 32645,
        },
        name_keysym {
            keysym: 0x1001ef2,
            offset: 32649,
        },
        name_keysym {
            keysym: 0x1001ef3,
            offset: 32656,
        },
        name_keysym {
            keysym: 0x1001ef6,
            offset: 32663,
        },
        name_keysym {
            keysym: 0x1001ef7,
            offset: 32669,
        },
        name_keysym {
            keysym: 0x1001ef8,
            offset: 32675,
        },
        name_keysym {
            keysym: 0x1001ef9,
            offset: 32682,
        },
        name_keysym {
            keysym: 0x5a,
            offset: 32689,
        },
        name_keysym {
            keysym: 0x7a,
            offset: 32691,
        },
        name_keysym {
            keysym: 0x1af,
            offset: 32693,
        },
        name_keysym {
            keysym: 0x1bf,
            offset: 32703,
        },
        name_keysym {
            keysym: 0x1ac,
            offset: 32713,
        },
        name_keysym {
            keysym: 0x1bc,
            offset: 32720,
        },
        name_keysym {
            keysym: 0x1ae,
            offset: 32727,
        },
        name_keysym {
            keysym: 0x1be,
            offset: 32734,
        },
        name_keysym {
            keysym: 0xff3d,
            offset: 32741,
        },
        name_keysym {
            keysym: 0xff28,
            offset: 32750,
        },
        name_keysym {
            keysym: 0xff2a,
            offset: 32758,
        },
        name_keysym {
            keysym: 0x1002080,
            offset: 32774,
        },
        name_keysym {
            keysym: 0x1002070,
            offset: 32788,
        },
        name_keysym {
            keysym: 0x10001b5,
            offset: 32801,
        },
        name_keysym {
            keysym: 0x10001b6,
            offset: 32809,
        },
    ];
    pub static mut keysym_to_name: [name_keysym; 2502] = [
        name_keysym {
            keysym: 0,
            offset: 20276,
        },
        name_keysym {
            keysym: 0x20,
            offset: 24027,
        },
        name_keysym {
            keysym: 0x21,
            offset: 12230,
        },
        name_keysym {
            keysym: 0x22,
            offset: 22385,
        },
        name_keysym {
            keysym: 0x23,
            offset: 20363,
        },
        name_keysym {
            keysym: 0x24,
            offset: 11382,
        },
        name_keysym {
            keysym: 0x25,
            offset: 21685,
        },
        name_keysym {
            keysym: 0x26,
            offset: 908,
        },
        name_keysym {
            keysym: 0x27,
            offset: 934,
        },
        name_keysym {
            keysym: 0x28,
            offset: 21623,
        },
        name_keysym {
            keysym: 0x29,
            offset: 21633,
        },
        name_keysym {
            keysym: 0x2a,
            offset: 3283,
        },
        name_keysym {
            keysym: 0x2b,
            offset: 21755,
        },
        name_keysym {
            keysym: 0x2c,
            offset: 8762,
        },
        name_keysym {
            keysym: 0x2d,
            offset: 19918,
        },
        name_keysym {
            keysym: 0x2e,
            offset: 21693,
        },
        name_keysym {
            keysym: 0x2f,
            offset: 23992,
        },
        name_keysym {
            keysym: 0x30,
            offset: 0,
        },
        name_keysym {
            keysym: 0x31,
            offset: 2,
        },
        name_keysym {
            keysym: 0x32,
            offset: 4,
        },
        name_keysym {
            keysym: 0x33,
            offset: 6,
        },
        name_keysym {
            keysym: 0x34,
            offset: 386,
        },
        name_keysym {
            keysym: 0x35,
            offset: 388,
        },
        name_keysym {
            keysym: 0x36,
            offset: 390,
        },
        name_keysym {
            keysym: 0x37,
            offset: 392,
        },
        name_keysym {
            keysym: 0x38,
            offset: 394,
        },
        name_keysym {
            keysym: 0x39,
            offset: 396,
        },
        name_keysym {
            keysym: 0x3a,
            offset: 8664,
        },
        name_keysym {
            keysym: 0x3b,
            offset: 22958,
        },
        name_keysym {
            keysym: 0x3c,
            offset: 19597,
        },
        name_keysym {
            keysym: 0x3d,
            offset: 12182,
        },
        name_keysym {
            keysym: 0x3e,
            offset: 13403,
        },
        name_keysym {
            keysym: 0x3f,
            offset: 22363,
        },
        name_keysym {
            keysym: 0x40,
            offset: 3292,
        },
        name_keysym {
            keysym: 0x41,
            offset: 398,
        },
        name_keysym {
            keysym: 0x42,
            offset: 3328,
        },
        name_keysym {
            keysym: 0x43,
            offset: 8439,
        },
        name_keysym {
            keysym: 0x44,
            offset: 10402,
        },
        name_keysym {
            keysym: 0x45,
            offset: 11555,
        },
        name_keysym {
            keysym: 0x46,
            offset: 12286,
        },
        name_keysym {
            keysym: 0x47,
            offset: 12796,
        },
        name_keysym {
            keysym: 0x48,
            offset: 14538,
        },
        name_keysym {
            keysym: 0x49,
            offset: 17229,
        },
        name_keysym {
            keysym: 0x4a,
            offset: 18274,
        },
        name_keysym {
            keysym: 0x4b,
            offset: 18306,
        },
        name_keysym {
            keysym: 0x4c,
            offset: 19272,
        },
        name_keysym {
            keysym: 0x4d,
            offset: 19718,
        },
        name_keysym {
            keysym: 0x4e,
            offset: 20128,
        },
        name_keysym {
            keysym: 0x4f,
            offset: 20385,
        },
        name_keysym {
            keysym: 0x50,
            offset: 21571,
        },
        name_keysym {
            keysym: 0x51,
            offset: 22354,
        },
        name_keysym {
            keysym: 0x52,
            offset: 22415,
        },
        name_keysym {
            keysym: 0x53,
            offset: 22803,
        },
        name_keysym {
            keysym: 0x54,
            offset: 24525,
        },
        name_keysym {
            keysym: 0x55,
            offset: 26078,
        },
        name_keysym {
            keysym: 0x56,
            offset: 26714,
        },
        name_keysym {
            keysym: 0x57,
            offset: 26776,
        },
        name_keysym {
            keysym: 0x58,
            offset: 26862,
        },
        name_keysym {
            keysym: 0x59,
            offset: 32561,
        },
        name_keysym {
            keysym: 0x5a,
            offset: 32689,
        },
        name_keysym {
            keysym: 0x5b,
            offset: 3603,
        },
        name_keysym {
            keysym: 0x5c,
            offset: 3352,
        },
        name_keysym {
            keysym: 0x5d,
            offset: 3615,
        },
        name_keysym {
            keysym: 0x5e,
            offset: 3260,
        },
        name_keysym {
            keysym: 0x5f,
            offset: 26577,
        },
        name_keysym {
            keysym: 0x60,
            offset: 13397,
        },
        name_keysym {
            keysym: 0x61,
            offset: 400,
        },
        name_keysym {
            keysym: 0x62,
            offset: 3330,
        },
        name_keysym {
            keysym: 0x63,
            offset: 8441,
        },
        name_keysym {
            keysym: 0x64,
            offset: 10404,
        },
        name_keysym {
            keysym: 0x65,
            offset: 11557,
        },
        name_keysym {
            keysym: 0x66,
            offset: 12288,
        },
        name_keysym {
            keysym: 0x67,
            offset: 12798,
        },
        name_keysym {
            keysym: 0x68,
            offset: 14540,
        },
        name_keysym {
            keysym: 0x69,
            offset: 17231,
        },
        name_keysym {
            keysym: 0x6a,
            offset: 18276,
        },
        name_keysym {
            keysym: 0x6b,
            offset: 18308,
        },
        name_keysym {
            keysym: 0x6c,
            offset: 19274,
        },
        name_keysym {
            keysym: 0x6d,
            offset: 19720,
        },
        name_keysym {
            keysym: 0x6e,
            offset: 20130,
        },
        name_keysym {
            keysym: 0x6f,
            offset: 20387,
        },
        name_keysym {
            keysym: 0x70,
            offset: 21573,
        },
        name_keysym {
            keysym: 0x71,
            offset: 22356,
        },
        name_keysym {
            keysym: 0x72,
            offset: 22417,
        },
        name_keysym {
            keysym: 0x73,
            offset: 22805,
        },
        name_keysym {
            keysym: 0x74,
            offset: 24527,
        },
        name_keysym {
            keysym: 0x75,
            offset: 26080,
        },
        name_keysym {
            keysym: 0x76,
            offset: 26716,
        },
        name_keysym {
            keysym: 0x77,
            offset: 26778,
        },
        name_keysym {
            keysym: 0x78,
            offset: 26864,
        },
        name_keysym {
            keysym: 0x79,
            offset: 32563,
        },
        name_keysym {
            keysym: 0x7a,
            offset: 32691,
        },
        name_keysym {
            keysym: 0x7b,
            offset: 3582,
        },
        name_keysym {
            keysym: 0x7c,
            offset: 3392,
        },
        name_keysym {
            keysym: 0x7d,
            offset: 3592,
        },
        name_keysym {
            keysym: 0x7e,
            offset: 3272,
        },
        name_keysym {
            keysym: 0xa0,
            offset: 20263,
        },
        name_keysym {
            keysym: 0xa1,
            offset: 12237,
        },
        name_keysym {
            keysym: 0xa2,
            offset: 8589,
        },
        name_keysym {
            keysym: 0xa3,
            offset: 24058,
        },
        name_keysym {
            keysym: 0xa4,
            offset: 8848,
        },
        name_keysym {
            keysym: 0xa5,
            offset: 32645,
        },
        name_keysym {
            keysym: 0xa6,
            offset: 8389,
        },
        name_keysym {
            keysym: 0xa7,
            offset: 22943,
        },
        name_keysym {
            keysym: 0xa8,
            offset: 11334,
        },
        name_keysym {
            keysym: 0xa9,
            offset: 8799,
        },
        name_keysym {
            keysym: 0xaa,
            offset: 21037,
        },
        name_keysym {
            keysym: 0xab,
            offset: 14509,
        },
        name_keysym {
            keysym: 0xac,
            offset: 20332,
        },
        name_keysym {
            keysym: 0xad,
            offset: 17222,
        },
        name_keysym {
            keysym: 0xae,
            offset: 22529,
        },
        name_keysym {
            keysym: 0xaf,
            offset: 19826,
        },
        name_keysym {
            keysym: 0xb0,
            offset: 11284,
        },
        name_keysym {
            keysym: 0xb1,
            offset: 21760,
        },
        name_keysym {
            keysym: 0xb2,
            offset: 26056,
        },
        name_keysym {
            keysym: 0xb3,
            offset: 25802,
        },
        name_keysym {
            keysym: 0xb4,
            offset: 820,
        },
        name_keysym {
            keysym: 0xb5,
            offset: 19984,
        },
        name_keysym {
            keysym: 0xb6,
            offset: 21613,
        },
        name_keysym {
            keysym: 0xb7,
            offset: 21700,
        },
        name_keysym {
            keysym: 0xb8,
            offset: 8581,
        },
        name_keysym {
            keysym: 0xb9,
            offset: 20940,
        },
        name_keysym {
            keysym: 0xba,
            offset: 19873,
        },
        name_keysym {
            keysym: 0xbb,
            offset: 14523,
        },
        name_keysym {
            keysym: 0xbc,
            offset: 20907,
        },
        name_keysym {
            keysym: 0xbd,
            offset: 20899,
        },
        name_keysym {
            keysym: 0xbe,
            offset: 25773,
        },
        name_keysym {
            keysym: 0xbf,
            offset: 22372,
        },
        name_keysym {
            keysym: 0xc0,
            offset: 854,
        },
        name_keysym {
            keysym: 0xc1,
            offset: 402,
        },
        name_keysym {
            keysym: 0xc2,
            offset: 622,
        },
        name_keysym {
            keysym: 0xc3,
            offset: 3295,
        },
        name_keysym {
            keysym: 0xc4,
            offset: 826,
        },
        name_keysym {
            keysym: 0xc5,
            offset: 2036,
        },
        name_keysym {
            keysym: 0xc6,
            offset: 848,
        },
        name_keysym {
            keysym: 0xc7,
            offset: 8539,
        },
        name_keysym {
            keysym: 0xc8,
            offset: 11855,
        },
        name_keysym {
            keysym: 0xc9,
            offset: 11579,
        },
        name_keysym {
            keysym: 0xca,
            offset: 11627,
        },
        name_keysym {
            keysym: 0xcb,
            offset: 11833,
        },
        name_keysym {
            keysym: 0xcc,
            offset: 17365,
        },
        name_keysym {
            keysym: 0xcd,
            offset: 17243,
        },
        name_keysym {
            keysym: 0xce,
            offset: 17291,
        },
        name_keysym {
            keysym: 0xcf,
            offset: 17325,
        },
        name_keysym {
            keysym: 0xd0,
            offset: 12195,
        },
        name_keysym {
            keysym: 0xd1,
            offset: 20340,
        },
        name_keysym {
            keysym: 0xd2,
            offset: 20712,
        },
        name_keysym {
            keysym: 0xd3,
            offset: 20389,
        },
        name_keysym {
            keysym: 0xd4,
            offset: 20453,
        },
        name_keysym {
            keysym: 0xd5,
            offset: 21508,
        },
        name_keysym {
            keysym: 0xd6,
            offset: 20651,
        },
        name_keysym {
            keysym: 0xd7,
            offset: 20024,
        },
        name_keysym {
            keysym: 0xd8,
            offset: 21494,
        },
        name_keysym {
            keysym: 0xd9,
            offset: 26202,
        },
        name_keysym {
            keysym: 0xda,
            offset: 26082,
        },
        name_keysym {
            keysym: 0xdb,
            offset: 26130,
        },
        name_keysym {
            keysym: 0xdc,
            offset: 26154,
        },
        name_keysym {
            keysym: 0xdd,
            offset: 32565,
        },
        name_keysym {
            keysym: 0xde,
            offset: 25730,
        },
        name_keysym {
            keysym: 0xdf,
            offset: 24051,
        },
        name_keysym {
            keysym: 0xe0,
            offset: 861,
        },
        name_keysym {
            keysym: 0xe1,
            offset: 409,
        },
        name_keysym {
            keysym: 0xe2,
            offset: 634,
        },
        name_keysym {
            keysym: 0xe3,
            offset: 3302,
        },
        name_keysym {
            keysym: 0xe4,
            offset: 837,
        },
        name_keysym {
            keysym: 0xe5,
            offset: 2042,
        },
        name_keysym {
            keysym: 0xe6,
            offset: 851,
        },
        name_keysym {
            keysym: 0xe7,
            offset: 8548,
        },
        name_keysym {
            keysym: 0xe8,
            offset: 11862,
        },
        name_keysym {
            keysym: 0xe9,
            offset: 11586,
        },
        name_keysym {
            keysym: 0xea,
            offset: 11639,
        },
        name_keysym {
            keysym: 0xeb,
            offset: 11844,
        },
        name_keysym {
            keysym: 0xec,
            offset: 17372,
        },
        name_keysym {
            keysym: 0xed,
            offset: 17250,
        },
        name_keysym {
            keysym: 0xee,
            offset: 17303,
        },
        name_keysym {
            keysym: 0xef,
            offset: 17336,
        },
        name_keysym {
            keysym: 0xf0,
            offset: 12203,
        },
        name_keysym {
            keysym: 0xf1,
            offset: 20347,
        },
        name_keysym {
            keysym: 0xf2,
            offset: 20719,
        },
        name_keysym {
            keysym: 0xf3,
            offset: 20396,
        },
        name_keysym {
            keysym: 0xf4,
            offset: 20465,
        },
        name_keysym {
            keysym: 0xf5,
            offset: 21515,
        },
        name_keysym {
            keysym: 0xf6,
            offset: 20662,
        },
        name_keysym {
            keysym: 0xf7,
            offset: 11373,
        },
        name_keysym {
            keysym: 0xf8,
            offset: 21501,
        },
        name_keysym {
            keysym: 0xf9,
            offset: 26209,
        },
        name_keysym {
            keysym: 0xfa,
            offset: 26089,
        },
        name_keysym {
            keysym: 0xfb,
            offset: 26142,
        },
        name_keysym {
            keysym: 0xfc,
            offset: 26165,
        },
        name_keysym {
            keysym: 0xfd,
            offset: 32572,
        },
        name_keysym {
            keysym: 0xfe,
            offset: 25742,
        },
        name_keysym {
            keysym: 0xff,
            offset: 32634,
        },
        name_keysym {
            keysym: 0x1a1,
            offset: 918,
        },
        name_keysym {
            keysym: 0x1a2,
            offset: 8383,
        },
        name_keysym {
            keysym: 0x1a3,
            offset: 19702,
        },
        name_keysym {
            keysym: 0x1a5,
            offset: 19372,
        },
        name_keysym {
            keysym: 0x1a6,
            offset: 22827,
        },
        name_keysym {
            keysym: 0x1a9,
            offset: 22841,
        },
        name_keysym {
            keysym: 0x1aa,
            offset: 22855,
        },
        name_keysym {
            keysym: 0x1ab,
            offset: 24553,
        },
        name_keysym {
            keysym: 0x1ac,
            offset: 32713,
        },
        name_keysym {
            keysym: 0x1ae,
            offset: 32727,
        },
        name_keysym {
            keysym: 0x1af,
            offset: 32693,
        },
        name_keysym {
            keysym: 0x1b1,
            offset: 926,
        },
        name_keysym {
            keysym: 0x1b2,
            offset: 20705,
        },
        name_keysym {
            keysym: 0x1b3,
            offset: 19710,
        },
        name_keysym {
            keysym: 0x1b5,
            offset: 19379,
        },
        name_keysym {
            keysym: 0x1b6,
            offset: 22834,
        },
        name_keysym {
            keysym: 0x1b7,
            offset: 8519,
        },
        name_keysym {
            keysym: 0x1b9,
            offset: 22848,
        },
        name_keysym {
            keysym: 0x1ba,
            offset: 22864,
        },
        name_keysym {
            keysym: 0x1bb,
            offset: 24560,
        },
        name_keysym {
            keysym: 0x1bc,
            offset: 32720,
        },
        name_keysym {
            keysym: 0x1bd,
            offset: 11414,
        },
        name_keysym {
            keysym: 0x1be,
            offset: 32734,
        },
        name_keysym {
            keysym: 0x1bf,
            offset: 32703,
        },
        name_keysym {
            keysym: 0x1c0,
            offset: 22470,
        },
        name_keysym {
            keysym: 0x1c3,
            offset: 445,
        },
        name_keysym {
            keysym: 0x1c5,
            offset: 19307,
        },
        name_keysym {
            keysym: 0x1c6,
            offset: 8475,
        },
        name_keysym {
            keysym: 0x1c8,
            offset: 8525,
        },
        name_keysym {
            keysym: 0x1ca,
            offset: 12166,
        },
        name_keysym {
            keysym: 0x1cc,
            offset: 11613,
        },
        name_keysym {
            keysym: 0x1cf,
            offset: 10447,
        },
        name_keysym {
            keysym: 0x1d0,
            offset: 11532,
        },
        name_keysym {
            keysym: 0x1d1,
            offset: 20138,
        },
        name_keysym {
            keysym: 0x1d2,
            offset: 20162,
        },
        name_keysym {
            keysym: 0x1d5,
            offset: 20673,
        },
        name_keysym {
            keysym: 0x1d8,
            offset: 22492,
        },
        name_keysym {
            keysym: 0x1d9,
            offset: 26683,
        },
        name_keysym {
            keysym: 0x1db,
            offset: 26176,
        },
        name_keysym {
            keysym: 0x1de,
            offset: 24567,
        },
        name_keysym {
            keysym: 0x1e0,
            offset: 22477,
        },
        name_keysym {
            keysym: 0x1e3,
            offset: 452,
        },
        name_keysym {
            keysym: 0x1e5,
            offset: 19314,
        },
        name_keysym {
            keysym: 0x1e6,
            offset: 8482,
        },
        name_keysym {
            keysym: 0x1e8,
            offset: 8532,
        },
        name_keysym {
            keysym: 0x1ea,
            offset: 12174,
        },
        name_keysym {
            keysym: 0x1ec,
            offset: 11620,
        },
        name_keysym {
            keysym: 0x1ef,
            offset: 10454,
        },
        name_keysym {
            keysym: 0x1f0,
            offset: 11540,
        },
        name_keysym {
            keysym: 0x1f1,
            offset: 20145,
        },
        name_keysym {
            keysym: 0x1f2,
            offset: 20169,
        },
        name_keysym {
            keysym: 0x1f5,
            offset: 20686,
        },
        name_keysym {
            keysym: 0x1f8,
            offset: 22499,
        },
        name_keysym {
            keysym: 0x1f9,
            offset: 26689,
        },
        name_keysym {
            keysym: 0x1fb,
            offset: 26189,
        },
        name_keysym {
            keysym: 0x1fe,
            offset: 24576,
        },
        name_keysym {
            keysym: 0x1ff,
            offset: 436,
        },
        name_keysym {
            keysym: 0x2a1,
            offset: 17187,
        },
        name_keysym {
            keysym: 0x2a6,
            offset: 16216,
        },
        name_keysym {
            keysym: 0x2a9,
            offset: 17233,
        },
        name_keysym {
            keysym: 0x2ab,
            offset: 12820,
        },
        name_keysym {
            keysym: 0x2ac,
            offset: 18278,
        },
        name_keysym {
            keysym: 0x2b1,
            offset: 17195,
        },
        name_keysym {
            keysym: 0x2b6,
            offset: 16228,
        },
        name_keysym {
            keysym: 0x2b9,
            offset: 17347,
        },
        name_keysym {
            keysym: 0x2bb,
            offset: 12827,
        },
        name_keysym {
            keysym: 0x2bc,
            offset: 18290,
        },
        name_keysym {
            keysym: 0x2c5,
            offset: 8455,
        },
        name_keysym {
            keysym: 0x2c6,
            offset: 8557,
        },
        name_keysym {
            keysym: 0x2d5,
            offset: 12800,
        },
        name_keysym {
            keysym: 0x2d8,
            offset: 12866,
        },
        name_keysym {
            keysym: 0x2dd,
            offset: 26116,
        },
        name_keysym {
            keysym: 0x2de,
            offset: 22885,
        },
        name_keysym {
            keysym: 0x2e5,
            offset: 8465,
        },
        name_keysym {
            keysym: 0x2e6,
            offset: 8569,
        },
        name_keysym {
            keysym: 0x2f5,
            offset: 12810,
        },
        name_keysym {
            keysym: 0x2f8,
            offset: 12878,
        },
        name_keysym {
            keysym: 0x2fd,
            offset: 26123,
        },
        name_keysym {
            keysym: 0x2fe,
            offset: 22897,
        },
        name_keysym {
            keysym: 0x3a2,
            offset: 19268,
        },
        name_keysym {
            keysym: 0x3a3,
            offset: 22506,
        },
        name_keysym {
            keysym: 0x3a5,
            offset: 18260,
        },
        name_keysym {
            keysym: 0x3a6,
            offset: 19386,
        },
        name_keysym {
            keysym: 0x3aa,
            offset: 11970,
        },
        name_keysym {
            keysym: 0x3ab,
            offset: 12848,
        },
        name_keysym {
            keysym: 0x3ac,
            offset: 26019,
        },
        name_keysym {
            keysym: 0x3b3,
            offset: 22515,
        },
        name_keysym {
            keysym: 0x3b5,
            offset: 18267,
        },
        name_keysym {
            keysym: 0x3b6,
            offset: 19395,
        },
        name_keysym {
            keysym: 0x3ba,
            offset: 11978,
        },
        name_keysym {
            keysym: 0x3bb,
            offset: 12857,
        },
        name_keysym {
            keysym: 0x3bc,
            offset: 26026,
        },
        name_keysym {
            keysym: 0x3bd,
            offset: 12114,
        },
        name_keysym {
            keysym: 0x3bf,
            offset: 12118,
        },
        name_keysym {
            keysym: 0x3c0,
            offset: 892,
        },
        name_keysym {
            keysym: 0x3c7,
            offset: 17498,
        },
        name_keysym {
            keysym: 0x3cc,
            offset: 11559,
        },
        name_keysym {
            keysym: 0x3cf,
            offset: 17391,
        },
        name_keysym {
            keysym: 0x3d1,
            offset: 20176,
        },
        name_keysym {
            keysym: 0x3d2,
            offset: 20864,
        },
        name_keysym {
            keysym: 0x3d3,
            offset: 18939,
        },
        name_keysym {
            keysym: 0x3d9,
            offset: 26599,
        },
        name_keysym {
            keysym: 0x3dd,
            offset: 26700,
        },
        name_keysym {
            keysym: 0x3de,
            offset: 26552,
        },
        name_keysym {
            keysym: 0x3e0,
            offset: 900,
        },
        name_keysym {
            keysym: 0x3e7,
            offset: 17506,
        },
        name_keysym {
            keysym: 0x3ec,
            offset: 11569,
        },
        name_keysym {
            keysym: 0x3ef,
            offset: 17399,
        },
        name_keysym {
            keysym: 0x3f1,
            offset: 20185,
        },
        name_keysym {
            keysym: 0x3f2,
            offset: 20872,
        },
        name_keysym {
            keysym: 0x3f3,
            offset: 18948,
        },
        name_keysym {
            keysym: 0x3f9,
            offset: 26607,
        },
        name_keysym {
            keysym: 0x3fd,
            offset: 26707,
        },
        name_keysym {
            keysym: 0x3fe,
            offset: 26560,
        },
        name_keysym {
            keysym: 0x47e,
            offset: 21562,
        },
        name_keysym {
            keysym: 0x4a1,
            offset: 18403,
        },
        name_keysym {
            keysym: 0x4a2,
            offset: 18637,
        },
        name_keysym {
            keysym: 0x4a3,
            offset: 18333,
        },
        name_keysym {
            keysym: 0x4a4,
            offset: 18353,
        },
        name_keysym {
            keysym: 0x4a5,
            offset: 18364,
        },
        name_keysym {
            keysym: 0x4a6,
            offset: 18849,
        },
        name_keysym {
            keysym: 0x4a7,
            offset: 18317,
        },
        name_keysym {
            keysym: 0x4a8,
            offset: 18464,
        },
        name_keysym {
            keysym: 0x4a9,
            offset: 18834,
        },
        name_keysym {
            keysym: 0x4aa,
            offset: 18388,
        },
        name_keysym {
            keysym: 0x4ab,
            offset: 18630,
        },
        name_keysym {
            keysym: 0x4ac,
            offset: 18865,
        },
        name_keysym {
            keysym: 0x4ad,
            offset: 18897,
        },
        name_keysym {
            keysym: 0x4ae,
            offset: 18881,
        },
        name_keysym {
            keysym: 0x4af,
            offset: 18802,
        },
        name_keysym {
            keysym: 0x4b0,
            offset: 22328,
        },
        name_keysym {
            keysym: 0x4b1,
            offset: 18310,
        },
        name_keysym {
            keysym: 0x4b2,
            offset: 18457,
        },
        name_keysym {
            keysym: 0x4b3,
            offset: 18827,
        },
        name_keysym {
            keysym: 0x4b4,
            offset: 18381,
        },
        name_keysym {
            keysym: 0x4b5,
            offset: 18623,
        },
        name_keysym {
            keysym: 0x4b6,
            offset: 18471,
        },
        name_keysym {
            keysym: 0x4b7,
            offset: 18487,
        },
        name_keysym {
            keysym: 0x4b8,
            offset: 18503,
        },
        name_keysym {
            keysym: 0x4b9,
            offset: 18479,
        },
        name_keysym {
            keysym: 0x4ba,
            offset: 18495,
        },
        name_keysym {
            keysym: 0x4bb,
            offset: 18697,
        },
        name_keysym {
            keysym: 0x4bc,
            offset: 18713,
        },
        name_keysym {
            keysym: 0x4bd,
            offset: 18741,
        },
        name_keysym {
            keysym: 0x4be,
            offset: 18705,
        },
        name_keysym {
            keysym: 0x4bf,
            offset: 18733,
        },
        name_keysym {
            keysym: 0x4c0,
            offset: 18761,
        },
        name_keysym {
            keysym: 0x4c1,
            offset: 18324,
        },
        name_keysym {
            keysym: 0x4c2,
            offset: 18793,
        },
        name_keysym {
            keysym: 0x4c3,
            offset: 18769,
        },
        name_keysym {
            keysym: 0x4c4,
            offset: 18785,
        },
        name_keysym {
            keysym: 0x4c5,
            offset: 18583,
        },
        name_keysym {
            keysym: 0x4c6,
            offset: 18599,
        },
        name_keysym {
            keysym: 0x4c7,
            offset: 18615,
        },
        name_keysym {
            keysym: 0x4c8,
            offset: 18591,
        },
        name_keysym {
            keysym: 0x4c9,
            offset: 18607,
        },
        name_keysym {
            keysym: 0x4ca,
            offset: 18417,
        },
        name_keysym {
            keysym: 0x4cb,
            offset: 18433,
        },
        name_keysym {
            keysym: 0x4cc,
            offset: 18395,
        },
        name_keysym {
            keysym: 0x4cd,
            offset: 18425,
        },
        name_keysym {
            keysym: 0x4ce,
            offset: 18441,
        },
        name_keysym {
            keysym: 0x4cf,
            offset: 18521,
        },
        name_keysym {
            keysym: 0x4d0,
            offset: 18537,
        },
        name_keysym {
            keysym: 0x4d1,
            offset: 18568,
        },
        name_keysym {
            keysym: 0x4d2,
            offset: 18529,
        },
        name_keysym {
            keysym: 0x4d3,
            offset: 18560,
        },
        name_keysym {
            keysym: 0x4d4,
            offset: 18857,
        },
        name_keysym {
            keysym: 0x4d5,
            offset: 18889,
        },
        name_keysym {
            keysym: 0x4d6,
            offset: 18873,
        },
        name_keysym {
            keysym: 0x4d7,
            offset: 18657,
        },
        name_keysym {
            keysym: 0x4d8,
            offset: 18673,
        },
        name_keysym {
            keysym: 0x4d9,
            offset: 18689,
        },
        name_keysym {
            keysym: 0x4da,
            offset: 18665,
        },
        name_keysym {
            keysym: 0x4db,
            offset: 18681,
        },
        name_keysym {
            keysym: 0x4dc,
            offset: 18841,
        },
        name_keysym {
            keysym: 0x4dd,
            offset: 18576,
        },
        name_keysym {
            keysym: 0x4de,
            offset: 26750,
        },
        name_keysym {
            keysym: 0x4df,
            offset: 22968,
        },
        name_keysym {
            keysym: 0x5ac,
            offset: 1109,
        },
        name_keysym {
            keysym: 0x5bb,
            offset: 1764,
        },
        name_keysym {
            keysym: 0x5bf,
            offset: 1698,
        },
        name_keysym {
            keysym: 0x5c1,
            offset: 1303,
        },
        name_keysym {
            keysym: 0x5c2,
            offset: 1599,
        },
        name_keysym {
            keysym: 0x5c3,
            offset: 1354,
        },
        name_keysym {
            keysym: 0x5c4,
            offset: 1373,
        },
        name_keysym {
            keysym: 0x5c5,
            offset: 1409,
        },
        name_keysym {
            keysym: 0x5c6,
            offset: 1391,
        },
        name_keysym {
            keysym: 0x5c7,
            offset: 1067,
        },
        name_keysym {
            keysym: 0x5c8,
            offset: 1098,
        },
        name_keysym {
            keysym: 0x5c9,
            offset: 1909,
        },
        name_keysym {
            keysym: 0x5ca,
            offset: 1898,
        },
        name_keysym {
            keysym: 0x5cb,
            offset: 1939,
        },
        name_keysym {
            keysym: 0x5cc,
            offset: 1481,
        },
        name_keysym {
            keysym: 0x5cd,
            offset: 1292,
        },
        name_keysym {
            keysym: 0x5ce,
            offset: 1557,
        },
        name_keysym {
            keysym: 0x5cf,
            offset: 1133,
        },
        name_keysym {
            keysym: 0x5d0,
            offset: 1927,
        },
        name_keysym {
            keysym: 0x5d1,
            offset: 1719,
        },
        name_keysym {
            keysym: 0x5d2,
            offset: 2024,
        },
        name_keysym {
            keysym: 0x5d3,
            offset: 1752,
        },
        name_keysym {
            keysym: 0x5d4,
            offset: 1795,
        },
        name_keysym {
            keysym: 0x5d5,
            offset: 1741,
        },
        name_keysym {
            keysym: 0x5d6,
            offset: 1122,
        },
        name_keysym {
            keysym: 0x5d7,
            offset: 1859,
        },
        name_keysym {
            keysym: 0x5d8,
            offset: 2013,
        },
        name_keysym {
            keysym: 0x5d9,
            offset: 1056,
        },
        name_keysym {
            keysym: 0x5da,
            offset: 1269,
        },
        name_keysym {
            keysym: 0x5e0,
            offset: 1870,
        },
        name_keysym {
            keysym: 0x5e1,
            offset: 1231,
        },
        name_keysym {
            keysym: 0x5e2,
            offset: 1687,
        },
        name_keysym {
            keysym: 0x5e3,
            offset: 1504,
        },
        name_keysym {
            keysym: 0x5e4,
            offset: 1569,
        },
        name_keysym {
            keysym: 0x5e5,
            offset: 1618,
        },
        name_keysym {
            keysym: 0x5e6,
            offset: 1630,
        },
        name_keysym {
            keysym: 0x5e7,
            offset: 1282,
        },
        name_keysym {
            keysym: 0x5e8,
            offset: 1974,
        },
        name_keysym {
            keysym: 0x5e9,
            offset: 1079,
        },
        name_keysym {
            keysym: 0x5ea,
            offset: 1985,
        },
        name_keysym {
            keysym: 0x5eb,
            offset: 1215,
        },
        name_keysym {
            keysym: 0x5ec,
            offset: 1157,
        },
        name_keysym {
            keysym: 0x5ed,
            offset: 1528,
        },
        name_keysym {
            keysym: 0x5ee,
            offset: 1202,
        },
        name_keysym {
            keysym: 0x5ef,
            offset: 1144,
        },
        name_keysym {
            keysym: 0x5f0,
            offset: 1515,
        },
        name_keysym {
            keysym: 0x5f1,
            offset: 1781,
        },
        name_keysym {
            keysym: 0x5f2,
            offset: 1808,
        },
        name_keysym {
            keysym: 0x6a1,
            offset: 22996,
        },
        name_keysym {
            keysym: 0x6a2,
            offset: 19784,
        },
        name_keysym {
            keysym: 0x6a3,
            offset: 9550,
        },
        name_keysym {
            keysym: 0x6a4,
            offset: 26443,
        },
        name_keysym {
            keysym: 0x6a5,
            offset: 19756,
        },
        name_keysym {
            keysym: 0x6a6,
            offset: 26418,
        },
        name_keysym {
            keysym: 0x6a7,
            offset: 26469,
        },
        name_keysym {
            keysym: 0x6a8,
            offset: 9574,
        },
        name_keysym {
            keysym: 0x6a9,
            offset: 9713,
        },
        name_keysym {
            keysym: 0x6aa,
            offset: 9739,
        },
        name_keysym {
            keysym: 0x6ab,
            offset: 23115,
        },
        name_keysym {
            keysym: 0x6ac,
            offset: 19812,
        },
        name_keysym {
            keysym: 0x6ad,
            offset: 26380,
        },
        name_keysym {
            keysym: 0x6ae,
            offset: 8419,
        },
        name_keysym {
            keysym: 0x6af,
            offset: 9068,
        },
        name_keysym {
            keysym: 0x6b0,
            offset: 20374,
        },
        name_keysym {
            keysym: 0x6b1,
            offset: 22984,
        },
        name_keysym {
            keysym: 0x6b2,
            offset: 19770,
        },
        name_keysym {
            keysym: 0x6b3,
            offset: 9538,
        },
        name_keysym {
            keysym: 0x6b4,
            offset: 26430,
        },
        name_keysym {
            keysym: 0x6b5,
            offset: 19742,
        },
        name_keysym {
            keysym: 0x6b6,
            offset: 26406,
        },
        name_keysym {
            keysym: 0x6b7,
            offset: 26456,
        },
        name_keysym {
            keysym: 0x6b8,
            offset: 9562,
        },
        name_keysym {
            keysym: 0x6b9,
            offset: 9700,
        },
        name_keysym {
            keysym: 0x6ba,
            offset: 9726,
        },
        name_keysym {
            keysym: 0x6bb,
            offset: 23102,
        },
        name_keysym {
            keysym: 0x6bc,
            offset: 19798,
        },
        name_keysym {
            keysym: 0x6bd,
            offset: 26354,
        },
        name_keysym {
            keysym: 0x6be,
            offset: 8399,
        },
        name_keysym {
            keysym: 0x6bf,
            offset: 9054,
        },
        name_keysym {
            keysym: 0x6c0,
            offset: 10294,
        },
        name_keysym {
            keysym: 0x6c1,
            offset: 8875,
        },
        name_keysym {
            keysym: 0x6c2,
            offset: 8898,
        },
        name_keysym {
            keysym: 0x6c3,
            offset: 10047,
        },
        name_keysym {
            keysym: 0x6c4,
            offset: 9042,
        },
        name_keysym {
            keysym: 0x6c5,
            offset: 9526,
        },
        name_keysym {
            keysym: 0x6c6,
            offset: 9116,
        },
        name_keysym {
            keysym: 0x6c7,
            offset: 9305,
        },
        name_keysym {
            keysym: 0x6c8,
            offset: 9364,
        },
        name_keysym {
            keysym: 0x6c9,
            offset: 9467,
        },
        name_keysym {
            keysym: 0x6ca,
            offset: 9958,
        },
        name_keysym {
            keysym: 0x6cb,
            offset: 9598,
        },
        name_keysym {
            keysym: 0x6cc,
            offset: 9140,
        },
        name_keysym {
            keysym: 0x6cd,
            offset: 9164,
        },
        name_keysym {
            keysym: 0x6ce,
            offset: 9188,
        },
        name_keysym {
            keysym: 0x6cf,
            offset: 9763,
        },
        name_keysym {
            keysym: 0x6d0,
            offset: 9816,
        },
        name_keysym {
            keysym: 0x6d1,
            offset: 10242,
        },
        name_keysym {
            keysym: 0x6d2,
            offset: 9256,
        },
        name_keysym {
            keysym: 0x6d3,
            offset: 9280,
        },
        name_keysym {
            keysym: 0x6d4,
            offset: 10022,
        },
        name_keysym {
            keysym: 0x6d5,
            offset: 10071,
        },
        name_keysym {
            keysym: 0x6d6,
            offset: 10343,
        },
        name_keysym {
            keysym: 0x6d7,
            offset: 10218,
        },
        name_keysym {
            keysym: 0x6d8,
            offset: 9992,
        },
        name_keysym {
            keysym: 0x6d9,
            offset: 10268,
        },
        name_keysym {
            keysym: 0x6da,
            offset: 10318,
        },
        name_keysym {
            keysym: 0x6db,
            offset: 9871,
        },
        name_keysym {
            keysym: 0x6dc,
            offset: 9093,
        },
        name_keysym {
            keysym: 0x6dd,
            offset: 9899,
        },
        name_keysym {
            keysym: 0x6de,
            offset: 8923,
        },
        name_keysym {
            keysym: 0x6df,
            offset: 9438,
        },
        name_keysym {
            keysym: 0x6e0,
            offset: 10282,
        },
        name_keysym {
            keysym: 0x6e1,
            offset: 8864,
        },
        name_keysym {
            keysym: 0x6e2,
            offset: 8886,
        },
        name_keysym {
            keysym: 0x6e3,
            offset: 10034,
        },
        name_keysym {
            keysym: 0x6e4,
            offset: 9030,
        },
        name_keysym {
            keysym: 0x6e5,
            offset: 9514,
        },
        name_keysym {
            keysym: 0x6e6,
            offset: 9104,
        },
        name_keysym {
            keysym: 0x6e7,
            offset: 9292,
        },
        name_keysym {
            keysym: 0x6e8,
            offset: 9352,
        },
        name_keysym {
            keysym: 0x6e9,
            offset: 9456,
        },
        name_keysym {
            keysym: 0x6ea,
            offset: 9942,
        },
        name_keysym {
            keysym: 0x6eb,
            offset: 9586,
        },
        name_keysym {
            keysym: 0x6ec,
            offset: 9128,
        },
        name_keysym {
            keysym: 0x6ed,
            offset: 9152,
        },
        name_keysym {
            keysym: 0x6ee,
            offset: 9176,
        },
        name_keysym {
            keysym: 0x6ef,
            offset: 9752,
        },
        name_keysym {
            keysym: 0x6f0,
            offset: 9804,
        },
        name_keysym {
            keysym: 0x6f1,
            offset: 10230,
        },
        name_keysym {
            keysym: 0x6f2,
            offset: 9244,
        },
        name_keysym {
            keysym: 0x6f3,
            offset: 9268,
        },
        name_keysym {
            keysym: 0x6f4,
            offset: 10010,
        },
        name_keysym {
            keysym: 0x6f5,
            offset: 10060,
        },
        name_keysym {
            keysym: 0x6f6,
            offset: 10330,
        },
        name_keysym {
            keysym: 0x6f7,
            offset: 10206,
        },
        name_keysym {
            keysym: 0x6f8,
            offset: 9974,
        },
        name_keysym {
            keysym: 0x6f9,
            offset: 10254,
        },
        name_keysym {
            keysym: 0x6fa,
            offset: 10306,
        },
        name_keysym {
            keysym: 0x6fb,
            offset: 9858,
        },
        name_keysym {
            keysym: 0x6fc,
            offset: 9082,
        },
        name_keysym {
            keysym: 0x6fd,
            offset: 9884,
        },
        name_keysym {
            keysym: 0x6fe,
            offset: 8910,
        },
        name_keysym {
            keysym: 0x6ff,
            offset: 9420,
        },
        name_keysym {
            keysym: 0x7a1,
            offset: 13473,
        },
        name_keysym {
            keysym: 0x7a2,
            offset: 13603,
        },
        name_keysym {
            keysym: 0x7a3,
            offset: 13663,
        },
        name_keysym {
            keysym: 0x7a4,
            offset: 13778,
        },
        name_keysym {
            keysym: 0x7a5,
            offset: 13857,
        },
        name_keysym {
            keysym: 0x7a7,
            offset: 14093,
        },
        name_keysym {
            keysym: 0x7a8,
            offset: 14320,
        },
        name_keysym {
            keysym: 0x7a9,
            offset: 14388,
        },
        name_keysym {
            keysym: 0x7ab,
            offset: 14029,
        },
        name_keysym {
            keysym: 0x7ae,
            offset: 13428,
        },
        name_keysym {
            keysym: 0x7af,
            offset: 13741,
        },
        name_keysym {
            keysym: 0x7b1,
            offset: 13491,
        },
        name_keysym {
            keysym: 0x7b2,
            offset: 13623,
        },
        name_keysym {
            keysym: 0x7b3,
            offset: 13679,
        },
        name_keysym {
            keysym: 0x7b4,
            offset: 13795,
        },
        name_keysym {
            keysym: 0x7b5,
            offset: 13876,
        },
        name_keysym {
            keysym: 0x7b6,
            offset: 13812,
        },
        name_keysym {
            keysym: 0x7b7,
            offset: 14113,
        },
        name_keysym {
            keysym: 0x7b8,
            offset: 14340,
        },
        name_keysym {
            keysym: 0x7b9,
            offset: 14410,
        },
        name_keysym {
            keysym: 0x7ba,
            offset: 14360,
        },
        name_keysym {
            keysym: 0x7bb,
            offset: 14047,
        },
        name_keysym {
            keysym: 0x7c1,
            offset: 13449,
        },
        name_keysym {
            keysym: 0x7c2,
            offset: 13509,
        },
        name_keysym {
            keysym: 0x7c3,
            offset: 13717,
        },
        name_keysym {
            keysym: 0x7c4,
            offset: 13551,
        },
        name_keysym {
            keysym: 0x7c5,
            offset: 13575,
        },
        name_keysym {
            keysym: 0x7c6,
            offset: 14450,
        },
        name_keysym {
            keysym: 0x7c7,
            offset: 13643,
        },
        name_keysym {
            keysym: 0x7c8,
            offset: 14268,
        },
        name_keysym {
            keysym: 0x7c9,
            offset: 13756,
        },
        name_keysym {
            keysym: 0x7ca,
            offset: 13895,
        },
        name_keysym {
            keysym: 0x7cb,
            offset: 13945,
        },
        name_keysym {
            keysym: 0x7cc,
            offset: 13969,
        },
        name_keysym {
            keysym: 0x7cd,
            offset: 13987,
        },
        name_keysym {
            keysym: 0x7ce,
            offset: 14432,
        },
        name_keysym {
            keysym: 0x7cf,
            offset: 14065,
        },
        name_keysym {
            keysym: 0x7d0,
            offset: 14153,
        },
        name_keysym {
            keysym: 0x7d1,
            offset: 14191,
        },
        name_keysym {
            keysym: 0x7d2,
            offset: 14211,
        },
        name_keysym {
            keysym: 0x7d4,
            offset: 14248,
        },
        name_keysym {
            keysym: 0x7d5,
            offset: 14292,
        },
        name_keysym {
            keysym: 0x7d6,
            offset: 14133,
        },
        name_keysym {
            keysym: 0x7d7,
            offset: 13531,
        },
        name_keysym {
            keysym: 0x7d8,
            offset: 14171,
        },
        name_keysym {
            keysym: 0x7d9,
            offset: 14005,
        },
        name_keysym {
            keysym: 0x7e1,
            offset: 13461,
        },
        name_keysym {
            keysym: 0x7e2,
            offset: 13520,
        },
        name_keysym {
            keysym: 0x7e3,
            offset: 13729,
        },
        name_keysym {
            keysym: 0x7e4,
            offset: 13563,
        },
        name_keysym {
            keysym: 0x7e5,
            offset: 13589,
        },
        name_keysym {
            keysym: 0x7e6,
            offset: 14461,
        },
        name_keysym {
            keysym: 0x7e7,
            offset: 13653,
        },
        name_keysym {
            keysym: 0x7e8,
            offset: 14280,
        },
        name_keysym {
            keysym: 0x7e9,
            offset: 13767,
        },
        name_keysym {
            keysym: 0x7ea,
            offset: 13907,
        },
        name_keysym {
            keysym: 0x7eb,
            offset: 13957,
        },
        name_keysym {
            keysym: 0x7ec,
            offset: 13978,
        },
        name_keysym {
            keysym: 0x7ed,
            offset: 13996,
        },
        name_keysym {
            keysym: 0x7ee,
            offset: 14441,
        },
        name_keysym {
            keysym: 0x7ef,
            offset: 14079,
        },
        name_keysym {
            keysym: 0x7f0,
            offset: 14162,
        },
        name_keysym {
            keysym: 0x7f1,
            offset: 14201,
        },
        name_keysym {
            keysym: 0x7f2,
            offset: 14223,
        },
        name_keysym {
            keysym: 0x7f3,
            offset: 13695,
        },
        name_keysym {
            keysym: 0x7f4,
            offset: 14258,
        },
        name_keysym {
            keysym: 0x7f5,
            offset: 14306,
        },
        name_keysym {
            keysym: 0x7f6,
            offset: 14143,
        },
        name_keysym {
            keysym: 0x7f7,
            offset: 13541,
        },
        name_keysym {
            keysym: 0x7f8,
            offset: 14181,
        },
        name_keysym {
            keysym: 0x7f9,
            offset: 14017,
        },
        name_keysym {
            keysym: 0x8a1,
            offset: 19516,
        },
        name_keysym {
            keysym: 0x8a2,
            offset: 25852,
        },
        name_keysym {
            keysym: 0x8a3,
            offset: 16827,
        },
        name_keysym {
            keysym: 0x8a4,
            offset: 25826,
        },
        name_keysym {
            keysym: 0x8a5,
            offset: 3422,
        },
        name_keysym {
            keysym: 0x8a6,
            offset: 26736,
        },
        name_keysym {
            keysym: 0x8a7,
            offset: 25867,
        },
        name_keysym {
            keysym: 0x8a8,
            offset: 3448,
        },
        name_keysym {
            keysym: 0x8a9,
            offset: 25916,
        },
        name_keysym {
            keysym: 0x8aa,
            offset: 3497,
        },
        name_keysym {
            keysym: 0x8ab,
            offset: 25838,
        },
        name_keysym {
            keysym: 0x8ac,
            offset: 3434,
        },
        name_keysym {
            keysym: 0x8ad,
            offset: 25901,
        },
        name_keysym {
            keysym: 0x8ae,
            offset: 3482,
        },
        name_keysym {
            keysym: 0x8af,
            offset: 19466,
        },
        name_keysym {
            keysym: 0x8b0,
            offset: 22638,
        },
        name_keysym {
            keysym: 0x8b1,
            offset: 25884,
        },
        name_keysym {
            keysym: 0x8b2,
            offset: 3465,
        },
        name_keysym {
            keysym: 0x8b3,
            offset: 25957,
        },
        name_keysym {
            keysym: 0x8b4,
            offset: 3538,
        },
        name_keysym {
            keysym: 0x8b5,
            offset: 25934,
        },
        name_keysym {
            keysym: 0x8b6,
            offset: 3515,
        },
        name_keysym {
            keysym: 0x8b7,
            offset: 22660,
        },
        name_keysym {
            keysym: 0x8bc,
            offset: 19602,
        },
        name_keysym {
            keysym: 0x8bd,
            offset: 20310,
        },
        name_keysym {
            keysym: 0x8be,
            offset: 13411,
        },
        name_keysym {
            keysym: 0x8bf,
            offset: 17473,
        },
        name_keysym {
            keysym: 0x8c0,
            offset: 25710,
        },
        name_keysym {
            keysym: 0x8c1,
            offset: 26718,
        },
        name_keysym {
            keysym: 0x8c2,
            offset: 17435,
        },
        name_keysym {
            keysym: 0x8c5,
            offset: 20132,
        },
        name_keysym {
            keysym: 0x8c8,
            offset: 954,
        },
        name_keysym {
            keysym: 0x8c9,
            offset: 23223,
        },
        name_keysym {
            keysym: 0x8cd,
            offset: 17356,
        },
        name_keysym {
            keysym: 0x8ce,
            offset: 17407,
        },
        name_keysym {
            keysym: 0x8cf,
            offset: 17315,
        },
        name_keysym {
            keysym: 0x8d6,
            offset: 22484,
        },
        name_keysym {
            keysym: 0x8da,
            offset: 17415,
        },
        name_keysym {
            keysym: 0x8db,
            offset: 17426,
        },
        name_keysym {
            keysym: 0x8dc,
            offset: 17482,
        },
        name_keysym {
            keysym: 0x8dd,
            offset: 26593,
        },
        name_keysym {
            keysym: 0x8de,
            offset: 19642,
        },
        name_keysym {
            keysym: 0x8df,
            offset: 19653,
        },
        name_keysym {
            keysym: 0x8ef,
            offset: 21661,
        },
        name_keysym {
            keysym: 0x8f6,
            offset: 12787,
        },
        name_keysym {
            keysym: 0x8fb,
            offset: 19426,
        },
        name_keysym {
            keysym: 0x8fc,
            offset: 26618,
        },
        name_keysym {
            keysym: 0x8fd,
            offset: 22595,
        },
        name_keysym {
            keysym: 0x8fe,
            offset: 11463,
        },
        name_keysym {
            keysym: 0x9df,
            offset: 3410,
        },
        name_keysym {
            keysym: 0x9e0,
            offset: 24014,
        },
        name_keysym {
            keysym: 0x9e1,
            offset: 8603,
        },
        name_keysym {
            keysym: 0x9e2,
            offset: 17203,
        },
        name_keysym {
            keysym: 0x9e3,
            offset: 12544,
        },
        name_keysym {
            keysym: 0x9e4,
            offset: 8809,
        },
        name_keysym {
            keysym: 0x9e5,
            offset: 19616,
        },
        name_keysym {
            keysym: 0x9e8,
            offset: 20260,
        },
        name_keysym {
            keysym: 0x9e9,
            offset: 26773,
        },
        name_keysym {
            keysym: 0x9ea,
            offset: 19687,
        },
        name_keysym {
            keysym: 0x9eb,
            offset: 26647,
        },
        name_keysym {
            keysym: 0x9ec,
            offset: 26634,
        },
        name_keysym {
            keysym: 0x9ed,
            offset: 19673,
        },
        name_keysym {
            keysym: 0x9ee,
            offset: 8812,
        },
        name_keysym {
            keysym: 0x9ef,
            offset: 16842,
        },
        name_keysym {
            keysym: 0x9f0,
            offset: 16857,
        },
        name_keysym {
            keysym: 0x9f1,
            offset: 16872,
        },
        name_keysym {
            keysym: 0x9f2,
            offset: 16887,
        },
        name_keysym {
            keysym: 0x9f3,
            offset: 16902,
        },
        name_keysym {
            keysym: 0x9f4,
            offset: 19582,
        },
        name_keysym {
            keysym: 0x9f5,
            offset: 22769,
        },
        name_keysym {
            keysym: 0x9f6,
            offset: 3533,
        },
        name_keysym {
            keysym: 0x9f7,
            offset: 25952,
        },
        name_keysym {
            keysym: 0x9f8,
            offset: 26728,
        },
        name_keysym {
            keysym: 0xaa1,
            offset: 12059,
        },
        name_keysym {
            keysym: 0xaa2,
            offset: 12158,
        },
        name_keysym {
            keysym: 0xaa3,
            offset: 11952,
        },
        name_keysym {
            keysym: 0xaa4,
            offset: 11961,
        },
        name_keysym {
            keysym: 0xaa5,
            offset: 11352,
        },
        name_keysym {
            keysym: 0xaa6,
            offset: 22343,
        },
        name_keysym {
            keysym: 0xaa7,
            offset: 25720,
        },
        name_keysym {
            keysym: 0xaa8,
            offset: 14542,
        },
        name_keysym {
            keysym: 0xaa9,
            offset: 11986,
        },
        name_keysym {
            keysym: 0xaaa,
            offset: 12071,
        },
        name_keysym {
            keysym: 0xaac,
            offset: 23211,
        },
        name_keysym {
            keysym: 0xaae,
            offset: 11943,
        },
        name_keysym {
            keysym: 0xaaf,
            offset: 11398,
        },
        name_keysym {
            keysym: 0xab0,
            offset: 20952,
        },
        name_keysym {
            keysym: 0xab1,
            offset: 26068,
        },
        name_keysym {
            keysym: 0xab2,
            offset: 20890,
        },
        name_keysym {
            keysym: 0xab3,
            offset: 26033,
        },
        name_keysym {
            keysym: 0xab4,
            offset: 25761,
        },
        name_keysym {
            keysym: 0xab5,
            offset: 12738,
        },
        name_keysym {
            keysym: 0xab6,
            offset: 20918,
        },
        name_keysym {
            keysym: 0xab7,
            offset: 12700,
        },
        name_keysym {
            keysym: 0xab8,
            offset: 8506,
        },
        name_keysym {
            keysym: 0xabb,
            offset: 12558,
        },
        name_keysym {
            keysym: 0xabc,
            offset: 19409,
        },
        name_keysym {
            keysym: 0xabd,
            offset: 11271,
        },
        name_keysym {
            keysym: 0xabe,
            offset: 22577,
        },
        name_keysym {
            keysym: 0xabf,
            offset: 19866,
        },
        name_keysym {
            keysym: 0xac3,
            offset: 20880,
        },
        name_keysym {
            keysym: 0xac4,
            offset: 25748,
        },
        name_keysym {
            keysym: 0xac5,
            offset: 12688,
        },
        name_keysym {
            keysym: 0xac6,
            offset: 23128,
        },
        name_keysym {
            keysym: 0xac9,
            offset: 25991,
        },
        name_keysym {
            keysym: 0xaca,
            offset: 23197,
        },
        name_keysym {
            keysym: 0xacb,
            offset: 26001,
        },
        name_keysym {
            keysym: 0xacc,
            offset: 19487,
        },
        name_keysym {
            keysym: 0xacd,
            offset: 22681,
        },
        name_keysym {
            keysym: 0xace,
            offset: 12021,
        },
        name_keysym {
            keysym: 0xacf,
            offset: 12034,
        },
        name_keysym {
            keysym: 0xad0,
            offset: 19562,
        },
        name_keysym {
            keysym: 0xad1,
            offset: 22748,
        },
        name_keysym {
            keysym: 0xad2,
            offset: 19446,
        },
        name_keysym {
            keysym: 0xad3,
            offset: 22617,
        },
        name_keysym {
            keysym: 0xad4,
            offset: 22265,
        },
        name_keysym {
            keysym: 0xad5,
            offset: 21715,
        },
        name_keysym {
            keysym: 0xad6,
            offset: 19924,
        },
        name_keysym {
            keysym: 0xad7,
            offset: 22935,
        },
        name_keysym {
            keysym: 0xad9,
            offset: 19341,
        },
        name_keysym {
            keysym: 0xada,
            offset: 16786,
        },
        name_keysym {
            keysym: 0xadb,
            offset: 12586,
        },
        name_keysym {
            keysym: 0xadc,
            offset: 12566,
        },
        name_keysym {
            keysym: 0xadd,
            offset: 12603,
        },
        name_keysym {
            keysym: 0xade,
            offset: 11993,
        },
        name_keysym {
            keysym: 0xadf,
            offset: 12008,
        },
        name_keysym {
            keysym: 0xae0,
            offset: 12122,
        },
        name_keysym {
            keysym: 0xae1,
            offset: 12139,
        },
        name_keysym {
            keysym: 0xae2,
            offset: 20979,
        },
        name_keysym {
            keysym: 0xae3,
            offset: 21021,
        },
        name_keysym {
            keysym: 0xae4,
            offset: 21003,
        },
        name_keysym {
            keysym: 0xae5,
            offset: 20994,
        },
        name_keysym {
            keysym: 0xae6,
            offset: 12078,
        },
        name_keysym {
            keysym: 0xae7,
            offset: 12097,
        },
        name_keysym {
            keysym: 0xae8,
            offset: 12644,
        },
        name_keysym {
            keysym: 0xae9,
            offset: 12624,
        },
        name_keysym {
            keysym: 0xaea,
            offset: 19504,
        },
        name_keysym {
            keysym: 0xaeb,
            offset: 22699,
        },
        name_keysym {
            keysym: 0xaec,
            offset: 8649,
        },
        name_keysym {
            keysym: 0xaed,
            offset: 11344,
        },
        name_keysym {
            keysym: 0xaee,
            offset: 16240,
        },
        name_keysym {
            keysym: 0xaf0,
            offset: 19853,
        },
        name_keysym {
            keysym: 0xaf1,
            offset: 10440,
        },
        name_keysym {
            keysym: 0xaf2,
            offset: 11426,
        },
        name_keysym {
            keysym: 0xaf3,
            offset: 8616,
        },
        name_keysym {
            keysym: 0xaf4,
            offset: 3380,
        },
        name_keysym {
            keysym: 0xaf5,
            offset: 20045,
        },
        name_keysym {
            keysym: 0xaf6,
            offset: 20033,
        },
        name_keysym {
            keysym: 0xaf7,
            offset: 19842,
        },
        name_keysym {
            keysym: 0xaf8,
            offset: 12531,
        },
        name_keysym {
            keysym: 0xaf9,
            offset: 24585,
        },
        name_keysym {
            keysym: 0xafa,
            offset: 24595,
        },
        name_keysym {
            keysym: 0xafb,
            offset: 21735,
        },
        name_keysym {
            keysym: 0xafc,
            offset: 8513,
        },
        name_keysym {
            keysym: 0xafd,
            offset: 23252,
        },
        name_keysym {
            keysym: 0xafe,
            offset: 11439,
        },
        name_keysym {
            keysym: 0xaff,
            offset: 8857,
        },
        name_keysym {
            keysym: 0xba3,
            offset: 19436,
        },
        name_keysym {
            keysym: 0xba6,
            offset: 22606,
        },
        name_keysym {
            keysym: 0xba8,
            offset: 11473,
        },
        name_keysym {
            keysym: 0xba9,
            offset: 26626,
        },
        name_keysym {
            keysym: 0xbc0,
            offset: 21522,
        },
        name_keysym {
            keysym: 0xbc2,
            offset: 11502,
        },
        name_keysym {
            keysym: 0xbc3,
            offset: 26661,
        },
        name_keysym {
            keysym: 0xbc4,
            offset: 11492,
        },
        name_keysym {
            keysym: 0xbc6,
            offset: 26568,
        },
        name_keysym {
            keysym: 0xbca,
            offset: 18302,
        },
        name_keysym {
            keysym: 0xbcc,
            offset: 22358,
        },
        name_keysym {
            keysym: 0xbce,
            offset: 26676,
        },
        name_keysym {
            keysym: 0xbcf,
            offset: 8626,
        },
        name_keysym {
            keysym: 0xbd3,
            offset: 26668,
        },
        name_keysym {
            keysym: 0xbd6,
            offset: 11483,
        },
        name_keysym {
            keysym: 0xbd8,
            offset: 22712,
        },
        name_keysym {
            keysym: 0xbda,
            offset: 19528,
        },
        name_keysym {
            keysym: 0xbdc,
            offset: 19588,
        },
        name_keysym {
            keysym: 0xbfc,
            offset: 22776,
        },
        name_keysym {
            keysym: 0xcdf,
            offset: 16333,
        },
        name_keysym {
            keysym: 0xce0,
            offset: 16246,
        },
        name_keysym {
            keysym: 0xce1,
            offset: 16271,
        },
        name_keysym {
            keysym: 0xce2,
            offset: 16452,
        },
        name_keysym {
            keysym: 0xce3,
            offset: 16306,
        },
        name_keysym {
            keysym: 0xce4,
            offset: 16479,
        },
        name_keysym {
            keysym: 0xce5,
            offset: 16691,
        },
        name_keysym {
            keysym: 0xce6,
            offset: 16737,
        },
        name_keysym {
            keysym: 0xce7,
            offset: 16294,
        },
        name_keysym {
            keysym: 0xce8,
            offset: 16668,
        },
        name_keysym {
            keysym: 0xce9,
            offset: 16702,
        },
        name_keysym {
            keysym: 0xcea,
            offset: 16354,
        },
        name_keysym {
            keysym: 0xceb,
            offset: 16500,
        },
        name_keysym {
            keysym: 0xcec,
            offset: 16523,
        },
        name_keysym {
            keysym: 0xced,
            offset: 16371,
        },
        name_keysym {
            keysym: 0xcee,
            offset: 16536,
        },
        name_keysym {
            keysym: 0xcef,
            offset: 16387,
        },
        name_keysym {
            keysym: 0xcf0,
            offset: 16547,
        },
        name_keysym {
            keysym: 0xcf1,
            offset: 16592,
        },
        name_keysym {
            keysym: 0xcf2,
            offset: 16259,
        },
        name_keysym {
            keysym: 0xcf3,
            offset: 16403,
        },
        name_keysym {
            keysym: 0xcf4,
            offset: 16558,
        },
        name_keysym {
            keysym: 0xcf5,
            offset: 16418,
        },
        name_keysym {
            keysym: 0xcf6,
            offset: 16713,
        },
        name_keysym {
            keysym: 0xcf7,
            offset: 16568,
        },
        name_keysym {
            keysym: 0xcf8,
            offset: 16580,
        },
        name_keysym {
            keysym: 0xcf9,
            offset: 16620,
        },
        name_keysym {
            keysym: 0xcfa,
            offset: 16657,
        },
        name_keysym {
            keysym: 0xda1,
            offset: 24848,
        },
        name_keysym {
            keysym: 0xda2,
            offset: 24778,
        },
        name_keysym {
            keysym: 0xda3,
            offset: 24804,
        },
        name_keysym {
            keysym: 0xda4,
            offset: 24818,
        },
        name_keysym {
            keysym: 0xda5,
            offset: 24791,
        },
        name_keysym {
            keysym: 0xda6,
            offset: 24832,
        },
        name_keysym {
            keysym: 0xda7,
            offset: 25161,
        },
        name_keysym {
            keysym: 0xda8,
            offset: 24654,
        },
        name_keysym {
            keysym: 0xda9,
            offset: 24681,
        },
        name_keysym {
            keysym: 0xdaa,
            offset: 24667,
        },
        name_keysym {
            keysym: 0xdab,
            offset: 25522,
        },
        name_keysym {
            keysym: 0xdac,
            offset: 24695,
        },
        name_keysym {
            keysym: 0xdad,
            offset: 25698,
        },
        name_keysym {
            keysym: 0xdae,
            offset: 24708,
        },
        name_keysym {
            keysym: 0xdaf,
            offset: 25651,
        },
        name_keysym {
            keysym: 0xdb0,
            offset: 25610,
        },
        name_keysym {
            keysym: 0xdb1,
            offset: 25560,
        },
        name_keysym {
            keysym: 0xdb2,
            offset: 25579,
        },
        name_keysym {
            keysym: 0xdb3,
            offset: 25187,
        },
        name_keysym {
            keysym: 0xdb4,
            offset: 24721,
        },
        name_keysym {
            keysym: 0xdb5,
            offset: 25664,
        },
        name_keysym {
            keysym: 0xdb6,
            offset: 25637,
        },
        name_keysym {
            keysym: 0xdb7,
            offset: 25595,
        },
        name_keysym {
            keysym: 0xdb8,
            offset: 25623,
        },
        name_keysym {
            keysym: 0xdb9,
            offset: 25198,
        },
        name_keysym {
            keysym: 0xdba,
            offset: 24640,
        },
        name_keysym {
            keysym: 0xdbb,
            offset: 25289,
        },
        name_keysym {
            keysym: 0xdbc,
            offset: 25259,
        },
        name_keysym {
            keysym: 0xdbd,
            offset: 24732,
        },
        name_keysym {
            keysym: 0xdbe,
            offset: 25246,
        },
        name_keysym {
            keysym: 0xdbf,
            offset: 24742,
        },
        name_keysym {
            keysym: 0xdc0,
            offset: 25273,
        },
        name_keysym {
            keysym: 0xdc1,
            offset: 25151,
        },
        name_keysym {
            keysym: 0xdc2,
            offset: 25687,
        },
        name_keysym {
            keysym: 0xdc3,
            offset: 25300,
        },
        name_keysym {
            keysym: 0xdc4,
            offset: 25311,
        },
        name_keysym {
            keysym: 0xdc5,
            offset: 25011,
        },
        name_keysym {
            keysym: 0xdc6,
            offset: 25023,
        },
        name_keysym {
            keysym: 0xdc7,
            offset: 25675,
        },
        name_keysym {
            keysym: 0xdc8,
            offset: 25510,
        },
        name_keysym {
            keysym: 0xdc9,
            offset: 25498,
        },
        name_keysym {
            keysym: 0xdca,
            offset: 25532,
        },
        name_keysym {
            keysym: 0xdcb,
            offset: 24753,
        },
        name_keysym {
            keysym: 0xdcc,
            offset: 24998,
        },
        name_keysym {
            keysym: 0xdcd,
            offset: 25208,
        },
        name_keysym {
            keysym: 0xdce,
            offset: 24764,
        },
        name_keysym {
            keysym: 0xdcf,
            offset: 25218,
        },
        name_keysym {
            keysym: 0xdd0,
            offset: 25319,
        },
        name_keysym {
            keysym: 0xdd1,
            offset: 25059,
        },
        name_keysym {
            keysym: 0xdd2,
            offset: 25330,
        },
        name_keysym {
            keysym: 0xdd3,
            offset: 25393,
        },
        name_keysym {
            keysym: 0xdd4,
            offset: 25416,
        },
        name_keysym {
            keysym: 0xdd5,
            offset: 25427,
        },
        name_keysym {
            keysym: 0xdd6,
            offset: 25461,
        },
        name_keysym {
            keysym: 0xdd7,
            offset: 25473,
        },
        name_keysym {
            keysym: 0xdd8,
            offset: 25450,
        },
        name_keysym {
            keysym: 0xdd9,
            offset: 25486,
        },
        name_keysym {
            keysym: 0xdda,
            offset: 25233,
        },
        name_keysym {
            keysym: 0xdde,
            offset: 25075,
        },
        name_keysym {
            keysym: 0xddf,
            offset: 24630,
        },
        name_keysym {
            keysym: 0xde0,
            offset: 25405,
        },
        name_keysym {
            keysym: 0xde1,
            offset: 25342,
        },
        name_keysym {
            keysym: 0xde2,
            offset: 25439,
        },
        name_keysym {
            keysym: 0xde3,
            offset: 25374,
        },
        name_keysym {
            keysym: 0xde4,
            offset: 25354,
        },
        name_keysym {
            keysym: 0xde5,
            offset: 24859,
        },
        name_keysym {
            keysym: 0xde6,
            offset: 25137,
        },
        name_keysym {
            keysym: 0xde7,
            offset: 25098,
        },
        name_keysym {
            keysym: 0xde8,
            offset: 25048,
        },
        name_keysym {
            keysym: 0xde9,
            offset: 25113,
        },
        name_keysym {
            keysym: 0xdea,
            offset: 25125,
        },
        name_keysym {
            keysym: 0xdeb,
            offset: 25031,
        },
        name_keysym {
            keysym: 0xdec,
            offset: 25543,
        },
        name_keysym {
            keysym: 0xded,
            offset: 25173,
        },
        name_keysym {
            keysym: 0xdf0,
            offset: 24986,
        },
        name_keysym {
            keysym: 0xdf1,
            offset: 24924,
        },
        name_keysym {
            keysym: 0xdf2,
            offset: 24973,
        },
        name_keysym {
            keysym: 0xdf3,
            offset: 24950,
        },
        name_keysym {
            keysym: 0xdf4,
            offset: 24962,
        },
        name_keysym {
            keysym: 0xdf5,
            offset: 24889,
        },
        name_keysym {
            keysym: 0xdf6,
            offset: 24900,
        },
        name_keysym {
            keysym: 0xdf7,
            offset: 24876,
        },
        name_keysym {
            keysym: 0xdf8,
            offset: 24937,
        },
        name_keysym {
            keysym: 0xdf9,
            offset: 24912,
        },
        name_keysym {
            keysym: 0xea1,
            offset: 15341,
        },
        name_keysym {
            keysym: 0xea2,
            offset: 15886,
        },
        name_keysym {
            keysym: 0xea3,
            offset: 15355,
        },
        name_keysym {
            keysym: 0xea4,
            offset: 15436,
        },
        name_keysym {
            keysym: 0xea5,
            offset: 15467,
        },
        name_keysym {
            keysym: 0xea6,
            offset: 15449,
        },
        name_keysym {
            keysym: 0xea7,
            offset: 14648,
        },
        name_keysym {
            keysym: 0xea8,
            offset: 15849,
        },
        name_keysym {
            keysym: 0xea9,
            offset: 15621,
        },
        name_keysym {
            keysym: 0xeaa,
            offset: 15652,
        },
        name_keysym {
            keysym: 0xeab,
            offset: 15671,
        },
        name_keysym {
            keysym: 0xeac,
            offset: 15708,
        },
        name_keysym {
            keysym: 0xead,
            offset: 15726,
        },
        name_keysym {
            keysym: 0xeae,
            offset: 15743,
        },
        name_keysym {
            keysym: 0xeaf,
            offset: 15689,
        },
        name_keysym {
            keysym: 0xeb0,
            offset: 15634,
        },
        name_keysym {
            keysym: 0xeb1,
            offset: 15398,
        },
        name_keysym {
            keysym: 0xeb2,
            offset: 15533,
        },
        name_keysym {
            keysym: 0xeb3,
            offset: 15905,
        },
        name_keysym {
            keysym: 0xeb4,
            offset: 15546,
        },
        name_keysym {
            keysym: 0xeb5,
            offset: 15822,
        },
        name_keysym {
            keysym: 0xeb6,
            offset: 15923,
        },
        name_keysym {
            keysym: 0xeb7,
            offset: 14737,
        },
        name_keysym {
            keysym: 0xeb8,
            offset: 15314,
        },
        name_keysym {
            keysym: 0xeb9,
            offset: 15868,
        },
        name_keysym {
            keysym: 0xeba,
            offset: 14618,
        },
        name_keysym {
            keysym: 0xebb,
            offset: 15327,
        },
        name_keysym {
            keysym: 0xebc,
            offset: 16043,
        },
        name_keysym {
            keysym: 0xebd,
            offset: 15519,
        },
        name_keysym {
            keysym: 0xebe,
            offset: 14715,
        },
        name_keysym {
            keysym: 0xebf,
            offset: 14559,
        },
        name_keysym {
            keysym: 0xec0,
            offset: 14568,
        },
        name_keysym {
            keysym: 0xec1,
            offset: 16117,
        },
        name_keysym {
            keysym: 0xec2,
            offset: 16127,
        },
        name_keysym {
            keysym: 0xec3,
            offset: 14682,
        },
        name_keysym {
            keysym: 0xec4,
            offset: 14662,
        },
        name_keysym {
            keysym: 0xec5,
            offset: 16148,
        },
        name_keysym {
            keysym: 0xec6,
            offset: 16138,
        },
        name_keysym {
            keysym: 0xec7,
            offset: 15485,
        },
        name_keysym {
            keysym: 0xec8,
            offset: 16065,
        },
        name_keysym {
            keysym: 0xec9,
            offset: 16075,
        },
        name_keysym {
            keysym: 0xeca,
            offset: 15494,
        },
        name_keysym {
            keysym: 0xecb,
            offset: 16188,
        },
        name_keysym {
            keysym: 0xecc,
            offset: 16056,
        },
        name_keysym {
            keysym: 0xecd,
            offset: 16096,
        },
        name_keysym {
            keysym: 0xece,
            offset: 16086,
        },
        name_keysym {
            keysym: 0xecf,
            offset: 16107,
        },
        name_keysym {
            keysym: 0xed0,
            offset: 16198,
        },
        name_keysym {
            keysym: 0xed1,
            offset: 14692,
        },
        name_keysym {
            keysym: 0xed2,
            offset: 16178,
        },
        name_keysym {
            keysym: 0xed3,
            offset: 14728,
        },
        name_keysym {
            keysym: 0xed4,
            offset: 14842,
        },
        name_keysym {
            keysym: 0xed5,
            offset: 15212,
        },
        name_keysym {
            keysym: 0xed6,
            offset: 14858,
        },
        name_keysym {
            keysym: 0xed7,
            offset: 14920,
        },
        name_keysym {
            keysym: 0xed8,
            offset: 14955,
        },
        name_keysym {
            keysym: 0xed9,
            offset: 14935,
        },
        name_keysym {
            keysym: 0xeda,
            offset: 14765,
        },
        name_keysym {
            keysym: 0xedb,
            offset: 15042,
        },
        name_keysym {
            keysym: 0xedc,
            offset: 15077,
        },
        name_keysym {
            keysym: 0xedd,
            offset: 15098,
        },
        name_keysym {
            keysym: 0xede,
            offset: 15139,
        },
        name_keysym {
            keysym: 0xedf,
            offset: 15159,
        },
        name_keysym {
            keysym: 0xee0,
            offset: 15178,
        },
        name_keysym {
            keysym: 0xee1,
            offset: 15118,
        },
        name_keysym {
            keysym: 0xee2,
            offset: 15057,
        },
        name_keysym {
            keysym: 0xee3,
            offset: 14905,
        },
        name_keysym {
            keysym: 0xee4,
            offset: 15008,
        },
        name_keysym {
            keysym: 0xee5,
            offset: 15023,
        },
        name_keysym {
            keysym: 0xee6,
            offset: 15198,
        },
        name_keysym {
            keysym: 0xee7,
            offset: 15233,
        },
        name_keysym {
            keysym: 0xee8,
            offset: 14796,
        },
        name_keysym {
            keysym: 0xee9,
            offset: 14811,
        },
        name_keysym {
            keysym: 0xeea,
            offset: 14750,
        },
        name_keysym {
            keysym: 0xeeb,
            offset: 14826,
        },
        name_keysym {
            keysym: 0xeec,
            offset: 15252,
        },
        name_keysym {
            keysym: 0xeed,
            offset: 14992,
        },
        name_keysym {
            keysym: 0xeee,
            offset: 14781,
        },
        name_keysym {
            keysym: 0xeef,
            offset: 15761,
        },
        name_keysym {
            keysym: 0xef0,
            offset: 15953,
        },
        name_keysym {
            keysym: 0xef1,
            offset: 16004,
        },
        name_keysym {
            keysym: 0xef2,
            offset: 15504,
        },
        name_keysym {
            keysym: 0xef3,
            offset: 15373,
        },
        name_keysym {
            keysym: 0xef4,
            offset: 15978,
        },
        name_keysym {
            keysym: 0xef5,
            offset: 16159,
        },
        name_keysym {
            keysym: 0xef6,
            offset: 14578,
        },
        name_keysym {
            keysym: 0xef7,
            offset: 14591,
        },
        name_keysym {
            keysym: 0xef8,
            offset: 14975,
        },
        name_keysym {
            keysym: 0xef9,
            offset: 14878,
        },
        name_keysym {
            keysym: 0xefa,
            offset: 15267,
        },
        name_keysym {
            keysym: 0xeff,
            offset: 18957,
        },
        name_keysym {
            keysym: 0x13bc,
            offset: 20699,
        },
        name_keysym {
            keysym: 0x13bd,
            offset: 20702,
        },
        name_keysym {
            keysym: 0x13be,
            offset: 32623,
        },
        name_keysym {
            keysym: 0x20ac,
            offset: 12221,
        },
        name_keysym {
            keysym: 0xfd01,
            offset: 125,
        },
        name_keysym {
            keysym: 0xfd02,
            offset: 195,
        },
        name_keysym {
            keysym: 0xfd03,
            offset: 343,
        },
        name_keysym {
            keysym: 0xfd04,
            offset: 245,
        },
        name_keysym {
            keysym: 0xfd05,
            offset: 33,
        },
        name_keysym {
            keysym: 0xfd06,
            offset: 151,
        },
        name_keysym {
            keysym: 0xfd07,
            offset: 165,
        },
        name_keysym {
            keysym: 0xfd08,
            offset: 332,
        },
        name_keysym {
            keysym: 0xfd09,
            offset: 310,
        },
        name_keysym {
            keysym: 0xfd0a,
            offset: 256,
        },
        name_keysym {
            keysym: 0xfd0b,
            offset: 265,
        },
        name_keysym {
            keysym: 0xfd0c,
            offset: 274,
        },
        name_keysym {
            keysym: 0xfd0d,
            offset: 376,
        },
        name_keysym {
            keysym: 0xfd0e,
            offset: 23,
        },
        name_keysym {
            keysym: 0xfd0f,
            offset: 74,
        },
        name_keysym {
            keysym: 0xfd10,
            offset: 8,
        },
        name_keysym {
            keysym: 0xfd11,
            offset: 231,
        },
        name_keysym {
            keysym: 0xfd12,
            offset: 221,
        },
        name_keysym {
            keysym: 0xfd13,
            offset: 210,
        },
        name_keysym {
            keysym: 0xfd14,
            offset: 355,
        },
        name_keysym {
            keysym: 0xfd15,
            offset: 64,
        },
        name_keysym {
            keysym: 0xfd16,
            offset: 283,
        },
        name_keysym {
            keysym: 0xfd17,
            offset: 365,
        },
        name_keysym {
            keysym: 0xfd18,
            offset: 320,
        },
        name_keysym {
            keysym: 0xfd19,
            offset: 46,
        },
        name_keysym {
            keysym: 0xfd1a,
            offset: 109,
        },
        name_keysym {
            keysym: 0xfd1b,
            offset: 181,
        },
        name_keysym {
            keysym: 0xfd1c,
            offset: 91,
        },
        name_keysym {
            keysym: 0xfd1d,
            offset: 293,
        },
        name_keysym {
            keysym: 0xfd1e,
            offset: 140,
        },
        name_keysym {
            keysym: 0xfe01,
            offset: 17941,
        },
        name_keysym {
            keysym: 0xfe02,
            offset: 17824,
        },
        name_keysym {
            keysym: 0xfe03,
            offset: 17874,
        },
        name_keysym {
            keysym: 0xfe04,
            offset: 17841,
        },
        name_keysym {
            keysym: 0xfe05,
            offset: 17858,
        },
        name_keysym {
            keysym: 0xfe06,
            offset: 17729,
        },
        name_keysym {
            keysym: 0xfe07,
            offset: 17745,
        },
        name_keysym {
            keysym: 0xfe08,
            offset: 17986,
        },
        name_keysym {
            keysym: 0xfe09,
            offset: 18001,
        },
        name_keysym {
            keysym: 0xfe0a,
            offset: 18110,
        },
        name_keysym {
            keysym: 0xfe0b,
            offset: 18125,
        },
        name_keysym {
            keysym: 0xfe0c,
            offset: 17692,
        },
        name_keysym {
            keysym: 0xfe0d,
            offset: 17708,
        },
        name_keysym {
            keysym: 0xfe0e,
            offset: 17776,
        },
        name_keysym {
            keysym: 0xfe0f,
            offset: 17791,
        },
        name_keysym {
            keysym: 0xfe11,
            offset: 17924,
        },
        name_keysym {
            keysym: 0xfe12,
            offset: 17891,
        },
        name_keysym {
            keysym: 0xfe13,
            offset: 17908,
        },
        name_keysym {
            keysym: 0xfe20,
            offset: 17811,
        },
        name_keysym {
            keysym: 0xfe21,
            offset: 17969,
        },
        name_keysym {
            keysym: 0xfe22,
            offset: 17950,
        },
        name_keysym {
            keysym: 0xfe23,
            offset: 18043,
        },
        name_keysym {
            keysym: 0xfe24,
            offset: 18021,
        },
        name_keysym {
            keysym: 0xfe25,
            offset: 18063,
        },
        name_keysym {
            keysym: 0xfe26,
            offset: 18086,
        },
        name_keysym {
            keysym: 0xfe27,
            offset: 18219,
        },
        name_keysym {
            keysym: 0xfe28,
            offset: 18239,
        },
        name_keysym {
            keysym: 0xfe29,
            offset: 18170,
        },
        name_keysym {
            keysym: 0xfe2a,
            offset: 18194,
        },
        name_keysym {
            keysym: 0xfe2b,
            offset: 18145,
        },
        name_keysym {
            keysym: 0xfe2c,
            offset: 17630,
        },
        name_keysym {
            keysym: 0xfe2d,
            offset: 17651,
        },
        name_keysym {
            keysym: 0xfe2e,
            offset: 17673,
        },
        name_keysym {
            keysym: 0xfe2f,
            offset: 17609,
        },
        name_keysym {
            keysym: 0xfe30,
            offset: 17532,
        },
        name_keysym {
            keysym: 0xfe31,
            offset: 17557,
        },
        name_keysym {
            keysym: 0xfe32,
            offset: 17585,
        },
        name_keysym {
            keysym: 0xfe33,
            offset: 17514,
        },
        name_keysym {
            keysym: 0xfe34,
            offset: 17599,
        },
        name_keysym {
            keysym: 0xfe50,
            offset: 10956,
        },
        name_keysym {
            keysym: 0xfe51,
            offset: 10613,
        },
        name_keysym {
            keysym: 0xfe52,
            offset: 10852,
        },
        name_keysym {
            keysym: 0xfe53,
            offset: 11228,
        },
        name_keysym {
            keysym: 0xfe54,
            offset: 11089,
        },
        name_keysym {
            keysym: 0xfe55,
            offset: 10798,
        },
        name_keysym {
            keysym: 0xfe56,
            offset: 10537,
        },
        name_keysym {
            keysym: 0xfe57,
            offset: 10893,
        },
        name_keysym {
            keysym: 0xfe58,
            offset: 10575,
        },
        name_keysym {
            keysym: 0xfe59,
            offset: 10908,
        },
        name_keysym {
            keysym: 0xfe5a,
            offset: 10828,
        },
        name_keysym {
            keysym: 0xfe5b,
            offset: 10839,
        },
        name_keysym {
            keysym: 0xfe5c,
            offset: 11115,
        },
        name_keysym {
            keysym: 0xfe5d,
            offset: 11042,
        },
        name_keysym {
            keysym: 0xfe5e,
            offset: 11253,
        },
        name_keysym {
            keysym: 0xfe5f,
            offset: 11177,
        },
        name_keysym {
            keysym: 0xfe60,
            offset: 10713,
        },
        name_keysym {
            keysym: 0xfe61,
            offset: 10989,
        },
        name_keysym {
            keysym: 0xfe62,
            offset: 10999,
        },
        name_keysym {
            keysym: 0xfe63,
            offset: 11216,
        },
        name_keysym {
            keysym: 0xfe64,
            offset: 10521,
        },
        name_keysym {
            keysym: 0xfe65,
            offset: 10551,
        },
        name_keysym {
            keysym: 0xfe66,
            offset: 10925,
        },
        name_keysym {
            keysym: 0xfe67,
            offset: 10744,
        },
        name_keysym {
            keysym: 0xfe68,
            offset: 10727,
        },
        name_keysym {
            keysym: 0xfe69,
            offset: 10656,
        },
        name_keysym {
            keysym: 0xfe6a,
            offset: 10759,
        },
        name_keysym {
            keysym: 0xfe6b,
            offset: 10640,
        },
        name_keysym {
            keysym: 0xfe6c,
            offset: 10693,
        },
        name_keysym {
            keysym: 0xfe6d,
            offset: 11023,
        },
        name_keysym {
            keysym: 0xfe6e,
            offset: 10677,
        },
        name_keysym {
            keysym: 0xfe6f,
            offset: 10868,
        },
        name_keysym {
            keysym: 0xfe70,
            offset: 583,
        },
        name_keysym {
            keysym: 0xfe71,
            offset: 598,
        },
        name_keysym {
            keysym: 0xfe72,
            offset: 22540,
        },
        name_keysym {
            keysym: 0xfe73,
            offset: 23998,
        },
        name_keysym {
            keysym: 0xfe74,
            offset: 3564,
        },
        name_keysym {
            keysym: 0xfe75,
            offset: 24067,
        },
        name_keysym {
            keysym: 0xfe76,
            offset: 19967,
        },
        name_keysym {
            keysym: 0xfe77,
            offset: 19944,
        },
        name_keysym {
            keysym: 0xfe78,
            offset: 21530,
        },
        name_keysym {
            keysym: 0xfe79,
            offset: 21546,
        },
        name_keysym {
            keysym: 0xfe7a,
            offset: 3309,
        },
        name_keysym {
            keysym: 0xfe80,
            offset: 10514,
        },
        name_keysym {
            keysym: 0xfe81,
            offset: 10507,
        },
        name_keysym {
            keysym: 0xfe82,
            offset: 10949,
        },
        name_keysym {
            keysym: 0xfe83,
            offset: 10942,
        },
        name_keysym {
            keysym: 0xfe84,
            offset: 11016,
        },
        name_keysym {
            keysym: 0xfe85,
            offset: 11009,
        },
        name_keysym {
            keysym: 0xfe86,
            offset: 11108,
        },
        name_keysym {
            keysym: 0xfe87,
            offset: 11101,
        },
        name_keysym {
            keysym: 0xfe88,
            offset: 11246,
        },
        name_keysym {
            keysym: 0xfe89,
            offset: 11239,
        },
        name_keysym {
            keysym: 0xfe8a,
            offset: 11199,
        },
        name_keysym {
            keysym: 0xfe8b,
            offset: 10809,
        },
        name_keysym {
            keysym: 0xfe8c,
            offset: 10967,
        },
        name_keysym {
            keysym: 0xfe8d,
            offset: 10978,
        },
        name_keysym {
            keysym: 0xfe8e,
            offset: 10624,
        },
        name_keysym {
            keysym: 0xfe90,
            offset: 11076,
        },
        name_keysym {
            keysym: 0xfe91,
            offset: 10590,
        },
        name_keysym {
            keysym: 0xfe92,
            offset: 10775,
        },
        name_keysym {
            keysym: 0xfe93,
            offset: 11052,
        },
        name_keysym {
            keysym: 0xfea0,
            offset: 8600,
        },
        name_keysym {
            keysym: 0xfea1,
            offset: 8597,
        },
        name_keysym {
            keysym: 0xfea2,
            offset: 8594,
        },
        name_keysym {
            keysym: 0xfea3,
            offset: 8451,
        },
        name_keysym {
            keysym: 0xfea4,
            offset: 8447,
        },
        name_keysym {
            keysym: 0xfea5,
            offset: 8443,
        },
        name_keysym {
            keysym: 0xfed0,
            offset: 12667,
        },
        name_keysym {
            keysym: 0xfed1,
            offset: 22278,
        },
        name_keysym {
            keysym: 0xfed2,
            offset: 20213,
        },
        name_keysym {
            keysym: 0xfed4,
            offset: 19321,
        },
        name_keysym {
            keysym: 0xfed5,
            offset: 24613,
        },
        name_keysym {
            keysym: 0xfee0,
            offset: 22196,
        },
        name_keysym {
            keysym: 0xfee1,
            offset: 22209,
        },
        name_keysym {
            keysym: 0xfee2,
            offset: 22223,
        },
        name_keysym {
            keysym: 0xfee3,
            offset: 22041,
        },
        name_keysym {
            keysym: 0xfee4,
            offset: 22234,
        },
        name_keysym {
            keysym: 0xfee5,
            offset: 22249,
        },
        name_keysym {
            keysym: 0xfee6,
            offset: 22054,
        },
        name_keysym {
            keysym: 0xfee7,
            offset: 22071,
        },
        name_keysym {
            keysym: 0xfee8,
            offset: 21869,
        },
        name_keysym {
            keysym: 0xfee9,
            offset: 21789,
        },
        name_keysym {
            keysym: 0xfeea,
            offset: 21805,
        },
        name_keysym {
            keysym: 0xfeeb,
            offset: 21821,
        },
        name_keysym {
            keysym: 0xfeec,
            offset: 21837,
        },
        name_keysym {
            keysym: 0xfeed,
            offset: 21853,
        },
        name_keysym {
            keysym: 0xfeee,
            offset: 21979,
        },
        name_keysym {
            keysym: 0xfeef,
            offset: 21889,
        },
        name_keysym {
            keysym: 0xfef0,
            offset: 21907,
        },
        name_keysym {
            keysym: 0xfef1,
            offset: 21925,
        },
        name_keysym {
            keysym: 0xfef2,
            offset: 21943,
        },
        name_keysym {
            keysym: 0xfef3,
            offset: 21961,
        },
        name_keysym {
            keysym: 0xfef4,
            offset: 22159,
        },
        name_keysym {
            keysym: 0xfef5,
            offset: 22089,
        },
        name_keysym {
            keysym: 0xfef6,
            offset: 22103,
        },
        name_keysym {
            keysym: 0xfef7,
            offset: 22117,
        },
        name_keysym {
            keysym: 0xfef8,
            offset: 22131,
        },
        name_keysym {
            keysym: 0xfef9,
            offset: 22177,
        },
        name_keysym {
            keysym: 0xfefa,
            offset: 21770,
        },
        name_keysym {
            keysym: 0xfefb,
            offset: 22001,
        },
        name_keysym {
            keysym: 0xfefc,
            offset: 22021,
        },
        name_keysym {
            keysym: 0xfefd,
            offset: 22145,
        },
        name_keysym {
            keysym: 0xff08,
            offset: 3362,
        },
        name_keysym {
            keysym: 0xff09,
            offset: 24529,
        },
        name_keysym {
            keysym: 0xff0a,
            offset: 19619,
        },
        name_keysym {
            keysym: 0xff0b,
            offset: 8633,
        },
        name_keysym {
            keysym: 0xff0d,
            offset: 22564,
        },
        name_keysym {
            keysym: 0xff13,
            offset: 21679,
        },
        name_keysym {
            keysym: 0xff14,
            offset: 22923,
        },
        name_keysym {
            keysym: 0xff15,
            offset: 24510,
        },
        name_keysym {
            keysym: 0xff1b,
            offset: 12188,
        },
        name_keysym {
            keysym: 0xff20,
            offset: 19996,
        },
        name_keysym {
            keysym: 0xff21,
            offset: 18905,
        },
        name_keysym {
            keysym: 0xff22,
            offset: 19987,
        },
        name_keysym {
            keysym: 0xff23,
            offset: 16774,
        },
        name_keysym {
            keysym: 0xff24,
            offset: 22786,
        },
        name_keysym {
            keysym: 0xff25,
            offset: 16795,
        },
        name_keysym {
            keysym: 0xff26,
            offset: 18930,
        },
        name_keysym {
            keysym: 0xff27,
            offset: 16804,
        },
        name_keysym {
            keysym: 0xff28,
            offset: 32750,
        },
        name_keysym {
            keysym: 0xff29,
            offset: 16208,
        },
        name_keysym {
            keysym: 0xff2a,
            offset: 32758,
        },
        name_keysym {
            keysym: 0xff2b,
            offset: 25983,
        },
        name_keysym {
            keysym: 0xff2c,
            offset: 19883,
        },
        name_keysym {
            keysym: 0xff2d,
            offset: 18511,
        },
        name_keysym {
            keysym: 0xff2e,
            offset: 18722,
        },
        name_keysym {
            keysym: 0xff2f,
            offset: 11910,
        },
        name_keysym {
            keysym: 0xff30,
            offset: 11921,
        },
        name_keysym {
            keysym: 0xff31,
            offset: 14552,
        },
        name_keysym {
            keysym: 0xff32,
            offset: 15940,
        },
        name_keysym {
            keysym: 0xff33,
            offset: 14671,
        },
        name_keysym {
            keysym: 0xff34,
            offset: 14702,
        },
        name_keysym {
            keysym: 0xff35,
            offset: 15288,
        },
        name_keysym {
            keysym: 0xff36,
            offset: 15785,
        },
        name_keysym {
            keysym: 0xff37,
            offset: 8654,
        },
        name_keysym {
            keysym: 0xff38,
            offset: 15300,
        },
        name_keysym {
            keysym: 0xff39,
            offset: 14605,
        },
        name_keysym {
            keysym: 0xff3a,
            offset: 15580,
        },
        name_keysym {
            keysym: 0xff3b,
            offset: 15563,
        },
        name_keysym {
            keysym: 0xff3c,
            offset: 23236,
        },
        name_keysym {
            keysym: 0xff3d,
            offset: 20006,
        },
        name_keysym {
            keysym: 0xff3e,
            offset: 22298,
        },
        name_keysym {
            keysym: 0xff3f,
            offset: 15834,
        },
        name_keysym {
            keysym: 0xff50,
            offset: 16822,
        },
        name_keysym {
            keysym: 0xff51,
            offset: 19404,
        },
        name_keysym {
            keysym: 0xff52,
            offset: 26615,
        },
        name_keysym {
            keysym: 0xff53,
            offset: 22571,
        },
        name_keysym {
            keysym: 0xff54,
            offset: 11458,
        },
        name_keysym {
            keysym: 0xff55,
            offset: 22322,
        },
        name_keysym {
            keysym: 0xff56,
            offset: 20208,
        },
        name_keysym {
            keysym: 0xff57,
            offset: 12067,
        },
        name_keysym {
            keysym: 0xff58,
            offset: 3404,
        },
        name_keysym {
            keysym: 0xff60,
            offset: 22951,
        },
        name_keysym {
            keysym: 0xff61,
            offset: 22316,
        },
        name_keysym {
            keysym: 0xff62,
            offset: 12248,
        },
        name_keysym {
            keysym: 0xff63,
            offset: 17444,
        },
        name_keysym {
            keysym: 0xff65,
            offset: 26588,
        },
        name_keysym {
            keysym: 0xff66,
            offset: 22524,
        },
        name_keysym {
            keysym: 0xff67,
            offset: 19890,
        },
        name_keysym {
            keysym: 0xff68,
            offset: 12662,
        },
        name_keysym {
            keysym: 0xff69,
            offset: 8489,
        },
        name_keysym {
            keysym: 0xff6a,
            offset: 16762,
        },
        name_keysym {
            keysym: 0xff6b,
            offset: 8377,
        },
        name_keysym {
            keysym: 0xff7e,
            offset: 17760,
        },
        name_keysym {
            keysym: 0xff7f,
            offset: 20354,
        },
        name_keysym {
            keysym: 0xff80,
            offset: 19234,
        },
        name_keysym {
            keysym: 0xff89,
            offset: 19255,
        },
        name_keysym {
            keysym: 0xff8d,
            offset: 19091,
        },
        name_keysym {
            keysym: 0xff91,
            offset: 19109,
        },
        name_keysym {
            keysym: 0xff92,
            offset: 19115,
        },
        name_keysym {
            keysym: 0xff93,
            offset: 19121,
        },
        name_keysym {
            keysym: 0xff94,
            offset: 19127,
        },
        name_keysym {
            keysym: 0xff95,
            offset: 19133,
        },
        name_keysym {
            keysym: 0xff96,
            offset: 19151,
        },
        name_keysym {
            keysym: 0xff97,
            offset: 19262,
        },
        name_keysym {
            keysym: 0xff98,
            offset: 19212,
        },
        name_keysym {
            keysym: 0xff99,
            offset: 19076,
        },
        name_keysym {
            keysym: 0xff9a,
            offset: 19203,
        },
        name_keysym {
            keysym: 0xff9b,
            offset: 19171,
        },
        name_keysym {
            keysym: 0xff9c,
            offset: 19084,
        },
        name_keysym {
            keysym: 0xff9d,
            offset: 19036,
        },
        name_keysym {
            keysym: 0xff9e,
            offset: 19141,
        },
        name_keysym {
            keysym: 0xff9f,
            offset: 19056,
        },
        name_keysym {
            keysym: 0xffaa,
            offset: 19159,
        },
        name_keysym {
            keysym: 0xffab,
            offset: 19018,
        },
        name_keysym {
            keysym: 0xffac,
            offset: 19221,
        },
        name_keysym {
            keysym: 0xffad,
            offset: 19243,
        },
        name_keysym {
            keysym: 0xffae,
            offset: 19045,
        },
        name_keysym {
            keysym: 0xffaf,
            offset: 19066,
        },
        name_keysym {
            keysym: 0xffb0,
            offset: 18968,
        },
        name_keysym {
            keysym: 0xffb1,
            offset: 18973,
        },
        name_keysym {
            keysym: 0xffb2,
            offset: 18978,
        },
        name_keysym {
            keysym: 0xffb3,
            offset: 18983,
        },
        name_keysym {
            keysym: 0xffb4,
            offset: 18988,
        },
        name_keysym {
            keysym: 0xffb5,
            offset: 18993,
        },
        name_keysym {
            keysym: 0xffb6,
            offset: 18998,
        },
        name_keysym {
            keysym: 0xffb7,
            offset: 19003,
        },
        name_keysym {
            keysym: 0xffb8,
            offset: 19008,
        },
        name_keysym {
            keysym: 0xffb9,
            offset: 19013,
        },
        name_keysym {
            keysym: 0xffbd,
            offset: 19100,
        },
        name_keysym {
            keysym: 0xffbe,
            offset: 12290,
        },
        name_keysym {
            keysym: 0xffbf,
            offset: 12333,
        },
        name_keysym {
            keysym: 0xffc0,
            offset: 12376,
        },
        name_keysym {
            keysym: 0xffc1,
            offset: 12403,
        },
        name_keysym {
            keysym: 0xffc2,
            offset: 12406,
        },
        name_keysym {
            keysym: 0xffc3,
            offset: 12409,
        },
        name_keysym {
            keysym: 0xffc4,
            offset: 12412,
        },
        name_keysym {
            keysym: 0xffc5,
            offset: 12415,
        },
        name_keysym {
            keysym: 0xffc6,
            offset: 12418,
        },
        name_keysym {
            keysym: 0xffc7,
            offset: 12293,
        },
        name_keysym {
            keysym: 0xffc8,
            offset: 12297,
        },
        name_keysym {
            keysym: 0xffc9,
            offset: 12301,
        },
        name_keysym {
            keysym: 0xffca,
            offset: 12305,
        },
        name_keysym {
            keysym: 0xffcb,
            offset: 12309,
        },
        name_keysym {
            keysym: 0xffcc,
            offset: 12313,
        },
        name_keysym {
            keysym: 0xffcd,
            offset: 12317,
        },
        name_keysym {
            keysym: 0xffce,
            offset: 12321,
        },
        name_keysym {
            keysym: 0xffcf,
            offset: 12325,
        },
        name_keysym {
            keysym: 0xffd0,
            offset: 12329,
        },
        name_keysym {
            keysym: 0xffd1,
            offset: 12336,
        },
        name_keysym {
            keysym: 0xffd2,
            offset: 12340,
        },
        name_keysym {
            keysym: 0xffd3,
            offset: 12344,
        },
        name_keysym {
            keysym: 0xffd4,
            offset: 12348,
        },
        name_keysym {
            keysym: 0xffd5,
            offset: 12352,
        },
        name_keysym {
            keysym: 0xffd6,
            offset: 12356,
        },
        name_keysym {
            keysym: 0xffd7,
            offset: 12360,
        },
        name_keysym {
            keysym: 0xffd8,
            offset: 12364,
        },
        name_keysym {
            keysym: 0xffd9,
            offset: 12368,
        },
        name_keysym {
            keysym: 0xffda,
            offset: 12372,
        },
        name_keysym {
            keysym: 0xffdb,
            offset: 12379,
        },
        name_keysym {
            keysym: 0xffdc,
            offset: 12383,
        },
        name_keysym {
            keysym: 0xffdd,
            offset: 12387,
        },
        name_keysym {
            keysym: 0xffde,
            offset: 12391,
        },
        name_keysym {
            keysym: 0xffdf,
            offset: 12395,
        },
        name_keysym {
            keysym: 0xffe0,
            offset: 12399,
        },
        name_keysym {
            keysym: 0xffe1,
            offset: 23170,
        },
        name_keysym {
            keysym: 0xffe2,
            offset: 23189,
        },
        name_keysym {
            keysym: 0xffe3,
            offset: 8779,
        },
        name_keysym {
            keysym: 0xffe4,
            offset: 8789,
        },
        name_keysym {
            keysym: 0xffe5,
            offset: 8496,
        },
        name_keysym {
            keysym: 0xffe6,
            offset: 23178,
        },
        name_keysym {
            keysym: 0xffe7,
            offset: 19895,
        },
        name_keysym {
            keysym: 0xffe8,
            offset: 19902,
        },
        name_keysym {
            keysym: 0xffe9,
            offset: 880,
        },
        name_keysym {
            keysym: 0xffea,
            offset: 886,
        },
        name_keysym {
            keysym: 0xffeb,
            offset: 24494,
        },
        name_keysym {
            keysym: 0xffec,
            offset: 24502,
        },
        name_keysym {
            keysym: 0xffed,
            offset: 17206,
        },
        name_keysym {
            keysym: 0xffee,
            offset: 17214,
        },
        name_keysym {
            keysym: 0xfff1,
            offset: 3642,
        },
        name_keysym {
            keysym: 0xfff2,
            offset: 3671,
        },
        name_keysym {
            keysym: 0xfff3,
            offset: 3685,
        },
        name_keysym {
            keysym: 0xfff4,
            offset: 3699,
        },
        name_keysym {
            keysym: 0xfff5,
            offset: 3713,
        },
        name_keysym {
            keysym: 0xfff6,
            offset: 3727,
        },
        name_keysym {
            keysym: 0xfff7,
            offset: 3741,
        },
        name_keysym {
            keysym: 0xfff8,
            offset: 3755,
        },
        name_keysym {
            keysym: 0xfff9,
            offset: 3769,
        },
        name_keysym {
            keysym: 0xfffa,
            offset: 3656,
        },
        name_keysym {
            keysym: 0xffff,
            offset: 11291,
        },
        name_keysym {
            keysym: 0xffffff,
            offset: 26762,
        },
        name_keysym {
            keysym: 0x100012c,
            offset: 17277,
        },
        name_keysym {
            keysym: 0x100012d,
            offset: 17284,
        },
        name_keysym {
            keysym: 0x1000174,
            offset: 26794,
        },
        name_keysym {
            keysym: 0x1000175,
            offset: 26806,
        },
        name_keysym {
            keysym: 0x1000176,
            offset: 32599,
        },
        name_keysym {
            keysym: 0x1000177,
            offset: 32611,
        },
        name_keysym {
            keysym: 0x100018f,
            offset: 22873,
        },
        name_keysym {
            keysym: 0x100019f,
            offset: 20403,
        },
        name_keysym {
            keysym: 0x10001a0,
            offset: 20738,
        },
        name_keysym {
            keysym: 0x10001a1,
            offset: 20744,
        },
        name_keysym {
            keysym: 0x10001af,
            offset: 26228,
        },
        name_keysym {
            keysym: 0x10001b0,
            offset: 26234,
        },
        name_keysym {
            keysym: 0x10001b5,
            offset: 32801,
        },
        name_keysym {
            keysym: 0x10001b6,
            offset: 32809,
        },
        name_keysym {
            keysym: 0x10001b7,
            offset: 12278,
        },
        name_keysym {
            keysym: 0x10001d1,
            offset: 20439,
        },
        name_keysym {
            keysym: 0x10001d2,
            offset: 20446,
        },
        name_keysym {
            keysym: 0x10001e6,
            offset: 12834,
        },
        name_keysym {
            keysym: 0x10001e7,
            offset: 12841,
        },
        name_keysym {
            keysym: 0x1000259,
            offset: 22879,
        },
        name_keysym {
            keysym: 0x1000275,
            offset: 20411,
        },
        name_keysym {
            keysym: 0x1000292,
            offset: 12282,
        },
        name_keysym {
            keysym: 0x1000300,
            offset: 8715,
        },
        name_keysym {
            keysym: 0x1000301,
            offset: 8680,
        },
        name_keysym {
            keysym: 0x1000303,
            offset: 8746,
        },
        name_keysym {
            keysym: 0x1000309,
            offset: 8731,
        },
        name_keysym {
            keysym: 0x1000323,
            offset: 8696,
        },
        name_keysym {
            keysym: 0x1000492,
            offset: 9318,
        },
        name_keysym {
            keysym: 0x1000493,
            offset: 9335,
        },
        name_keysym {
            keysym: 0x1000496,
            offset: 10356,
        },
        name_keysym {
            keysym: 0x1000497,
            offset: 10379,
        },
        name_keysym {
            keysym: 0x100049a,
            offset: 9610,
        },
        name_keysym {
            keysym: 0x100049b,
            offset: 9632,
        },
        name_keysym {
            keysym: 0x100049c,
            offset: 9654,
        },
        name_keysym {
            keysym: 0x100049d,
            offset: 9677,
        },
        name_keysym {
            keysym: 0x10004a2,
            offset: 9200,
        },
        name_keysym {
            keysym: 0x10004a3,
            offset: 9222,
        },
        name_keysym {
            keysym: 0x10004ae,
            offset: 10118,
        },
        name_keysym {
            keysym: 0x10004af,
            offset: 10138,
        },
        name_keysym {
            keysym: 0x10004b0,
            offset: 10158,
        },
        name_keysym {
            keysym: 0x10004b1,
            offset: 10182,
        },
        name_keysym {
            keysym: 0x10004b2,
            offset: 9376,
        },
        name_keysym {
            keysym: 0x10004b3,
            offset: 9398,
        },
        name_keysym {
            keysym: 0x10004b6,
            offset: 8936,
        },
        name_keysym {
            keysym: 0x10004b7,
            offset: 8959,
        },
        name_keysym {
            keysym: 0x10004b8,
            offset: 8982,
        },
        name_keysym {
            keysym: 0x10004b9,
            offset: 9006,
        },
        name_keysym {
            keysym: 0x10004ba,
            offset: 9914,
        },
        name_keysym {
            keysym: 0x10004bb,
            offset: 9928,
        },
        name_keysym {
            keysym: 0x10004d8,
            offset: 9828,
        },
        name_keysym {
            keysym: 0x10004d9,
            offset: 9843,
        },
        name_keysym {
            keysym: 0x10004e2,
            offset: 9478,
        },
        name_keysym {
            keysym: 0x10004e3,
            offset: 9496,
        },
        name_keysym {
            keysym: 0x10004e8,
            offset: 9774,
        },
        name_keysym {
            keysym: 0x10004e9,
            offset: 9789,
        },
        name_keysym {
            keysym: 0x10004ee,
            offset: 10082,
        },
        name_keysym {
            keysym: 0x10004ef,
            offset: 10100,
        },
        name_keysym {
            keysym: 0x1000531,
            offset: 2124,
        },
        name_keysym {
            keysym: 0x1000532,
            offset: 2150,
        },
        name_keysym {
            keysym: 0x1000533,
            offset: 2374,
        },
        name_keysym {
            keysym: 0x1000534,
            offset: 2215,
        },
        name_keysym {
            keysym: 0x1000535,
            offset: 3164,
        },
        name_keysym {
            keysym: 0x1000536,
            offset: 3210,
        },
        name_keysym {
            keysym: 0x1000537,
            offset: 2265,
        },
        name_keysym {
            keysym: 0x1000538,
            offset: 2100,
        },
        name_keysym {
            keysym: 0x1000539,
            offset: 2964,
        },
        name_keysym {
            keysym: 0x100053a,
            offset: 3234,
        },
        name_keysym {
            keysym: 0x100053b,
            offset: 2464,
        },
        name_keysym {
            keysym: 0x100053c,
            offset: 2611,
        },
        name_keysym {
            keysym: 0x100053d,
            offset: 2564,
        },
        name_keysym {
            keysym: 0x100053e,
            offset: 2988,
        },
        name_keysym {
            keysym: 0x100053f,
            offset: 2538,
        },
        name_keysym {
            keysym: 0x1000540,
            offset: 2424,
        },
        name_keysym {
            keysym: 0x1000541,
            offset: 2239,
        },
        name_keysym {
            keysym: 0x1000542,
            offset: 2346,
        },
        name_keysym {
            keysym: 0x1000543,
            offset: 2936,
        },
        name_keysym {
            keysym: 0x1000544,
            offset: 2639,
        },
        name_keysym {
            keysym: 0x1000545,
            offset: 2400,
        },
        name_keysym {
            keysym: 0x1000546,
            offset: 2665,
        },
        name_keysym {
            keysym: 0x1000547,
            offset: 2894,
        },
        name_keysym {
            keysym: 0x1000548,
            offset: 3112,
        },
        name_keysym {
            keysym: 0x1000549,
            offset: 2189,
        },
        name_keysym {
            keysym: 0x100054a,
            offset: 2727,
        },
        name_keysym {
            keysym: 0x100054b,
            offset: 2490,
        },
        name_keysym {
            keysym: 0x100054c,
            offset: 2797,
        },
        name_keysym {
            keysym: 0x100054d,
            offset: 2845,
        },
        name_keysym {
            keysym: 0x100054e,
            offset: 3086,
        },
        name_keysym {
            keysym: 0x100054f,
            offset: 3040,
        },
        name_keysym {
            keysym: 0x1000550,
            offset: 2821,
        },
        name_keysym {
            keysym: 0x1000551,
            offset: 3014,
        },
        name_keysym {
            keysym: 0x1000552,
            offset: 3136,
        },
        name_keysym {
            keysym: 0x1000553,
            offset: 2751,
        },
        name_keysym {
            keysym: 0x1000554,
            offset: 2514,
        },
        name_keysym {
            keysym: 0x1000555,
            offset: 2689,
        },
        name_keysym {
            keysym: 0x1000556,
            offset: 2303,
        },
        name_keysym {
            keysym: 0x100055a,
            offset: 2080,
        },
        name_keysym {
            keysym: 0x100055b,
            offset: 2048,
        },
        name_keysym {
            keysym: 0x100055c,
            offset: 2287,
        },
        name_keysym {
            keysym: 0x100055d,
            offset: 2869,
        },
        name_keysym {
            keysym: 0x100055e,
            offset: 2779,
        },
        name_keysym {
            keysym: 0x1000561,
            offset: 2137,
        },
        name_keysym {
            keysym: 0x1000562,
            offset: 2163,
        },
        name_keysym {
            keysym: 0x1000563,
            offset: 2387,
        },
        name_keysym {
            keysym: 0x1000564,
            offset: 2227,
        },
        name_keysym {
            keysym: 0x1000565,
            offset: 3178,
        },
        name_keysym {
            keysym: 0x1000566,
            offset: 3222,
        },
        name_keysym {
            keysym: 0x1000567,
            offset: 2276,
        },
        name_keysym {
            keysym: 0x1000568,
            offset: 2112,
        },
        name_keysym {
            keysym: 0x1000569,
            offset: 2976,
        },
        name_keysym {
            keysym: 0x100056a,
            offset: 3247,
        },
        name_keysym {
            keysym: 0x100056b,
            offset: 2477,
        },
        name_keysym {
            keysym: 0x100056c,
            offset: 2625,
        },
        name_keysym {
            keysym: 0x100056d,
            offset: 2577,
        },
        name_keysym {
            keysym: 0x100056e,
            offset: 3001,
        },
        name_keysym {
            keysym: 0x100056f,
            offset: 2551,
        },
        name_keysym {
            keysym: 0x1000570,
            offset: 2436,
        },
        name_keysym {
            keysym: 0x1000571,
            offset: 2252,
        },
        name_keysym {
            keysym: 0x1000572,
            offset: 2360,
        },
        name_keysym {
            keysym: 0x1000573,
            offset: 2950,
        },
        name_keysym {
            keysym: 0x1000574,
            offset: 2652,
        },
        name_keysym {
            keysym: 0x1000575,
            offset: 2412,
        },
        name_keysym {
            keysym: 0x1000576,
            offset: 2677,
        },
        name_keysym {
            keysym: 0x1000577,
            offset: 2907,
        },
        name_keysym {
            keysym: 0x1000578,
            offset: 3124,
        },
        name_keysym {
            keysym: 0x1000579,
            offset: 2202,
        },
        name_keysym {
            keysym: 0x100057a,
            offset: 2739,
        },
        name_keysym {
            keysym: 0x100057b,
            offset: 2502,
        },
        name_keysym {
            keysym: 0x100057c,
            offset: 2809,
        },
        name_keysym {
            keysym: 0x100057d,
            offset: 2857,
        },
        name_keysym {
            keysym: 0x100057e,
            offset: 3099,
        },
        name_keysym {
            keysym: 0x100057f,
            offset: 3054,
        },
        name_keysym {
            keysym: 0x1000580,
            offset: 2833,
        },
        name_keysym {
            keysym: 0x1000581,
            offset: 3027,
        },
        name_keysym {
            keysym: 0x1000582,
            offset: 3150,
        },
        name_keysym {
            keysym: 0x1000583,
            offset: 2765,
        },
        name_keysym {
            keysym: 0x1000584,
            offset: 2526,
        },
        name_keysym {
            keysym: 0x1000585,
            offset: 2700,
        },
        name_keysym {
            keysym: 0x1000586,
            offset: 2315,
        },
        name_keysym {
            keysym: 0x1000587,
            offset: 2590,
        },
        name_keysym {
            keysym: 0x1000589,
            offset: 2327,
        },
        name_keysym {
            keysym: 0x100058a,
            offset: 2448,
        },
        name_keysym {
            keysym: 0x1000653,
            offset: 1580,
        },
        name_keysym {
            keysym: 0x1000654,
            offset: 1316,
        },
        name_keysym {
            keysym: 0x1000655,
            offset: 1335,
        },
        name_keysym {
            keysym: 0x1000660,
            offset: 966,
        },
        name_keysym {
            keysym: 0x1000661,
            offset: 975,
        },
        name_keysym {
            keysym: 0x1000662,
            offset: 984,
        },
        name_keysym {
            keysym: 0x1000663,
            offset: 993,
        },
        name_keysym {
            keysym: 0x1000664,
            offset: 1002,
        },
        name_keysym {
            keysym: 0x1000665,
            offset: 1011,
        },
        name_keysym {
            keysym: 0x1000666,
            offset: 1020,
        },
        name_keysym {
            keysym: 0x1000667,
            offset: 1029,
        },
        name_keysym {
            keysym: 0x1000668,
            offset: 1038,
        },
        name_keysym {
            keysym: 0x1000669,
            offset: 1047,
        },
        name_keysym {
            keysym: 0x100066a,
            offset: 1672,
        },
        name_keysym {
            keysym: 0x1000670,
            offset: 1821,
        },
        name_keysym {
            keysym: 0x1000679,
            offset: 1951,
        },
        name_keysym {
            keysym: 0x100067e,
            offset: 1661,
        },
        name_keysym {
            keysym: 0x1000686,
            offset: 1885,
        },
        name_keysym {
            keysym: 0x1000688,
            offset: 1173,
        },
        name_keysym {
            keysym: 0x1000691,
            offset: 1729,
        },
        name_keysym {
            keysym: 0x1000698,
            offset: 1493,
        },
        name_keysym {
            keysym: 0x10006a4,
            offset: 1963,
        },
        name_keysym {
            keysym: 0x10006a9,
            offset: 1544,
        },
        name_keysym {
            keysym: 0x10006af,
            offset: 1258,
        },
        name_keysym {
            keysym: 0x10006ba,
            offset: 1642,
        },
        name_keysym {
            keysym: 0x10006be,
            offset: 1442,
        },
        name_keysym {
            keysym: 0x10006c1,
            offset: 1465,
        },
        name_keysym {
            keysym: 0x10006cc,
            offset: 12521,
        },
        name_keysym {
            keysym: 0x10006d2,
            offset: 1996,
        },
        name_keysym {
            keysym: 0x10006d4,
            offset: 1242,
        },
        name_keysym {
            keysym: 0x10006f0,
            offset: 12441,
        },
        name_keysym {
            keysym: 0x10006f1,
            offset: 12449,
        },
        name_keysym {
            keysym: 0x10006f2,
            offset: 12457,
        },
        name_keysym {
            keysym: 0x10006f3,
            offset: 12465,
        },
        name_keysym {
            keysym: 0x10006f4,
            offset: 12473,
        },
        name_keysym {
            keysym: 0x10006f5,
            offset: 12481,
        },
        name_keysym {
            keysym: 0x10006f6,
            offset: 12489,
        },
        name_keysym {
            keysym: 0x10006f7,
            offset: 12497,
        },
        name_keysym {
            keysym: 0x10006f8,
            offset: 12505,
        },
        name_keysym {
            keysym: 0x10006f9,
            offset: 12513,
        },
        name_keysym {
            keysym: 0x1000d82,
            offset: 23708,
        },
        name_keysym {
            keysym: 0x1000d83,
            offset: 23502,
        },
        name_keysym {
            keysym: 0x1000d85,
            offset: 23271,
        },
        name_keysym {
            keysym: 0x1000d86,
            offset: 23278,
        },
        name_keysym {
            keysym: 0x1000d87,
            offset: 23295,
        },
        name_keysym {
            keysym: 0x1000d88,
            offset: 23312,
        },
        name_keysym {
            keysym: 0x1000d89,
            offset: 23518,
        },
        name_keysym {
            keysym: 0x1000d8a,
            offset: 23533,
        },
        name_keysym {
            keysym: 0x1000d8b,
            offset: 23919,
        },
        name_keysym {
            keysym: 0x1000d8c,
            offset: 23934,
        },
        name_keysym {
            keysym: 0x1000d8d,
            offset: 23818,
        },
        name_keysym {
            keysym: 0x1000d8e,
            offset: 23826,
        },
        name_keysym {
            keysym: 0x1000d8f,
            offset: 23627,
        },
        name_keysym {
            keysym: 0x1000d90,
            offset: 23644,
        },
        name_keysym {
            keysym: 0x1000d91,
            offset: 23445,
        },
        name_keysym {
            keysym: 0x1000d92,
            offset: 23460,
        },
        name_keysym {
            keysym: 0x1000d93,
            offset: 23331,
        },
        name_keysym {
            keysym: 0x1000d94,
            offset: 23761,
        },
        name_keysym {
            keysym: 0x1000d95,
            offset: 23776,
        },
        name_keysym {
            keysym: 0x1000d96,
            offset: 23356,
        },
        name_keysym {
            keysym: 0x1000d9a,
            offset: 23577,
        },
        name_keysym {
            keysym: 0x1000d9b,
            offset: 23585,
        },
        name_keysym {
            keysym: 0x1000d9c,
            offset: 23485,
        },
        name_keysym {
            keysym: 0x1000d9d,
            offset: 23493,
        },
        name_keysym {
            keysym: 0x1000d9e,
            offset: 23716,
        },
        name_keysym {
            keysym: 0x1000d9f,
            offset: 23725,
        },
        name_keysym {
            keysym: 0x1000da0,
            offset: 23390,
        },
        name_keysym {
            keysym: 0x1000da1,
            offset: 23398,
        },
        name_keysym {
            keysym: 0x1000da2,
            offset: 23550,
        },
        name_keysym {
            keysym: 0x1000da3,
            offset: 23558,
        },
        name_keysym {
            keysym: 0x1000da4,
            offset: 23752,
        },
        name_keysym {
            keysym: 0x1000da5,
            offset: 23567,
        },
        name_keysym {
            keysym: 0x1000da6,
            offset: 23734,
        },
        name_keysym {
            keysym: 0x1000da7,
            offset: 23900,
        },
        name_keysym {
            keysym: 0x1000da8,
            offset: 23909,
        },
        name_keysym {
            keysym: 0x1000da9,
            offset: 23407,
        },
        name_keysym {
            keysym: 0x1000daa,
            offset: 23416,
        },
        name_keysym {
            keysym: 0x1000dab,
            offset: 23743,
        },
        name_keysym {
            keysym: 0x1000dac,
            offset: 23688,
        },
        name_keysym {
            keysym: 0x1000dad,
            offset: 23881,
        },
        name_keysym {
            keysym: 0x1000dae,
            offset: 23890,
        },
        name_keysym {
            keysym: 0x1000daf,
            offset: 23426,
        },
        name_keysym {
            keysym: 0x1000db0,
            offset: 23435,
        },
        name_keysym {
            keysym: 0x1000db1,
            offset: 23680,
        },
        name_keysym {
            keysym: 0x1000db3,
            offset: 23698,
        },
        name_keysym {
            keysym: 0x1000db4,
            offset: 23793,
        },
        name_keysym {
            keysym: 0x1000db5,
            offset: 23801,
        },
        name_keysym {
            keysym: 0x1000db6,
            offset: 23373,
        },
        name_keysym {
            keysym: 0x1000db7,
            offset: 23381,
        },
        name_keysym {
            keysym: 0x1000db8,
            offset: 23663,
        },
        name_keysym {
            keysym: 0x1000db9,
            offset: 23671,
        },
        name_keysym {
            keysym: 0x1000dba,
            offset: 23959,
        },
        name_keysym {
            keysym: 0x1000dbb,
            offset: 23810,
        },
        name_keysym {
            keysym: 0x1000dbd,
            offset: 23610,
        },
        name_keysym {
            keysym: 0x1000dc0,
            offset: 23951,
        },
        name_keysym {
            keysym: 0x1000dc1,
            offset: 23862,
        },
        name_keysym {
            keysym: 0x1000dc2,
            offset: 23871,
        },
        name_keysym {
            keysym: 0x1000dc3,
            offset: 23854,
        },
        name_keysym {
            keysym: 0x1000dc4,
            offset: 23510,
        },
        name_keysym {
            keysym: 0x1000dc5,
            offset: 23618,
        },
        name_keysym {
            keysym: 0x1000dc6,
            offset: 23477,
        },
        name_keysym {
            keysym: 0x1000dca,
            offset: 23348,
        },
        name_keysym {
            keysym: 0x1000dcf,
            offset: 23286,
        },
        name_keysym {
            keysym: 0x1000dd0,
            offset: 23303,
        },
        name_keysym {
            keysym: 0x1000dd1,
            offset: 23321,
        },
        name_keysym {
            keysym: 0x1000dd2,
            offset: 23525,
        },
        name_keysym {
            keysym: 0x1000dd3,
            offset: 23541,
        },
        name_keysym {
            keysym: 0x1000dd4,
            offset: 23926,
        },
        name_keysym {
            keysym: 0x1000dd6,
            offset: 23942,
        },
        name_keysym {
            keysym: 0x1000dd8,
            offset: 23835,
        },
        name_keysym {
            keysym: 0x1000dd9,
            offset: 23452,
        },
        name_keysym {
            keysym: 0x1000dda,
            offset: 23468,
        },
        name_keysym {
            keysym: 0x1000ddb,
            offset: 23339,
        },
        name_keysym {
            keysym: 0x1000ddc,
            offset: 23768,
        },
        name_keysym {
            keysym: 0x1000ddd,
            offset: 23784,
        },
        name_keysym {
            keysym: 0x1000dde,
            offset: 23364,
        },
        name_keysym {
            keysym: 0x1000ddf,
            offset: 23635,
        },
        name_keysym {
            keysym: 0x1000df2,
            offset: 23844,
        },
        name_keysym {
            keysym: 0x1000df3,
            offset: 23653,
        },
        name_keysym {
            keysym: 0x1000df4,
            offset: 23594,
        },
        name_keysym {
            keysym: 0x10010d0,
            offset: 12890,
        },
        name_keysym {
            keysym: 0x10010d1,
            offset: 12902,
        },
        name_keysym {
            keysym: 0x10010d2,
            offset: 13006,
        },
        name_keysym {
            keysym: 0x10010d3,
            offset: 12969,
        },
        name_keysym {
            keysym: 0x10010d4,
            offset: 12982,
        },
        name_keysym {
            keysym: 0x10010d5,
            offset: 13332,
        },
        name_keysym {
            keysym: 0x10010d6,
            offset: 13370,
        },
        name_keysym {
            keysym: 0x10010d7,
            offset: 13294,
        },
        name_keysym {
            keysym: 0x10010d8,
            offset: 13097,
        },
        name_keysym {
            keysym: 0x10010d9,
            offset: 13136,
        },
        name_keysym {
            keysym: 0x10010da,
            offset: 13163,
        },
        name_keysym {
            keysym: 0x10010db,
            offset: 13176,
        },
        name_keysym {
            keysym: 0x10010dc,
            offset: 13189,
        },
        name_keysym {
            keysym: 0x10010dd,
            offset: 13202,
        },
        name_keysym {
            keysym: 0x10010de,
            offset: 13214,
        },
        name_keysym {
            keysym: 0x10010df,
            offset: 13383,
        },
        name_keysym {
            keysym: 0x10010e0,
            offset: 13254,
        },
        name_keysym {
            keysym: 0x10010e1,
            offset: 13267,
        },
        name_keysym {
            keysym: 0x10010e2,
            offset: 13307,
        },
        name_keysym {
            keysym: 0x10010e3,
            offset: 13320,
        },
        name_keysym {
            keysym: 0x10010e4,
            offset: 13227,
        },
        name_keysym {
            keysym: 0x10010e5,
            offset: 13149,
        },
        name_keysym {
            keysym: 0x10010e6,
            offset: 13019,
        },
        name_keysym {
            keysym: 0x10010e7,
            offset: 13241,
        },
        name_keysym {
            keysym: 0x10010e8,
            offset: 13280,
        },
        name_keysym {
            keysym: 0x10010e9,
            offset: 12942,
        },
        name_keysym {
            keysym: 0x10010ea,
            offset: 12915,
        },
        name_keysym {
            keysym: 0x10010eb,
            offset: 13123,
        },
        name_keysym {
            keysym: 0x10010ec,
            offset: 12956,
        },
        name_keysym {
            keysym: 0x10010ed,
            offset: 12928,
        },
        name_keysym {
            keysym: 0x10010ee,
            offset: 13357,
        },
        name_keysym {
            keysym: 0x10010ef,
            offset: 13109,
        },
        name_keysym {
            keysym: 0x10010f0,
            offset: 13033,
        },
        name_keysym {
            keysym: 0x10010f1,
            offset: 13059,
        },
        name_keysym {
            keysym: 0x10010f2,
            offset: 13071,
        },
        name_keysym {
            keysym: 0x10010f3,
            offset: 13345,
        },
        name_keysym {
            keysym: 0x10010f4,
            offset: 13046,
        },
        name_keysym {
            keysym: 0x10010f5,
            offset: 13084,
        },
        name_keysym {
            keysym: 0x10010f6,
            offset: 12994,
        },
        name_keysym {
            keysym: 0x1001e02,
            offset: 3332,
        },
        name_keysym {
            keysym: 0x1001e03,
            offset: 3342,
        },
        name_keysym {
            keysym: 0x1001e0a,
            offset: 10406,
        },
        name_keysym {
            keysym: 0x1001e0b,
            offset: 10416,
        },
        name_keysym {
            keysym: 0x1001e1e,
            offset: 12421,
        },
        name_keysym {
            keysym: 0x1001e1f,
            offset: 12431,
        },
        name_keysym {
            keysym: 0x1001e36,
            offset: 19352,
        },
        name_keysym {
            keysym: 0x1001e37,
            offset: 19362,
        },
        name_keysym {
            keysym: 0x1001e40,
            offset: 19722,
        },
        name_keysym {
            keysym: 0x1001e41,
            offset: 19732,
        },
        name_keysym {
            keysym: 0x1001e56,
            offset: 21575,
        },
        name_keysym {
            keysym: 0x1001e57,
            offset: 21585,
        },
        name_keysym {
            keysym: 0x1001e60,
            offset: 22807,
        },
        name_keysym {
            keysym: 0x1001e61,
            offset: 22817,
        },
        name_keysym {
            keysym: 0x1001e6a,
            offset: 24533,
        },
        name_keysym {
            keysym: 0x1001e6b,
            offset: 24543,
        },
        name_keysym {
            keysym: 0x1001e80,
            offset: 26840,
        },
        name_keysym {
            keysym: 0x1001e81,
            offset: 26847,
        },
        name_keysym {
            keysym: 0x1001e82,
            offset: 26780,
        },
        name_keysym {
            keysym: 0x1001e83,
            offset: 26787,
        },
        name_keysym {
            keysym: 0x1001e84,
            offset: 26818,
        },
        name_keysym {
            keysym: 0x1001e85,
            offset: 26829,
        },
        name_keysym {
            keysym: 0x1001e8a,
            offset: 26866,
        },
        name_keysym {
            keysym: 0x1001e8b,
            offset: 26876,
        },
        name_keysym {
            keysym: 0x1001e9e,
            offset: 24044,
        },
        name_keysym {
            keysym: 0x1001ea0,
            offset: 416,
        },
        name_keysym {
            keysym: 0x1001ea1,
            offset: 426,
        },
        name_keysym {
            keysym: 0x1001ea2,
            offset: 868,
        },
        name_keysym {
            keysym: 0x1001ea3,
            offset: 874,
        },
        name_keysym {
            keysym: 0x1001ea4,
            offset: 646,
        },
        name_keysym {
            keysym: 0x1001ea5,
            offset: 663,
        },
        name_keysym {
            keysym: 0x1001ea6,
            offset: 720,
        },
        name_keysym {
            keysym: 0x1001ea7,
            offset: 737,
        },
        name_keysym {
            keysym: 0x1001ea8,
            offset: 754,
        },
        name_keysym {
            keysym: 0x1001ea9,
            offset: 770,
        },
        name_keysym {
            keysym: 0x1001eaa,
            offset: 786,
        },
        name_keysym {
            keysym: 0x1001eab,
            offset: 803,
        },
        name_keysym {
            keysym: 0x1001eac,
            offset: 680,
        },
        name_keysym {
            keysym: 0x1001ead,
            offset: 700,
        },
        name_keysym {
            keysym: 0x1001eae,
            offset: 459,
        },
        name_keysym {
            keysym: 0x1001eaf,
            offset: 471,
        },
        name_keysym {
            keysym: 0x1001eb0,
            offset: 513,
        },
        name_keysym {
            keysym: 0x1001eb1,
            offset: 525,
        },
        name_keysym {
            keysym: 0x1001eb2,
            offset: 537,
        },
        name_keysym {
            keysym: 0x1001eb3,
            offset: 548,
        },
        name_keysym {
            keysym: 0x1001eb4,
            offset: 559,
        },
        name_keysym {
            keysym: 0x1001eb5,
            offset: 571,
        },
        name_keysym {
            keysym: 0x1001eb6,
            offset: 483,
        },
        name_keysym {
            keysym: 0x1001eb7,
            offset: 498,
        },
        name_keysym {
            keysym: 0x1001eb8,
            offset: 11593,
        },
        name_keysym {
            keysym: 0x1001eb9,
            offset: 11603,
        },
        name_keysym {
            keysym: 0x1001eba,
            offset: 11869,
        },
        name_keysym {
            keysym: 0x1001ebb,
            offset: 11875,
        },
        name_keysym {
            keysym: 0x1001ebc,
            offset: 12207,
        },
        name_keysym {
            keysym: 0x1001ebd,
            offset: 12214,
        },
        name_keysym {
            keysym: 0x1001ebe,
            offset: 11651,
        },
        name_keysym {
            keysym: 0x1001ebf,
            offset: 11668,
        },
        name_keysym {
            keysym: 0x1001ec0,
            offset: 11725,
        },
        name_keysym {
            keysym: 0x1001ec1,
            offset: 11742,
        },
        name_keysym {
            keysym: 0x1001ec2,
            offset: 11759,
        },
        name_keysym {
            keysym: 0x1001ec3,
            offset: 11775,
        },
        name_keysym {
            keysym: 0x1001ec4,
            offset: 11791,
        },
        name_keysym {
            keysym: 0x1001ec5,
            offset: 11808,
        },
        name_keysym {
            keysym: 0x1001ec6,
            offset: 11685,
        },
        name_keysym {
            keysym: 0x1001ec7,
            offset: 11705,
        },
        name_keysym {
            keysym: 0x1001ec8,
            offset: 17379,
        },
        name_keysym {
            keysym: 0x1001ec9,
            offset: 17385,
        },
        name_keysym {
            keysym: 0x1001eca,
            offset: 17257,
        },
        name_keysym {
            keysym: 0x1001ecb,
            offset: 17267,
        },
        name_keysym {
            keysym: 0x1001ecc,
            offset: 20419,
        },
        name_keysym {
            keysym: 0x1001ecd,
            offset: 20429,
        },
        name_keysym {
            keysym: 0x1001ece,
            offset: 20726,
        },
        name_keysym {
            keysym: 0x1001ecf,
            offset: 20732,
        },
        name_keysym {
            keysym: 0x1001ed0,
            offset: 20477,
        },
        name_keysym {
            keysym: 0x1001ed1,
            offset: 20494,
        },
        name_keysym {
            keysym: 0x1001ed2,
            offset: 20551,
        },
        name_keysym {
            keysym: 0x1001ed3,
            offset: 20568,
        },
        name_keysym {
            keysym: 0x1001ed4,
            offset: 20585,
        },
        name_keysym {
            keysym: 0x1001ed5,
            offset: 20601,
        },
        name_keysym {
            keysym: 0x1001ed6,
            offset: 20617,
        },
        name_keysym {
            keysym: 0x1001ed7,
            offset: 20634,
        },
        name_keysym {
            keysym: 0x1001ed8,
            offset: 20511,
        },
        name_keysym {
            keysym: 0x1001ed9,
            offset: 20531,
        },
        name_keysym {
            keysym: 0x1001eda,
            offset: 20750,
        },
        name_keysym {
            keysym: 0x1001edb,
            offset: 20761,
        },
        name_keysym {
            keysym: 0x1001edc,
            offset: 20800,
        },
        name_keysym {
            keysym: 0x1001edd,
            offset: 20811,
        },
        name_keysym {
            keysym: 0x1001ede,
            offset: 20822,
        },
        name_keysym {
            keysym: 0x1001edf,
            offset: 20832,
        },
        name_keysym {
            keysym: 0x1001ee0,
            offset: 20842,
        },
        name_keysym {
            keysym: 0x1001ee1,
            offset: 20853,
        },
        name_keysym {
            keysym: 0x1001ee2,
            offset: 20772,
        },
        name_keysym {
            keysym: 0x1001ee3,
            offset: 20786,
        },
        name_keysym {
            keysym: 0x1001ee4,
            offset: 26096,
        },
        name_keysym {
            keysym: 0x1001ee5,
            offset: 26106,
        },
        name_keysym {
            keysym: 0x1001ee6,
            offset: 26216,
        },
        name_keysym {
            keysym: 0x1001ee7,
            offset: 26222,
        },
        name_keysym {
            keysym: 0x1001ee8,
            offset: 26240,
        },
        name_keysym {
            keysym: 0x1001ee9,
            offset: 26251,
        },
        name_keysym {
            keysym: 0x1001eea,
            offset: 26290,
        },
        name_keysym {
            keysym: 0x1001eeb,
            offset: 26301,
        },
        name_keysym {
            keysym: 0x1001eec,
            offset: 26312,
        },
        name_keysym {
            keysym: 0x1001eed,
            offset: 26322,
        },
        name_keysym {
            keysym: 0x1001eee,
            offset: 26332,
        },
        name_keysym {
            keysym: 0x1001eef,
            offset: 26343,
        },
        name_keysym {
            keysym: 0x1001ef0,
            offset: 26262,
        },
        name_keysym {
            keysym: 0x1001ef1,
            offset: 26276,
        },
        name_keysym {
            keysym: 0x1001ef2,
            offset: 32649,
        },
        name_keysym {
            keysym: 0x1001ef3,
            offset: 32656,
        },
        name_keysym {
            keysym: 0x1001ef4,
            offset: 32579,
        },
        name_keysym {
            keysym: 0x1001ef5,
            offset: 32589,
        },
        name_keysym {
            keysym: 0x1001ef6,
            offset: 32663,
        },
        name_keysym {
            keysym: 0x1001ef7,
            offset: 32669,
        },
        name_keysym {
            keysym: 0x1001ef8,
            offset: 32675,
        },
        name_keysym {
            keysym: 0x1001ef9,
            offset: 32682,
        },
        name_keysym {
            keysym: 0x1002039,
            offset: 19537,
        },
        name_keysym {
            keysym: 0x100203a,
            offset: 22722,
        },
        name_keysym {
            keysym: 0x1002070,
            offset: 32788,
        },
        name_keysym {
            keysym: 0x1002074,
            offset: 12763,
        },
        name_keysym {
            keysym: 0x1002075,
            offset: 12725,
        },
        name_keysym {
            keysym: 0x1002076,
            offset: 23980,
        },
        name_keysym {
            keysym: 0x1002077,
            offset: 23156,
        },
        name_keysym {
            keysym: 0x1002078,
            offset: 11896,
        },
        name_keysym {
            keysym: 0x1002079,
            offset: 20247,
        },
        name_keysym {
            keysym: 0x1002080,
            offset: 32774,
        },
        name_keysym {
            keysym: 0x1002081,
            offset: 20927,
        },
        name_keysym {
            keysym: 0x1002082,
            offset: 26043,
        },
        name_keysym {
            keysym: 0x1002083,
            offset: 25787,
        },
        name_keysym {
            keysym: 0x1002084,
            offset: 12749,
        },
        name_keysym {
            keysym: 0x1002085,
            offset: 12711,
        },
        name_keysym {
            keysym: 0x1002086,
            offset: 23967,
        },
        name_keysym {
            keysym: 0x1002087,
            offset: 23141,
        },
        name_keysym {
            keysym: 0x1002088,
            offset: 11881,
        },
        name_keysym {
            keysym: 0x1002089,
            offset: 20233,
        },
        name_keysym {
            keysym: 0x10020a0,
            offset: 11825,
        },
        name_keysym {
            keysym: 0x10020a1,
            offset: 8670,
        },
        name_keysym {
            keysym: 0x10020a2,
            offset: 8826,
        },
        name_keysym {
            keysym: 0x10020a3,
            offset: 12547,
        },
        name_keysym {
            keysym: 0x10020a4,
            offset: 19633,
        },
        name_keysym {
            keysym: 0x10020a5,
            offset: 19909,
        },
        name_keysym {
            keysym: 0x10020a6,
            offset: 20152,
        },
        name_keysym {
            keysym: 0x10020a7,
            offset: 21724,
        },
        name_keysym {
            keysym: 0x10020a8,
            offset: 22793,
        },
        name_keysym {
            keysym: 0x10020a9,
            offset: 26854,
        },
        name_keysym {
            keysym: 0x10020aa,
            offset: 20194,
        },
        name_keysym {
            keysym: 0x10020ab,
            offset: 11389,
        },
        name_keysym {
            keysym: 0x1002202,
            offset: 21644,
        },
        name_keysym {
            keysym: 0x1002205,
            offset: 12050,
        },
        name_keysym {
            keysym: 0x1002208,
            offset: 11933,
        },
        name_keysym {
            keysym: 0x1002209,
            offset: 20297,
        },
        name_keysym {
            keysym: 0x100220b,
            offset: 8768,
        },
        name_keysym {
            keysym: 0x100221a,
            offset: 24033,
        },
        name_keysym {
            keysym: 0x100221b,
            offset: 8839,
        },
        name_keysym {
            keysym: 0x100221c,
            offset: 12776,
        },
        name_keysym {
            keysym: 0x100222c,
            offset: 11363,
        },
        name_keysym {
            keysym: 0x100222d,
            offset: 25816,
        },
        name_keysym {
            keysym: 0x1002235,
            offset: 3396,
        },
        name_keysym {
            keysym: 0x1002247,
            offset: 20285,
        },
        name_keysym {
            keysym: 0x1002248,
            offset: 945,
        },
        name_keysym {
            keysym: 0x1002262,
            offset: 20319,
        },
        name_keysym {
            keysym: 0x1002263,
            offset: 24085,
        },
        name_keysym {
            keysym: 0x1002800,
            offset: 3628,
        },
        name_keysym {
            keysym: 0x1002801,
            offset: 3783,
        },
        name_keysym {
            keysym: 0x1002802,
            offset: 6151,
        },
        name_keysym {
            keysym: 0x1002803,
            offset: 3798,
        },
        name_keysym {
            keysym: 0x1002804,
            offset: 7303,
        },
        name_keysym {
            keysym: 0x1002805,
            offset: 5014,
        },
        name_keysym {
            keysym: 0x1002806,
            offset: 6166,
        },
        name_keysym {
            keysym: 0x1002807,
            offset: 3814,
        },
        name_keysym {
            keysym: 0x1002808,
            offset: 7863,
        },
        name_keysym {
            keysym: 0x1002809,
            offset: 5606,
        },
        name_keysym {
            keysym: 0x100280a,
            offset: 6758,
        },
        name_keysym {
            keysym: 0x100280b,
            offset: 4438,
        },
        name_keysym {
            keysym: 0x100280c,
            offset: 7318,
        },
        name_keysym {
            keysym: 0x100280d,
            offset: 5030,
        },
        name_keysym {
            keysym: 0x100280e,
            offset: 6182,
        },
        name_keysym {
            keysym: 0x100280f,
            offset: 3831,
        },
        name_keysym {
            keysym: 0x1002810,
            offset: 8135,
        },
        name_keysym {
            keysym: 0x1002811,
            offset: 5894,
        },
        name_keysym {
            keysym: 0x1002812,
            offset: 7046,
        },
        name_keysym {
            keysym: 0x1002813,
            offset: 4742,
        },
        name_keysym {
            keysym: 0x1002814,
            offset: 7606,
        },
        name_keysym {
            keysym: 0x1002815,
            offset: 5334,
        },
        name_keysym {
            keysym: 0x1002816,
            offset: 6486,
        },
        name_keysym {
            keysym: 0x1002817,
            offset: 4151,
        },
        name_keysym {
            keysym: 0x1002818,
            offset: 7878,
        },
        name_keysym {
            keysym: 0x1002819,
            offset: 5622,
        },
        name_keysym {
            keysym: 0x100281a,
            offset: 6774,
        },
        name_keysym {
            keysym: 0x100281b,
            offset: 4455,
        },
        name_keysym {
            keysym: 0x100281c,
            offset: 7334,
        },
        name_keysym {
            keysym: 0x100281d,
            offset: 5047,
        },
        name_keysym {
            keysym: 0x100281e,
            offset: 6199,
        },
        name_keysym {
            keysym: 0x100281f,
            offset: 3849,
        },
        name_keysym {
            keysym: 0x1002820,
            offset: 8267,
        },
        name_keysym {
            keysym: 0x1002821,
            offset: 6034,
        },
        name_keysym {
            keysym: 0x1002822,
            offset: 7186,
        },
        name_keysym {
            keysym: 0x1002823,
            offset: 4890,
        },
        name_keysym {
            keysym: 0x1002824,
            offset: 7746,
        },
        name_keysym {
            keysym: 0x1002825,
            offset: 5482,
        },
        name_keysym {
            keysym: 0x1002826,
            offset: 6634,
        },
        name_keysym {
            keysym: 0x1002827,
            offset: 4307,
        },
        name_keysym {
            keysym: 0x1002828,
            offset: 8018,
        },
        name_keysym {
            keysym: 0x1002829,
            offset: 5770,
        },
        name_keysym {
            keysym: 0x100282a,
            offset: 6922,
        },
        name_keysym {
            keysym: 0x100282b,
            offset: 4611,
        },
        name_keysym {
            keysym: 0x100282c,
            offset: 7482,
        },
        name_keysym {
            keysym: 0x100282d,
            offset: 5203,
        },
        name_keysym {
            keysym: 0x100282e,
            offset: 6355,
        },
        name_keysym {
            keysym: 0x100282f,
            offset: 4013,
        },
        name_keysym {
            keysym: 0x1002830,
            offset: 8150,
        },
        name_keysym {
            keysym: 0x1002831,
            offset: 5910,
        },
        name_keysym {
            keysym: 0x1002832,
            offset: 7062,
        },
        name_keysym {
            keysym: 0x1002833,
            offset: 4759,
        },
        name_keysym {
            keysym: 0x1002834,
            offset: 7622,
        },
        name_keysym {
            keysym: 0x1002835,
            offset: 5351,
        },
        name_keysym {
            keysym: 0x1002836,
            offset: 6503,
        },
        name_keysym {
            keysym: 0x1002837,
            offset: 4169,
        },
        name_keysym {
            keysym: 0x1002838,
            offset: 7894,
        },
        name_keysym {
            keysym: 0x1002839,
            offset: 5639,
        },
        name_keysym {
            keysym: 0x100283a,
            offset: 6791,
        },
        name_keysym {
            keysym: 0x100283b,
            offset: 4473,
        },
        name_keysym {
            keysym: 0x100283c,
            offset: 7351,
        },
        name_keysym {
            keysym: 0x100283d,
            offset: 5065,
        },
        name_keysym {
            keysym: 0x100283e,
            offset: 6217,
        },
        name_keysym {
            keysym: 0x100283f,
            offset: 3868,
        },
        name_keysym {
            keysym: 0x1002840,
            offset: 8331,
        },
        name_keysym {
            keysym: 0x1002841,
            offset: 6102,
        },
        name_keysym {
            keysym: 0x1002842,
            offset: 7254,
        },
        name_keysym {
            keysym: 0x1002843,
            offset: 4962,
        },
        name_keysym {
            keysym: 0x1002844,
            offset: 7814,
        },
        name_keysym {
            keysym: 0x1002845,
            offset: 5554,
        },
        name_keysym {
            keysym: 0x1002846,
            offset: 6706,
        },
        name_keysym {
            keysym: 0x1002847,
            offset: 4383,
        },
        name_keysym {
            keysym: 0x1002848,
            offset: 8086,
        },
        name_keysym {
            keysym: 0x1002849,
            offset: 5842,
        },
        name_keysym {
            keysym: 0x100284a,
            offset: 6994,
        },
        name_keysym {
            keysym: 0x100284b,
            offset: 4687,
        },
        name_keysym {
            keysym: 0x100284c,
            offset: 7554,
        },
        name_keysym {
            keysym: 0x100284d,
            offset: 5279,
        },
        name_keysym {
            keysym: 0x100284e,
            offset: 6431,
        },
        name_keysym {
            keysym: 0x100284f,
            offset: 4093,
        },
        name_keysym {
            keysym: 0x1002850,
            offset: 8218,
        },
        name_keysym {
            keysym: 0x1002851,
            offset: 5982,
        },
        name_keysym {
            keysym: 0x1002852,
            offset: 7134,
        },
        name_keysym {
            keysym: 0x1002853,
            offset: 4835,
        },
        name_keysym {
            keysym: 0x1002854,
            offset: 7694,
        },
        name_keysym {
            keysym: 0x1002855,
            offset: 5427,
        },
        name_keysym {
            keysym: 0x1002856,
            offset: 6579,
        },
        name_keysym {
            keysym: 0x1002857,
            offset: 4249,
        },
        name_keysym {
            keysym: 0x1002858,
            offset: 7966,
        },
        name_keysym {
            keysym: 0x1002859,
            offset: 5715,
        },
        name_keysym {
            keysym: 0x100285a,
            offset: 6867,
        },
        name_keysym {
            keysym: 0x100285b,
            offset: 4553,
        },
        name_keysym {
            keysym: 0x100285c,
            offset: 7427,
        },
        name_keysym {
            keysym: 0x100285d,
            offset: 5145,
        },
        name_keysym {
            keysym: 0x100285e,
            offset: 6297,
        },
        name_keysym {
            keysym: 0x100285f,
            offset: 3952,
        },
        name_keysym {
            keysym: 0x1002860,
            offset: 8282,
        },
        name_keysym {
            keysym: 0x1002861,
            offset: 6050,
        },
        name_keysym {
            keysym: 0x1002862,
            offset: 7202,
        },
        name_keysym {
            keysym: 0x1002863,
            offset: 4907,
        },
        name_keysym {
            keysym: 0x1002864,
            offset: 7762,
        },
        name_keysym {
            keysym: 0x1002865,
            offset: 5499,
        },
        name_keysym {
            keysym: 0x1002866,
            offset: 6651,
        },
        name_keysym {
            keysym: 0x1002867,
            offset: 4325,
        },
        name_keysym {
            keysym: 0x1002868,
            offset: 8034,
        },
        name_keysym {
            keysym: 0x1002869,
            offset: 5787,
        },
        name_keysym {
            keysym: 0x100286a,
            offset: 6939,
        },
        name_keysym {
            keysym: 0x100286b,
            offset: 4629,
        },
        name_keysym {
            keysym: 0x100286c,
            offset: 7499,
        },
        name_keysym {
            keysym: 0x100286d,
            offset: 5221,
        },
        name_keysym {
            keysym: 0x100286e,
            offset: 6373,
        },
        name_keysym {
            keysym: 0x100286f,
            offset: 4032,
        },
        name_keysym {
            keysym: 0x1002870,
            offset: 8166,
        },
        name_keysym {
            keysym: 0x1002871,
            offset: 5927,
        },
        name_keysym {
            keysym: 0x1002872,
            offset: 7079,
        },
        name_keysym {
            keysym: 0x1002873,
            offset: 4777,
        },
        name_keysym {
            keysym: 0x1002874,
            offset: 7639,
        },
        name_keysym {
            keysym: 0x1002875,
            offset: 5369,
        },
        name_keysym {
            keysym: 0x1002876,
            offset: 6521,
        },
        name_keysym {
            keysym: 0x1002877,
            offset: 4188,
        },
        name_keysym {
            keysym: 0x1002878,
            offset: 7911,
        },
        name_keysym {
            keysym: 0x1002879,
            offset: 5657,
        },
        name_keysym {
            keysym: 0x100287a,
            offset: 6809,
        },
        name_keysym {
            keysym: 0x100287b,
            offset: 4492,
        },
        name_keysym {
            keysym: 0x100287c,
            offset: 7369,
        },
        name_keysym {
            keysym: 0x100287d,
            offset: 5084,
        },
        name_keysym {
            keysym: 0x100287e,
            offset: 6236,
        },
        name_keysym {
            keysym: 0x100287f,
            offset: 3888,
        },
        name_keysym {
            keysym: 0x1002880,
            offset: 8362,
        },
        name_keysym {
            keysym: 0x1002881,
            offset: 6135,
        },
        name_keysym {
            keysym: 0x1002882,
            offset: 7287,
        },
        name_keysym {
            keysym: 0x1002883,
            offset: 4997,
        },
        name_keysym {
            keysym: 0x1002884,
            offset: 7847,
        },
        name_keysym {
            keysym: 0x1002885,
            offset: 5589,
        },
        name_keysym {
            keysym: 0x1002886,
            offset: 6741,
        },
        name_keysym {
            keysym: 0x1002887,
            offset: 4420,
        },
        name_keysym {
            keysym: 0x1002888,
            offset: 8119,
        },
        name_keysym {
            keysym: 0x1002889,
            offset: 5877,
        },
        name_keysym {
            keysym: 0x100288a,
            offset: 7029,
        },
        name_keysym {
            keysym: 0x100288b,
            offset: 4724,
        },
        name_keysym {
            keysym: 0x100288c,
            offset: 7589,
        },
        name_keysym {
            keysym: 0x100288d,
            offset: 5316,
        },
        name_keysym {
            keysym: 0x100288e,
            offset: 6468,
        },
        name_keysym {
            keysym: 0x100288f,
            offset: 4132,
        },
        name_keysym {
            keysym: 0x1002890,
            offset: 8251,
        },
        name_keysym {
            keysym: 0x1002891,
            offset: 6017,
        },
        name_keysym {
            keysym: 0x1002892,
            offset: 7169,
        },
        name_keysym {
            keysym: 0x1002893,
            offset: 4872,
        },
        name_keysym {
            keysym: 0x1002894,
            offset: 7729,
        },
        name_keysym {
            keysym: 0x1002895,
            offset: 5464,
        },
        name_keysym {
            keysym: 0x1002896,
            offset: 6616,
        },
        name_keysym {
            keysym: 0x1002897,
            offset: 4288,
        },
        name_keysym {
            keysym: 0x1002898,
            offset: 8001,
        },
        name_keysym {
            keysym: 0x1002899,
            offset: 5752,
        },
        name_keysym {
            keysym: 0x100289a,
            offset: 6904,
        },
        name_keysym {
            keysym: 0x100289b,
            offset: 4592,
        },
        name_keysym {
            keysym: 0x100289c,
            offset: 7464,
        },
        name_keysym {
            keysym: 0x100289d,
            offset: 5184,
        },
        name_keysym {
            keysym: 0x100289e,
            offset: 6336,
        },
        name_keysym {
            keysym: 0x100289f,
            offset: 3993,
        },
        name_keysym {
            keysym: 0x10028a0,
            offset: 8315,
        },
        name_keysym {
            keysym: 0x10028a1,
            offset: 6085,
        },
        name_keysym {
            keysym: 0x10028a2,
            offset: 7237,
        },
        name_keysym {
            keysym: 0x10028a3,
            offset: 4944,
        },
        name_keysym {
            keysym: 0x10028a4,
            offset: 7797,
        },
        name_keysym {
            keysym: 0x10028a5,
            offset: 5536,
        },
        name_keysym {
            keysym: 0x10028a6,
            offset: 6688,
        },
        name_keysym {
            keysym: 0x10028a7,
            offset: 4364,
        },
        name_keysym {
            keysym: 0x10028a8,
            offset: 8069,
        },
        name_keysym {
            keysym: 0x10028a9,
            offset: 5824,
        },
        name_keysym {
            keysym: 0x10028aa,
            offset: 6976,
        },
        name_keysym {
            keysym: 0x10028ab,
            offset: 4668,
        },
        name_keysym {
            keysym: 0x10028ac,
            offset: 7536,
        },
        name_keysym {
            keysym: 0x10028ad,
            offset: 5260,
        },
        name_keysym {
            keysym: 0x10028ae,
            offset: 6412,
        },
        name_keysym {
            keysym: 0x10028af,
            offset: 4073,
        },
        name_keysym {
            keysym: 0x10028b0,
            offset: 8201,
        },
        name_keysym {
            keysym: 0x10028b1,
            offset: 5964,
        },
        name_keysym {
            keysym: 0x10028b2,
            offset: 7116,
        },
        name_keysym {
            keysym: 0x10028b3,
            offset: 4816,
        },
        name_keysym {
            keysym: 0x10028b4,
            offset: 7676,
        },
        name_keysym {
            keysym: 0x10028b5,
            offset: 5408,
        },
        name_keysym {
            keysym: 0x10028b6,
            offset: 6560,
        },
        name_keysym {
            keysym: 0x10028b7,
            offset: 4229,
        },
        name_keysym {
            keysym: 0x10028b8,
            offset: 7948,
        },
        name_keysym {
            keysym: 0x10028b9,
            offset: 5696,
        },
        name_keysym {
            keysym: 0x10028ba,
            offset: 6848,
        },
        name_keysym {
            keysym: 0x10028bb,
            offset: 4533,
        },
        name_keysym {
            keysym: 0x10028bc,
            offset: 7408,
        },
        name_keysym {
            keysym: 0x10028bd,
            offset: 5125,
        },
        name_keysym {
            keysym: 0x10028be,
            offset: 6277,
        },
        name_keysym {
            keysym: 0x10028bf,
            offset: 3931,
        },
        name_keysym {
            keysym: 0x10028c0,
            offset: 8346,
        },
        name_keysym {
            keysym: 0x10028c1,
            offset: 6118,
        },
        name_keysym {
            keysym: 0x10028c2,
            offset: 7270,
        },
        name_keysym {
            keysym: 0x10028c3,
            offset: 4979,
        },
        name_keysym {
            keysym: 0x10028c4,
            offset: 7830,
        },
        name_keysym {
            keysym: 0x10028c5,
            offset: 5571,
        },
        name_keysym {
            keysym: 0x10028c6,
            offset: 6723,
        },
        name_keysym {
            keysym: 0x10028c7,
            offset: 4401,
        },
        name_keysym {
            keysym: 0x10028c8,
            offset: 8102,
        },
        name_keysym {
            keysym: 0x10028c9,
            offset: 5859,
        },
        name_keysym {
            keysym: 0x10028ca,
            offset: 7011,
        },
        name_keysym {
            keysym: 0x10028cb,
            offset: 4705,
        },
        name_keysym {
            keysym: 0x10028cc,
            offset: 7571,
        },
        name_keysym {
            keysym: 0x10028cd,
            offset: 5297,
        },
        name_keysym {
            keysym: 0x10028ce,
            offset: 6449,
        },
        name_keysym {
            keysym: 0x10028cf,
            offset: 4112,
        },
        name_keysym {
            keysym: 0x10028d0,
            offset: 8234,
        },
        name_keysym {
            keysym: 0x10028d1,
            offset: 5999,
        },
        name_keysym {
            keysym: 0x10028d2,
            offset: 7151,
        },
        name_keysym {
            keysym: 0x10028d3,
            offset: 4853,
        },
        name_keysym {
            keysym: 0x10028d4,
            offset: 7711,
        },
        name_keysym {
            keysym: 0x10028d5,
            offset: 5445,
        },
        name_keysym {
            keysym: 0x10028d6,
            offset: 6597,
        },
        name_keysym {
            keysym: 0x10028d7,
            offset: 4268,
        },
        name_keysym {
            keysym: 0x10028d8,
            offset: 7983,
        },
        name_keysym {
            keysym: 0x10028d9,
            offset: 5733,
        },
        name_keysym {
            keysym: 0x10028da,
            offset: 6885,
        },
        name_keysym {
            keysym: 0x10028db,
            offset: 4572,
        },
        name_keysym {
            keysym: 0x10028dc,
            offset: 7445,
        },
        name_keysym {
            keysym: 0x10028dd,
            offset: 5164,
        },
        name_keysym {
            keysym: 0x10028de,
            offset: 6316,
        },
        name_keysym {
            keysym: 0x10028df,
            offset: 3972,
        },
        name_keysym {
            keysym: 0x10028e0,
            offset: 8298,
        },
        name_keysym {
            keysym: 0x10028e1,
            offset: 6067,
        },
        name_keysym {
            keysym: 0x10028e2,
            offset: 7219,
        },
        name_keysym {
            keysym: 0x10028e3,
            offset: 4925,
        },
        name_keysym {
            keysym: 0x10028e4,
            offset: 7779,
        },
        name_keysym {
            keysym: 0x10028e5,
            offset: 5517,
        },
        name_keysym {
            keysym: 0x10028e6,
            offset: 6669,
        },
        name_keysym {
            keysym: 0x10028e7,
            offset: 4344,
        },
        name_keysym {
            keysym: 0x10028e8,
            offset: 8051,
        },
        name_keysym {
            keysym: 0x10028e9,
            offset: 5805,
        },
        name_keysym {
            keysym: 0x10028ea,
            offset: 6957,
        },
        name_keysym {
            keysym: 0x10028eb,
            offset: 4648,
        },
        name_keysym {
            keysym: 0x10028ec,
            offset: 7517,
        },
        name_keysym {
            keysym: 0x10028ed,
            offset: 5240,
        },
        name_keysym {
            keysym: 0x10028ee,
            offset: 6392,
        },
        name_keysym {
            keysym: 0x10028ef,
            offset: 4052,
        },
        name_keysym {
            keysym: 0x10028f0,
            offset: 8183,
        },
        name_keysym {
            keysym: 0x10028f1,
            offset: 5945,
        },
        name_keysym {
            keysym: 0x10028f2,
            offset: 7097,
        },
        name_keysym {
            keysym: 0x10028f3,
            offset: 4796,
        },
        name_keysym {
            keysym: 0x10028f4,
            offset: 7657,
        },
        name_keysym {
            keysym: 0x10028f5,
            offset: 5388,
        },
        name_keysym {
            keysym: 0x10028f6,
            offset: 6540,
        },
        name_keysym {
            keysym: 0x10028f7,
            offset: 4208,
        },
        name_keysym {
            keysym: 0x10028f8,
            offset: 7929,
        },
        name_keysym {
            keysym: 0x10028f9,
            offset: 5676,
        },
        name_keysym {
            keysym: 0x10028fa,
            offset: 6828,
        },
        name_keysym {
            keysym: 0x10028fb,
            offset: 4512,
        },
        name_keysym {
            keysym: 0x10028fc,
            offset: 7388,
        },
        name_keysym {
            keysym: 0x10028fd,
            offset: 5104,
        },
        name_keysym {
            keysym: 0x10028fe,
            offset: 6256,
        },
        name_keysym {
            keysym: 0x10028ff,
            offset: 3909,
        },
        name_keysym {
            keysym: 0x100000a8,
            offset: 17070,
        },
        name_keysym {
            keysym: 0x100000a9,
            offset: 17137,
        },
        name_keysym {
            keysym: 0x100000aa,
            offset: 17083,
        },
        name_keysym {
            keysym: 0x100000ab,
            offset: 17120,
        },
        name_keysym {
            keysym: 0x100000ac,
            offset: 17102,
        },
        name_keysym {
            keysym: 0x100000af,
            offset: 17027,
        },
        name_keysym {
            keysym: 0x100000be,
            offset: 16973,
        },
        name_keysym {
            keysym: 0x100000ee,
            offset: 17174,
        },
        name_keysym {
            keysym: 0x100000f6,
            offset: 17034,
        },
        name_keysym {
            keysym: 0x100000fc,
            offset: 16927,
        },
        name_keysym {
            keysym: 0x1000fe22,
            offset: 10496,
        },
        name_keysym {
            keysym: 0x1000fe27,
            offset: 10426,
        },
        name_keysym {
            keysym: 0x1000fe2c,
            offset: 10461,
        },
        name_keysym {
            keysym: 0x1000fe5e,
            offset: 10477,
        },
        name_keysym {
            keysym: 0x1000fe60,
            offset: 11320,
        },
        name_keysym {
            keysym: 0x1000fe7e,
            offset: 11548,
        },
        name_keysym {
            keysym: 0x1000feb0,
            offset: 11519,
        },
        name_keysym {
            keysym: 0x1000ff00,
            offset: 11511,
        },
        name_keysym {
            keysym: 0x1000ff48,
            offset: 17046,
        },
        name_keysym {
            keysym: 0x1000ff49,
            offset: 17058,
        },
        name_keysym {
            keysym: 0x1000ff6c,
            offset: 17150,
        },
        name_keysym {
            keysym: 0x1000ff6d,
            offset: 17158,
        },
        name_keysym {
            keysym: 0x1000ff6e,
            offset: 17167,
        },
        name_keysym {
            keysym: 0x1000ff6f,
            offset: 16935,
        },
        name_keysym {
            keysym: 0x1000ff70,
            offset: 16996,
        },
        name_keysym {
            keysym: 0x1000ff71,
            offset: 16960,
        },
        name_keysym {
            keysym: 0x1000ff72,
            offset: 16983,
        },
        name_keysym {
            keysym: 0x1000ff73,
            offset: 16947,
        },
        name_keysym {
            keysym: 0x1000ff74,
            offset: 16917,
        },
        name_keysym {
            keysym: 0x1000ff75,
            offset: 17014,
        },
        name_keysym {
            keysym: 0x1000ff76,
            offset: 12256,
        },
        name_keysym {
            keysym: 0x1000ff77,
            offset: 12267,
        },
        name_keysym {
            keysym: 0x1004ff02,
            offset: 21154,
        },
        name_keysym {
            keysym: 0x1004ff03,
            offset: 21162,
        },
        name_keysym {
            keysym: 0x1004ff04,
            offset: 21361,
        },
        name_keysym {
            keysym: 0x1004ff07,
            offset: 21098,
        },
        name_keysym {
            keysym: 0x1004ff08,
            offset: 21085,
        },
        name_keysym {
            keysym: 0x1004ff0b,
            offset: 21145,
        },
        name_keysym {
            keysym: 0x1004ff1b,
            offset: 21224,
        },
        name_keysym {
            keysym: 0x1004ff31,
            offset: 21074,
        },
        name_keysym {
            keysym: 0x1004ff32,
            offset: 21395,
        },
        name_keysym {
            keysym: 0x1004ff33,
            offset: 21411,
        },
        name_keysym {
            keysym: 0x1004ff40,
            offset: 21326,
        },
        name_keysym {
            keysym: 0x1004ff41,
            offset: 21351,
        },
        name_keysym {
            keysym: 0x1004ff42,
            offset: 21314,
        },
        name_keysym {
            keysym: 0x1004ff43,
            offset: 21338,
        },
        name_keysym {
            keysym: 0x1004ff44,
            offset: 21062,
        },
        name_keysym {
            keysym: 0x1004ff45,
            offset: 21278,
        },
        name_keysym {
            keysym: 0x1004ff51,
            offset: 21262,
        },
        name_keysym {
            keysym: 0x1004ff52,
            offset: 21488,
        },
        name_keysym {
            keysym: 0x1004ff53,
            offset: 21448,
        },
        name_keysym {
            keysym: 0x1004ff54,
            offset: 21194,
        },
        name_keysym {
            keysym: 0x1004ff57,
            offset: 21213,
        },
        name_keysym {
            keysym: 0x1004ff58,
            offset: 21122,
        },
        name_keysym {
            keysym: 0x1004ff59,
            offset: 21202,
        },
        name_keysym {
            keysym: 0x1004ff5a,
            offset: 21109,
        },
        name_keysym {
            keysym: 0x1004ff5b,
            offset: 21383,
        },
        name_keysym {
            keysym: 0x1004ff5c,
            offset: 21302,
        },
        name_keysym {
            keysym: 0x1004ff5d,
            offset: 21370,
        },
        name_keysym {
            keysym: 0x1004ff5e,
            offset: 21289,
        },
        name_keysym {
            keysym: 0x1004ff60,
            offset: 21457,
        },
        name_keysym {
            keysym: 0x1004ff63,
            offset: 21252,
        },
        name_keysym {
            keysym: 0x1004ff65,
            offset: 21480,
        },
        name_keysym {
            keysym: 0x1004ff67,
            offset: 21270,
        },
        name_keysym {
            keysym: 0x1004ff69,
            offset: 21135,
        },
        name_keysym {
            keysym: 0x1004ff6a,
            offset: 21244,
        },
        name_keysym {
            keysym: 0x1004ff71,
            offset: 21467,
        },
        name_keysym {
            keysym: 0x1004ff72,
            offset: 21179,
        },
        name_keysym {
            keysym: 0x1004ff73,
            offset: 21425,
        },
        name_keysym {
            keysym: 0x1004ff74,
            offset: 21234,
        },
        name_keysym {
            keysym: 0x1004ff78,
            offset: 21437,
        },
        name_keysym {
            keysym: 0x1004ffff,
            offset: 21169,
        },
        name_keysym {
            keysym: 0x1005ff00,
            offset: 24263,
        },
        name_keysym {
            keysym: 0x1005ff01,
            offset: 24234,
        },
        name_keysym {
            keysym: 0x1005ff02,
            offset: 24275,
        },
        name_keysym {
            keysym: 0x1005ff03,
            offset: 24208,
        },
        name_keysym {
            keysym: 0x1005ff04,
            offset: 24247,
        },
        name_keysym {
            keysym: 0x1005ff05,
            offset: 24220,
        },
        name_keysym {
            keysym: 0x1005ff10,
            offset: 24194,
        },
        name_keysym {
            keysym: 0x1005ff11,
            offset: 24201,
        },
        name_keysym {
            keysym: 0x1005ff60,
            offset: 24411,
        },
        name_keysym {
            keysym: 0x1005ff70,
            offset: 24394,
        },
        name_keysym {
            keysym: 0x1005ff71,
            offset: 24295,
        },
        name_keysym {
            keysym: 0x1005ff72,
            offset: 24179,
        },
        name_keysym {
            keysym: 0x1005ff73,
            offset: 24304,
        },
        name_keysym {
            keysym: 0x1005ff74,
            offset: 24334,
        },
        name_keysym {
            keysym: 0x1005ff75,
            offset: 24187,
        },
        name_keysym {
            keysym: 0x1005ff76,
            offset: 24343,
        },
        name_keysym {
            keysym: 0x1005ff77,
            offset: 24115,
        },
        name_keysym {
            keysym: 0x1005ff78,
            offset: 24135,
        },
        name_keysym {
            keysym: 0x1005ff79,
            offset: 24148,
        },
        name_keysym {
            keysym: 0x1005ff7a,
            offset: 24430,
        },
        name_keysym {
            keysym: 0x1005ff7b,
            offset: 24446,
        },
        name_keysym {
            keysym: 0x1005ff7c,
            offset: 24470,
        },
        name_keysym {
            keysym: 0x1005ff7d,
            offset: 24358,
        },
        name_keysym {
            keysym: 0x100810a4,
            offset: 30039,
        },
        name_keysym {
            keysym: 0x100810ae,
            offset: 28348,
        },
        name_keysym {
            keysym: 0x100810d1,
            offset: 27145,
        },
        name_keysym {
            keysym: 0x100810dc,
            offset: 31793,
        },
        name_keysym {
            keysym: 0x100810f4,
            offset: 27614,
        },
        name_keysym {
            keysym: 0x100810f5,
            offset: 28141,
        },
        name_keysym {
            keysym: 0x10081160,
            offset: 31033,
        },
        name_keysym {
            keysym: 0x10081162,
            offset: 28724,
        },
        name_keysym {
            keysym: 0x10081166,
            offset: 28841,
        },
        name_keysym {
            keysym: 0x10081168,
            offset: 32329,
        },
        name_keysym {
            keysym: 0x1008116a,
            offset: 30213,
        },
        name_keysym {
            keysym: 0x1008116e,
            offset: 30175,
        },
        name_keysym {
            keysym: 0x10081170,
            offset: 30001,
        },
        name_keysym {
            keysym: 0x10081171,
            offset: 30412,
        },
        name_keysym {
            keysym: 0x10081175,
            offset: 27164,
        },
        name_keysym {
            keysym: 0x10081177,
            offset: 27052,
        },
        name_keysym {
            keysym: 0x10081178,
            offset: 30195,
        },
        name_keysym {
            keysym: 0x10081179,
            offset: 30352,
        },
        name_keysym {
            keysym: 0x1008117a,
            offset: 30117,
        },
        name_keysym {
            keysym: 0x1008117b,
            offset: 30370,
        },
        name_keysym {
            keysym: 0x1008117c,
            offset: 30389,
        },
        name_keysym {
            keysym: 0x1008117d,
            offset: 30262,
        },
        name_keysym {
            keysym: 0x10081180,
            offset: 30287,
        },
        name_keysym {
            keysym: 0x10081181,
            offset: 30241,
        },
        name_keysym {
            keysym: 0x10081182,
            offset: 30331,
        },
        name_keysym {
            keysym: 0x10081183,
            offset: 30023,
        },
        name_keysym {
            keysym: 0x10081184,
            offset: 30307,
        },
        name_keysym {
            keysym: 0x10081185,
            offset: 28232,
        },
        name_keysym {
            keysym: 0x10081186,
            offset: 30092,
        },
        name_keysym {
            keysym: 0x10081188,
            offset: 27135,
        },
        name_keysym {
            keysym: 0x10081192,
            offset: 27932,
        },
        name_keysym {
            keysym: 0x10081193,
            offset: 27916,
        },
        name_keysym {
            keysym: 0x10081199,
            offset: 30058,
        },
        name_keysym {
            keysym: 0x1008119b,
            offset: 27583,
        },
        name_keysym {
            keysym: 0x1008119d,
            offset: 30755,
        },
        name_keysym {
            keysym: 0x100811a0,
            offset: 32354,
        },
        name_keysym {
            keysym: 0x100811a4,
            offset: 32547,
        },
        name_keysym {
            keysym: 0x100811a6,
            offset: 28240,
        },
        name_keysym {
            keysym: 0x100811a8,
            offset: 28733,
        },
        name_keysym {
            keysym: 0x100811a9,
            offset: 31205,
        },
        name_keysym {
            keysym: 0x100811aa,
            offset: 28104,
        },
        name_keysym {
            keysym: 0x100811ac,
            offset: 32403,
        },
        name_keysym {
            keysym: 0x100811ad,
            offset: 26967,
        },
        name_keysym {
            keysym: 0x100811af,
            offset: 28156,
        },
        name_keysym {
            keysym: 0x100811b0,
            offset: 31762,
        },
        name_keysym {
            keysym: 0x100811b6,
            offset: 28011,
        },
        name_keysym {
            keysym: 0x100811b7,
            offset: 30076,
        },
        name_keysym {
            keysym: 0x100811b8,
            offset: 26905,
        },
        name_keysym {
            keysym: 0x100811b9,
            offset: 26886,
        },
        name_keysym {
            keysym: 0x100811ba,
            offset: 28830,
        },
        name_keysym {
            keysym: 0x100811bc,
            offset: 30732,
        },
        name_keysym {
            keysym: 0x100811bd,
            offset: 31149,
        },
        name_keysym {
            keysym: 0x100811be,
            offset: 28762,
        },
        name_keysym {
            keysym: 0x100811bf,
            offset: 29425,
        },
        name_keysym {
            keysym: 0x100811d0,
            offset: 28429,
        },
        name_keysym {
            keysym: 0x100811d1,
            offset: 28481,
        },
        name_keysym {
            keysym: 0x100811d2,
            offset: 28501,
        },
        name_keysym {
            keysym: 0x100811d3,
            offset: 28544,
        },
        name_keysym {
            keysym: 0x100811d4,
            offset: 28554,
        },
        name_keysym {
            keysym: 0x100811d5,
            offset: 28564,
        },
        name_keysym {
            keysym: 0x100811d6,
            offset: 28574,
        },
        name_keysym {
            keysym: 0x100811d7,
            offset: 28584,
        },
        name_keysym {
            keysym: 0x100811d8,
            offset: 28594,
        },
        name_keysym {
            keysym: 0x100811d9,
            offset: 28604,
        },
        name_keysym {
            keysym: 0x100811da,
            offset: 28614,
        },
        name_keysym {
            keysym: 0x100811db,
            offset: 28511,
        },
        name_keysym {
            keysym: 0x100811dc,
            offset: 28522,
        },
        name_keysym {
            keysym: 0x100811dd,
            offset: 28533,
        },
        name_keysym {
            keysym: 0x100811de,
            offset: 28436,
        },
        name_keysym {
            keysym: 0x100811df,
            offset: 28445,
        },
        name_keysym {
            keysym: 0x100811e0,
            offset: 28463,
        },
        name_keysym {
            keysym: 0x100811e1,
            offset: 28472,
        },
        name_keysym {
            keysym: 0x100811e2,
            offset: 28492,
        },
        name_keysym {
            keysym: 0x100811e3,
            offset: 28624,
        },
        name_keysym {
            keysym: 0x100811e4,
            offset: 28454,
        },
        name_keysym {
            keysym: 0x100811e5,
            offset: 28633,
        },
        name_keysym {
            keysym: 0x10081200,
            offset: 30775,
        },
        name_keysym {
            keysym: 0x10081201,
            offset: 30788,
        },
        name_keysym {
            keysym: 0x10081202,
            offset: 30829,
        },
        name_keysym {
            keysym: 0x10081203,
            offset: 30842,
        },
        name_keysym {
            keysym: 0x10081204,
            offset: 30855,
        },
        name_keysym {
            keysym: 0x10081205,
            offset: 30868,
        },
        name_keysym {
            keysym: 0x10081206,
            offset: 30881,
        },
        name_keysym {
            keysym: 0x10081207,
            offset: 30894,
        },
        name_keysym {
            keysym: 0x10081208,
            offset: 30907,
        },
        name_keysym {
            keysym: 0x10081209,
            offset: 30920,
        },
        name_keysym {
            keysym: 0x1008120a,
            offset: 31002,
        },
        name_keysym {
            keysym: 0x1008120b,
            offset: 30985,
        },
        name_keysym {
            keysym: 0x1008120c,
            offset: 30933,
        },
        name_keysym {
            keysym: 0x1008120d,
            offset: 30946,
        },
        name_keysym {
            keysym: 0x1008120e,
            offset: 30959,
        },
        name_keysym {
            keysym: 0x1008120f,
            offset: 30972,
        },
        name_keysym {
            keysym: 0x10081210,
            offset: 27814,
        },
        name_keysym {
            keysym: 0x10081211,
            offset: 32473,
        },
        name_keysym {
            keysym: 0x10081215,
            offset: 27874,
        },
        name_keysym {
            keysym: 0x10081216,
            offset: 27891,
        },
        name_keysym {
            keysym: 0x10081217,
            offset: 27861,
        },
        name_keysym {
            keysym: 0x10081218,
            offset: 27799,
        },
        name_keysym {
            keysym: 0x10081219,
            offset: 27830,
        },
        name_keysym {
            keysym: 0x1008121a,
            offset: 27845,
        },
        name_keysym {
            keysym: 0x1008121b,
            offset: 27099,
        },
        name_keysym {
            keysym: 0x1008121c,
            offset: 27082,
        },
        name_keysym {
            keysym: 0x1008121d,
            offset: 27115,
        },
        name_keysym {
            keysym: 0x1008121e,
            offset: 29408,
        },
        name_keysym {
            keysym: 0x10081230,
            offset: 26983,
        },
        name_keysym {
            keysym: 0x10081232,
            offset: 31324,
        },
        name_keysym {
            keysym: 0x10081240,
            offset: 27669,
        },
        name_keysym {
            keysym: 0x10081241,
            offset: 32081,
        },
        name_keysym {
            keysym: 0x10081242,
            offset: 28861,
        },
        name_keysym {
            keysym: 0x10081243,
            offset: 28046,
        },
        name_keysym {
            keysym: 0x10081244,
            offset: 27038,
        },
        name_keysym {
            keysym: 0x10081245,
            offset: 31557,
        },
        name_keysym {
            keysym: 0x10081246,
            offset: 32386,
        },
        name_keysym {
            keysym: 0x10081247,
            offset: 27068,
        },
        name_keysym {
            keysym: 0x10081249,
            offset: 28322,
        },
        name_keysym {
            keysym: 0x1008124a,
            offset: 28117,
        },
        name_keysym {
            keysym: 0x1008124b,
            offset: 27753,
        },
        name_keysym {
            keysym: 0x1008124c,
            offset: 27729,
        },
        name_keysym {
            keysym: 0x1008124d,
            offset: 27776,
        },
        name_keysym {
            keysym: 0x1008124e,
            offset: 26933,
        },
        name_keysym {
            keysym: 0x1008124f,
            offset: 28188,
        },
        name_keysym {
            keysym: 0x10081250,
            offset: 27651,
        },
        name_keysym {
            keysym: 0x10081251,
            offset: 27633,
        },
        name_keysym {
            keysym: 0x10081252,
            offset: 28292,
        },
        name_keysym {
            keysym: 0x10081253,
            offset: 28261,
        },
        name_keysym {
            keysym: 0x10081260,
            offset: 29016,
        },
        name_keysym {
            keysym: 0x10081261,
            offset: 28965,
        },
        name_keysym {
            keysym: 0x10081262,
            offset: 29039,
        },
        name_keysym {
            keysym: 0x10081263,
            offset: 28988,
        },
        name_keysym {
            keysym: 0x10081264,
            offset: 28915,
        },
        name_keysym {
            keysym: 0x10081265,
            offset: 28940,
        },
        name_keysym {
            keysym: 0x10081266,
            offset: 31392,
        },
        name_keysym {
            keysym: 0x10081267,
            offset: 31378,
        },
        name_keysym {
            keysym: 0x10081268,
            offset: 29383,
        },
        name_keysym {
            keysym: 0x10081269,
            offset: 29370,
        },
        name_keysym {
            keysym: 0x1008126a,
            offset: 31448,
        },
        name_keysym {
            keysym: 0x1008126b,
            offset: 30431,
        },
        name_keysym {
            keysym: 0x1008126c,
            offset: 30801,
        },
        name_keysym {
            keysym: 0x1008126d,
            offset: 30815,
        },
        name_keysym {
            keysym: 0x1008126e,
            offset: 27205,
        },
        name_keysym {
            keysym: 0x1008126f,
            offset: 26922,
        },
        name_keysym {
            keysym: 0x10081270,
            offset: 30715,
        },
        name_keysym {
            keysym: 0x10081271,
            offset: 31834,
        },
        name_keysym {
            keysym: 0x10081272,
            offset: 31103,
        },
        name_keysym {
            keysym: 0x10081273,
            offset: 32378,
        },
        name_keysym {
            keysym: 0x10081274,
            offset: 32260,
        },
        name_keysym {
            keysym: 0x10081275,
            offset: 28370,
        },
        name_keysym {
            keysym: 0x10081276,
            offset: 31728,
        },
        name_keysym {
            keysym: 0x10081277,
            offset: 28095,
        },
        name_keysym {
            keysym: 0x10081278,
            offset: 31040,
        },
        name_keysym {
            keysym: 0x10081279,
            offset: 31257,
        },
        name_keysym {
            keysym: 0x1008127a,
            offset: 31639,
        },
        name_keysym {
            keysym: 0x1008127b,
            offset: 30699,
        },
        name_keysym {
            keysym: 0x1008127c,
            offset: 31237,
        },
        name_keysym {
            keysym: 0x1008127d,
            offset: 27479,
        },
        name_keysym {
            keysym: 0x1008127e,
            offset: 29984,
        },
        name_keysym {
            keysym: 0x1008127f,
            offset: 31744,
        },
        name_keysym {
            keysym: 0x10081280,
            offset: 30642,
        },
        name_keysym {
            keysym: 0x10081281,
            offset: 28412,
        },
        name_keysym {
            keysym: 0x10081282,
            offset: 31697,
        },
        name_keysym {
            keysym: 0x10081283,
            offset: 28213,
        },
        name_keysym {
            keysym: 0x10081284,
            offset: 31287,
        },
        name_keysym {
            keysym: 0x10081285,
            offset: 32217,
        },
        name_keysym {
            keysym: 0x10081286,
            offset: 27970,
        },
        name_keysym {
            keysym: 0x10081287,
            offset: 31681,
        },
        name_keysym {
            keysym: 0x10081288,
            offset: 30655,
        },
        name_keysym {
            keysym: 0x10081290,
            offset: 29484,
        },
        name_keysym {
            keysym: 0x10081291,
            offset: 29615,
        },
        name_keysym {
            keysym: 0x10081292,
            offset: 29746,
        },
        name_keysym {
            keysym: 0x10081293,
            offset: 29769,
        },
        name_keysym {
            keysym: 0x10081294,
            offset: 29780,
        },
        name_keysym {
            keysym: 0x10081295,
            offset: 29791,
        },
        name_keysym {
            keysym: 0x10081296,
            offset: 29802,
        },
        name_keysym {
            keysym: 0x10081297,
            offset: 29813,
        },
        name_keysym {
            keysym: 0x10081298,
            offset: 29824,
        },
        name_keysym {
            keysym: 0x10081299,
            offset: 29495,
        },
        name_keysym {
            keysym: 0x1008129a,
            offset: 29507,
        },
        name_keysym {
            keysym: 0x1008129b,
            offset: 29519,
        },
        name_keysym {
            keysym: 0x1008129c,
            offset: 29531,
        },
        name_keysym {
            keysym: 0x1008129d,
            offset: 29543,
        },
        name_keysym {
            keysym: 0x1008129e,
            offset: 29555,
        },
        name_keysym {
            keysym: 0x1008129f,
            offset: 29567,
        },
        name_keysym {
            keysym: 0x100812a0,
            offset: 29579,
        },
        name_keysym {
            keysym: 0x100812a1,
            offset: 29591,
        },
        name_keysym {
            keysym: 0x100812a2,
            offset: 29603,
        },
        name_keysym {
            keysym: 0x100812a3,
            offset: 29626,
        },
        name_keysym {
            keysym: 0x100812a4,
            offset: 29638,
        },
        name_keysym {
            keysym: 0x100812a5,
            offset: 29650,
        },
        name_keysym {
            keysym: 0x100812a6,
            offset: 29662,
        },
        name_keysym {
            keysym: 0x100812a7,
            offset: 29674,
        },
        name_keysym {
            keysym: 0x100812a8,
            offset: 29686,
        },
        name_keysym {
            keysym: 0x100812a9,
            offset: 29698,
        },
        name_keysym {
            keysym: 0x100812aa,
            offset: 29710,
        },
        name_keysym {
            keysym: 0x100812ab,
            offset: 29722,
        },
        name_keysym {
            keysym: 0x100812ac,
            offset: 29734,
        },
        name_keysym {
            keysym: 0x100812ad,
            offset: 29757,
        },
        name_keysym {
            keysym: 0x100812b0,
            offset: 29907,
        },
        name_keysym {
            keysym: 0x100812b1,
            offset: 29928,
        },
        name_keysym {
            keysym: 0x100812b2,
            offset: 29886,
        },
        name_keysym {
            keysym: 0x100812b3,
            offset: 29835,
        },
        name_keysym {
            keysym: 0x100812b4,
            offset: 29852,
        },
        name_keysym {
            keysym: 0x100812b5,
            offset: 29869,
        },
        name_keysym {
            keysym: 0x100812b8,
            offset: 29067,
        },
        name_keysym {
            keysym: 0x100812b9,
            offset: 29083,
        },
        name_keysym {
            keysym: 0x100812ba,
            offset: 29099,
        },
        name_keysym {
            keysym: 0x100812bb,
            offset: 29115,
        },
        name_keysym {
            keysym: 0x100812bc,
            offset: 29131,
        },
        name_keysym {
            keysym: 0x100812bd,
            offset: 31119,
        },
        name_keysym {
            keysym: 0x1008fe01,
            offset: 31886,
        },
        name_keysym {
            keysym: 0x1008fe02,
            offset: 31953,
        },
        name_keysym {
            keysym: 0x1008fe03,
            offset: 31969,
        },
        name_keysym {
            keysym: 0x1008fe04,
            offset: 31985,
        },
        name_keysym {
            keysym: 0x1008fe05,
            offset: 32001,
        },
        name_keysym {
            keysym: 0x1008fe06,
            offset: 32017,
        },
        name_keysym {
            keysym: 0x1008fe07,
            offset: 32033,
        },
        name_keysym {
            keysym: 0x1008fe08,
            offset: 32049,
        },
        name_keysym {
            keysym: 0x1008fe09,
            offset: 32065,
        },
        name_keysym {
            keysym: 0x1008fe0a,
            offset: 31902,
        },
        name_keysym {
            keysym: 0x1008fe0b,
            offset: 31919,
        },
        name_keysym {
            keysym: 0x1008fe0c,
            offset: 31936,
        },
        name_keysym {
            keysym: 0x1008fe20,
            offset: 32249,
        },
        name_keysym {
            keysym: 0x1008fe21,
            offset: 27956,
        },
        name_keysym {
            keysym: 0x1008fe22,
            offset: 30684,
        },
        name_keysym {
            keysym: 0x1008fe23,
            offset: 31222,
        },
        name_keysym {
            keysym: 0x1008fe24,
            offset: 29466,
        },
        name_keysym {
            keysym: 0x1008fe25,
            offset: 29439,
        },
        name_keysym {
            keysym: 0x1008ff01,
            offset: 30505,
        },
        name_keysym {
            keysym: 0x1008ff02,
            offset: 30585,
        },
        name_keysym {
            keysym: 0x1008ff03,
            offset: 30563,
        },
        name_keysym {
            keysym: 0x1008ff04,
            offset: 29147,
        },
        name_keysym {
            keysym: 0x1008ff05,
            offset: 28895,
        },
        name_keysym {
            keysym: 0x1008ff06,
            offset: 28873,
        },
        name_keysym {
            keysym: 0x1008ff07,
            offset: 30540,
        },
        name_keysym {
            keysym: 0x1008ff10,
            offset: 31803,
        },
        name_keysym {
            keysym: 0x1008ff11,
            offset: 27236,
        },
        name_keysym {
            keysym: 0x1008ff12,
            offset: 27289,
        },
        name_keysym {
            keysym: 0x1008ff13,
            offset: 27376,
        },
        name_keysym {
            keysym: 0x1008ff14,
            offset: 27332,
        },
        name_keysym {
            keysym: 0x1008ff15,
            offset: 27465,
        },
        name_keysym {
            keysym: 0x1008ff16,
            offset: 27362,
        },
        name_keysym {
            keysym: 0x1008ff17,
            offset: 27303,
        },
        name_keysym {
            keysym: 0x1008ff18,
            offset: 28804,
        },
        name_keysym {
            keysym: 0x1008ff19,
            offset: 29948,
        },
        name_keysym {
            keysym: 0x1008ff1a,
            offset: 31815,
        },
        name_keysym {
            keysym: 0x1008ff1b,
            offset: 31617,
        },
        name_keysym {
            keysym: 0x1008ff1c,
            offset: 27417,
        },
        name_keysym {
            keysym: 0x1008ff1d,
            offset: 27701,
        },
        name_keysym {
            keysym: 0x1008ff1e,
            offset: 30460,
        },
        name_keysym {
            keysym: 0x1008ff1f,
            offset: 32132,
        },
        name_keysym {
            keysym: 0x1008ff20,
            offset: 27716,
        },
        name_keysym {
            keysym: 0x1008ff21,
            offset: 31178,
        },
        name_keysym {
            keysym: 0x1008ff22,
            offset: 28027,
        },
        name_keysym {
            keysym: 0x1008ff23,
            offset: 31435,
        },
        name_keysym {
            keysym: 0x1008ff24,
            offset: 31404,
        },
        name_keysym {
            keysym: 0x1008ff25,
            offset: 31419,
        },
        name_keysym {
            keysym: 0x1008ff26,
            offset: 27514,
        },
        name_keysym {
            keysym: 0x1008ff27,
            offset: 28650,
        },
        name_keysym {
            keysym: 0x1008ff28,
            offset: 31825,
        },
        name_keysym {
            keysym: 0x1008ff29,
            offset: 31312,
        },
        name_keysym {
            keysym: 0x1008ff2a,
            offset: 31192,
        },
        name_keysym {
            keysym: 0x1008ff2b,
            offset: 32417,
        },
        name_keysym {
            keysym: 0x1008ff2c,
            offset: 28251,
        },
        name_keysym {
            keysym: 0x1008ff2d,
            offset: 31541,
        },
        name_keysym {
            keysym: 0x1008ff2e,
            offset: 32496,
        },
        name_keysym {
            keysym: 0x1008ff2f,
            offset: 31718,
        },
        name_keysym {
            keysym: 0x1008ff30,
            offset: 28386,
        },
        name_keysym {
            keysym: 0x1008ff31,
            offset: 27317,
        },
        name_keysym {
            keysym: 0x1008ff32,
            offset: 27257,
        },
        name_keysym {
            keysym: 0x1008ff33,
            offset: 30615,
        },
        name_keysym {
            keysym: 0x1008ff34,
            offset: 32314,
        },
        name_keysym {
            keysym: 0x1008ff35,
            offset: 29394,
        },
        name_keysym {
            keysym: 0x1008ff36,
            offset: 31672,
        },
        name_keysym {
            keysym: 0x1008ff37,
            offset: 28792,
        },
        name_keysym {
            keysym: 0x1008ff38,
            offset: 31070,
        },
        name_keysym {
            keysym: 0x1008ff39,
            offset: 26951,
        },
        name_keysym {
            keysym: 0x1008ff3a,
            offset: 28817,
        },
        name_keysym {
            keysym: 0x1008ff3b,
            offset: 27593,
        },
        name_keysym {
            keysym: 0x1008ff3c,
            offset: 28400,
        },
        name_keysym {
            keysym: 0x1008ff3d,
            offset: 27997,
        },
        name_keysym {
            keysym: 0x1008ff3e,
            offset: 27449,
        },
        name_keysym {
            keysym: 0x1008ff3f,
            offset: 27523,
        },
        name_keysym {
            keysym: 0x1008ff40,
            offset: 29178,
        },
        name_keysym {
            keysym: 0x1008ff41,
            offset: 29190,
        },
        name_keysym {
            keysym: 0x1008ff42,
            offset: 29202,
        },
        name_keysym {
            keysym: 0x1008ff43,
            offset: 29214,
        },
        name_keysym {
            keysym: 0x1008ff44,
            offset: 29226,
        },
        name_keysym {
            keysym: 0x1008ff45,
            offset: 29238,
        },
        name_keysym {
            keysym: 0x1008ff46,
            offset: 29250,
        },
        name_keysym {
            keysym: 0x1008ff47,
            offset: 29262,
        },
        name_keysym {
            keysym: 0x1008ff48,
            offset: 29274,
        },
        name_keysym {
            keysym: 0x1008ff49,
            offset: 29286,
        },
        name_keysym {
            keysym: 0x1008ff4a,
            offset: 29298,
        },
        name_keysym {
            keysym: 0x1008ff4b,
            offset: 29310,
        },
        name_keysym {
            keysym: 0x1008ff4c,
            offset: 29322,
        },
        name_keysym {
            keysym: 0x1008ff4d,
            offset: 29334,
        },
        name_keysym {
            keysym: 0x1008ff4e,
            offset: 29346,
        },
        name_keysym {
            keysym: 0x1008ff4f,
            offset: 29358,
        },
        name_keysym {
            keysym: 0x1008ff50,
            offset: 26997,
        },
        name_keysym {
            keysym: 0x1008ff51,
            offset: 27017,
        },
        name_keysym {
            keysym: 0x1008ff52,
            offset: 27574,
        },
        name_keysym {
            keysym: 0x1008ff53,
            offset: 27909,
        },
        name_keysym {
            keysym: 0x1008ff54,
            offset: 27686,
        },
        name_keysym {
            keysym: 0x1008ff55,
            offset: 27946,
        },
        name_keysym {
            keysym: 0x1008ff56,
            offset: 27987,
        },
        name_keysym {
            keysym: 0x1008ff57,
            offset: 28063,
        },
        name_keysym {
            keysym: 0x1008ff58,
            offset: 28072,
        },
        name_keysym {
            keysym: 0x1008ff59,
            offset: 28129,
        },
        name_keysym {
            keysym: 0x1008ff5a,
            offset: 28205,
        },
        name_keysym {
            keysym: 0x1008ff5b,
            offset: 28174,
        },
        name_keysym {
            keysym: 0x1008ff5c,
            offset: 28338,
        },
        name_keysym {
            keysym: 0x1008ff5d,
            offset: 28357,
        },
        name_keysym {
            keysym: 0x1008ff5e,
            offset: 28708,
        },
        name_keysym {
            keysym: 0x1008ff5f,
            offset: 28717,
        },
        name_keysym {
            keysym: 0x1008ff60,
            offset: 28850,
        },
        name_keysym {
            keysym: 0x1008ff61,
            offset: 29455,
        },
        name_keysym {
            keysym: 0x1008ff62,
            offset: 29973,
        },
        name_keysym {
            keysym: 0x1008ff63,
            offset: 30448,
        },
        name_keysym {
            keysym: 0x1008ff65,
            offset: 30469,
        },
        name_keysym {
            keysym: 0x1008ff66,
            offset: 30480,
        },
        name_keysym {
            keysym: 0x1008ff67,
            offset: 30630,
        },
        name_keysym {
            keysym: 0x1008ff68,
            offset: 30667,
        },
        name_keysym {
            keysym: 0x1008ff69,
            offset: 30675,
        },
        name_keysym {
            keysym: 0x1008ff6a,
            offset: 31018,
        },
        name_keysym {
            keysym: 0x1008ff6b,
            offset: 31061,
        },
        name_keysym {
            keysym: 0x1008ff6c,
            offset: 31082,
        },
        name_keysym {
            keysym: 0x1008ff6d,
            offset: 31093,
        },
        name_keysym {
            keysym: 0x1008ff6e,
            offset: 31139,
        },
        name_keysym {
            keysym: 0x1008ff70,
            offset: 31281,
        },
        name_keysym {
            keysym: 0x1008ff72,
            offset: 31357,
        },
        name_keysym {
            keysym: 0x1008ff73,
            offset: 31346,
        },
        name_keysym {
            keysym: 0x1008ff74,
            offset: 31461,
        },
        name_keysym {
            keysym: 0x1008ff75,
            offset: 31517,
        },
        name_keysym {
            keysym: 0x1008ff76,
            offset: 31479,
        },
        name_keysym {
            keysym: 0x1008ff77,
            offset: 31532,
        },
        name_keysym {
            keysym: 0x1008ff78,
            offset: 31604,
        },
        name_keysym {
            keysym: 0x1008ff79,
            offset: 31589,
        },
        name_keysym {
            keysym: 0x1008ff7a,
            offset: 31573,
        },
        name_keysym {
            keysym: 0x1008ff7b,
            offset: 31663,
        },
        name_keysym {
            keysym: 0x1008ff7c,
            offset: 31752,
        },
        name_keysym {
            keysym: 0x1008ff7d,
            offset: 31777,
        },
        name_keysym {
            keysym: 0x1008ff7e,
            offset: 31862,
        },
        name_keysym {
            keysym: 0x1008ff7f,
            offset: 32097,
        },
        name_keysym {
            keysym: 0x1008ff80,
            offset: 32110,
        },
        name_keysym {
            keysym: 0x1008ff81,
            offset: 32145,
        },
        name_keysym {
            keysym: 0x1008ff82,
            offset: 32238,
        },
        name_keysym {
            keysym: 0x1008ff84,
            offset: 32295,
        },
        name_keysym {
            keysym: 0x1008ff85,
            offset: 32271,
        },
        name_keysym {
            keysym: 0x1008ff86,
            offset: 32283,
        },
        name_keysym {
            keysym: 0x1008ff87,
            offset: 32344,
        },
        name_keysym {
            keysym: 0x1008ff88,
            offset: 32439,
        },
        name_keysym {
            keysym: 0x1008ff89,
            offset: 32464,
        },
        name_keysym {
            keysym: 0x1008ff8a,
            offset: 32504,
        },
        name_keysym {
            keysym: 0x1008ff8b,
            offset: 32524,
        },
        name_keysym {
            keysym: 0x1008ff8c,
            offset: 32535,
        },
        name_keysym {
            keysym: 0x1008ff8d,
            offset: 27505,
        },
        name_keysym {
            keysym: 0x1008ff8e,
            offset: 30491,
        },
        name_keysym {
            keysym: 0x1008ff8f,
            offset: 32428,
        },
        name_keysym {
            keysym: 0x1008ff90,
            offset: 29957,
        },
        name_keysym {
            keysym: 0x1008ff91,
            offset: 31165,
        },
        name_keysym {
            keysym: 0x1008ff92,
            offset: 30605,
        },
        name_keysym {
            keysym: 0x1008ff93,
            offset: 27539,
        },
        name_keysym {
            keysym: 0x1008ff94,
            offset: 27560,
        },
        name_keysym {
            keysym: 0x1008ff95,
            offset: 32455,
        },
        name_keysym {
            keysym: 0x1008ff96,
            offset: 32306,
        },
        name_keysym {
            keysym: 0x1008ff97,
            offset: 27219,
        },
        name_keysym {
            keysym: 0x1008ff98,
            offset: 27433,
        },
        name_keysym {
            keysym: 0x1008ff99,
            offset: 27397,
        },
        name_keysym {
            keysym: 0x1008ff9a,
            offset: 31849,
        },
        name_keysym {
            keysym: 0x1008ff9b,
            offset: 27185,
        },
        name_keysym {
            keysym: 0x1008ff9c,
            offset: 28080,
        },
        name_keysym {
            keysym: 0x1008ff9d,
            offset: 28662,
        },
        name_keysym {
            keysym: 0x1008ff9e,
            offset: 28676,
        },
        name_keysym {
            keysym: 0x1008ff9f,
            offset: 32123,
        },
        name_keysym {
            keysym: 0x1008ffa0,
            offset: 31628,
        },
        name_keysym {
            keysym: 0x1008ffa1,
            offset: 32369,
        },
        name_keysym {
            keysym: 0x1008ffa2,
            offset: 32155,
        },
        name_keysym {
            keysym: 0x1008ffa3,
            offset: 31304,
        },
        name_keysym {
            keysym: 0x1008ffa4,
            offset: 28752,
        },
        name_keysym {
            keysym: 0x1008ffa5,
            offset: 32513,
        },
        name_keysym {
            keysym: 0x1008ffa6,
            offset: 27551,
        },
        name_keysym {
            keysym: 0x1008ffa7,
            offset: 31874,
        },
        name_keysym {
            keysym: 0x1008ffa8,
            offset: 28778,
        },
        name_keysym {
            keysym: 0x1008ffa9,
            offset: 32198,
        },
        name_keysym {
            keysym: 0x1008ffb0,
            offset: 32183,
        },
        name_keysym {
            keysym: 0x1008ffb1,
            offset: 32167,
        },
        name_keysym {
            keysym: 0x1008ffb2,
            offset: 27272,
        },
        name_keysym {
            keysym: 0x1008ffb3,
            offset: 29165,
        },
        name_keysym {
            keysym: 0x1008ffb4,
            offset: 32487,
        },
        name_keysym {
            keysym: 0x1008ffb5,
            offset: 31367,
        },
        name_keysym {
            keysym: 0x1008ffb6,
            offset: 27346,
        },
        name_keysym {
            keysym: 0x1008ffb7,
            offset: 31494,
        },
        name_keysym {
            keysym: 0x1008ffb8,
            offset: 28693,
        },
    ];
    pub const UNICODE_KEYSYM: u16 = 0xfffe;
    pub const DEPRECATED_KEYSYM: u16 = 0xffff;
    pub static mut deprecated_keysyms: [deprecated_keysym; 157] = [
        deprecated_keysym {
            keysym: 0x27,
            offset: 934,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x60,
            offset: 13397,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xab,
            offset: 14480,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xba,
            offset: 21049,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbb,
            offset: 14494,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xd0,
            offset: 12195,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xd8,
            offset: 21494,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xde,
            offset: 25730,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xf8,
            offset: 21501,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x3a2,
            offset: 19268,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x4a5,
            offset: 18364,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x4af,
            offset: 18802,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x4c1,
            offset: 18324,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x4c2,
            offset: 18793,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x4cc,
            offset: 18395,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x5e7,
            offset: 1282,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6a4,
            offset: 26443,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6a6,
            offset: 26418,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6a7,
            offset: 26469,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6a8,
            offset: 9574,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6a9,
            offset: 9713,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6aa,
            offset: 9739,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6af,
            offset: 9068,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6b4,
            offset: 26430,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6b6,
            offset: 26406,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6b7,
            offset: 26456,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6b8,
            offset: 9562,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6b9,
            offset: 9700,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6ba,
            offset: 9726,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x6bf,
            offset: 9054,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x7a5,
            offset: 13857,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x8a2,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x8a3,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x8a6,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xaac,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xabc,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xabd,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xabe,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xaca,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xacc,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xacd,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xace,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xacf,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xadb,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xadc,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xadd,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xade,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xadf,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae0,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae1,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae2,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae3,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae4,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae5,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae6,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae7,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae8,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xae9,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xaea,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xaeb,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xba3,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xba6,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xba8,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xba9,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbc0,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbc3,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbc6,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbd6,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbd8,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xbda,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce1,
            offset: 16271,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce2,
            offset: 16452,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce3,
            offset: 16306,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce6,
            offset: 16737,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce7,
            offset: 16294,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xce8,
            offset: 16668,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xcf1,
            offset: 16592,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xcf5,
            offset: 16418,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xcf6,
            offset: 16713,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xcf7,
            offset: 16568,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xcfa,
            offset: 16657,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xdde,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xeff,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xfe8a,
            offset: 11166,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xfe8b,
            offset: 11155,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff20,
            offset: 19996,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff37,
            offset: 8654,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff3c,
            offset: 23236,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff3d,
            offset: 20006,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff3e,
            offset: 22298,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff55,
            offset: 22322,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff56,
            offset: 20208,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff61,
            offset: 22316,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff65,
            offset: 26588,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff66,
            offset: 22524,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff68,
            offset: 12662,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff69,
            offset: 8489,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff7e,
            offset: 17760,
            explicit_index: 0 as u8,
            explicit_count: 1 as u8,
        },
        deprecated_keysym {
            keysym: 0xff9a,
            offset: 19203,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xff9b,
            offset: 19171,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffc8,
            offset: 12297,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffc9,
            offset: 12301,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffca,
            offset: 12305,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffcb,
            offset: 12309,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffcc,
            offset: 12313,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffcd,
            offset: 12317,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffce,
            offset: 12321,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffcf,
            offset: 12325,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd0,
            offset: 12329,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd1,
            offset: 12336,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd2,
            offset: 12340,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd3,
            offset: 12344,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd4,
            offset: 12348,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd5,
            offset: 12352,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd6,
            offset: 12356,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd7,
            offset: 12360,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd8,
            offset: 12364,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffd9,
            offset: 12368,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffda,
            offset: 12372,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffdb,
            offset: 12379,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffdc,
            offset: 12383,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffdd,
            offset: 12387,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffde,
            offset: 12391,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffdf,
            offset: 12395,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0xffe0,
            offset: 12399,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100055b,
            offset: 2048,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100055c,
            offset: 2287,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100055d,
            offset: 2869,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100055e,
            offset: 2779,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000589,
            offset: 2327,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100058a,
            offset: 2448,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x10006cc,
            offset: 12521,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1002247,
            offset: UNICODE_KEYSYM,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1002248,
            offset: UNICODE_KEYSYM,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000a8,
            offset: 17070,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000a9,
            offset: 17137,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000aa,
            offset: 17083,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000ab,
            offset: 17120,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000ac,
            offset: 17102,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000af,
            offset: 17027,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000be,
            offset: 16973,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000ee,
            offset: 17174,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000f6,
            offset: 17034,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100000fc,
            offset: 16927,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff6c,
            offset: 17150,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff6d,
            offset: 17158,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff6e,
            offset: 17167,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff6f,
            offset: 16935,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff70,
            offset: 16996,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff71,
            offset: 16960,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff72,
            offset: 16983,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff73,
            offset: 16947,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff74,
            offset: 16917,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff75,
            offset: 17014,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff76,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x1000ff77,
            offset: DEPRECATED_KEYSYM as u16,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
        deprecated_keysym {
            keysym: 0x100810f4,
            offset: 30518,
            explicit_index: 0 as u8,
            explicit_count: 0 as u8,
        },
    ];
    pub static mut explicit_deprecated_aliases: [u32; 1] = [24103 as u32];

    use crate::xkb::shared_types::xkb_keysym_t;
}
pub mod utf8_decoding_h {
    pub const INVALID_UTF8_CODE_POINT: u32 = u32::MAX;

    pub use crate::xkb::utf8_decoding::utf8_next_code_point;
}

pub const XKB_KEYSYM_MAX_EXPLICIT: i32 = 0x1008ffb8;
pub const XKB_KEYSYM_UNICODE_OFFSET: i32 = 0x1000000;
pub const XKB_KEYSYM_UNICODE_MIN: i32 = 0x1000100;
pub const XKB_KEYSYM_UNICODE_MAX: i32 = 0x110ffff;

pub use self::keysym_names_h::{
    deprecated_keysyms, explicit_deprecated_aliases, keysym_name_G, keysym_name_perfect_hash,
    keysym_names, keysym_to_name, name_keysym, name_to_keysym, DEPRECATED_KEYSYM, UNICODE_KEYSYM,
};
pub use self::utf8_decoding_h::{utf8_next_code_point, INVALID_UTF8_CODE_POINT};
use crate::xkb::utils::cstr_dup;
use crate::xkb::utils::{cstr_cmp, cstr_len, cstr_ncmp};
pub use crate::xkb::utils::{digits__, parse_hex_to_uint32_t};
pub use crate::xkb::utils::{is_xdigit, istrcmp, istrncmp};
use libc::free;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_keysym_iterator {
    pub explicit: bool,
    pub index: i32,
    pub keysym: xkb_keysym_t,
}
fn find_keysym_index(mut ks: u32) -> isize {
    if ks > XKB_KEYSYM_MAX_EXPLICIT as u32 {
        return -1 as i32 as isize;
    }
    let mut lo: isize = 0 as isize;
    let mut hi: isize = (::core::mem::size_of::<[name_keysym; 2502]>() as usize)
        .wrapping_div(::core::mem::size_of::<name_keysym>() as usize)
        .wrapping_sub(1 as usize) as isize;
    while hi >= lo {
        let mut mid: isize = (lo + hi) / 2 as isize;
        unsafe {
            if ks > keysym_to_name[mid as usize].keysym {
                lo = mid + 1 as isize;
            } else if ks < keysym_to_name[mid as usize].keysym {
                hi = mid - 1 as isize;
            } else {
                return mid;
            }
        }
    }
    return -1 as i32 as isize;
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
        return crate::xkb::utils::snprintf_c(buffer, size, format_args!("U{:04X}", ks & 0xffffff));
    }
}
pub unsafe fn xkb_keysym_get_name(
    mut ks: xkb_keysym_t,
    mut buffer: *mut i8,
    mut size: usize,
) -> i32 {
    unsafe {
        if ks > XKB_KEYSYM_MAX as u32 {
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
        if ks >= XKB_KEYSYM_UNICODE_MIN as u32 && ks <= XKB_KEYSYM_UNICODE_MAX as u32 {
            return get_unicode_name(ks, buffer, size);
        }
        return crate::xkb::utils::snprintf_c(buffer, size, format_args!("0x{:08x}", ks));
    }
}

pub fn xkb_keysym_is_assigned(mut ks: u32) -> bool {
    return XKB_KEYSYM_UNICODE_MIN as u32 <= ks && ks <= XKB_KEYSYM_UNICODE_MAX as u32
        || find_keysym_index(ks) != -1 as i32 as isize;
}

pub unsafe fn xkb_keysym_get_explicit_names(
    mut ks: xkb_keysym_t,
    mut buffer: *mut *const i8,
    mut size: usize,
) -> i32 {
    unsafe {
        if ks > XKB_KEYSYM_MAX as u32 {
            return -1 as i32;
        }
        let index: isize = find_keysym_index(ks) as isize;
        if index < 0 as isize {
            return 0 as i32;
        }
        let canonical: u16 = keysym_to_name[index as usize].offset;
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
        let mut iter: *mut xkb_keysym_iterator =
            Box::into_raw(Box::new(std::mem::zeroed::<xkb_keysym_iterator>()));
        (*iter).explicit = iterate_only_explicit_keysyms;
        (*iter).index = -1 as i32 as i32;
        (*iter).keysym = XKB_KEYSYM_UNICODE_MAX as u32;
        return iter;
    }
}

pub unsafe fn xkb_keysym_iterator_unref(
    mut iter: *mut xkb_keysym_iterator,
) -> *mut xkb_keysym_iterator {
    unsafe {
        if !iter.is_null() {
            drop(Box::from_raw(iter));
        }
        return std::ptr::null_mut();
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
            return false;
        }
        if (*iter).explicit as i32 != 0
            || (*iter).keysym >= XKB_KEYSYM_UNICODE_MAX as u32
            || keysym_to_name[((*iter).index + 1 as i32) as usize].keysym
                < XKB_KEYSYM_UNICODE_MIN as u32
        {
            (*iter).index += 1;
            (*iter).keysym = keysym_to_name[(*iter).index as usize].keysym;
        } else {
            if (*iter).keysym >= keysym_to_name[(*iter).index as usize].keysym {
                (*iter).index += 1;
            }
            if (*iter).keysym >= XKB_KEYSYM_UNICODE_MIN as u32 {
                (*iter).keysym = (*iter).keysym.wrapping_add(1);
            } else {
                (*iter).keysym = XKB_KEYSYM_UNICODE_MIN as u32;
            }
        }
        return true;
    }
}
unsafe fn parse_keysym_hex(mut s: *const i8, mut out: *mut u32) -> bool {
    unsafe {
        let count: i32 = parse_hex_to_uint32_t(s, 8 as usize, out as *mut u32) as i32;
        return count > 0 as i32 && *s.offset(count as isize) as i32 == '\0' as i32;
    }
}
pub unsafe fn xkb_keysym_from_name(mut name: *const i8, mut flags: xkb_keysym_flags) -> u32 {
    unsafe {
        static mut XKB_KEYSYM_FLAGS: xkb_keysym_flags = XKB_KEYSYM_CASE_INSENSITIVE;
        if flags as u32 & !(XKB_KEYSYM_FLAGS as u32) != 0 {
            return XKB_KEY_NoSymbol as u32;
        }
        let mut entry: *const name_keysym = std::ptr::null();
        let mut tmp: *mut i8 = std::ptr::null_mut();
        let mut val: u32 = 0;
        let mut icase: bool = flags as u32 & XKB_KEYSYM_CASE_INSENSITIVE as u32 != 0;
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
                let mut last: *const name_keysym = std::ptr::null();
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
                return XKB_KEY_NoSymbol as u32;
            }
            return if val > 0xff as u32 && val <= 0x10ffff as u32 {
                (XKB_KEYSYM_UNICODE_OFFSET as u32).wrapping_add(val)
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
                return XKB_KEY_NoSymbol as u32;
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
                return XKB_KEY_NoSymbol as u32;
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
        return XKB_KEY_NoSymbol as u32;
    }
}

pub unsafe fn xkb_utf8_to_keysym(mut buffer: *const i8, mut size: usize) -> xkb_keysym_t {
    unsafe {
        if buffer.is_null() || size == 0 {
            return 0;
        }
        let mut length: usize = 0 as usize;
        let codepoint: u32 = utf8_next_code_point(buffer, size, &raw mut length) as u32;
        return if codepoint == INVALID_UTF8_CODE_POINT as u32 || length == 0 as usize {
            XKB_KEY_NoSymbol as u32
        } else {
            xkb_utf32_to_keysym(codepoint)
        };
    }
}
pub fn xkb_keysym_is_keypad(mut keysym: u32) -> bool {
    return keysym >= XKB_KEY_KP_Space as u32 && keysym <= XKB_KEY_KP_Equal as u32;
}

pub fn xkb_keysym_is_modifier(mut keysym: u32) -> bool {
    return keysym >= XKB_KEY_Shift_L as u32 && keysym <= XKB_KEY_Hyper_R as u32
        || keysym >= XKB_KEY_ISO_Lock as u32 && keysym <= XKB_KEY_ISO_Level5_Lock as u32
        || keysym == XKB_KEY_Mode_switch as u32
        || keysym == XKB_KEY_Num_Lock as u32;
}
pub unsafe fn xkb_keysym_is_deprecated(
    mut _keysym: xkb_keysym_t,
    mut _name: *const i8,
    mut reference_name: *mut *const i8,
) -> bool {
    unsafe {
        // Stub implementation: For Wayland-only usage, we don't need to track deprecated keysym names
        // This function is used by the parser to warn about deprecated keysym names
        *reference_name = std::ptr::null();
        return false;
    }
}
