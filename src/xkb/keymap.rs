#![allow(clippy::incompatible_msrv, clippy::type_complexity)]
use std::rc::Rc;

pub use super::shared_types::XKB_KEYMAP_COMPILE_FLAGS_VALUES;
use super::shared_types::{atom_lookup_ref, atom_text};
pub(crate) use super::shared_types::{
    XkbAction, XkbContext, XkbKeymap, XkbLed, XkbLevel, XkbModSet, XkbRuleNames, MOD_BOTH,
    MOD_REAL, MOD_REAL_MASK_ALL, XKB_ATOM_NONE, XKB_KEYCODE_INVALID, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_INVALID, XKB_LED_INVALID, XKB_MOD_INVALID,
};

pub(crate) fn xkb_keymap_new_from_names(
    ctx: XkbContext,
    rmlvo_in: Option<&XkbRuleNames>,
    flags: u32,
) -> Option<Rc<XkbKeymap>> {
    let format = XKB_KEYMAP_FORMAT_TEXT_V2;
    let mut rmlvo: XkbRuleNames = match rmlvo_in {
        Some(r) => r.clone(),
        None => XkbRuleNames {
            rules: std::ffi::CString::new("").unwrap(),
            model: std::ffi::CString::new("").unwrap(),
            layout: std::ffi::CString::new("").unwrap(),
            variant: std::ffi::CString::new("").unwrap(),
            options: std::ffi::CString::new("").unwrap(),
        },
    };
    xkb_context_sanitize_rule_names(&ctx, &mut rmlvo);
    let mut keymap = xkb_keymap_new(ctx, "xkb_keymap_new_from_names2", format, flags)?;
    if !super::xkbcomp::text_v1_keymap_new_from_names(&mut keymap, &rmlvo) {
        return None;
    }
    Some(Rc::new(*keymap))
}
pub(crate) fn xkb_keymap_new_from_string(
    ctx: XkbContext,
    string: &std::ffi::CStr,
    format: u32,
    flags: u32,
) -> Option<Rc<XkbKeymap>> {
    let bytes = string.to_bytes();
    let mut length = bytes.len();
    if bytes.is_empty() {
        log::error!("{}: no buffer specified\n", "xkb_keymap_new_from_buffer");
        return None;
    }
    let mut keymap = xkb_keymap_new(ctx, "xkb_keymap_new_from_buffer", format, flags)?;
    let _ptr = string.as_ptr();
    if length > 0 && bytes[length - 1] == 0 {
        length -= 1;
    }
    if !super::xkbcomp::text_v1_keymap_new_from_string(&mut keymap, &bytes[..length]) {
        return None;
    }
    Some(Rc::new(*keymap))
}

pub(crate) fn xkb_keymap_num_mods(keymap: &XkbKeymap) -> u32 {
    keymap.mods.num_mods
}
pub(crate) fn xkb_keymap_mod_get_name(keymap: &XkbKeymap, idx: u32) -> Option<&str> {
    if idx >= keymap.mods.num_mods {
        return None;
    }
    let s = atom_text(&keymap.ctx.atom_table, keymap.mods.mods[idx as usize].name);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub(crate) fn xkb_keymap_num_layouts(keymap: &XkbKeymap) -> u32 {
    keymap.num_groups
}
pub(crate) fn xkb_keymap_layout_get_name(keymap: &XkbKeymap, idx: u32) -> Option<&str> {
    if idx as usize >= keymap.group_names.len() {
        return None;
    }
    let s = atom_text(&keymap.ctx.atom_table, keymap.group_names[idx as usize]);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
// ── Compose table support (merged from compose.rs) ──

use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

const LOCALE_DIR: &str = "/usr/share/X11/locale";

/// A parsed Compose file entry.
pub struct ComposeEntry {
    pub keys: Vec<char>,
    pub keysym_names: Vec<String>,
    pub multi_key_index: Option<usize>,
    pub output: char,
}

/// Resolve an XKB keysym name to its Unicode character using our existing
/// keysym database.
pub fn keysym_name_to_char(name: &str) -> Option<char> {
    use super::keysym::keysym_to_utf32;
    use super::keysym::xkb_keysym_from_name;
    use super::shared_types::{XKB_KEYSYM_NO_FLAGS, XKB_KEY_NO_SYMBOL};

    let ks = xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS);
    if ks == XKB_KEY_NO_SYMBOL as u32 {
        if let Some(hex) = name.strip_prefix('U') {
            if !hex.is_empty() && hex.len() <= 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
            }
        }
        if name.len() == 1 {
            return name.chars().next();
        }
        return None;
    }
    let utf32 = keysym_to_utf32(ks);
    if utf32 == 0 {
        return None;
    }
    char::from_u32(utf32)
}

/// Parse a Compose file at the given path, recursively handling `include` directives.
pub fn parse_compose_file(path: &Path) -> Vec<ComposeEntry> {
    let mut out = Vec::new();
    parse_compose_file_impl(path, &mut out);
    out
}

fn parse_compose_file_impl(path: &Path, out: &mut Vec<ComposeEntry>) {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("include") {
            let rest = rest.trim();
            if let Some(include_str) = rest.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                if include_str.is_empty() {
                    continue;
                }
                let include_path = Path::new(include_str);
                let resolved = if include_path.is_absolute() {
                    include_path.to_path_buf()
                } else if let Some(parent) = path.parent() {
                    parent.join(include_path)
                } else {
                    include_path.to_path_buf()
                };
                parse_compose_file_impl(&resolved, out);
            }
            continue;
        }

        if let Some(entry) = parse_rule_line(trimmed) {
            out.push(entry);
        }
    }
}

/// Parse a single rule line like `<Multi_key> <a> <e> : "æ" ae`
fn parse_rule_line(line: &str) -> Option<ComposeEntry> {
    let colon_pos = line.find(':')?;
    let lhs = &line[..colon_pos];
    let rhs = line[colon_pos + 1..].trim();

    let rhs = if let Some(hash) = rhs.find('#') {
        rhs[..hash].trim()
    } else {
        rhs
    };

    let mut keys = Vec::new();
    let mut keysym_names = Vec::new();
    let mut multi_key_index: Option<usize> = None;
    let mut pos = 0;
    let lhs_bytes = lhs.as_bytes();
    while pos < lhs_bytes.len() {
        if lhs_bytes[pos] == b'<' {
            let end = lhs[pos..].find('>')? + pos;
            let name = &lhs[pos + 1..end];
            if name.eq_ignore_ascii_case("Multi_key") {
                if multi_key_index.is_none() {
                    multi_key_index = Some(keys.len());
                }
            } else {
                let ch = keysym_name_to_char(name)?;
                keys.push(ch);
                keysym_names.push(name.to_string());
            }
            pos = end + 1;
        } else {
            pos += 1;
        }
    }

    if keys.is_empty() {
        return None;
    }

    let output = parse_rhs_value(rhs)?;

    Some(ComposeEntry {
        keys,
        keysym_names,
        multi_key_index,
        output,
    })
}

#[allow(clippy::manual_strip)]
/// Parse the RHS value: `"string" [keysym]` or bare `keysym_name`
fn parse_rhs_value(rhs: &str) -> Option<char> {
    let rhs = rhs.trim();
    if rhs.starts_with('"') {
        let end_quote = rhs[1..].find('"')? + 1;
        let s = &rhs[1..end_quote];
        if !s.is_empty() && !s.starts_with('\\') {
            if let Some(ch) = s.chars().next() {
                if !ch.is_ascii_digit() {
                    return Some(ch);
                }
            }
        }
        let after = rhs[end_quote + 1..].trim();
        if !after.is_empty() {
            let name = after.split_whitespace().next()?;
            return keysym_name_to_char(name);
        }
        s.chars().next()
    } else {
        let name = rhs.split_whitespace().next()?;
        keysym_name_to_char(name)
    }
}

fn lookup_locale_file(
    filename: &str,
    match_index: usize,
    return_index: usize,
    locale: &str,
) -> Option<String> {
    let path = Path::new(LOCALE_DIR).join(filename);
    let file = fs::File::open(path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() > match_index && parts.len() > return_index && parts[match_index] == locale {
            return Some(parts[return_index].to_string());
        }
    }
    None
}

fn lookup_compose_dir(locale: &str) -> Option<String> {
    lookup_locale_file("compose.dir", 1, 0, locale)
}

/// Resolve a locale name to the compose file sub-path (relative to
/// `/usr/share/X11/locale/`) that should be used.
pub fn resolve_compose_file(locale: &str) -> Option<String> {
    if let Some(&mapped_locale) = XKB_COMPOSE_MAP.get(locale) {
        if let Some(compose_file) = lookup_compose_dir(mapped_locale) {
            return Some(compose_file);
        }
    }

    if let Some(compose_file) = lookup_compose_dir(locale) {
        return Some(compose_file);
    }

    if let Some(resolved) = lookup_locale_file("locale.alias", 0, 1, locale) {
        if let Some(dot_pos) = resolved.find('.') {
            let base = &resolved[..dot_pos];
            if !resolved[dot_pos..].eq_ignore_ascii_case(".UTF-8") {
                let utf8_locale = format!("{}.UTF-8", base);
                if let Some(compose_file) = lookup_compose_dir(&utf8_locale) {
                    return Some(compose_file);
                }
            }
        }

        if let Some(compose_file) = lookup_compose_dir(&resolved) {
            return Some(compose_file);
        }
    }

    if locale.len() >= 2 && locale.len() <= 3 && locale.chars().all(|c| c.is_ascii_lowercase()) {
        let upper = locale.to_ascii_uppercase();
        let candidate = format!("{}_{}.UTF-8", locale, upper);
        if let Some(compose_file) = lookup_compose_dir(&candidate) {
            return Some(compose_file);
        }
    }

    lookup_compose_dir("en_US.UTF-8")
}

use std::collections::BTreeMap;

static XKB_COMPOSE_MAP: LazyLock<BTreeMap<&'static str, &'static str>> = LazyLock::new(|| {
    [
        ("us", "en_US.UTF-8"),
        ("gb", "en_GB.UTF-8"),
        ("au", "en_AU.UTF-8"),
        ("nz", "en_NZ.UTF-8"),
        ("za", "en_ZA.UTF-8"),
        ("bw", "en_BW.UTF-8"),
        ("no", "nb_NO.UTF-8"),
        ("dk", "da_DK.UTF-8"),
        ("se", "sv_SE.UTF-8"),
        ("at", "de_AT.UTF-8"),
        ("ch", "de_CH.UTF-8"),
        ("cz", "cs_CZ.UTF-8"),
        ("gr", "el_GR.UTF-8"),
        ("rs", "sr_RS.UTF-8"),
        ("me", "sr_ME.UTF-8"),
        ("al", "sq_AL.UTF-8"),
        ("ba", "bs_BA.UTF-8"),
        ("by", "be_BY.UTF-8"),
        ("ge", "ka_GE.UTF-8"),
        ("ua", "uk_UA.UTF-8"),
        ("jp", "ja_JP.UTF-8"),
        ("kr", "ko_KR.UTF-8"),
        ("cn", "zh_CN.UTF-8"),
        ("tw", "zh_TW.UTF-8"),
        ("kh", "km_KH.UTF-8"),
        ("vn", "vi_VN.UTF-8"),
        ("in", "hi_IN.UTF-8"),
        ("bd", "bn_BD.UTF-8"),
        ("lk", "si_LK.UTF-8"),
        ("np", "ne_NP.UTF-8"),
        ("pk", "ur_PK.UTF-8"),
        ("il", "he_IL.UTF-8"),
        ("ara", "ar_SA.UTF-8"),
        ("iq", "ar_IQ.UTF-8"),
        ("ir", "fa_IR.UTF-8"),
        ("sy", "ar_SY.UTF-8"),
        ("eg", "ar_EG.UTF-8"),
        ("dz", "ar_DZ.UTF-8"),
        ("ma", "ar_MA.UTF-8"),
        ("kg", "ky_KG.UTF-8"),
        ("kz", "kk_KZ.UTF-8"),
        ("tj", "tg_TJ.UTF-8"),
        ("la", "lo_LA.UTF-8"),
        ("my", "ms_MY.UTF-8"),
        ("ie", "ga_IE.UTF-8"),
        ("epo", "eo_XX.UTF-8"),
        ("latam", "es_MX.UTF-8"),
    ]
    .into()
});
pub(crate) fn xkb_keymap_key_get_name(keymap: &XkbKeymap, kc: u32) -> Option<&str> {
    let key = keymap.get_key(kc)?;
    let s = atom_text(&keymap.ctx.atom_table, key.name);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub(crate) fn xkb_keymap_key_repeats(keymap: &XkbKeymap, kc: u32) -> i32 {
    match keymap.get_key(kc) {
        Some(key) => key.repeats as i32,
        None => 0_i32,
    }
}

pub(crate) fn xkb_keymap_min_keycode(keymap: &XkbKeymap) -> u32 {
    keymap.min_key_code
}

pub(crate) fn xkb_keymap_max_keycode(keymap: &XkbKeymap) -> u32 {
    keymap.max_key_code
}

pub(crate) fn xkb_keymap_num_levels_for_key(keymap: &XkbKeymap, keycode: u32, layout: u32) -> u32 {
    keymap
        .get_key(keycode)
        .and_then(|k| k.groups.get(layout as usize))
        .map(|g| g.levels.len() as u32)
        .unwrap_or(0)
}

pub(crate) fn xkb_keymap_mod_get_index_ref(keymap: &XkbKeymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        XKB_MOD_INVALID
    } else {
        xkb_mod_name_to_index(&keymap.mods, atom, MOD_BOTH)
    }
}

pub(crate) fn xkb_keymap_key_get_syms_by_level_ref(
    keymap: &XkbKeymap,
    kc: u32,
    layout: u32,
    level: u32,
) -> &[u32] {
    keymap
        .get_key(kc)
        .and_then(|k| k.groups.get(layout as usize))
        .and_then(|g| g.levels.get(level as usize))
        .map(|lvl| lvl.syms.as_slice())
        .unwrap_or(&[])
}

// --- Merged from keymap_priv.rs ---

pub(crate) const XKB_MOD_NAME_SHIFT: &str = "Shift";
pub(crate) const XKB_MOD_NAME_CAPS: &str = "Lock";
pub(crate) const XKB_MOD_NAME_CTRL: &str = "Control";
pub(crate) const XKB_MOD_NAME_MOD1: &str = "Mod1";
pub(crate) const XKB_MOD_NAME_MOD2: &str = "Mod2";
pub(crate) const XKB_MOD_NAME_MOD3: &str = "Mod3";
pub(crate) const XKB_MOD_NAME_MOD4: &str = "Mod4";
pub(crate) const XKB_MOD_NAME_MOD5: &str = "Mod5";

pub(crate) fn xkb_keymap_new(
    ctx: XkbContext,
    func: &str,
    format: u32,
    flags: u32,
) -> Option<Box<XkbKeymap>> {
    static XKB_KEYMAP_COMPILE_FLAGS: u32 = XKB_KEYMAP_COMPILE_FLAGS_VALUES;
    if flags & !XKB_KEYMAP_COMPILE_FLAGS != 0 {
        log::error!(
            "{}: unrecognized keymap compilation flags: 0x{:x}\n",
            func,
            flags & !XKB_KEYMAP_COMPILE_FLAGS
        );
        return None;
    }
    let mut keymap = Box::new(XkbKeymap {
        ctx,
        flags: 0,
        format: 0,
        num_leds: 0,
        leds: [XkbLed::default(); 32],
        min_key_code: 0,
        max_key_code: 0,
        num_keys: 0,
        num_keys_low: 0,
        keys: Vec::new(),
        key_names: Vec::new(),
        key_aliases: Vec::new(),
        types: Vec::new(),
        sym_interprets: Vec::new(),
        mods: XkbModSet::default(),
        canonical_state_mask: 0,
        redirect_key_auto: 0,
        num_groups: 0,
        group_names: Vec::new(),
        keycodes_section_name: String::new(),
        symbols_section_name: String::new(),
        types_section_name: String::new(),
        compat_section_name: String::new(),
    });
    keymap.flags = flags;
    keymap.format = format;

    static BUILTIN_MODS: [&str; 8] = [
        XKB_MOD_NAME_SHIFT,
        XKB_MOD_NAME_CAPS,
        XKB_MOD_NAME_CTRL,
        XKB_MOD_NAME_MOD1,
        XKB_MOD_NAME_MOD2,
        XKB_MOD_NAME_MOD3,
        XKB_MOD_NAME_MOD4,
        XKB_MOD_NAME_MOD5,
    ];
    for (i, name) in BUILTIN_MODS.iter().enumerate() {
        keymap.mods.mods[i].name = atom_intern(&mut keymap.ctx.atom_table, name.as_bytes(), true);
        keymap.mods.mods[i].type_0 = MOD_REAL;
        keymap.mods.mods[i].mapping = 1_u32 << i;
    }
    keymap.mods.num_mods = BUILTIN_MODS.len() as u32;
    keymap.canonical_state_mask = MOD_REAL_MASK_ALL;
    Some(keymap)
}

pub(crate) fn xkb_escape_map_name(name: &mut String) {
    static LEGAL: [u8; 32] = [
        0, 0, 0, 0, 0, 0xa7, 0xff, 0x83, 0xfe, 0xff, 0xff, 0x87, 0xfe, 0xff, 0xff, 0x7, 0, 0, 0, 0,
        0, 0, 0, 0, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0x7f, 0xff,
    ];
    // Replace illegal bytes with '_'. Only ASCII bytes can be illegal,
    // so replacing with '_' preserves UTF-8 validity.
    *name = name
        .bytes()
        .map(|b| {
            if LEGAL[(b as usize) / 8] & (1u8 << (b % 8)) == 0 {
                b'_'
            } else {
                b
            }
        })
        .collect::<Vec<u8>>()
        .into_iter()
        .map(|b| b as char)
        .collect();
}

