//! Hardcoded mapping from XKB keyboard layout names to `compose.dir`
//! locale names.
//!
//! XKB layout names (e.g. `"us"`, `"gb"`, `"jp"`) are country-based
//! identifiers that live in `/usr/share/X11/xkb/symbols/`.  Compose
//! tables in `/usr/share/X11/locale/compose.dir` are indexed by full
//! locale names like `"en_US.UTF-8"` or `"ja_JP.UTF-8"`.
//!
//! The automatic resolution through `locale.alias` + UTF-8 fallback
//! handles many cases (e.g. `"de"` → `"de_DE.ISO8859-1"` →
//! `"de_DE.UTF-8"` → `en_US.UTF-8/Compose`), but it fails for XKB
//! layout names that don't appear in `locale.alias` at all — typically
//! because the layout name is a country code (`"gb"`, `"us"`) while
//! `locale.alias` uses language codes (`"en"`, `"ja"`).
//!
//! This table fills the gap by mapping every known XKB layout name
//! that the automatic resolution cannot handle to the appropriate
//! `compose.dir` locale.  It is consulted as the very first step in
//! [`super::resolve_compose_file`], before `compose.dir` lookup,
//! `locale.alias` resolution, or any heuristic.

use crate::xkb::shared_types::*;
use std::collections::BTreeMap;

lazy_static::lazy_static! {
    /// Maps an XKB layout name to the `compose.dir` locale name that
    /// should be used to look up the compose file.
    ///
    /// Only contains entries for layout names that cannot be resolved
    /// through `locale.alias` + UTF-8 fallback.  If a layout name is
    /// found here, the value is used as the locale to look up in
    /// `compose.dir`.
    pub static ref XKB_COMPOSE_MAP: BTreeMap<&'static str, &'static str> = [
        // --- Layouts where XKB name ≠ language code ---
        ("us",    "en_US.UTF-8"),   // United States → English
        ("gb",    "en_GB.UTF-8"),   // Great Britain → English
        ("au",    "en_AU.UTF-8"),   // Australia → English
        ("nz",    "en_NZ.UTF-8"),   // New Zealand → English
        ("za",    "en_ZA.UTF-8"),   // South Africa → English
        ("bw",    "en_BW.UTF-8"),   // Botswana → English
        ("no",    "nb_NO.UTF-8"),   // Norway → Norwegian Bokmål
        ("dk",    "da_DK.UTF-8"),   // Denmark → Danish
        ("se",    "sv_SE.UTF-8"),   // Sweden → Swedish
        ("at",    "de_AT.UTF-8"),   // Austria → German
        ("ch",    "de_CH.UTF-8"),   // Switzerland → German
        ("cz",    "cs_CZ.UTF-8"),   // Czech Republic → Czech
        ("gr",    "el_GR.UTF-8"),   // Greece → Greek
        ("rs",    "sr_RS.UTF-8"),   // Serbia → Serbian
        ("me",    "sr_ME.UTF-8"),   // Montenegro → Serbian
        ("al",    "sq_AL.UTF-8"),   // Albania → Albanian
        ("ba",    "bs_BA.UTF-8"),   // Bosnia → Bosnian
        ("by",    "be_BY.UTF-8"),   // Belarus → Belarusian
        ("ge",    "ka_GE.UTF-8"),   // Georgia → Georgian
        ("ua",    "uk_UA.UTF-8"),   // Ukraine → Ukrainian
        ("jp",    "ja_JP.UTF-8"),   // Japan → Japanese
        ("kr",    "ko_KR.UTF-8"),   // Korea → Korean
        ("cn",    "zh_CN.UTF-8"),   // China → Chinese
        ("tw",    "zh_TW.UTF-8"),   // Taiwan → Chinese
        ("kh",    "km_KH.UTF-8"),   // Cambodia → Khmer
        ("vn",    "vi_VN.UTF-8"),   // Vietnam → Vietnamese
        ("in",    "hi_IN.UTF-8"),   // India → Hindi
        ("bd",    "bn_BD.UTF-8"),   // Bangladesh → Bengali
        ("lk",    "si_LK.UTF-8"),   // Sri Lanka → Sinhala
        ("np",    "ne_NP.UTF-8"),   // Nepal → Nepali
        ("pk",    "ur_PK.UTF-8"),   // Pakistan → Urdu
        ("il",    "he_IL.UTF-8"),   // Israel → Hebrew

        // --- Arabic-script countries ---
        ("ara",   "ar_SA.UTF-8"),   // Arabic (generic) → Arabic
        ("iq",    "ar_IQ.UTF-8"),   // Iraq → Arabic
        ("ir",    "fa_IR.UTF-8"),   // Iran → Persian
        ("sy",    "ar_SY.UTF-8"),   // Syria → Arabic
        ("eg",    "ar_EG.UTF-8"),   // Egypt → Arabic
        ("dz",    "ar_DZ.UTF-8"),   // Algeria → Arabic
        ("ma",    "ar_MA.UTF-8"),   // Morocco → Arabic

        // --- Central Asian ---
        ("kg",    "ky_KG.UTF-8"),   // Kyrgyzstan → Kyrgyz
        ("kz",    "kk_KZ.UTF-8"),   // Kazakhstan → Kazakh
        ("tj",    "tg_TJ.UTF-8"),   // Tajikistan → Tajik
        ("la",    "lo_LA.UTF-8"),   // Laos → Lao
        ("my",    "ms_MY.UTF-8"),   // Malaysia → Malay

        // --- Irish / Esperanto / Latin American ---
        ("ie",    "ga_IE.UTF-8"),   // Ireland → Irish
        ("epo",   "eo_XX.UTF-8"),   // Esperanto
        ("latam", "es_MX.UTF-8"),   // Latin America → Spanish (Mexico)
    ].into();
}