pub(crate) fn xkb_mod_name_to_index(mods: &XkbModSet, name: u32, type_0: u32) -> u32 {
    for (i, mod_0) in mods.mods[..mods.num_mods as usize].iter().enumerate() {
        if mod_0.type_0 & type_0 != 0 && name == mod_0.name {
            return i as u32;
        }
    }
    XKB_MOD_INVALID
}
pub(crate) fn xkb_levels_same_syms(a: &XkbLevel, b: &XkbLevel) -> bool {
    a.syms == b.syms
}
pub(crate) fn action_equal(a: &XkbAction, b: &XkbAction) -> bool {
    if a.action_type() != b.action_type() {
        return false;
    }
    match a.action_type() {
        0 | 1 => true,
        2..=4 => {
            let am = a.as_mods();
            let bm = b.as_mods();
            am.flags == bm.flags && am.mods.mask == bm.mods.mask && am.mods.mods == bm.mods.mods
        }
        5..=7 => {
            let ag = a.as_group();
            let bg = b.as_group();
            ag.flags == bg.flags && ag.group == bg.group
        }
        8 => {
            let ap = a.as_ptr();
            let bp = b.as_ptr();
            ap.flags == bp.flags && ap.x as i32 == bp.x as i32 && ap.y as i32 == bp.y as i32
        }
        9 | 10 => {
            let ab = a.as_btn();
            let bb = b.as_btn();
            ab.flags == bb.flags
                && ab.button as i32 == bb.button as i32
                && ab.count as i32 == bb.count as i32
        }
        11 => {
            let ad = a.as_dflt();
            let bd = b.as_dflt();
            ad.flags == bd.flags && ad.value as i32 == bd.value as i32
        }
        12 => true,
        13 => {
            let as_ = a.as_screen();
            let bs = b.as_screen();
            as_.flags == bs.flags && as_.screen as i32 == bs.screen as i32
        }
        14 | 15 => {
            let ac = a.as_ctrls();
            let bc = b.as_ctrls();
            ac.flags == bc.flags && ac.ctrls == bc.ctrls
        }
        16 => {
            let ar = a.as_redirect();
            let br = b.as_redirect();
            ar.keycode == br.keycode && ar.affect == br.affect && ar.mods == br.mods
        }
        17 | 18 => true,
        20 => {
            let ai = a.as_internal();
            let bi = b.as_internal();
            ai.flags == bi.flags && ai.clear_latched_mods == bi.clear_latched_mods
        }
        _ => a.as_priv().data == b.as_priv().data,
    }
}
pub(crate) fn xkb_levels_same_actions(a: &XkbLevel, b: &XkbLevel) -> bool {
    if a.actions.len() != b.actions.len() {
        return false;
    }
    for k in 0..a.actions.len() {
        if !action_equal(&a.actions[k], &b.actions[k]) {
            return false;
        }
    }
    true
}
pub(crate) fn xkb_wrap_group_into_range(
    group: i32,
    num_groups: u32,
    out_of_range_group_policy: u32,
    out_of_range_group_number: u32,
) -> u32 {
    if num_groups == 0_u32 {
        return XKB_LAYOUT_INVALID;
    }
    if group >= 0_i32 && (group as u32) < num_groups {
        return group as u32;
    }
    match out_of_range_group_policy {
        2 => {
            if out_of_range_group_number >= num_groups {
                return 0_u32;
            }
            out_of_range_group_number
        }
        1 => {
            if group < 0_i32 {
                0_u32
            } else {
                num_groups.wrapping_sub(1_u32)
            }
        }
        _ => {
            let rem: i32 = group % num_groups as i32;
            (if rem >= 0_i32 {
                rem
            } else {
                rem + num_groups as i32
            }) as u32
        }
    }
}
pub(crate) fn xkb_keymap_num_layouts_for_key(keymap: &XkbKeymap, kc: u32) -> u32 {
    keymap.get_key(kc).map(|k| k.num_groups).unwrap_or(0)
}

pub(crate) fn xkb_keymap_num_leds(keymap: &XkbKeymap) -> u32 {
    keymap.leds.len() as u32
}

pub(crate) fn xkb_keymap_led_get_name(keymap: &XkbKeymap, idx: u32) -> Option<&str> {
    keymap
        .leds
        .get(idx as usize)
        .map(|l| atom_text(&keymap.ctx.atom_table, l.name))
}

pub(crate) fn xkb_keymap_layout_get_index_ref(keymap: &XkbKeymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        XKB_LAYOUT_INVALID
    } else {
        keymap
            .group_names
            .iter()
            .position(|&n| n == atom)
            .map_or(XKB_LAYOUT_INVALID, |i| i as u32)
    }
}

pub(crate) fn xkb_keymap_led_get_index_ref(keymap: &XkbKeymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        XKB_LED_INVALID
    } else {
        keymap
            .leds
            .iter()
            .position(|l| l.name == atom)
            .map_or(XKB_LED_INVALID, |i| i as u32)
    }
}
use std::env::VarError;

use super::shared_types::{atom_intern, atom_table_new};

pub(crate) use super::shared_types::{
    Rmlvo, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
use super::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use super::shared_types::{XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_LOG_VERBOSITY_DEFAULT};
fn context_include_path_append(ctx: &mut XkbContext, path: &str) -> i32 {
    let is_dir = std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false);
    if is_dir {
        log::info!("Include path added: {}\n", path);
        ctx.includes.push(path.to_string());
        return 1_i32;
    }
    if !path.is_empty() {
        ctx.failed_includes.push(path.to_string());
    }
    log::info!("Include path failed: \"{}\"\n", path);
    0_i32
}

pub(crate) fn xkb_context_include_path_get_extra_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_EXTRA_PATH") {
        Ok(extra) => extra,
        Err(_) => DFLT_XKB_CONFIG_EXTRA_PATH.to_string(),
    }
}

pub(crate) fn xkb_context_include_path_get_unversioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.to_string(),
    }
}

pub(crate) fn xkb_context_include_path_get_versioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.to_string(),
    }
}
/// Convert a null-terminated `[i8]` constant to a Rust `String`.
fn add_direct_subdirectories(
    ctx: &mut XkbContext,
    path: &str,
    extensions: &mut Vec<String>,
    versioned_count: usize,
    versioned_path_length: usize,
) -> i32 {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(e) => {
            log::debug!("Include extensions path failed: {} ({})\n", path, e);
            return 0_i32;
        }
    };

    // The +1 accounts for the '/' separator between the base path and entry name
    let name_offset = if versioned_path_length > 0 {
        versioned_path_length + 1
    } else {
        0
    };

    for entry in dir.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == "." || name_str == ".." {
            continue;
        }
        let full_path = format!("{}/{}", path, name_str);
        // Check if it's a directory
        if !std::fs::metadata(&full_path)
            .map(|m| m.is_dir())
            .unwrap_or(false)
        {
            continue;
        }
        // Check if already in versioned list
        let mut duplicate = false;
        for ext in extensions.iter().take(versioned_count) {
            if name_offset <= ext.len() && *name_str == ext[name_offset..] {
                duplicate = true;
                break;
            }
        }
        if duplicate {
            continue;
        }
        extensions.push(full_path);
    }

    let mut ret = 0_i32;
    // Sort the newly added entries and append as include paths
    if extensions.len() > versioned_count {
        extensions[versioned_count..].sort();
        for ext in extensions.iter().skip(versioned_count) {
            ret |= context_include_path_append(ctx, ext);
        }
    }

    ret
}

pub(crate) fn xkb_context_include_path_get_system_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_ROOT") {
        Ok(root) => root,
        Err(_) => DFLT_XKB_CONFIG_ROOT.to_string(),
    }
}

pub(crate) fn xkb_context_include_path_append_default(ctx: &mut XkbContext) -> i32 {
    {
        let mut ret: i32 = 0_i32;
        let home = xkb_context_getenv("HOME");
        let xdg = xkb_context_getenv("XDG_CONFIG_HOME");
        if let Ok(ref xdg) = xdg {
            ret |= context_include_path_append(ctx, &format!("{}/xkb", xdg));
        } else if let Ok(ref home) = home {
            ret |= context_include_path_append(ctx, &format!("{}/.config/xkb", home));
        }
        if let Ok(ref home) = home {
            ret |= context_include_path_append(ctx, &format!("{}/.xkb", home));
        }
        let extra = xkb_context_include_path_get_extra_path();
        ret |= context_include_path_append(ctx, &extra);

        let mut extensions: Vec<String> = Vec::new();
        let versioned_path = xkb_context_include_path_get_versioned_extensions_path();
        let mut versioned_path_length: usize = 0;
        if !versioned_path.is_empty() {
            ret |= add_direct_subdirectories(ctx, &versioned_path, &mut extensions, 0, 0);
            versioned_path_length = versioned_path.len();
        }
        let unversioned_path = xkb_context_include_path_get_unversioned_extensions_path();
        if !unversioned_path.is_empty() {
            let versioned_count = extensions.len();
            ret |= add_direct_subdirectories(
                ctx,
                &unversioned_path,
                &mut extensions,
                versioned_count,
                versioned_path_length,
            );
        }

        let root = xkb_context_include_path_get_system_path();
        let has_root: bool = context_include_path_append(ctx, &root) != 0;
        ret |= has_root as i32;
        if !has_root && !root.is_empty() {
            let legacy = DFLT_XKB_LEGACY_ROOT.to_string();
            log::warn!("Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                "/usr/share/X11/xkb",
                root);
            ret |= context_include_path_append(ctx, &legacy);
        }
        ret
    }
}

pub(crate) fn xkb_context_num_include_paths(ctx: &mut XkbContext) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.includes.len() as u32
    } else {
        0_u32
    }
}
pub(crate) fn xkb_context_include_path_get(ctx: &mut XkbContext, idx: u32) -> String {
    if idx >= xkb_context_num_include_paths(ctx) {
        return "".to_string();
    }
    ctx.includes.get(idx as usize).unwrap().clone()
}

fn log_level_from_str(level: &str) -> u32 {
    let bytes = level.as_bytes();
    // Try parsing as integer first
    if let Ok(val) = level.trim().parse::<i64>() {
        return val as u32;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"crit"))
    {
        return XKB_LOG_LEVEL_CRITICAL;
    }
    if bytes
        .get(..3)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"err"))
    {
        return XKB_LOG_LEVEL_ERROR;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"warn"))
    {
        return XKB_LOG_LEVEL_WARNING;
    }
    if bytes
        .get(..4)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"info"))
    {
        return XKB_LOG_LEVEL_INFO;
    }
    if bytes
        .get(..5)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"debug"))
        || bytes
            .get(..3)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"dbg"))
    {
        return XKB_LOG_LEVEL_DEBUG;
    }
    XKB_LOG_LEVEL_ERROR
}
fn log_verbosity_from_str(verbosity: &str) -> i32 {
    if let Ok(val) = verbosity.trim().parse::<i64>() {
        return val as i32;
    }
    XKB_LOG_VERBOSITY_DEFAULT
}
pub(crate) fn xkb_context_new(flags: XkbContextFlags) -> XkbContext {
    let mut ctx = XkbContext {
        log_level: XKB_LOG_LEVEL_ERROR,
        log_verbosity: XKB_LOG_VERBOSITY_DEFAULT,
        includes: Vec::new(),
        failed_includes: Vec::new(),
        atom_table: atom_table_new(),
        use_environment_names: false,
        use_secure_getenv: false,
        pending_default_includes: false,
    };
    const XKB_CONTEXT_ALL_FLAGS: XkbContextFlags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
        | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32
        | XKB_CONTEXT_NO_SECURE_GETENV as i32)
        as XkbContextFlags;
    if flags & !XKB_CONTEXT_ALL_FLAGS != 0 {
        log::error!(
            "Invalid context flags: 0x{:x}\n",
            flags & !XKB_CONTEXT_ALL_FLAGS
        );
        return ctx;
    }
    ctx.use_environment_names = flags & XKB_CONTEXT_NO_ENVIRONMENT_NAMES == 0;
    ctx.use_secure_getenv = flags & XKB_CONTEXT_NO_SECURE_GETENV == 0;
    ctx.pending_default_includes = flags & XKB_CONTEXT_NO_DEFAULT_INCLUDES == 0;
    if let Ok(env) = xkb_context_getenv("XKB_LOG_LEVEL") {
        ctx.log_level = log_level_from_str(&env);
    }
    if let Ok(env) = xkb_context_getenv("XKB_LOG_VERBOSITY") {
        ctx.log_verbosity = log_verbosity_from_str(&env);
    }
    ctx
}

pub(crate) fn xkb_context_get_log_verbosity(ctx: &XkbContext) -> i32 {
    ctx.log_verbosity
}

// --- Merged from context_priv.rs ---

pub(crate) fn xkb_context_getenv(name: &str) -> Result<String, VarError> {
    std::env::var(name)
}
pub(crate) fn xkb_context_init_includes(ctx: &mut XkbContext) -> bool {
    if ctx.pending_default_includes {
        if ctx.failed_includes.is_empty() {
            if xkb_context_include_path_append_default(ctx) == 0 {
                log::error!(
                    "[XKB-{:03}] Failed to add any default include path (system path: {})\n",
                    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                    xkb_context_include_path_get_system_path()
                );
                return false;
            }
            ctx.pending_default_includes = false;
        } else {
            return false;
        }
    }
    true
}
pub(crate) fn xkb_context_num_failed_include_paths(ctx: &mut XkbContext) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.failed_includes.len() as u32
    } else {
        0_u32
    }
}
pub(crate) fn xkb_context_failed_include_path_get(ctx: &mut XkbContext, idx: u32) -> String {
    if idx >= xkb_context_num_failed_include_paths(ctx) {
        return "".to_string();
    }
    ctx.failed_includes.get(idx as usize).unwrap().clone()
}

pub(crate) fn xkb_context_sanitize_rule_names(ctx: &XkbContext, rmlvo: &mut XkbRuleNames) -> Rmlvo {
    let mut modified: Rmlvo = 0 as Rmlvo;
    if rmlvo.rules.as_bytes().is_empty() {
        let env = if ctx.use_environment_names {
            xkb_context_getenv("XKB_DEFAULT_RULES")
        } else {
            Err(VarError::NotPresent)
        };
        rmlvo.rules = match env {
            Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
            Err(_) => std::ffi::CString::new("evdev").unwrap(),
        };
        modified = (modified as u32 | RMLVO_RULES) as Rmlvo;
    }
    if rmlvo.model.as_bytes().is_empty() {
        let env = if ctx.use_environment_names {
            xkb_context_getenv("XKB_DEFAULT_MODEL")
        } else {
            Err(VarError::NotPresent)
        };
        rmlvo.model = match env {
            Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
            Err(_) => std::ffi::CString::new("pc105").unwrap(),
        };
        modified = (modified as u32 | RMLVO_MODEL) as Rmlvo;
    }
    if rmlvo.layout.as_bytes().is_empty() {
        {
            let env = if ctx.use_environment_names {
                xkb_context_getenv("XKB_DEFAULT_LAYOUT")
            } else {
                Err(VarError::NotPresent)
            };
            rmlvo.layout = match env {
                Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                Err(_) => std::ffi::CString::new("us").unwrap(),
            };
        }
        modified = (modified as u32 | RMLVO_LAYOUT) as Rmlvo;
        let variant: std::ffi::CString = {
            let layout = xkb_context_getenv("XKB_DEFAULT_LAYOUT");
            let default_variant = xkb_context_getenv("XKB_DEFAULT_VARIANT");
            match (layout, ctx.use_environment_names, default_variant) {
                (Ok(_), true, Ok(default_variant)) => {
                    std::ffi::CString::new(default_variant).unwrap_or_default()
                }
                (_, _, _) => std::ffi::CString::new("").unwrap(),
            }
        };
        if !rmlvo.variant.as_bytes().is_empty() {
            log::warn!("Layout not provided, but variant set to \"{}\": ignoring variant and using defaults for both: layout=\"{}\", variant=\"{}\".\n",
                    rmlvo.variant.to_str().unwrap_or(""),
                    rmlvo.layout.to_str().unwrap_or(""),
                    variant.to_str().unwrap_or(""));
        }
        rmlvo.variant = variant;
        modified = (modified as u32 | RMLVO_VARIANT) as Rmlvo;
    }
    if rmlvo.options.as_bytes().is_empty() {
        if ctx.use_environment_names {
            let env = xkb_context_getenv("XKB_DEFAULT_OPTIONS");
            rmlvo.options = match env {
                Ok(env) => std::ffi::CString::new(env).unwrap_or_default(),
                Err(_) => std::ffi::CString::new("").unwrap(),
            };
        } else {
            rmlvo.options = std::ffi::CString::new("").unwrap();
        };
        modified = (modified as u32 | RMLVO_OPTIONS) as Rmlvo;
    }
    modified
}

use super::shared_types::*;
pub(crate) const CONTROL_NAMES_MIN_V2_INDEX: u32 = 0;
pub(crate) const CONTROL_NAMES_MIN_V1_INDEX: u32 = 7;
pub(crate) const GROUP_LAST_INDEX_NAME: &str = "last";
#[inline]
pub(crate) fn format_control_names_offset(format: u32) -> u8 {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        CONTROL_NAMES_MIN_V1_INDEX as i32
    } else {
        CONTROL_NAMES_MIN_V2_INDEX as i32
    }) as u8
}
use super::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;

pub(crate) const XKB_KEYSYM_NAME_MAX_SIZE: i32 = 31;

pub use super::shared_types::{
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK,
    ACTION_TYPE_GROUP_SET, ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET,
    ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT,
    ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT,
    ACTION_TYPE_TERMINATE, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, CONTROL_ALL_BOOLEAN,
    CONTROL_ALL_BOOLEAN_V1, CONTROL_AX, CONTROL_AX_FEEDBACK, CONTROL_AX_TIMEOUT, CONTROL_BELL,
    CONTROL_DEBOUNCE, CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL,
    CONTROL_OVERLAY1, CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5,
    CONTROL_OVERLAY6, CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW,
    CONTROL_STICKY_KEYS, MATCH_ALL, MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE,
    XKB_MOD_NONE,
};
pub(crate) fn lookup_string(tab: &[LookupEntry], string: &str, value_rtrn: &mut u32) -> bool {
    if string.is_empty() {
        return false;
    }
    for entry in tab {
        if entry.name.is_empty() {
            break;
        }
        if entry.name.eq_ignore_ascii_case(string) {
            *value_rtrn = entry.value;
            return true;
        }
    }
    false
}
pub(crate) fn lookup_value(tab: &[LookupEntry], value: u32) -> &'static str {
    for entry in tab {
        if entry.name.is_empty() {
            break;
        }
        if entry.value == value {
            return entry.name;
        }
    }
    ""
}
pub(crate) static CTRL_MASK_NAMES: [LookupEntry; 25] = [
    LookupEntry {
        name: "Overlay3",
        value: CONTROL_OVERLAY3,
    },
    LookupEntry {
        name: "Overlay4",
        value: CONTROL_OVERLAY4,
    },
    LookupEntry {
        name: "Overlay5",
        value: CONTROL_OVERLAY5,
    },
    LookupEntry {
        name: "Overlay6",
        value: CONTROL_OVERLAY6,
    },
    LookupEntry {
        name: "Overlay7",
        value: CONTROL_OVERLAY7,
    },
    LookupEntry {
        name: "Overlay8",
        value: CONTROL_OVERLAY8,
    },
    LookupEntry {
        name: "all",
        value: CONTROL_ALL_BOOLEAN,
    },
    LookupEntry {
        name: "RepeatKeys",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "Repeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "AutoRepeat",
        value: CONTROL_REPEAT,
    },
    LookupEntry {
        name: "SlowKeys",
        value: CONTROL_SLOW,
    },
    LookupEntry {
        name: "BounceKeys",
        value: CONTROL_DEBOUNCE,
    },
    LookupEntry {
        name: "StickyKeys",
        value: CONTROL_STICKY_KEYS,
    },
    LookupEntry {
        name: "MouseKeys",
        value: CONTROL_MOUSE_KEYS,
    },
    LookupEntry {
        name: "MouseKeysAccel",
        value: CONTROL_MOUSE_KEYS_ACCEL,
    },
    LookupEntry {
        name: "AccessXKeys",
        value: CONTROL_AX,
    },
    LookupEntry {
        name: "AccessXTimeout",
        value: CONTROL_AX_TIMEOUT,
    },
    LookupEntry {
        name: "AccessXFeedback",
        value: CONTROL_AX_FEEDBACK,
    },
    LookupEntry {
        name: "AudibleBell",
        value: CONTROL_BELL,
    },
    LookupEntry {
        name: "IgnoreGroupLock",
        value: CONTROL_IGNORE_GROUP_LOCK,
    },
    LookupEntry {
        name: "Overlay1",
        value: CONTROL_OVERLAY1,
    },
    LookupEntry {
        name: "Overlay2",
        value: CONTROL_OVERLAY2,
    },
    LookupEntry {
        name: "all",
        value: CONTROL_ALL_BOOLEAN_V1,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) static MOD_COMPONENT_MASK_NAMES: [LookupEntry; 8] = [
    LookupEntry {
        name: "base",
        value: XKB_STATE_MODS_DEPRESSED,
    },
    LookupEntry {
        name: "latched",
        value: XKB_STATE_MODS_LATCHED,
    },
    LookupEntry {
        name: "locked",
        value: XKB_STATE_MODS_LOCKED,
    },
    LookupEntry {
        name: "effective",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "compat",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "any",
        value: XKB_STATE_MODS_EFFECTIVE,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) static GROUP_COMPONENT_MASK_NAMES: [LookupEntry; 7] = [
    LookupEntry {
        name: "base",
        value: XKB_STATE_LAYOUT_DEPRESSED,
    },
    LookupEntry {
        name: "latched",
        value: XKB_STATE_LAYOUT_LATCHED,
    },
    LookupEntry {
        name: "locked",
        value: XKB_STATE_LAYOUT_LOCKED,
    },
    LookupEntry {
        name: "effective",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: "any",
        value: XKB_STATE_LAYOUT_EFFECTIVE,
    },
    LookupEntry {
        name: "none",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) static BUTTON_NAMES: [LookupEntry; 7] = [
    LookupEntry {
        name: "Button1",
        value: 1_u32,
    },
    LookupEntry {
        name: "Button2",
        value: 2_u32,
    },
    LookupEntry {
        name: "Button3",
        value: 3_u32,
    },
    LookupEntry {
        name: "Button4",
        value: 4_u32,
    },
    LookupEntry {
        name: "Button5",
        value: 5_u32,
    },
    LookupEntry {
        name: "default",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) static USE_MOD_MAP_VALUE_NAMES: [LookupEntry; 5] = [
    LookupEntry {
        name: "LevelOne",
        value: 1_u32,
    },
    LookupEntry {
        name: "Level1",
        value: 1_u32,
    },
    LookupEntry {
        name: "AnyLevel",
        value: 0_u32,
    },
    LookupEntry {
        name: "any",
        value: 0_u32,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];

pub static ACTION_TYPE_NAMES: [LookupEntry; 43] = [
    LookupEntry {
        name: "NoAction",
        value: ACTION_TYPE_NONE,
    },
    LookupEntry {
        name: "VoidAction",
        value: ACTION_TYPE_VOID,
    },
    LookupEntry {
        name: "SetMods",
        value: ACTION_TYPE_MOD_SET,
    },
    LookupEntry {
        name: "LatchMods",
        value: ACTION_TYPE_MOD_LATCH,
    },
    LookupEntry {
        name: "LockMods",
        value: ACTION_TYPE_MOD_LOCK,
    },
    LookupEntry {
        name: "SetGroup",
        value: ACTION_TYPE_GROUP_SET,
    },
    LookupEntry {
        name: "LatchGroup",
        value: ACTION_TYPE_GROUP_LATCH,
    },
    LookupEntry {
        name: "LockGroup",
        value: ACTION_TYPE_GROUP_LOCK,
    },
    LookupEntry {
        name: "MovePtr",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: "MovePointer",
        value: ACTION_TYPE_PTR_MOVE,
    },
    LookupEntry {
        name: "PtrBtn",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: "PointerButton",
        value: ACTION_TYPE_PTR_BUTTON,
    },
    LookupEntry {
        name: "LockPtrBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPtrButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPointerButton",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "LockPointerBtn",
        value: ACTION_TYPE_PTR_LOCK,
    },
    LookupEntry {
        name: "SetPtrDflt",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: "SetPointerDefault",
        value: ACTION_TYPE_PTR_DEFAULT,
    },
    LookupEntry {
        name: "Terminate",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: "TerminateServer",
        value: ACTION_TYPE_TERMINATE,
    },
    LookupEntry {
        name: "SwitchScreen",
        value: ACTION_TYPE_SWITCH_VT,
    },
    LookupEntry {
        name: "SetControls",
        value: ACTION_TYPE_CTRL_SET,
    },
    LookupEntry {
        name: "LockControls",
        value: ACTION_TYPE_CTRL_LOCK,
    },
    LookupEntry {
        name: "RedirectKey",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: "Redirect",
        value: ACTION_TYPE_REDIRECT_KEY,
    },
    LookupEntry {
        name: "Private",
        value: ACTION_TYPE_PRIVATE,
    },
    LookupEntry {
        name: "ISOLock",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "ActionMessage",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "MessageAction",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "Message",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDeviceBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDevBtn",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDevButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "LockDeviceButton",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DeviceVal",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "DevValuator",
        value: ACTION_TYPE_UNSUPPORTED_LEGACY,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) static SYM_INTERPRET_MATCH_MASK_NAMES: [LookupEntry; 6] = [
    LookupEntry {
        name: "NoneOf",
        value: MATCH_NONE,
    },
    LookupEntry {
        name: "AnyOfOrNone",
        value: MATCH_ANY_OR_NONE,
    },
    LookupEntry {
        name: "AnyOf",
        value: MATCH_ANY,
    },
    LookupEntry {
        name: "AllOf",
        value: MATCH_ALL,
    },
    LookupEntry {
        name: "Exactly",
        value: MATCH_EXACTLY,
    },
    LookupEntry {
        name: "",
        value: 0_u32,
    },
];
pub(crate) fn mod_index_text<'a>(ctx: &'a XkbContext, mods: &XkbModSet, ndx: u32) -> &'a str {
    if ndx == XKB_MOD_INVALID {
        return "none";
    }
    if ndx == XKB_MOD_NONE {
        return "None";
    }
    if ndx >= mods.num_mods {
        return "";
    }
    atom_text(&ctx.atom_table, mods.mods[ndx as usize].name)
}
pub(crate) fn action_type_text(type_0: u32) -> &'static str {
    let name: &'static str = lookup_value(&ACTION_TYPE_NAMES, type_0);
    if !name.is_empty() {
        name
    } else {
        "Private"
    }
}
pub(crate) fn keysym_text(sym: u32) -> String {
    xkb_keysym_get_name(sym)
}
pub(crate) fn simatch_text(type_0: u32) -> &'static str {
    lookup_value(&SYM_INTERPRET_MATCH_MASK_NAMES, type_0)
}
pub(crate) fn mod_mask_text(ctx: &XkbContext, type_0: u32, mods: &XkbModSet, mask: u32) -> String {
    if mask == 0_u32 {
        return "none".to_string();
    }
    if mask == MOD_REAL_MASK_ALL {
        return "all".to_string();
    }
    if type_0 == MOD_REAL && mask & !MOD_REAL_MASK_ALL != 0
        || (mask as u64 & !(1_u64 << mods.num_mods).wrapping_sub(1_u64) != 0) as i32 as i64 != 0
    {
        return format!("{:#x}", mask);
    }
    let mut result = String::new();
    let mut remaining = mask;
    for i in 0..mods.num_mods as usize {
        if remaining == 0 {
            break;
        }
        if remaining & 0x1_u32 != 0 {
            if !result.is_empty() {
                result.push('+');
            }
            result.push_str(atom_text(&ctx.atom_table, mods.mods[i].name));
        }
        remaining >>= 1_i32;
    }
    result
}

use super::keysym::xkb_keysym_get_name;
// Rust-native wrapper types for XKB FFI structures
//
// These types use String instead of *const c_char for safer, more idiomatic Rust code.
// They provide conversion methods to/from the C FFI types.

use std::ffi::CString;

/// Rust-native version of XkbRuleNames
#[derive(Debug, Clone, Default)]
pub(crate) struct RuleNames {
    pub(crate) rules: String,
    pub(crate) model: String,
    pub(crate) layout: String,
    pub(crate) variant: String,
    pub(crate) options: String,
}

impl RuleNames {
    /// Create new RuleNames with all fields set to given values
    pub(crate) fn new(
        rules: String,
        model: String,
        layout: String,
        variant: String,
        options: String,
    ) -> Self {
        Self {
            rules,
            model,
            layout,
            variant,
            options,
        }
    }

    /// Create RuleNames for evdev with given layout and variant
    pub(crate) fn evdev(layout: String, variant: Option<String>) -> Self {
        Self {
            rules: "evdev".to_string(),
            model: "".to_string(),
            layout,
            variant: variant.unwrap_or_default(),
            options: "".to_string(),
        }
    }

    /// Convert to XkbRuleNames structure
    pub(crate) fn to_c_keymap(&self) -> super::shared_types::XkbRuleNames {
        use std::ffi::CString;
        super::shared_types::XkbRuleNames {
            rules: CString::new(self.rules.as_str()).unwrap(),
            model: CString::new(self.model.as_str()).unwrap(),
            layout: CString::new(self.layout.as_str()).unwrap(),
            variant: CString::new(self.variant.as_str()).unwrap(),
            options: CString::new(self.options.as_str()).unwrap(),
        }
    }
}

// ============================================================================
// Unicode Preprocessing
// ============================================================================

/// Convert non-ASCII characters in XKB keymap strings to UXXXX keysym notation.
///
/// The XKB scanner only accepts ASCII identifiers. When a keymap contains raw
/// Unicode characters as keysym names (e.g., `ㄙ` instead of `U3119`), this
/// function converts them so the parser can handle them.
///
/// Characters inside strings (`"..."`), comments (`//` or `/* */`), and key
/// names (`<...>`) are left untouched.
fn preprocess_unicode_keysyms(input: &str) -> std::borrow::Cow<'_, str> {
    use std::borrow::Cow;
    use std::fmt::Write;
    // Fast path: if there are no non-ASCII bytes, return as-is.
    if input.is_ascii() {
        return Cow::Borrowed(input);
    }

    let mut result = String::with_capacity(input.len() + 64);
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_keyname = false;
    let mut prev_char = '\0';

    while let Some(ch) = chars.next() {
        if in_line_comment {
            result.push(ch);
            if ch == '\n' {
                in_line_comment = false;
            }
            prev_char = ch;
            continue;
        }

        if in_block_comment {
            result.push(ch);
            if prev_char == '*' && ch == '/' {
                in_block_comment = false;
            }
            prev_char = ch;
            continue;
        }

        if in_string {
            result.push(ch);
            if ch == '"' && prev_char != '\\' {
                in_string = false;
            }
            prev_char = ch;
            continue;
        }

        if in_keyname {
            result.push(ch);
            if ch == '>' {
                in_keyname = false;
            }
            prev_char = ch;
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                result.push(ch);
            }
            '/' if chars.peek() == Some(&'/') => {
                in_line_comment = true;
                result.push(ch);
            }
            '/' if chars.peek() == Some(&'*') => {
                in_block_comment = true;
                result.push(ch);
            }
            '<' => {
                in_keyname = true;
                result.push(ch);
            }
            c if !c.is_ascii() => {
                let cp = c as u32;
                if cp <= 0xFFFF {
                    write!(result, "U{:04X}", cp).unwrap();
                } else {
                    write!(result, "U{:05X}", cp).unwrap();
                }
            }
            _ => result.push(ch),
        }
        prev_char = ch;
    }

    Cow::Owned(result)
}

// ============================================================================
// Safe RAII Wrappers for XKB FFI Types
// ============================================================================

/// Safe wrapper around XkbContext with automatic cleanup
#[derive(Clone)]
pub(crate) struct Context {
    entity: XkbContext,
}

impl Context {
    /// Create a new XKB context
    pub(crate) fn new() -> Option<Self> {
        use super::shared_types::XKB_CONTEXT_NO_FLAGS;
        let ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        Some(Context { entity: ctx })
    }

    /// Create a keymap from RMLVO names. Consumes the context.
    pub(crate) fn keymap_from_names(self, rules: &RuleNames) -> Option<Keymap> {
        use super::shared_types::XKB_KEYMAP_COMPILE_NO_FLAGS;

        let rmlvo_c = rules.to_c_keymap();
        let keymap =
            xkb_keymap_new_from_names(self.entity, Some(&rmlvo_c), XKB_KEYMAP_COMPILE_NO_FLAGS)?;
        Some(Keymap { inner: keymap })
    }

    /// Create a keymap from a keymap string. Consumes the context.
    pub(crate) fn keymap_from_string(self, keymap_str: &str) -> Option<Keymap> {
        use super::shared_types::{XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_FORMAT_TEXT_V1};

        let processed = preprocess_unicode_keysyms(keymap_str);
        let keymap_cstr = CString::new(processed.as_ref()).ok()?;
        let keymap = xkb_keymap_new_from_string(
            self.entity,
            &keymap_cstr,
            XKB_KEYMAP_FORMAT_TEXT_V1,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        )?;
        Some(Keymap { inner: keymap })
    }
}

/// Safe wrapper around XkbKeymap with automatic cleanup
#[derive(Clone)]
pub(crate) struct Keymap {
    inner: Rc<super::shared_types::XkbKeymap>,
}

impl std::fmt::Debug for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keymap")
            .field("min_key", &self.inner.min_key_code)
            .field("max_key", &self.inner.max_key_code)
            .finish()
    }
}

impl Keymap {
    /// Get minimum keycode
    pub(crate) fn min_keycode(&self) -> u32 {
        xkb_keymap_min_keycode(&self.inner)
    }

    /// Get maximum keycode
    pub(crate) fn max_keycode(&self) -> u32 {
        xkb_keymap_max_keycode(&self.inner)
    }

    /// Get number of levels for a key
    pub(crate) fn num_levels_for_key(&self, keycode: u32, layout: u32) -> u32 {
        xkb_keymap_num_levels_for_key(&self.inner, keycode, layout)
    }

    /// Get keysyms for a key at a specific level (safe via get_key_level)
    pub(crate) fn key_get_syms_by_level(&self, keycode: u32, layout: u32, level: u32) -> &[u32] {
        if let Some(key) = self.inner.get_key(keycode) {
            if let Some(leveli) = self.inner.get_key_level(key, layout, level) {
                if !leveli.syms.is_empty() {
                    return &leveli.syms[..];
                }
            }
        }
        &[]
    }

    /// Get number of modifiers in the keymap
    pub(crate) fn num_mods(&self) -> u32 {
        xkb_keymap_num_mods(&self.inner)
    }

    /// Get modifier name by index
    pub(crate) fn mod_get_name(&self, idx: u32) -> Option<String> {
        xkb_keymap_mod_get_name(&self.inner, idx).map(|s| s.to_string())
    }

    /// Get modifier mask by name (safe via atom_lookup_ref)
    pub(crate) fn mod_get_mask(&self, name: &str) -> u32 {
        let idx = xkb_keymap_mod_get_index_ref(&self.inner, name);
        if idx >= self.inner.mods.num_mods {
            0_u32
        } else {
            self.inner.mods.mods[idx as usize].mapping
        }
    }

    /// Check if a key can repeat
    pub(crate) fn key_repeats(&self, keycode: u32) -> bool {
        xkb_keymap_key_repeats(&self.inner, keycode) != 0
    }

    /// Get modifier maps for a key (returns (modmap, vmodmap) or None if key doesn't exist)
    pub(crate) fn key_get_mods(&self, keycode: u32) -> Option<(u32, u32)> {
        let key = self.inner.get_key(keycode)?;
        Some((key.modmap, key.vmodmap))
    }

    /// Iterate over all keycodes in the keymap
    ///
    /// Returns an iterator that yields (keycode, evdev_code) pairs.
    /// evdev_code is keycode - 8 (the standard offset for evdev)
    pub(crate) fn keycodes(&self) -> KeycodeIter {
        KeycodeIter {
            current: self.min_keycode(),
            max: self.max_keycode(),
            evdev_offset: 8,
        }
    }

    /// Create a new state for this keymap
    pub(crate) fn new_state(&self) -> Option<State> {
        let state = xkb_state_new(self.inner.clone());
        Some(State { inner: state })
    }

    /// Get number of layouts in the keymap
    pub(crate) fn num_layouts(&self) -> u32 {
        xkb_keymap_num_layouts(&self.inner)
    }

    /// Get layout name by index
    pub(crate) fn layout_get_name(&self, idx: u32) -> Option<String> {
        xkb_keymap_layout_get_name(&self.inner, idx).map(|s| s.to_string())
    }

    /// Get layout index by name (safe via atom_lookup_ref)
    pub(crate) fn layout_get_index(&self, name: &str) -> Option<u32> {
        let idx = xkb_keymap_layout_get_index_ref(&self.inner, name);
        if idx == XKB_LAYOUT_INVALID {
            None
        } else {
            Some(idx)
        }
    }

    /// Get number of LEDs in the keymap
    pub(crate) fn num_leds(&self) -> u32 {
        xkb_keymap_num_leds(&self.inner)
    }

    /// Get LED name by index
    pub(crate) fn led_get_name(&self, idx: u32) -> Option<String> {
        xkb_keymap_led_get_name(&self.inner, idx).map(|s| s.to_string())
    }

    /// Get LED index by name (safe via atom_lookup_ref)
    pub(crate) fn led_get_index(&self, name: &str) -> Option<u32> {
        let idx = xkb_keymap_led_get_index_ref(&self.inner, name);
        if idx == XKB_LED_INVALID {
            None
        } else {
            Some(idx)
        }
    }



    /// Get modifier index by name (safe via atom_lookup_ref)
    pub(crate) fn mod_get_index(&self, name: &str) -> Option<u32> {
        let idx = xkb_keymap_mod_get_index_ref(&self.inner, name);
        if idx == XKB_MOD_INVALID {
            None
        } else {
            Some(idx)
        }
    }

    /// Get all layout names as a Vec
    pub(crate) fn get_all_layouts(&self) -> Vec<String> {
        let num_layouts = self.num_layouts();
        (0..num_layouts)
            .filter_map(|idx| self.layout_get_name(idx))
            .collect()
    }

    /// Get all modifier names as a Vec
    pub(crate) fn get_all_mods(&self) -> Vec<String> {
        let num_mods = self.num_mods();
        (0..num_mods)
            .filter_map(|idx| self.mod_get_name(idx))
            .collect()
    }

    /// Get all LED names as a Vec
    pub(crate) fn get_all_leds(&self) -> Vec<String> {
        let num_leds = self.num_leds();
        (0..num_leds)
            .filter_map(|idx| self.led_get_name(idx))
            .collect()
    }
}

/// Iterator over keycode ranges in a keymap
pub(crate) struct KeycodeIter {
    current: u32,
    max: u32,
    evdev_offset: u32,
}

impl Iterator for KeycodeIter {
    type Item = (u32, u32); // (xkb_keycode, evdev_code)

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.max {
            let keycode = self.current;
            let evdev_code = keycode - self.evdev_offset;
            self.current += 1;
            Some((keycode, evdev_code))
        } else {
            None
        }
    }
}

impl ExactSizeIterator for KeycodeIter {
    fn len(&self) -> usize {
        if self.current <= self.max {
            (self.max - self.current + 1) as usize
        } else {
            0
        }
    }
}

/// Safe wrapper around XkbState with automatic cleanup
///
/// Owns the XkbState via Box. The state was originally allocated by
/// `xkb_state_new` (which uses `Box::into_raw`), and we reclaim it
/// via `Box::from_raw` in `Keymap::new_state`.
pub(crate) struct State {
    inner: Box<XkbState>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish_non_exhaustive()
    }
}

impl State {
    /// Update key state (press or release)
    pub(crate) fn update_key(
        &mut self,
        keycode: u32,
        direction: super::shared_types::XkbKeyDirection,
    ) {
        xkb_state_update_key(&mut self.inner, keycode, direction);
    }

    /// Get keysym for a key in the current state
    pub(crate) fn key_get_one_sym(&self, keycode: u32) -> u32 {
        xkb_state_key_get_one_sym(&self.inner, keycode)
    }

    /// Get all keysyms for a key in the current state
    pub(crate) fn key_get_syms(&self, keycode: u32) -> &[u32] {
        xkb_state_key_get_syms(&self.inner, keycode)
    }

    /// Check if a modifier is active
    pub(crate) fn mod_name_is_active(&self, name: &str, state_type: u32) -> bool {
        xkb_state_mod_name_is_active(&self.inner, name, state_type) > 0
    }

    /// Check if a modifier index is active
    pub(crate) fn mod_index_is_active(&self, idx: u32, state_type: u32) -> bool {
        xkb_state_mod_index_is_active(&self.inner, idx, state_type) > 0
    }

    /// Get keysyms for a key at a specific layout and level (delegates to keymap)
    pub(crate) fn key_get_syms_by_level(&self, keycode: u32, layout: u32, level: u32) -> &[u32] {
        let keymap = self.inner.keymap();
        if let Some(key) = keymap.get_key(keycode) {
            if let Some(leveli) = keymap.get_key_level(key, layout, level) {
                if !leveli.syms.is_empty() {
                    return &leveli.syms[..];
                }
            }
        }
        &[]
    }

    /// Get the number of layouts in the underlying keymap
    pub(crate) fn num_keymap_layouts(&self) -> u32 {
        xkb_keymap_num_layouts(self.inner.keymap())
    }

    /// Update state from modifier/layout masks (e.g., from Wayland compositor)
    pub(crate) fn update_mask(
        &mut self,
        depressed_mods: u32,
        latched_mods: u32,
        locked_mods: u32,
        depressed_layout: u32,
        latched_layout: u32,
        locked_layout: u32,
    ) -> u32 {
        xkb_state_update_mask(
            &mut self.inner,
            depressed_mods,
            latched_mods,
            locked_mods,
            depressed_layout,
            latched_layout,
            locked_layout,
        )
    }
}

impl Drop for State {
    fn drop(&mut self) {
        // Rc<XkbKeymap> inside the state drops automatically.
        // Nothing else to clean up — Box<XkbState> handles deallocation.
    }
}

// ============================================================================
// Registry (rxkb) Wrappers for Layout Enumeration
// ============================================================================

/// Safe wrapper around RxkbContext for keyboard layout registry
pub(crate) struct RegistryContext {
    inner: Box<RxkbContext>,
}

impl RegistryContext {
    /// Create a new registry context
    pub(crate) fn new() -> Option<Self> {
        let inner = RxkbContext::new(RXKB_CONTEXT_NO_FLAGS)?;
        Some(RegistryContext { inner })
    }

    /// Load default registry paths
    pub(crate) fn include_path_append_default(&mut self) {
        self.inner.include_path_append_default();
    }

    /// Parse the registry for the given ruleset (typically "evdev")
    pub(crate) fn parse(&mut self, ruleset: &str) -> bool {
        self.inner.parse(ruleset)
    }

    /// Iterate over all layouts in the registry
    pub(crate) fn layouts(&self) -> impl Iterator<Item = &RxkbLayout> {
        self.inner.layouts().iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_names_evdev() {
        let names = RuleNames::evdev("us".to_string(), None);
        assert_eq!(names.rules, "evdev");
        assert_eq!(names.layout, "us");
        assert_eq!(names.variant, "");
    }

    #[test]
    fn test_rule_names_to_c() {
        let names = RuleNames::evdev("us".to_string(), Some("dvorak".to_string()));
        let c_struct = names.to_c_keymap();

        assert_eq!(c_struct.rules.to_str().unwrap(), "evdev");
        assert_eq!(c_struct.layout.to_str().unwrap(), "us");
        assert_eq!(c_struct.variant.to_str().unwrap(), "dvorak");
    }

    #[test]
    fn test_context_new() {
        let ctx = Context::new();
        assert!(ctx.is_some());
    }

    #[test]
    fn test_context_keymap() {
        let ctx = Context::new().expect("Failed to create context");
        let rules = RuleNames::evdev("us".to_string(), None);
        let keymap = ctx.keymap_from_names(&rules);
        assert!(keymap.is_some());
    }

    #[test]
    fn test_keymap_keycodes() {
        let ctx = Context::new().expect("Failed to create context");
        let rules = RuleNames::evdev("us".to_string(), None);
        let keymap = ctx
            .keymap_from_names(&rules)
            .expect("Failed to create keymap");

        let min = keymap.min_keycode();
        let max = keymap.max_keycode();
        assert!(min < max);
        assert!(min >= 8); // evdev starts at 8
    }
}

#[derive(Copy, Clone)]

pub(crate) struct XkbEvent {
    pub(crate) type_0: XkbEventType,
    pub(crate) data: XkbEventData,
}
#[derive(Copy, Clone)]

pub(crate) enum XkbEventData {
    Keycode(u32),
    Components(XkbEventComponents),
}
#[derive(Copy, Clone)]

pub(crate) struct XkbEventComponents {
    pub(crate) components: StateComponents,
    pub(crate) changed: u32,
}
#[derive(Copy, Clone)]

pub(crate) struct StateComponents {
    pub(crate) base_group: i32,
    pub(crate) latched_group: i32,
    pub(crate) locked_group: i32,
    pub(crate) group: u32,
    pub(crate) base_mods: u32,
    pub(crate) latched_mods: u32,
    pub(crate) locked_mods: u32,
    pub(crate) mods: u32,
    pub(crate) leds: XkbLedMaskT,
    pub(crate) controls: XkbActionControls,
}
#[derive(Copy, Clone)]

pub(crate) struct XkbStateUpdateV1 {
    pub(crate) size: usize,
    pub(crate) components: *const XkbStateComponentsUpdateV1,
    pub(crate) layout_policy: *const XkbLayoutPolicyUpdateV1,
}
#[derive(Copy, Clone)]

pub(crate) struct XkbLayoutPolicyUpdateV1 {
    pub(crate) size: usize,
    pub(crate) policy: u32,
    pub(crate) redirect: u32,
}
#[derive(Copy, Clone)]

pub(crate) struct XkbStateComponentsUpdateV1 {
    pub(crate) size: usize,
    pub(crate) components: u32,
    pub(crate) affect_latched_mods: u32,
    pub(crate) latched_mods: u32,
    pub(crate) affect_locked_mods: u32,
    pub(crate) locked_mods: u32,
    pub(crate) latched_layout: i32,
    pub(crate) locked_layout: i32,
    pub(crate) affect_controls: XkbKeyboardControlFlags,
    pub(crate) controls: XkbKeyboardControlFlags,
}
use super::shared_types::{XkbActionControls, XkbEventType, XkbKeyboardControlFlags, XkbLedMaskT};

fn vec_resize_zero<T: Default>(v: &mut Vec<T>, new_len: usize) {
    v.resize_with(new_len, Default::default);
}

#[derive(Clone)]

pub(crate) struct XkbState {
    pub(crate) components: StateComponents,
    pub(crate) controls: MachineControls,
    pub(crate) set_mods: u32,
    pub(crate) clear_mods: u32,
    pub(crate) mod_key_count: [i16; 32],
    pub(crate) flags: XkbA11yFlags,
    pub(crate) refcnt: i32,
    pub(crate) filters: Vec<XkbFilter>,
    pub(crate) keymap: Rc<XkbKeymap>,
}

impl XkbState {
    /// Safe accessor for the keymap reference.
    #[inline]
    pub(crate) fn keymap(&self) -> &XkbKeymap {
        &self.keymap
    }
}
// C2Rust_Unnamed_15 removed: replaced by Vec<XkbFilter>
#[derive(Copy, Clone)]

pub(crate) struct XkbFilter {
    pub(crate) action: XkbAction,
    pub(crate) key: *const XkbKey,
    pub(crate) func:
        Option<fn(&mut XkbState, &mut XkbEvents, &mut XkbFilter, &XkbKey, XkbKeyDirection) -> bool>,
    pub(crate) priv_0: u64,
    pub(crate) refcnt: i32,
}
impl Default for XkbFilter {
    fn default() -> Self {
        Self {
            action: XkbAction::default(),
            key: std::ptr::null(),
            func: None,
            priv_0: 0u64,
            refcnt: 0,
        }
    }
}
#[derive(Clone)]

pub(crate) struct XkbEvents {
    pub(crate) next: u32,
    pub(crate) queue: Vec<XkbEvent>,
    pub(crate) ctx: XkbContext,
}
impl XkbEvents {
    fn dummy() -> Self {
        Self {
            next: 0,
            queue: Vec::new(),
            ctx: xkb_context_new(0),
        }
    }
}
// C2Rust_Unnamed_16 removed: replaced by Vec<XkbEvent>
#[derive(Copy, Clone)]

pub(crate) struct MachineControls {
    pub(crate) out_of_range_group_policy: u32,
    pub(crate) out_of_range_redirect_group: u32,
}

#[derive(Copy, Clone)]

pub(crate) struct FilterActionFuncs {
    pub(crate) new: Option<fn(&mut XkbState, &mut XkbEvents, &mut XkbFilter) -> ()>,
    pub(crate) func:
        Option<fn(&mut XkbState, &mut XkbEvents, &mut XkbFilter, &XkbKey, XkbKeyDirection) -> bool>,
}

pub(crate) const XKB_FILTER_CONSUME: XkbFilterResult = 0;

pub(crate) const XKB_FILTER_CONTINUE: XkbFilterResult = 1;

/// Pack latch state and group delta into a u64 (replacing the old union group_latch_priv).
#[inline]
fn pack_group_latch(latch: u32, group_delta: i32) -> u64 {
    (latch as u64) | ((group_delta as u32 as u64) << 32)
}

/// Unpack latch state from a u64.
#[inline]
fn unpack_group_latch_state(packed: u64) -> u32 {
    packed as u32
}

/// Unpack group delta from a u64.
#[inline]
fn unpack_group_delta(packed: u64) -> i32 {
    (packed >> 32) as u32 as i32
}

pub(crate) type XkbKeyLatchState = u32;

pub(crate) const _KEY_LATCH_STATE_NUM_ENTRIES: XkbKeyLatchState = 3;

pub(crate) const LATCH_PENDING: XkbKeyLatchState = 2;

pub(crate) const LATCH_KEY_DOWN: XkbKeyLatchState = 1;

pub(crate) const NO_LATCH: XkbKeyLatchState = 0;

pub(crate) const XKB_STATE_MATCH_FLAGS: u32 = 65539;

pub(crate) type XkbFilterResult = u32;

use std::sync::LazyLock;

static SYNTHETIC_KEY_BREAK_GROUP_LATCH: LazyLock<XkbKey> = LazyLock::new(|| XkbKey {
    keycode: 0,
    name: 0,
    explicit: 0 as XkbExplicitComponents,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    repeats: false,
    implicit_actions: false,
    out_of_range_pending_group: false,
    out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    out_of_range_group_number: 0,
    num_groups: 1,
    groups: vec![XkbGroup {
        explicit_symbols: false,
        explicit_actions: false,
        implicit_actions: false,
        explicit_type: false,
        type_idx: 0,
        levels: vec![XkbLevel {
            upper: XKB_KEY_NO_SYMBOL as u32,
            has_upper: false,
            syms: Vec::new(),
            actions: vec![XkbAction::Internal(XkbInternalAction {
                flags: INTERNAL_BREAKS_GROUP_LATCH,
                clear_latched_mods: 0,
            })],
        }],
    }],
    overlay_keys: Vec::new(),
});

fn get_entry_for_mods(type_0: &XkbKeyType, mods: u32) -> Option<&XkbKeyTypeEntry> {
    type_0
        .entries
        .iter()
        .find(|&entry| entry_is_active(entry) && entry.mods.mask == mods)
        .map(|v| v as _)
}

fn get_entry_for_key_state<'a>(
    state: &'a XkbState,
    key: &XkbKey,
    group: u32,
) -> Option<&'a XkbKeyTypeEntry> {
    let keymap = state.keymap();
    let type_0 = &keymap.types[key.groups[group as usize].type_idx as usize];
    let active_mods: u32 = state.components.mods & type_0.mods.mask;
    get_entry_for_mods(type_0, active_mods)
}
#[inline]
fn state_key_get_level(state: &XkbState, key: &XkbKey, layout: u32) -> u32 {
    if layout >= key.num_groups {
        return XKB_LEVEL_INVALID;
    }
    match get_entry_for_key_state(state, key, layout) {
        Some(entry) => entry.level,
        None => 0_u32,
    }
}

pub(crate) fn xkb_state_key_get_level(state: &XkbState, kc: u32, layout: u32) -> u32 {
    match state.keymap().get_key(kc) {
        Some(key) => state_key_get_level(state, key, layout),
        None => XKB_LEVEL_INVALID,
    }
}
#[inline]
fn state_key_get_layout(state: &XkbState, key: &XkbKey) -> u32 {
    xkb_wrap_group_into_range(
        state.components.group as i32,
        key.num_groups,
        key.out_of_range_group_policy,
        key.out_of_range_group_number,
    )
}

pub(crate) fn xkb_state_key_get_layout(state: &XkbState, kc: u32) -> u32 {
    match state.keymap().get_key(kc) {
        Some(key) => state_key_get_layout(state, key),
        None => XKB_LAYOUT_INVALID,
    }
}

static DUMMY_ACTION: XkbAction = XkbAction::None;

fn xkb_key_get_actions<'a>(state: &'a XkbState, key: &'a XkbKey) -> &'a [XkbAction] {
    let layout: u32 = state_key_get_layout(state, key);
    let level: u32 = state_key_get_level(state, key, layout);
    if level != XKB_LEVEL_INVALID {
        let wrapped_layout = xkb_wrap_group_into_range(
            layout as i32,
            key.num_groups,
            key.out_of_range_group_policy,
            key.out_of_range_group_number,
        );
        if wrapped_layout != XKB_LAYOUT_INVALID {
            let keymap = state.keymap();
            if level < keymap.key_num_levels(key, wrapped_layout) {
                let actions = &key.groups[wrapped_layout as usize].levels[level as usize].actions;
                if !actions.is_empty() {
                    return actions.as_slice();
                }
            }
        }
    }
    std::slice::from_ref(&DUMMY_ACTION)
}

fn xkb_filter_new(state: &mut XkbState) -> usize {
    let filters = &mut state.filters;
    for (i, filter) in filters.iter_mut().enumerate() {
        if filter.func.is_none() {
            filter.refcnt = 1;
            return i;
        }
    }
    let new_len = state.filters.len().wrapping_add(1);
    vec_resize_zero(&mut state.filters, new_len);
    let last = state.filters.len() - 1;
    state.filters[last].refcnt = 1;
    last
}

fn xkb_filter_group_set_new(state: &mut XkbState, _events: &mut XkbEvents, filter: &mut XkbFilter) {
    filter.priv_0 = state.components.base_group as u32 as u64;
    if filter.action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
        state.components.base_group = filter.action.as_group().group;
    } else {
        state.components.base_group += filter.action.as_group().group;
    };
}

fn xkb_filter_group_set_func(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        filter.action.as_group_mut().flags =
            (filter.action.as_group().flags & !(ACTION_LOCK_CLEAR as i32) as u32) as XkbActionFlags;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    's_38: {
        match direction {
            1 => {
                filter.refcnt += 1;
            }
            2 => {}
            _ => {
                filter.refcnt -= 1;
                if filter.refcnt > 0_i32 {
                    return XKB_FILTER_CONSUME as i32 != 0;
                }
                break 's_38;
            }
        }
        return XKB_FILTER_CONSUME as i32 != 0;
    }
    state.components.base_group = filter.priv_0 as i32;
    if filter.action.as_group().flags & ACTION_LOCK_CLEAR != 0 {
        state.components.locked_group = 0_i32;
    }
    filter.func = None;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn get_state_component_changes(a: &StateComponents, b: &StateComponents) -> u32 {
    let mut mask: u32 = 0_u32;
    if a.group != b.group {
        mask |= XKB_STATE_LAYOUT_EFFECTIVE;
    }
    if a.base_group != b.base_group {
        mask |= XKB_STATE_LAYOUT_DEPRESSED;
    }
    if a.latched_group != b.latched_group {
        mask |= XKB_STATE_LAYOUT_LATCHED;
    }
    if a.locked_group != b.locked_group {
        mask |= XKB_STATE_LAYOUT_LOCKED;
    }
    if a.mods != b.mods {
        mask |= XKB_STATE_MODS_EFFECTIVE;
    }
    if a.base_mods != b.base_mods {
        mask |= XKB_STATE_MODS_DEPRESSED;
    }
    if a.latched_mods != b.latched_mods {
        mask |= XKB_STATE_MODS_LATCHED;
    }
    if a.locked_mods != b.locked_mods {
        mask |= XKB_STATE_MODS_LOCKED;
    }
    if a.leds != b.leds {
        mask |= XKB_STATE_LEDS;
    }
    if a.controls != b.controls {
        mask |= XKB_STATE_CONTROLS;
    }
    mask
}

fn xkb_filter_group_lock_new(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
) {
    if filter.action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0 {
    } else if filter.action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
        state.components.locked_group = filter.action.as_group().group;
    } else {
        state.components.locked_group += filter.action.as_group().group;
    }
}

fn xkb_filter_group_lock_func(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        if filter.action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0 && direction == XKB_KEY_DOWN
        {
            filter.action.as_group_mut().flags = (filter.action.as_group().flags
                & !(ACTION_LOCK_ON_RELEASE as i32) as u32)
                as XkbActionFlags;
        }
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    's_47: {
        match direction {
            1 => {
                filter.refcnt += 1;
            }
            2 => {}
            _ => {
                filter.refcnt -= 1;
                if filter.refcnt > 0_i32 {
                    return XKB_FILTER_CONSUME as i32 != 0;
                }
                break 's_47;
            }
        }
        return XKB_FILTER_CONSUME as i32 != 0;
    }
    if filter.action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0 {
        if filter.action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
            state.components.locked_group = filter.action.as_group().group;
        } else {
            state.components.locked_group += filter.action.as_group().group;
        }
    }
    filter.func = None;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn xkb_action_breaks_latch(action: &XkbAction, flag: u32, mask: u32) -> bool {
    match action.action_type() {
        0 | 1 | 9 | 10 | 14 | 15 | 13 | 12 | 16 => true,
        20 => {
            action.as_internal().flags & flag != 0
                && action.as_internal().clear_latched_mods & mask == mask
        }
        _ => false,
    }
}

fn xkb_filter_group_latch_new(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
) {
    let group_delta = if filter.action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
        filter.action.as_group().group - state.components.base_group
    } else {
        filter.action.as_group().group
    };
    filter.priv_0 = pack_group_latch(LATCH_KEY_DOWN, group_delta);
    if filter.action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
        state.components.base_group = filter.action.as_group().group;
    } else {
        state.components.base_group += filter.action.as_group().group;
    };
}

fn xkb_filter_group_latch_func(
    state: &mut XkbState,
    events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    let group_delta = unpack_group_delta(filter.priv_0);
    let mut latch: XkbKeyLatchState = unpack_group_latch_state(filter.priv_0) as XkbKeyLatchState;
    if direction == XKB_KEY_DOWN {
        let actions = xkb_key_get_actions(state, key);
        if latch as u32 == LATCH_KEY_DOWN {
            if state.flags & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS != 0 {
                let mut k: u16 = 0_u16;
                while (k as usize) < actions.len() {
                    if xkb_action_breaks_latch(
                        &actions[k as usize],
                        INTERNAL_BREAKS_GROUP_LATCH,
                        0_u32,
                    ) {
                        latch = NO_LATCH;
                        break;
                    } else {
                        k = k.wrapping_add(1);
                    }
                }
            } else {
                latch = NO_LATCH;
            }
        } else if latch as u32 == LATCH_PENDING {
            let sticky_keys: bool = state.components.controls & CONTROL_STICKY_KEYS != 0;
            let flags: XkbActionFlags = (filter.action.as_group().flags
                & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                as XkbActionFlags;
            let mut k_0: u16 = 0_u16;
            while (k_0 as usize) < actions.len() {
                if actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_GROUP_LATCH
                    && actions[k_0 as usize].as_group().group == filter.action.as_group().group
                    && actions[k_0 as usize].as_group().flags as u32
                        == filter.action.as_group().flags
                    || actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_GROUP_SET
                        && sticky_keys as i32 != 0
                        && actions[k_0 as usize].as_group().flags as u32 == flags as u32
                {
                    if filter.action.as_group().flags & ACTION_LATCH_TO_LOCK != 0
                        && filter.action.as_group().group != 0_i32
                    {
                        let group_data = *filter.action.as_group();
                        filter.action = XkbAction::GroupLock(group_data);
                        filter.func = Some(xkb_filter_group_lock_func);
                        xkb_filter_group_lock_new(state, events, filter);
                        state.components.latched_group -= group_delta;
                        filter.key = key;
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                } else if xkb_action_breaks_latch(
                    &actions[k_0 as usize],
                    INTERNAL_BREAKS_GROUP_LATCH,
                    0_u32,
                ) {
                    state.components.latched_group -= group_delta;
                    filter.func = None;
                    return XKB_FILTER_CONTINUE as i32 != 0;
                }
                k_0 = k_0.wrapping_add(1);
            }
        }
    } else if std::ptr::eq(key, filter.key) {
        if direction == XKB_KEY_REPEATED {
            return XKB_FILTER_CONSUME as i32 != 0;
        } else if filter.action.as_group().flags & ACTION_LOCK_CLEAR != 0
            && state.components.locked_group != 0
        {
            if latch as u32 == LATCH_PENDING {
                state.components.latched_group -= group_delta;
            } else {
                state.components.base_group -= group_delta;
            }
            state.components.locked_group = 0_i32;
            filter.func = None;
        } else if latch as u32 == NO_LATCH {
            state.components.base_group -= group_delta;
            filter.func = None;
        } else if latch as u32 == LATCH_KEY_DOWN {
            latch = LATCH_PENDING;
            state.components.base_group -= group_delta;
            state.components.latched_group += group_delta;
        }
    }
    filter.priv_0 = pack_group_latch(latch as u32, group_delta);
    XKB_FILTER_CONTINUE as i32 != 0
}

fn xkb_filter_mod_set_new(state: &mut XkbState, _events: &mut XkbEvents, filter: &mut XkbFilter) {
    let unlock: XkbActionFlags =
        (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LOCK_CLEAR as i32) as XkbActionFlags;
    if filter.action.as_mods().flags & unlock as u32 == unlock as u32 {
        filter.priv_0 = (filter.action.as_mods().mods.mask & !state.components.locked_mods) as u64;
        state.components.locked_mods &= !filter.action.as_mods().mods.mask;
    } else {
        filter.priv_0 = filter.action.as_mods().mods.mask as u64;
    }
    state.set_mods |= filter.priv_0 as u32;
}

fn xkb_filter_mod_set_func(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        filter.action.as_mods_mut().flags =
            (filter.action.as_mods().flags & !(ACTION_LOCK_CLEAR as i32) as u32) as XkbActionFlags;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    's_38: {
        match direction {
            1 => {
                filter.refcnt += 1;
            }
            2 => {}
            _ => {
                filter.refcnt -= 1;
                if filter.refcnt > 0_i32 {
                    return XKB_FILTER_CONSUME as i32 != 0;
                }
                break 's_38;
            }
        }
        return XKB_FILTER_CONSUME as i32 != 0;
    }
    state.clear_mods |= filter.priv_0 as u32;
    let unlock: XkbActionFlags =
        (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LOCK_CLEAR as i32) as XkbActionFlags;
    if filter.action.as_mods().flags & unlock as u32 == ACTION_LOCK_CLEAR {
        state.components.locked_mods &= !filter.action.as_mods().mods.mask;
    }
    filter.func = None;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn xkb_filter_mod_lock_new(state: &mut XkbState, _events: &mut XkbEvents, filter: &mut XkbFilter) {
    filter.priv_0 = (state.components.locked_mods & filter.action.as_mods().mods.mask) as u64;
    if filter.priv_0 != 0 && filter.action.as_mods().flags & ACTION_UNLOCK_ON_PRESS != 0 {
        if filter.action.as_mods().flags & ACTION_LOCK_NO_UNLOCK == 0 {
            state.components.locked_mods &= !(filter.priv_0 as u32);
        }
        filter.func = None;
    } else {
        state.set_mods |= filter.action.as_mods().mods.mask;
        if filter.action.as_mods().flags & ACTION_LOCK_NO_LOCK == 0 {
            state.components.locked_mods |= filter.action.as_mods().mods.mask;
        }
    };
}

fn xkb_filter_mod_lock_func(
    state: &mut XkbState,
    _events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    's_32: {
        match direction {
            1 => {
                filter.refcnt += 1;
            }
            2 => {}
            _ => {
                filter.refcnt -= 1;
                if filter.refcnt > 0_i32 {
                    return XKB_FILTER_CONSUME as i32 != 0;
                }
                break 's_32;
            }
        }
        return XKB_FILTER_CONSUME as i32 != 0;
    }
    state.clear_mods |= filter.action.as_mods().mods.mask;
    if filter.action.as_mods().flags & ACTION_LOCK_NO_UNLOCK == 0 {
        state.components.locked_mods &= !(filter.priv_0 as u32);
    }
    filter.func = None;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn xkb_filter_mod_latch_new(state: &mut XkbState, _events: &mut XkbEvents, filter: &mut XkbFilter) {
    let unlock_on_press: XkbActionFlags =
        (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LATCH_ON_PRESS as i32) as XkbActionFlags;
    if filter.action.as_mods().flags & ACTION_LOCK_CLEAR != 0
        && filter.action.as_mods().flags & unlock_on_press as u32 != 0
        && state.components.locked_mods & filter.action.as_mods().mods.mask
            == filter.action.as_mods().mods.mask
    {
        state.components.locked_mods &= !filter.action.as_mods().mods.mask;
        filter.func = None;
    } else if filter.action.as_mods().flags & ACTION_LATCH_ON_PRESS != 0 {
        filter.priv_0 = LATCH_PENDING as u64;
        state.components.latched_mods |= filter.action.as_mods().mods.mask;
    } else {
        filter.priv_0 = LATCH_KEY_DOWN as u64;
        state.set_mods |= filter.action.as_mods().mods.mask;
    };
}

fn xkb_filter_mod_latch_func(
    state: &mut XkbState,
    events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    let mut latch: XkbKeyLatchState = filter.priv_0 as XkbKeyLatchState;
    if direction == XKB_KEY_DOWN {
        let actions = xkb_key_get_actions(state, key);
        if latch as u32 == LATCH_KEY_DOWN {
            if state.flags & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS != 0 {
                let mut k: u16 = 0_u16;
                while (k as usize) < actions.len() {
                    if xkb_action_breaks_latch(
                        &actions[k as usize],
                        INTERNAL_BREAKS_MOD_LATCH,
                        filter.action.as_mods().mods.mask,
                    ) {
                        latch = NO_LATCH;
                        break;
                    } else {
                        k = k.wrapping_add(1);
                    }
                }
            } else {
                latch = NO_LATCH;
            }
        } else if latch as u32 == LATCH_PENDING {
            let sticky_keys: bool = state.components.controls & CONTROL_STICKY_KEYS != 0;
            let flags: XkbActionFlags = (filter.action.as_mods().flags
                & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                as XkbActionFlags;
            let mut k_0: u16 = 0_u16;
            while (k_0 as usize) < actions.len() {
                if (actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_MOD_LATCH
                    && actions[k_0 as usize].as_mods().flags as u32
                        == filter.action.as_mods().flags
                    || actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_MOD_SET
                        && sticky_keys as i32 != 0
                        && actions[k_0 as usize].as_mods().flags as u32 == flags as u32)
                    && actions[k_0 as usize].as_mods().mods.mask
                        == filter.action.as_mods().mods.mask
                {
                    if filter.action.as_mods().flags & ACTION_LATCH_TO_LOCK != 0 {
                        let mods_data = *filter.action.as_mods();
                        filter.action = XkbAction::ModLock(mods_data);
                        filter.func = Some(xkb_filter_mod_lock_func);
                        xkb_filter_mod_lock_new(state, events, filter);
                    } else {
                        let mods_data = *filter.action.as_mods();
                        filter.action = XkbAction::ModSet(mods_data);
                        filter.func = Some(xkb_filter_mod_set_func);
                        xkb_filter_mod_set_new(state, events, filter);
                    }
                    filter.key = key;
                    state.components.latched_mods &= !filter.action.as_mods().mods.mask;
                    return XKB_FILTER_CONSUME as i32 != 0;
                } else if xkb_action_breaks_latch(
                    &actions[k_0 as usize],
                    INTERNAL_BREAKS_MOD_LATCH,
                    filter.action.as_mods().mods.mask,
                ) {
                    state.components.latched_mods &= !filter.action.as_mods().mods.mask;
                    filter.func = None;
                    return XKB_FILTER_CONTINUE as i32 != 0;
                }
                k_0 = k_0.wrapping_add(1);
            }
        }
    } else if std::ptr::eq(key, filter.key) {
        if direction == XKB_KEY_REPEATED {
            return XKB_FILTER_CONSUME as i32 != 0;
        } else {
            let unlock_on_press: XkbActionFlags =
                (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LATCH_ON_PRESS as i32) as XkbActionFlags;
            if filter.action.as_mods().flags & ACTION_LOCK_CLEAR != 0
                && filter.action.as_mods().flags & unlock_on_press as u32 == 0
                && state.components.locked_mods & filter.action.as_mods().mods.mask
                    == filter.action.as_mods().mods.mask
            {
                if latch as u32 == LATCH_PENDING {
                    state.components.latched_mods &= !filter.action.as_mods().mods.mask;
                } else {
                    state.clear_mods |= filter.action.as_mods().mods.mask;
                }
                state.components.locked_mods &= !filter.action.as_mods().mods.mask;
                filter.func = None;
            } else if latch as u32 == NO_LATCH {
                state.clear_mods |= filter.action.as_mods().mods.mask;
                filter.func = None;
            } else if filter.action.as_mods().flags & ACTION_LATCH_ON_PRESS == 0 {
                latch = LATCH_PENDING;
                state.clear_mods |= filter.action.as_mods().mods.mask;
                state.components.latched_mods |= filter.action.as_mods().mods.mask;
            }
        }
    }
    filter.priv_0 = latch as u64;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn xkb_filter_ctrls_new(state: &mut XkbState, _events: &mut XkbEvents, filter: &mut XkbFilter) {
    if filter.action.action_type() == ACTION_TYPE_CTRL_SET {
        filter.priv_0 = (!state.components.controls & filter.action.as_ctrls().ctrls) as u64;
    } else {
        filter.priv_0 = (state.components.controls & filter.action.as_ctrls().ctrls) as u64;
    }
    if filter.action.action_type() == ACTION_TYPE_CTRL_SET
        || filter.action.as_ctrls().flags & ACTION_LOCK_NO_LOCK == 0
    {
        state.components.controls =
            (state.components.controls | filter.action.as_ctrls().ctrls) as XkbActionControls;
    }
}

fn xkb_filter_ctrls_func(
    state: &mut XkbState,
    events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    's_32: {
        match direction {
            1 => {
                filter.refcnt += 1;
            }
            2 => {}
            _ => {
                filter.refcnt -= 1;
                if filter.refcnt > 0_i32 {
                    return XKB_FILTER_CONSUME as i32 != 0;
                }
                break 's_32;
            }
        }
        return XKB_FILTER_CONSUME as i32 != 0;
    }
    if filter.action.action_type() == ACTION_TYPE_CTRL_SET
        || filter.action.as_ctrls().flags & ACTION_LOCK_NO_UNLOCK == 0
    {
        let old: XkbActionControls = state.components.controls;
        state.components.controls = (state.components.controls
            & !(filter.priv_0 as XkbActionControls))
            as XkbActionControls;
        if old as u32 & CONTROL_STICKY_KEYS != 0
            && state.components.controls & CONTROL_STICKY_KEYS == 0
        {
            clear_all_latches_and_locks(state, events);
        }
    }
    filter.func = None;
    XKB_FILTER_CONTINUE as i32 != 0
}

fn append_redirect_key_events(
    state: &mut XkbState,
    events: &mut XkbEvents,
    redirect: &XkbRedirectKeyAction,
    direction: XkbKeyDirection,
) -> bool {
    let mut changed: u32 = 0_u32;
    let mask: u32 = redirect.affect;
    let mut last_components: StateComponents = state.components;
    {
        let queue = &events.queue;
        if !queue.is_empty() {
            let mut idx = queue.len() - 1;
            loop {
                if queue[idx].type_0 == XKB_EVENT_TYPE_COMPONENTS_CHANGE {
                    if let XkbEventData::Components(comp) = &queue[idx].data {
                        last_components = comp.components;
                    }
                    break;
                }
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    }
    if mask != 0 {
        let mut new: StateComponents = last_components;
        new.base_mods = new.base_mods & !mask | redirect.mods;
        new.latched_mods = new.latched_mods & !mask | redirect.mods;
        new.locked_mods = new.locked_mods & !mask | redirect.mods;
        new.mods = new.mods & !mask | redirect.mods;
        changed = get_state_component_changes(&last_components, &new);
        if changed as u64 != 0 {
            events.queue.push(XkbEvent {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                data: XkbEventData::Components(XkbEventComponents {
                    components: new,
                    changed,
                }),
            });
        }
    }
    events.queue.push(XkbEvent {
        type_0: (if direction == XKB_KEY_UP {
            XKB_EVENT_TYPE_KEY_UP as i32
        } else if direction == XKB_KEY_REPEATED {
            XKB_EVENT_TYPE_KEY_REPEATED as i32
        } else {
            XKB_EVENT_TYPE_KEY_DOWN as i32
        }) as XkbEventType,
        data: XkbEventData::Keycode(redirect.keycode),
    });
    if mask != 0 && changed != 0 {
        events.queue.push(XkbEvent {
            type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
            data: XkbEventData::Components(XkbEventComponents {
                components: last_components,
                changed,
            }),
        });
    }
    true
}

fn xkb_filter_redirect_key_new(
    state: &mut XkbState,
    events: &mut XkbEvents,
    filter: &mut XkbFilter,
) {
    if filter.action.as_redirect().keycode == XKB_KEYCODE_INVALID {
        filter.func = None;
        return;
    }
    append_redirect_key_events(state, events, filter.action.as_redirect(), XKB_KEY_DOWN);
}

fn xkb_filter_redirect_key_func(
    state: &mut XkbState,
    events: &mut XkbEvents,
    filter: &mut XkbFilter,
    key: &XkbKey,
    direction: XkbKeyDirection,
) -> bool {
    if !std::ptr::eq(key, filter.key) {
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
    if direction == XKB_KEY_UP {
        append_redirect_key_events(state, events, filter.action.as_redirect(), XKB_KEY_UP);
        filter.func = None;
        return XKB_FILTER_CONSUME as i32 != 0;
    } else if direction == XKB_KEY_DOWN {
        let actions = xkb_key_get_actions(state, key);
        let mut a: u16 = 0_u16;
        while (a as usize) < actions.len() {
            if actions[a as usize].action_type() as u32 == ACTION_TYPE_REDIRECT_KEY
                && actions[a as usize].as_redirect().keycode != filter.action.as_redirect().keycode
            {
                append_redirect_key_events(state, events, filter.action.as_redirect(), XKB_KEY_UP);
                filter.func = None;
                return XKB_FILTER_CONTINUE as i32 != 0;
            }
            a = a.wrapping_add(1);
        }
    }
    append_redirect_key_events(state, events, filter.action.as_redirect(), direction);
    XKB_FILTER_CONSUME as i32 != 0
}

static FILTER_ACTION_FUNCS: [FilterActionFuncs; 21] = {
    [
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: Some(xkb_filter_mod_set_new),
            func: Some(xkb_filter_mod_set_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_mod_latch_new),
            func: Some(xkb_filter_mod_latch_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_mod_lock_new),
            func: Some(xkb_filter_mod_lock_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_group_set_new),
            func: Some(xkb_filter_group_set_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_group_latch_new),
            func: Some(xkb_filter_group_latch_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_group_lock_new),
            func: Some(xkb_filter_group_lock_func),
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: Some(xkb_filter_ctrls_new),
            func: Some(xkb_filter_ctrls_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_ctrls_new),
            func: Some(xkb_filter_ctrls_func),
        },
        FilterActionFuncs {
            new: Some(xkb_filter_redirect_key_new),
            func: Some(xkb_filter_redirect_key_func),
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
        FilterActionFuncs {
            new: None,
            func: None,
        },
    ]
};

fn xkb_filter_apply_all(
    state: &mut XkbState,
    events: &mut XkbEvents,
    key: &XkbKey,
    direction: XkbKeyDirection,
) {
    let mut consumed: bool = false;
    // Phase 1: run existing filters' func callbacks
    let mut i = 0;
    while i < state.filters.len() {
        if state.filters[i].func.is_some() {
            // Swap out the filter to avoid aliasing &mut state and &mut filter
            let mut filter = std::mem::take(&mut state.filters[i]);
            let func = filter.func.expect("non-null function pointer");
            if func(state, events, &mut filter, key, direction) as i32 == XKB_FILTER_CONSUME as i32
            {
                consumed = true;
            }
            state.filters[i] = filter;
        }
        i += 1;
    }
    if consumed || direction == XKB_KEY_UP {
        return;
    }
    // Phase 2: create new filters for key-down actions
    let actions = xkb_key_get_actions(state, key);
    let actions: Vec<XkbAction> = actions.to_vec();
    let mut k: u16 = 0_u16;
    while (k as usize) < actions.len() {
        if ((actions[k as usize].action_type() as u32) < _ACTION_TYPE_NUM_ENTRIES)
            && FILTER_ACTION_FUNCS[actions[k as usize].action_type() as usize]
                .new
                .is_some()
        {
            let idx = xkb_filter_new(state);
            state.filters[idx].key = key;
            state.filters[idx].action = actions[k as usize];
            if state.components.controls & CONTROL_STICKY_KEYS != 0 {
                if state.filters[idx].action.action_type() == ACTION_TYPE_MOD_SET {
                    let mods_data = *state.filters[idx].action.as_mods();
                    state.filters[idx].action = XkbAction::ModLatch(mods_data);
                    if state.flags & XKB_A11Y_LATCH_TO_LOCK != 0 {
                        state.filters[idx].action.as_mods_mut().flags =
                            (state.filters[idx].action.as_mods().flags | ACTION_LATCH_TO_LOCK)
                                as XkbActionFlags;
                    }
                } else if state.filters[idx].action.action_type() == ACTION_TYPE_GROUP_SET {
                    let group_data = *state.filters[idx].action.as_group();
                    state.filters[idx].action = XkbAction::GroupLatch(group_data);
                    if state.flags & XKB_A11Y_LATCH_TO_LOCK != 0 {
                        state.filters[idx].action.as_group_mut().flags =
                            (state.filters[idx].action.as_group().flags | ACTION_LATCH_TO_LOCK)
                                as XkbActionFlags;
                    }
                }
            }
            if state.filters[idx].action.action_type() == ACTION_TYPE_REDIRECT_KEY {
                let km = Rc::clone(&state.keymap);
                state.filters[idx].action.as_redirect_mut().affect =
                    mod_mask_get_effective(&km, state.filters[idx].action.as_redirect().affect);
                state.filters[idx].action.as_redirect_mut().mods =
                    mod_mask_get_effective(&km, state.filters[idx].action.as_redirect().mods);
            }
            state.filters[idx].func =
                FILTER_ACTION_FUNCS[state.filters[idx].action.action_type() as usize].func;
            let new_fn = FILTER_ACTION_FUNCS[state.filters[idx].action.action_type() as usize]
                .new
                .expect("non-null function pointer");
            // Swap out filter for the new callback
            let mut filter = std::mem::take(&mut state.filters[idx]);
            new_fn(state, events, &mut filter);
            state.filters[idx] = filter;
        }
        k = k.wrapping_add(1);
    }
}
pub(crate) fn xkb_state_new(keymap: Rc<XkbKeymap>) -> Box<XkbState> {
    let mut state = Box::new(XkbState {
        components: StateComponents {
            base_group: 0,
            latched_group: 0,
            locked_group: 0,
            group: 0,
            base_mods: 0,
            latched_mods: 0,
            locked_mods: 0,
            mods: 0,
            leds: 0,
            controls: 0,
        },
        controls: MachineControls {
            out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
            out_of_range_redirect_group: 0,
        },
        set_mods: 0,
        clear_mods: 0,
        mod_key_count: [0; 32],
        flags: XKB_A11Y_NO_FLAGS,
        refcnt: 1,
        filters: Vec::new(),
        keymap: keymap.clone(),
    });
    if keymap.format != XKB_KEYMAP_FORMAT_TEXT_V1
        && XKB_A11Y_NO_FLAGS & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS == 0
    {
        state.flags = (state.flags | XKB_A11Y_LATCH_SIMULTANEOUS_KEYS) as XkbA11yFlags;
    }
    xkb_state_update_derived(&mut state);
    state
}
fn xkb_state_led_update_all(state: &mut XkbState) {
    let keymap = &*state.keymap;
    state.components.leds = 0 as XkbLedMaskT;
    let mut idx: u32 = 0_u32;
    while idx < keymap.num_leds {
        let led = &keymap.leds[idx as usize];
        let mut set_led = false;
        if led.which_mods != 0_u32 && led.mods.mask != 0_u32 {
            let mut mod_mask: u32 = 0_u32;
            if led.which_mods & XKB_STATE_MODS_EFFECTIVE != 0 {
                mod_mask |= state.components.mods;
            }
            if led.which_mods & XKB_STATE_MODS_DEPRESSED != 0 {
                mod_mask |= state.components.base_mods;
            }
            if led.which_mods & XKB_STATE_MODS_LATCHED != 0 {
                mod_mask |= state.components.latched_mods;
            }
            if led.which_mods & XKB_STATE_MODS_LOCKED != 0 {
                mod_mask |= state.components.locked_mods;
            }
            if led.mods.mask & mod_mask != 0 {
                state.components.leds = (state.components.leds | 1_u32 << idx) as XkbLedMaskT;
                set_led = true;
            }
        }
        if !set_led {
            if led.which_groups as i32 != 0_i32 {
                if (led.groups != 0) as i64 != 0_i64 {
                    let mut group_mask: u32 = 0_u32;
                    if led.which_groups as i32 & XKB_STATE_LAYOUT_EFFECTIVE as i32 != 0 {
                        group_mask |= 1_u32 << state.components.group;
                    }
                    if led.which_groups as i32 & XKB_STATE_LAYOUT_LOCKED as i32 != 0 {
                        group_mask |= 1_u32 << state.components.locked_group;
                    }
                    if led.which_groups as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32 != 0
                        && state.components.base_group != 0_i32
                    {
                        group_mask |= led.groups;
                    }
                    if led.which_groups as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
                        && state.components.latched_group != 0_i32
                    {
                        group_mask |= led.groups;
                    }
                    if led.groups & group_mask != 0 {
                        state.components.leds =
                            (state.components.leds | 1_u32 << idx) as XkbLedMaskT;
                        set_led = true;
                    }
                } else if led.which_groups as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32 != 0
                    && state.components.base_group == 0_i32
                    || led.which_groups as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
                        && state.components.latched_group == 0_i32
                {
                    state.components.leds = (state.components.leds | 1_u32 << idx) as XkbLedMaskT;
                    set_led = true;
                }
            }
            if !set_led && led.ctrls & state.components.controls != 0 {
                state.components.leds = (state.components.leds | 1_u32 << idx) as XkbLedMaskT;
            }
        }
        idx = idx.wrapping_add(1);
    }
}

fn xkb_state_update_derived(state: &mut XkbState) {
    let mut wrapped: u32;
    state.components.mods =
        state.components.base_mods | state.components.latched_mods | state.components.locked_mods;
    // SAFETY: raw pointer deref avoids borrowing `state` (needed for mutation below)
    let keymap = &*state.keymap;
    wrapped = xkb_wrap_group_into_range(
        state.components.locked_group,
        keymap.num_groups,
        state.controls.out_of_range_group_policy,
        state.controls.out_of_range_redirect_group,
    );
    state.components.locked_group = (if wrapped == XKB_LAYOUT_INVALID {
        0_u32
    } else {
        wrapped
    }) as i32;
    wrapped = xkb_wrap_group_into_range(
        state.components.base_group
            + state.components.latched_group
            + state.components.locked_group,
        keymap.num_groups,
        state.controls.out_of_range_group_policy,
        state.controls.out_of_range_redirect_group,
    );
    state.components.group = if wrapped == XKB_LAYOUT_INVALID {
        0_u32
    } else {
        wrapped
    };
    xkb_state_led_update_all(state);
}
pub(crate) fn xkb_state_update_key(
    state: &mut XkbState,
    kc: u32,
    direction: XkbKeyDirection,
) -> u32 {
    let keymap_rc = Rc::clone(&state.keymap);
    let keymap = &*keymap_rc;
    let key_ref = match keymap.get_key(kc) {
        Some(k) => k,
        None => return 0_u32,
    };
    if direction == XKB_KEY_REPEATED && !key_ref.repeats {
        return 0_u32;
    }
    let prev_components: StateComponents = state.components;
    state.set_mods = 0_u32;
    state.clear_mods = 0_u32;
    let mut dummy_events = XkbEvents::dummy();
    xkb_filter_apply_all(state, &mut dummy_events, key_ref, direction);
    let mut i: u32;
    let mut bit: u32;
    i = 0_u32;
    bit = 1_u32;
    while state.set_mods != 0 {
        if state.set_mods & bit != 0 {
            state.mod_key_count[i as usize] += 1;
            state.components.base_mods |= bit;
            state.set_mods &= !bit;
        }
        i = i.wrapping_add(1);
        bit <<= 1_i32;
    }
    i = 0_u32;
    bit = 1_u32;
    while state.clear_mods != 0 {
        if state.clear_mods & bit != 0 {
            state.mod_key_count[i as usize] -= 1;
            if state.mod_key_count[i as usize] as i32 <= 0_i32 {
                state.components.base_mods &= !bit;
                state.mod_key_count[i as usize] = 0_i16;
            }
            state.clear_mods &= !bit;
        }
        i = i.wrapping_add(1);
        bit <<= 1_i32;
    }
    xkb_state_update_derived(state);
    get_state_component_changes(&prev_components, &state.components)
}

static SYNTHETIC_KEY: LazyLock<XkbKey> = LazyLock::new(|| XkbKey {
    keycode: 0_u32,
    name: 0,
    explicit: 0 as XkbExplicitComponents,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    repeats: false,
    implicit_actions: false,
    out_of_range_pending_group: false,
    out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    out_of_range_group_number: 0,
    num_groups: 0,
    groups: Vec::new(),
    overlay_keys: Vec::new(),
});

fn update_latch_modifiers(state: &mut XkbState, events: &mut XkbEvents, mask: u32, latches: u32) {
    let clear: u32 = mask & !latches;
    state.components.latched_mods &= !clear;
    let synthetic_key_level_break_mod_latch: XkbLevel = XkbLevel {
        upper: XKB_KEY_NO_SYMBOL as u32,
        has_upper: false,
        syms: Vec::new(),
        actions: vec![XkbAction::Internal(XkbInternalAction {
            flags: INTERNAL_BREAKS_MOD_LATCH,
            clear_latched_mods: clear,
        })],
    };
    let synthetic_key_group_break_mod_latch: XkbGroup = XkbGroup {
        explicit_symbols: false,
        explicit_actions: false,
        implicit_actions: false,
        explicit_type: false,
        type_idx: 0,
        levels: vec![synthetic_key_level_break_mod_latch],
    };
    let synthetic_key_break_mod_latch = XkbKey {
        keycode: 0,
        name: 0,
        explicit: 0 as XkbExplicitComponents,
        modmap: 0,
        vmodmap: 0,
        overlays: 0,
        repeats: false,
        implicit_actions: false,
        out_of_range_pending_group: false,
        out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
        out_of_range_group_number: 0,
        num_groups: 1,
        groups: vec![synthetic_key_group_break_mod_latch],
        overlay_keys: Vec::new(),
    };
    xkb_filter_apply_all(state, events, &synthetic_key_break_mod_latch, XKB_KEY_DOWN);
    let key: &XkbKey = &SYNTHETIC_KEY;
    let latch_mods: XkbAction = XkbAction::ModLatch(XkbModAction {
        type_0: ACTION_TYPE_MOD_LATCH,
        flags: 0 as XkbActionFlags,
        mods: XkbMods {
            mods: 0,
            mask: mask & latches,
        },
    });
    let idx = xkb_filter_new(state);
    state.filters[idx].key = key;
    state.filters[idx].func = Some(xkb_filter_mod_latch_func);
    state.filters[idx].action = latch_mods;
    // Swap out for callbacks
    let mut filter = std::mem::take(&mut state.filters[idx]);
    xkb_filter_mod_latch_new(state, events, &mut filter);
    xkb_filter_mod_latch_func(state, events, &mut filter, key, XKB_KEY_UP);
    state.filters[idx] = filter;
}

fn update_latch_group(state: &mut XkbState, events: &mut XkbEvents, group: i32) {
    xkb_filter_apply_all(
        state,
        events,
        &SYNTHETIC_KEY_BREAK_GROUP_LATCH,
        XKB_KEY_DOWN,
    );
    let key: &XkbKey = &SYNTHETIC_KEY;
    let latch_group: XkbAction = XkbAction::GroupLatch(XkbGroupAction {
        flags: ACTION_ABSOLUTE_SWITCH,
        group,
    });
    let idx = xkb_filter_new(state);
    state.filters[idx].key = key;
    state.filters[idx].func = Some(xkb_filter_group_latch_func);
    state.filters[idx].action = latch_group;
    let mut filter = std::mem::take(&mut state.filters[idx]);
    xkb_filter_group_latch_new(state, events, &mut filter);
    xkb_filter_group_latch_func(state, events, &mut filter, key, XKB_KEY_UP);
    state.filters[idx] = filter;
}
#[inline]
fn resolve_to_canonical_mods(keymap: &XkbKeymap, mods: u32) -> u32 {
    mods & keymap.canonical_state_mask
        | mod_mask_get_effective(keymap, mods & !keymap.canonical_state_mask)
}

fn state_update_latched_locked(
    state: &mut XkbState,
    update: &XkbStateComponentsUpdate,
    events: &mut XkbEvents,
) {
    let keymap = Rc::clone(&state.keymap);
    let affect_locked_mods: u32 = resolve_to_canonical_mods(&keymap, update.affect_locked_mods);
    if affect_locked_mods != 0 {
        let locked_mods: u32 = resolve_to_canonical_mods(&keymap, update.locked_mods);
        state.components.locked_mods &= !affect_locked_mods;
        state.components.locked_mods |= locked_mods & affect_locked_mods;
    }
    if update.components & XKB_STATE_LAYOUT_LOCKED != 0 {
        state.components.locked_group = update.locked_layout;
    }
    let affect_latched_mods: u32 = resolve_to_canonical_mods(&keymap, update.affect_latched_mods);
    if affect_latched_mods != 0 {
        let latched_mods: u32 = resolve_to_canonical_mods(&keymap, update.latched_mods);
        update_latch_modifiers(state, events, affect_latched_mods, latched_mods);
    }
    if update.components & XKB_STATE_LAYOUT_LATCHED != 0 {
        update_latch_group(state, events, update.latched_layout);
    }
}

#[inline]
fn clear_all_latches_and_locks(state: &mut XkbState, events: &mut XkbEvents) {
    static COMPONENTS: u32 = (XKB_STATE_MODS_LATCHED as i32
        | XKB_STATE_MODS_LOCKED as i32
        | XKB_STATE_LAYOUT_LATCHED as i32
        | XKB_STATE_LAYOUT_LOCKED as i32) as u32;
    let update: XkbStateComponentsUpdate = XkbStateComponentsUpdate {
        components: COMPONENTS,
        affect_latched_mods: XKB_MOD_ALL,
        latched_mods: 0_u32,
        affect_locked_mods: XKB_MOD_ALL,
        locked_mods: 0_u32,
        latched_layout: 0_i32,
        locked_layout: 0_i32,
    };
    state_update_latched_locked(state, &update, events);
}

pub(crate) fn xkb_state_update_mask(
    state: &mut XkbState,
    base_mods: u32,
    latched_mods: u32,
    locked_mods: u32,
    base_group: u32,
    latched_group: u32,
    locked_group: u32,
) -> u32 {
    let prev_components: StateComponents = state.components;
    let keymap = state.keymap();
    let base_mods = resolve_to_canonical_mods(keymap, base_mods);
    let latched_mods = resolve_to_canonical_mods(keymap, latched_mods);
    let locked_mods = resolve_to_canonical_mods(keymap, locked_mods);
    state.components.base_mods = base_mods;
    state.components.latched_mods = latched_mods;
    state.components.locked_mods = locked_mods;
    state.components.base_group = base_group as i32;
    state.components.latched_group = latched_group as i32;
    state.components.locked_group = locked_group as i32;
    xkb_state_update_derived(state);
    get_state_component_changes(&prev_components, &state.components)
}

fn should_do_caps_transformation(state: &XkbState, kc: u32) -> bool {
    xkb_state_mod_index_is_active(
        state,
        XKB_MOD_INDEX_CAPS as i32 as u32,
        XKB_STATE_MODS_EFFECTIVE,
    ) > 0_i32
        && xkb_state_mod_index_is_consumed(state, kc, XKB_MOD_INDEX_CAPS as i32 as u32) == 0_i32
}

pub(crate) fn xkb_state_key_get_syms(state: &XkbState, kc: u32) -> &[u32] {
    let layout: u32 = xkb_state_key_get_layout(state, kc);
    if layout != XKB_LAYOUT_INVALID {
        let level = xkb_state_key_get_level(state, kc, layout);
        if level != XKB_LEVEL_INVALID {
            let keymap = state.keymap();
            if let Some(key) = keymap.get_key(kc) {
                if let Some(leveli) = keymap.get_key_level(key, layout, level) {
                    let num_syms = leveli.syms.len();
                    if num_syms > 0 {
                        if should_do_caps_transformation(state, kc) {
                            if num_syms > 1 {
                                return if leveli.has_upper {
                                    &leveli.syms[num_syms..]
                                } else {
                                    &leveli.syms[..num_syms]
                                };
                            } else {
                                return std::slice::from_ref(&leveli.upper);
                            }
                        } else {
                            return &leveli.syms[..num_syms];
                        }
                    }
                }
            }
        }
    }
    &[]
}

pub(crate) fn xkb_state_key_get_one_sym(state: &XkbState, kc: u32) -> u32 {
    let syms = xkb_state_key_get_syms(state, kc);
    if syms.len() != 1 {
        XKB_KEY_NO_SYMBOL as u32
    } else {
        syms[0]
    }
}

#[inline]
fn serialize_mods(components: &StateComponents, type_0: u32) -> u32 {
    let mut ret: u32 = 0_u32;
    if type_0 & XKB_STATE_MODS_EFFECTIVE != 0 {
        return components.mods;
    }
    if type_0 & XKB_STATE_MODS_DEPRESSED != 0 {
        ret |= components.base_mods;
    }
    if type_0 & XKB_STATE_MODS_LATCHED != 0 {
        ret |= components.latched_mods;
    }
    if type_0 & XKB_STATE_MODS_LOCKED != 0 {
        ret |= components.locked_mods;
    }
    ret
}

pub(crate) fn xkb_state_serialize_mods(state: &XkbState, type_0: u32) -> u32 {
    serialize_mods(&state.components, type_0)
}

pub(crate) fn mod_mask_get_effective(keymap: &XkbKeymap, mods: u32) -> u32 {
    let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
    let mut i: u32 = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
    while i < keymap.mods.num_mods {
        if mods & 1_u32 << i != 0 {
            mask |= keymap.mods.mods[i as usize].mapping;
        }
        i = i.wrapping_add(1);
    }
    mask
}

pub(crate) fn xkb_state_mod_index_is_active(state: &XkbState, idx: u32, type_0: u32) -> i32 {
    let keymap = state.keymap();
    if (idx >= xkb_keymap_num_mods(keymap)) as i64 != 0 {
        return -1_i32;
    }
    let mapping: u32 = keymap.mods.mods[idx as usize].mapping;
    if mapping == 0 {
        return 0_i32;
    }
    (xkb_state_serialize_mods(state, type_0) & mapping == mapping) as i32
}

pub(crate) fn xkb_state_mod_name_is_active(state: &XkbState, name: &str, type_0: u32) -> i32 {
    let idx = xkb_keymap_mod_get_index_ref(state.keymap(), name);
    if idx == XKB_MOD_INVALID {
        return -1_i32;
    }
    xkb_state_mod_index_is_active(state, idx, type_0)
}

fn key_get_consumed(state: &XkbState, key: &XkbKey, mode: XkbConsumedMode) -> u32 {
    let group: u32 = xkb_state_key_get_layout(state, key.keycode);
    if group == XKB_LAYOUT_INVALID {
        return 0_u32;
    }
    let mut preserve: u32 = 0_u32;
    let mut consumed: u32 = 0_u32;
    let matching_entry = get_entry_for_key_state(state, key, group);
    if let Some(entry) = matching_entry {
        preserve = entry.preserve.mask;
    }
    let keymap = state.keymap();
    let type_0 = &keymap.types[key.groups[group as usize].type_idx as usize];
    match mode {
        0 => {
            consumed = type_0.mods.mask;
        }
        1 => {
            let no_mods_entry = get_entry_for_mods(type_0, 0_u32);
            let no_mods_leveli: u32 = match no_mods_entry {
                Some(e) => e.level,
                None => 0_u32,
            };
            let no_mods_level: &XkbLevel =
                &key.groups[group as usize].levels[no_mods_leveli as usize];
            for entry in &type_0.entries {
                if entry_is_active(entry) {
                    let level: &XkbLevel = &key.groups[group as usize].levels[entry.level as usize];
                    if !xkb_levels_same_syms(level, no_mods_level)
                        && (matching_entry.is_some_and(|m| std::ptr::eq(entry, m))
                            || (entry.mods.mask != 0 && entry.mods.mask.is_power_of_two()))
                    {
                        consumed |= entry.mods.mask & !entry.preserve.mask;
                    }
                }
            }
        }
        _ => {}
    }
    consumed & !preserve
}

pub(crate) fn xkb_state_mod_index_is_consumed2(
    state: &XkbState,
    kc: u32,
    idx: u32,
    mode: XkbConsumedMode,
) -> i32 {
    let keymap = state.keymap();
    let key = match keymap.get_key(kc) {
        Some(k) => k,
        None => return -1_i32,
    };
    if (idx >= xkb_keymap_num_mods(keymap)) as i64 != 0 {
        return -1_i32;
    }
    let mapping: u32 = keymap.mods.mods[idx as usize].mapping;
    if mapping == 0 {
        return 0_i32;
    }
    (mapping & key_get_consumed(state, key, mode) == mapping) as i32
}

pub(crate) fn xkb_state_mod_index_is_consumed(state: &XkbState, kc: u32, idx: u32) -> i32 {
    xkb_state_mod_index_is_consumed2(state, kc, idx, XKB_CONSUMED_MODE_XKB)
}

pub(crate) type RxkbLogLevel = u32;
pub(crate) const RXKB_LOG_LEVEL_DEBUG: RxkbLogLevel = 50;
pub(crate) const RXKB_LOG_LEVEL_INFO: RxkbLogLevel = 40;
pub(crate) const RXKB_LOG_LEVEL_WARNING: RxkbLogLevel = 30;
pub(crate) const RXKB_LOG_LEVEL_ERROR: RxkbLogLevel = 20;
pub(crate) const RXKB_LOG_LEVEL_CRITICAL: RxkbLogLevel = 10;
pub(crate) type RxkbPopularity = u32;
pub(crate) const RXKB_POPULARITY_EXOTIC: RxkbPopularity = 2;
pub(crate) const RXKB_POPULARITY_STANDARD: RxkbPopularity = 1;
pub(crate) type RxkbContextFlags = u32;
pub(crate) const RXKB_CONTEXT_NO_SECURE_GETENV: RxkbContextFlags = 4;
pub(crate) const RXKB_CONTEXT_LOAD_EXOTIC_RULES: RxkbContextFlags = 2;
pub(crate) const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: RxkbContextFlags = 1;
pub(crate) const RXKB_CONTEXT_NO_FLAGS: RxkbContextFlags = 0;

pub(crate) type ContextState = u32;
pub(crate) const CONTEXT_FAILED: ContextState = 2;
pub(crate) const CONTEXT_PARSED: ContextState = 1;
pub(crate) const CONTEXT_NEW: ContextState = 0;

pub(crate) struct RxkbContext {
    pub(crate) context_state: ContextState,
    pub(crate) load_extra_rules_files: bool,
    pub(crate) models: Vec<RxkbModel>,
    pub(crate) layouts: Vec<RxkbLayout>,
    pub(crate) option_groups: Vec<RxkbOptionGroup>,
    pub(crate) includes: Vec<String>,
    pub(crate) log_fn: Option<fn(&RxkbContext, RxkbLogLevel, &str)>,
    pub(crate) log_level: RxkbLogLevel,
}

#[derive(Clone)]
pub(crate) struct RxkbModel {
    pub(crate) name: String,
    pub(crate) vendor: String,
    pub(crate) description: String,
    pub(crate) popularity: RxkbPopularity,
}

pub(crate) struct RxkbLayout {
    pub(crate) name: String,
    pub(crate) brief: String,
    pub(crate) description: String,
    pub(crate) variant: String,
    pub(crate) popularity: RxkbPopularity,
    pub(crate) iso639s: Vec<String>,
    pub(crate) iso3166s: Vec<String>,
}

pub(crate) struct RxkbOptionGroup {
    pub(crate) allow_multiple: bool,
    pub(crate) options: Vec<RxkbOption>,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) popularity: RxkbPopularity,
}

#[derive(Clone)]
pub(crate) struct RxkbOption {
    pub(crate) name: String,
    pub(crate) brief: String,
    pub(crate) description: String,
    pub(crate) popularity: RxkbPopularity,
    pub(crate) layout_specific: bool,
}

#[derive(Clone)]
pub(crate) struct ConfigItem {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) brief: String,
    pub(crate) vendor: String,
    pub(crate) popularity: RxkbPopularity,
    pub(crate) layout_specific: bool,
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

impl RxkbLayout {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn brief(&self) -> &str {
        &self.brief
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn variant(&self) -> &str {
        &self.variant
    }
    pub(crate) fn iso639s(&self) -> &[String] {
        &self.iso639s
    }
    pub(crate) fn iso3166s(&self) -> &[String] {
        &self.iso3166s
    }
}

impl RxkbModel {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn vendor(&self) -> &str {
        &self.vendor
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
}

impl RxkbOptionGroup {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn allows_multiple(&self) -> bool {
        self.allow_multiple
    }
    pub(crate) fn options(&self) -> &[RxkbOption] {
        &self.options
    }
}

impl RxkbOption {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn brief(&self) -> &str {
        &self.brief
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn is_layout_specific(&self) -> bool {
        self.layout_specific
    }
}

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------

macro_rules! rxkb_logf {
    ($ctx:expr, $level:expr, $($arg:tt)*) => {{
        rxkb_log($ctx, $level, &format!($($arg)*));
    }};
}

fn rxkb_log(ctx: &RxkbContext, level: RxkbLogLevel, msg: &str) {
    if ctx.log_level < level {
        return;
    }
    if let Some(f) = ctx.log_fn {
        f(ctx, level, msg);
    }
}

fn log_level_to_prefix(level: RxkbLogLevel) -> &'static str {
    match level {
        50 => "xkbregistry: DEBUG: ",
        40 => "xkbregistry: INFO: ",
        30 => "xkbregistry: WARNING: ",
        20 => "xkbregistry: ERROR: ",
        10 => "xkbregistry: CRITICAL: ",
        _ => "",
    }
}

fn default_log_fn(_ctx: &RxkbContext, level: RxkbLogLevel, msg: &str) {
    let prefix = log_level_to_prefix(level);
    if !prefix.is_empty() {
        eprint!("{}", prefix);
    }
    eprint!("{}", msg);
}

// ---------------------------------------------------------------------------
// Context construction & pub(cratelic API
// ---------------------------------------------------------------------------

impl RxkbContext {
    pub(crate) fn new(flags: RxkbContextFlags) -> Option<Box<RxkbContext>> {
        let valid_flags: RxkbContextFlags = RXKB_CONTEXT_NO_DEFAULT_INCLUDES
            | RXKB_CONTEXT_LOAD_EXOTIC_RULES
            | RXKB_CONTEXT_NO_SECURE_GETENV;

        let mut ctx = Box::new(RxkbContext {
            context_state: CONTEXT_NEW,
            load_extra_rules_files: flags & RXKB_CONTEXT_LOAD_EXOTIC_RULES != 0,
            models: Vec::new(),
            layouts: Vec::new(),
            option_groups: Vec::new(),
            includes: Vec::new(),
            log_fn: Some(default_log_fn as fn(&RxkbContext, RxkbLogLevel, &str)),
            log_level: RXKB_LOG_LEVEL_ERROR,
        });

        if let Ok(env) = std::env::var("RXKB_LOG_LEVEL") {
            ctx.log_level = log_level_from_str(&env);
        }

        if flags & !valid_flags != 0 {
            rxkb_logf!(
                &*ctx,
                RXKB_LOG_LEVEL_ERROR,
                "{}: Invalid context flags: 0x{:x}\n",
                "RxkbContext_new",
                flags & !valid_flags,
            );
            return None;
        }

        if flags & RXKB_CONTEXT_NO_DEFAULT_INCLUDES == 0 && !ctx.include_path_append_default() {
            rxkb_logf!(
                &*ctx,
                RXKB_LOG_LEVEL_ERROR,
                "[XKB-{:03}] Failed to add any default include path (default system path: {})\n",
                XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                "/usr/share/xkeyboard-config-2",
            );
            return None;
        }

        Some(ctx)
    }

    pub(crate) fn include_path_append(&mut self, path: &str) {
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return;
        }

        match std::fs::metadata(path) {
            Err(e) => {
                rxkb_logf!(
                    self,
                    RXKB_LOG_LEVEL_INFO,
                    "Include path failed: \"{}\" ({})\n",
                    path,
                    e,
                );
            }
            Ok(m) if !m.is_dir() => {
                rxkb_logf!(
                    self,
                    RXKB_LOG_LEVEL_INFO,
                    "Include path failed: \"{}\" (Not a directory)\n",
                    path,
                );
            }
            Ok(_) => {
                // Validate that rules/evdev.xml exists beneath this path
                let rules_path = format!("{}/rules/evdev.xml", path);
                // We don't actually check the file here — the original just checked the dir.
                let _ = rules_path;
                self.includes.push(path.to_string());
                rxkb_logf!(self, RXKB_LOG_LEVEL_INFO, "Include path added: {}\n", path,);
            }
        }
    }

    pub(crate) fn include_path_append_default(&mut self) -> bool {
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return false;
        }

        let mut ret = false;

        let home = std::env::var("HOME").ok();
        let xdg = std::env::var("XDG_CONFIG_HOME").ok();

        if let Some(ref xdg) = xdg {
            let p = format!("{}/xkb", xdg);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        } else if let Some(ref home) = home {
            let p = format!("{}/.config/xkb", home);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        }

        if let Some(ref home) = home {
            let p = format!("{}/.xkb", home);
            self.include_path_append(&p);
            if self.includes.last().map(|s| s.as_str()) == Some(&p) {
                ret = true;
            }
        }

        // Extra path
        let extra = std::env::var("XKB_CONFIG_EXTRA_PATH").ok();
        let extra_path = extra.as_deref().unwrap_or(DFLT_XKB_CONFIG_EXTRA_PATH_STR);
        self.include_path_append(extra_path);
        if self.includes.last().map(|s| s.as_str()) == Some(extra_path) {
            ret = true;
        }

        // Versioned extensions
        let versioned_ext = std::env::var("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH").ok();
        let versioned_path = versioned_ext
            .as_deref()
            .unwrap_or(DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH_STR);
        self.add_direct_subdirectories(versioned_path);

        // Unversioned extensions
        let unversioned_ext = std::env::var("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH").ok();
        let unversioned_path = unversioned_ext
            .as_deref()
            .unwrap_or(DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH_STR);
        self.add_direct_subdirectories(unversioned_path);

        // Root path
        let root_env = std::env::var("XKB_CONFIG_ROOT").ok();
        let root_path = root_env.as_deref().unwrap_or(DFLT_XKB_CONFIG_ROOT_STR);
        let prev_len = self.includes.len();
        self.include_path_append(root_path);
        let has_root = self.includes.len() > prev_len;
        if has_root {
            ret = true;
        }

        if !has_root && !root_path.is_empty() {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_WARNING,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                "/usr/share/X11/xkb",
                root_path,
            );
            let prev_len2 = self.includes.len();
            self.include_path_append(DFLT_XKB_LEGACY_ROOT_STR);
            if self.includes.len() > prev_len2 {
                ret = true;
            }
        }

        ret
    }

    fn add_direct_subdirectories(&mut self, path: &str) {
        let meta = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return,
        };
        if !meta.is_dir() {
            return;
        }

        let dir_entries = match std::fs::read_dir(path) {
            Ok(d) => d,
            Err(_) => return,
        };

        let mut subdirs: Vec<String> = Vec::new();
        for entry in dir_entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let name = entry.file_name();
            let name_str = match name.to_str() {
                Some(s) => s,
                None => continue,
            };
            if name_str == "." || name_str == ".." {
                continue;
            }
            let entry_path = entry.path();
            let entry_meta = match std::fs::metadata(&entry_path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if !entry_meta.is_dir() {
                continue;
            }
            if let Some(s) = entry_path.to_str() {
                subdirs.push(s.to_string());
            }
        }

        subdirs.sort();
        for p in &subdirs {
            self.include_path_append(p);
        }
    }

    pub(crate) fn parse(&mut self, ruleset: &str) -> bool {
        let mut success = false;
        if self.context_state != CONTEXT_NEW {
            rxkb_logf!(
                self,
                RXKB_LOG_LEVEL_ERROR,
                "parse must only be called on a new context\n",
            );
            return false;
        }

        // Iterate includes in reverse order (like the original)
        let includes: Vec<String> = self.includes.clone();
        let mut idx = includes.len();
        while idx > 0 {
            idx -= 1;
            let path_str = &includes[idx];

            let rules_path = format!("{}/rules/{}.xml", path_str, ruleset);
            rxkb_logf!(self, RXKB_LOG_LEVEL_DEBUG, "Parsing {}\n", rules_path);
            if parse_xml_file(self, &rules_path, RXKB_POPULARITY_STANDARD) {
                success = true;
            }

            if self.load_extra_rules_files {
                let extras_path = format!("{}/rules/{}.extras.xml", path_str, ruleset);
                rxkb_logf!(self, RXKB_LOG_LEVEL_DEBUG, "Parsing {}\n", extras_path);
                if parse_xml_file(self, &extras_path, RXKB_POPULARITY_EXOTIC) {
                    success = true;
                }
            }
        }

        self.context_state = if success {
            CONTEXT_PARSED
        } else {
            CONTEXT_FAILED
        };
        success
    }

    pub(crate) fn layouts(&self) -> &[RxkbLayout] {
        &self.layouts
    }

    pub(crate) fn models(&self) -> &[RxkbModel] {
        &self.models
    }

    pub(crate) fn option_groups(&self) -> &[RxkbOptionGroup] {
        &self.option_groups
    }
}

// ---------------------------------------------------------------------------
// Default paths (as &str)
// ---------------------------------------------------------------------------

const DFLT_XKB_CONFIG_EXTRA_PATH_STR: &str = "/etc/xkb";
const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH_STR: &str =
    "/usr/share/xkeyboard-config-2/extensions";
const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH_STR: &str = "/usr/share/X11/xkb/extensions";
const DFLT_XKB_CONFIG_ROOT_STR: &str = "/usr/share/xkeyboard-config-2";
const DFLT_XKB_LEGACY_ROOT_STR: &str = "/usr/share/X11/xkb";

// ---------------------------------------------------------------------------
// XML parsing helpers (safe)
// ---------------------------------------------------------------------------

fn get_attr<'a>(
    doc: &'a xmloxide::Document,
    node: xmloxide::tree::NodeId,
    name: &str,
) -> Option<&'a str> {
    doc.attributes(node)
        .iter()
        .find(|a| a.name == name)
        .map(|a| a.value.as_str())
}

#[inline]
fn is_node(doc: &xmloxide::Document, node: xmloxide::tree::NodeId, name: &str) -> bool {
    doc.is_element(node) && doc.node_name(node) == Some(name)
}

fn extract_text(doc: &xmloxide::Document, node: xmloxide::tree::NodeId) -> String {
    for child in doc.children(node) {
        if let Some(text) = doc.node_text(child) {
            if !text.is_empty() {
                return text.to_string();
            }
        }
    }
    String::new()
}

fn parse_config_item(
    ctx: &RxkbContext,
    doc: &xmloxide::Document,
    parent: xmloxide::tree::NodeId,
    config: &mut ConfigItem,
) -> bool {
    for ci in doc.children(parent) {
        if is_node(doc, ci, "configItem") {
            if let Some(raw_popularity) = get_attr(doc, ci, "popularity") {
                if raw_popularity == "standard" {
                    config.popularity = RXKB_POPULARITY_STANDARD;
                } else if raw_popularity == "exotic" {
                    config.popularity = RXKB_POPULARITY_EXOTIC;
                } else {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "xml: invalid popularity attribute: expected 'standard' or 'exotic', got: '{}'\n",
                        raw_popularity,
                    );
                }
            }
            if let Some(raw_layout_specific) = get_attr(doc, ci, "layout-specific") {
                if raw_layout_specific == "true" {
                    config.layout_specific = true;
                }
            }
            for node in doc.children(ci) {
                if is_node(doc, node, "name") {
                    config.name = extract_text(doc, node);
                } else if is_node(doc, node, "description") {
                    config.description = extract_text(doc, node);
                } else if is_node(doc, node, "shortDescription") {
                    config.brief = extract_text(doc, node);
                } else if is_node(doc, node, "vendor") {
                    config.vendor = extract_text(doc, node);
                }
            }
            if config.name.is_empty() {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    "xml: missing required element 'name'\n",
                );
                return false;
            }
            return true;
        }
    }
    false
}

fn parse_model(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    model: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if parse_config_item(ctx, doc, model, &mut config) {
        // Check for duplicate
        if ctx.models.iter().any(|m| m.name == config.name) {
            return;
        }
        ctx.models.push(RxkbModel {
            name: std::mem::take(&mut config.name),
            description: std::mem::take(&mut config.description),
            vendor: std::mem::take(&mut config.vendor),
            popularity: config.popularity,
        });
    }
}

fn parse_language_list(
    doc: &xmloxide::Document,
    language_list: xmloxide::tree::NodeId,
    iso639s: &mut Vec<String>,
) {
    for node in doc.children(language_list) {
        if is_node(doc, node, "iso639Id") {
            let s = extract_text(doc, node);
            if !s.is_empty() && s.len() == 3 {
                iso639s.push(s);
            }
        }
    }
}

fn parse_country_list(
    doc: &xmloxide::Document,
    country_list: xmloxide::tree::NodeId,
    iso3166s: &mut Vec<String>,
) {
    for node in doc.children(country_list) {
        if is_node(doc, node, "iso3166Id") {
            let s = extract_text(doc, node);
            if !s.is_empty() && s.len() == 2 {
                iso3166s.push(s);
            }
        }
    }
}

fn parse_variant(
    ctx: &mut RxkbContext,
    parent_layout_idx: usize,
    doc: &xmloxide::Document,
    variant: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, variant, &mut config) {
        return;
    }

    let parent_name = ctx.layouts[parent_layout_idx].name.clone();

    // Check for duplicate
    let exists = ctx
        .layouts
        .iter()
        .any(|v| v.variant == config.name && v.name == parent_name);
    if exists {
        return;
    }

    let parent_brief = ctx.layouts[parent_layout_idx].brief.clone();
    let brief = if config.brief.is_empty() {
        parent_brief
    } else {
        std::mem::take(&mut config.brief)
    };

    let mut new_layout = RxkbLayout {
        name: parent_name,
        variant: std::mem::take(&mut config.name),
        description: std::mem::take(&mut config.description),
        brief,
        popularity: config.popularity,
        iso639s: Vec::new(),
        iso3166s: Vec::new(),
    };

    // Parse language/country lists from variant's configItem
    for ci in doc.children(variant) {
        if is_node(doc, ci, "configItem") {
            let mut found_language_list = false;
            let mut found_country_list = false;
            for node in doc.children(ci) {
                if is_node(doc, node, "languageList") {
                    parse_language_list(doc, node, &mut new_layout.iso639s);
                    found_language_list = true;
                }
                if is_node(doc, node, "countryList") {
                    parse_country_list(doc, node, &mut new_layout.iso3166s);
                    found_country_list = true;
                }
            }
            // Inherit from parent if not found
            if !found_language_list {
                new_layout.iso639s = ctx.layouts[parent_layout_idx].iso639s.clone();
            }
            if !found_country_list {
                new_layout.iso3166s = ctx.layouts[parent_layout_idx].iso3166s.clone();
            }
        }
    }

    ctx.layouts.push(new_layout);
}

fn parse_layout(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    layout: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, layout, &mut config) {
        return;
    }

    // Find existing layout with same name and empty variant
    let existing_idx = ctx
        .layouts
        .iter()
        .position(|el| el.name == config.name && el.variant.is_empty());
    let layout_idx;

    if let Some(idx) = existing_idx {
        layout_idx = idx;
        // Layout already exists, don't overwrite
    } else {
        ctx.layouts.push(RxkbLayout {
            name: std::mem::take(&mut config.name),
            variant: String::new(),
            description: std::mem::take(&mut config.description),
            brief: std::mem::take(&mut config.brief),
            popularity: config.popularity,
            iso639s: Vec::new(),
            iso3166s: Vec::new(),
        });
        layout_idx = ctx.layouts.len() - 1;
    }

    // Parse variants and language/country lists
    for node in doc.children(layout) {
        if is_node(doc, node, "variantList") {
            for vnode in doc.children(node) {
                if is_node(doc, vnode, "variant") {
                    parse_variant(ctx, layout_idx, doc, vnode, popularity);
                }
            }
        }
        if existing_idx.is_none() && is_node(doc, node, "configItem") {
            for ll in doc.children(node) {
                if is_node(doc, ll, "languageList") {
                    parse_language_list(doc, ll, &mut ctx.layouts[layout_idx].iso639s);
                }
                if is_node(doc, ll, "countryList") {
                    parse_country_list(doc, ll, &mut ctx.layouts[layout_idx].iso3166s);
                }
            }
        }
    }
}

fn parse_option(
    ctx: &mut RxkbContext,
    group_idx: usize,
    doc: &xmloxide::Document,
    option: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if parse_config_item(ctx, doc, option, &mut config) {
        // Check for duplicate
        if ctx.option_groups[group_idx]
            .options
            .iter()
            .any(|o| o.name == config.name)
        {
            return;
        }
        ctx.option_groups[group_idx].options.push(RxkbOption {
            name: std::mem::take(&mut config.name),
            brief: String::new(),
            description: std::mem::take(&mut config.description),
            popularity: config.popularity,
            layout_specific: config.layout_specific,
        });
    }
}

fn parse_group(
    ctx: &mut RxkbContext,
    doc: &xmloxide::Document,
    group: xmloxide::tree::NodeId,
    popularity: RxkbPopularity,
) {
    let mut config = ConfigItem {
        name: String::new(),
        description: String::new(),
        brief: String::new(),
        vendor: String::new(),
        popularity,
        layout_specific: false,
    };
    if !parse_config_item(ctx, doc, group, &mut config) {
        return;
    }

    let existing_idx = ctx
        .option_groups
        .iter()
        .position(|el| el.name == config.name);
    let group_idx;

    if let Some(idx) = existing_idx {
        group_idx = idx;
    } else {
        let mut og = RxkbOptionGroup {
            allow_multiple: false,
            options: Vec::new(),
            name: std::mem::take(&mut config.name),
            description: std::mem::take(&mut config.description),
            popularity: config.popularity,
        };
        if let Some(multiple) = get_attr(doc, group, "allowMultipleSelection") {
            if multiple == "true" {
                og.allow_multiple = true;
            }
        }
        ctx.option_groups.push(og);
        group_idx = ctx.option_groups.len() - 1;
    }

    for node in doc.children(group) {
        if is_node(doc, node, "option") {
            parse_option(ctx, group_idx, doc, node, popularity);
        }
    }
}

// ---------------------------------------------------------------------------
// DTD
// ---------------------------------------------------------------------------

const XKBCONFIG_DTD: &str = "\
<!ELEMENT xkbConfigRegistry (modelList?, layoutList?, optionList?)>\n\
<!ATTLIST xkbConfigRegistry version CDATA \"1.1\">\n\
<!ELEMENT modelList (model*)>\n\
<!ELEMENT model (configItem)>\n\
<!ELEMENT layoutList (layout*)>\n\
<!ELEMENT layout (configItem,  variantList?)>\n\
<!ELEMENT optionList (group*)>\n\
<!ELEMENT variantList (variant*)>\n\
<!ELEMENT variant (configItem)>\n\
<!ELEMENT group (configItem, option*)>\n\
<!ATTLIST group allowMultipleSelection (true|false) \"false\">\n\
<!ELEMENT option (configItem)>\n\
<!ELEMENT configItem (name, shortDescription?, description?, vendor?, countryList?, languageList?, hwList?)>\n\
<!ATTLIST configItem layout-specific (true|false) \"false\">\n\
<!ATTLIST configItem popularity (standard|exotic) #IMPLIED>\n\
<!ELEMENT name (#PCDATA)>\n\
<!ELEMENT shortDescription (#PCDATA)>\n\
<!ELEMENT description (#PCDATA)>\n\
<!ELEMENT vendor (#PCDATA)>\n\
<!ELEMENT countryList (iso3166Id+)>\n\
<!ELEMENT iso3166Id (#PCDATA)>\n\
<!ELEMENT languageList (iso639Id+)>\n\
<!ELEMENT iso639Id (#PCDATA)>\n\
<!ELEMENT hwList (hwId+)>\n\
<!ELEMENT hwId (#PCDATA)>";

// ---------------------------------------------------------------------------
// XML file parsing
// ---------------------------------------------------------------------------

fn parse_xml_file(ctx: &mut RxkbContext, path: &str, popularity: RxkbPopularity) -> bool {
    let mut doc = match xmloxide::Document::parse_file(path) {
        Ok(d) => d,
        Err(_) => return false,
    };
    // Validate
    {
        let dtd = match xmloxide::validation::dtd::parse_dtd(XKBCONFIG_DTD) {
            Ok(dtd) => dtd,
            Err(_) => {
                rxkb_logf!(ctx, RXKB_LOG_LEVEL_ERROR, "Failed to load DTD\n");
                return false;
            }
        };
        let result = xmloxide::validation::dtd::validate(&mut doc, &dtd);
        if !result.is_valid {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "XML error: failed to validate document at {}\n",
                path,
            );
            return false;
        }
    }
    let root = match doc.root_element() {
        Some(r) => r,
        None => return false,
    };
    for node in doc.children(root) {
        if is_node(&doc, node, "modelList") {
            for mnode in doc.children(node) {
                if is_node(&doc, mnode, "model") {
                    parse_model(ctx, &doc, mnode, popularity);
                }
            }
        } else if is_node(&doc, node, "layoutList") {
            for lnode in doc.children(node) {
                if is_node(&doc, lnode, "layout") {
                    parse_layout(ctx, &doc, lnode, popularity);
                }
            }
        } else if is_node(&doc, node, "optionList") {
            for onode in doc.children(node) {
                if is_node(&doc, onode, "group") {
                    parse_group(ctx, &doc, onode, popularity);
                }
            }
        }
    }
    true
}
