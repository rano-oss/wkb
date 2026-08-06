use std::rc::Rc;
use std::sync::LazyLock;

use arrayvec::ArrayVec;

pub use super::parser::XKB_KEYMAP_COMPILE_FLAGS_VALUES;
pub(crate) use super::parser::{
    XkbAction, XkbContext, XkbKeymap, XkbLed, XkbLevel, XkbModSet, XkbRuleNames, MOD_REAL, MOD_REAL_MASK_ALL, XKB_KEYMAP_FORMAT_TEXT_V2,
};

pub(crate) fn xkb_keymap_new_from_names(
    ctx: XkbContext,
    rmlvo: &XkbRuleNames,
    flags: u32,
) -> Option<Rc<XkbKeymap>> {
    let format = XKB_KEYMAP_FORMAT_TEXT_V2;
    let mut rmlvo = rmlvo.clone();
    xkb_context_sanitize_rule_names(&ctx, &mut rmlvo);
    let mut keymap = xkb_keymap_new(ctx, "xkb_keymap_new_from_names2", format, flags)?;
    if !super::parser::text_v1_keymap_new_from_names(&mut keymap, &rmlvo) {
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
        return None;
    }
    let mut keymap = xkb_keymap_new(ctx, "xkb_keymap_new_from_buffer", format, flags)?;
    if length > 0 && bytes[length - 1] == 0 {
        length -= 1;
    }
    if !super::parser::text_v1_keymap_new_from_string(&mut keymap, &bytes[..length]) {
        return None;
    }
    Some(Rc::new(*keymap))
}

use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

const LOCALE_DIR: &str = "/usr/share/X11/locale";

/// A parsed Compose file entry.
#[derive(Clone)]
pub struct ComposeEntry {
    pub keys: ArrayVec<char, 8>,
    pub multi_key_index: Option<usize>,
    pub output: char,
}

/// Resolve an XKB keysym name to its Unicode character using our existing
/// keysym database.
pub fn keysym_name_to_char(name: &str) -> Option<char> {
    // Fast path: single ASCII alphanumeric maps to itself (most compose key names)
    if name.len() == 1 {
        let b = name.as_bytes()[0];
        if b.is_ascii_alphanumeric() {
            return Some(b as char);
        }
    }

    use super::keysym::keysym_to_utf32;
    use super::keysym::xkb_keysym_from_name;
    use super::parser::XKB_KEYSYM_NO_FLAGS;

    let ks = match xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS) {
        Some(ks) => ks,
        None => {
            if let Some(hex) = name.strip_prefix('U') {
                if !hex.is_empty() && hex.len() <= 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
                }
            }
            return None;
        }
    };
    let utf32 = keysym_to_utf32(ks);
    if utf32 == 0 {
        return None;
    }
    char::from_u32(utf32)
}

pub(crate) fn parse_compose_file_impl<F>(path: &Path, f: &mut F) -> bool
where
    F: FnMut(ComposeEntry),
{
    use super::parser::read_file_cached;

    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };
    let Some(data) = read_file_cached(path_str) else {
        return false;
    };
    let content = match std::str::from_utf8(&data[..]) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let mut complete = true;
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
                complete &= parse_compose_file_impl(&resolved, f);
            }
            continue;
        }

        if let Some(entry) = parse_rule_line(trimmed) {
            f(entry);
        }
    }
    complete
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

    let mut keys: ArrayVec<char, 8> = ArrayVec::new();
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
    _func: &str,
    format: u32,
    flags: u32,
) -> Option<Box<XkbKeymap>> {
    static XKB_KEYMAP_COMPILE_FLAGS: u32 = XKB_KEYMAP_COMPILE_FLAGS_VALUES;
    if flags & !XKB_KEYMAP_COMPILE_FLAGS != 0 {
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
        types: Vec::new(),
        mods: XkbModSet::default(),
        num_groups: 0,
        group_names: Vec::new(),
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
        keymap.mods.mods[i].name = atom_intern(&mut keymap.ctx.atom_table, name.as_bytes());
        keymap.mods.mods[i].type_0 = MOD_REAL;
        keymap.mods.mods[i].mapping = 1_u32 << i;
    }
    keymap.mods.num_mods = BUILTIN_MODS.len() as u32;
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

pub(crate) fn xkb_mod_name_to_index(mods: &XkbModSet, name: u32, type_0: u32) -> Option<u32> {
    for (i, mod_0) in mods.mods[..mods.num_mods as usize].iter().enumerate() {
        if mod_0.type_0 & type_0 != 0 && name == mod_0.name {
            return Some(i as u32);
        }
    }
    None
}
pub(crate) fn xkb_levels_same_syms(a: &XkbLevel, b: &XkbLevel) -> bool {
    a.syms == b.syms
}
pub(crate) fn action_equal(a: &XkbAction, b: &XkbAction) -> bool {
    match (a, b) {
        (XkbAction::None, XkbAction::None) | (XkbAction::Void, XkbAction::Void) => true,
        (
            XkbAction::ModSet(am) | XkbAction::ModLatch(am) | XkbAction::ModLock(am),
            XkbAction::ModSet(bm) | XkbAction::ModLatch(bm) | XkbAction::ModLock(bm),
        ) => am.flags == bm.flags && am.mods.mask == bm.mods.mask && am.mods.mods == bm.mods.mods,
        (
            XkbAction::GroupSet(ag) | XkbAction::GroupLatch(ag) | XkbAction::GroupLock(ag),
            XkbAction::GroupSet(bg) | XkbAction::GroupLatch(bg) | XkbAction::GroupLock(bg),
        ) => ag.flags == bg.flags && ag.group == bg.group,
        (
            XkbAction::CtrlSet(ac) | XkbAction::CtrlLock(ac),
            XkbAction::CtrlSet(bc) | XkbAction::CtrlLock(bc),
        ) => ac.flags == bc.flags && ac.ctrls == bc.ctrls,
        (XkbAction::Internal(ai), XkbAction::Internal(bi)) => {
            ai.flags == bi.flags && ai.clear_latched_mods == bi.clear_latched_mods
        }
        (XkbAction::Private(ap), XkbAction::Private(bp)) => ap.data == bp.data,
        _ => false,
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
) -> Option<u32> {
    if num_groups == 0 {
        return None;
    }
    if group >= 0_i32 && (group as u32) < num_groups {
        return Some(group as u32);
    }
    match out_of_range_group_policy {
        2 => {
            if out_of_range_group_number >= num_groups {
                return Some(0);
            }
            Some(out_of_range_group_number)
        }
        1 => {
            if group < 0_i32 {
                Some(0_u32)
            } else {
                Some(num_groups.wrapping_sub(1))
            }
        }
        _ => {
            let rem: i32 = group % num_groups as i32;
            Some(
                (if rem >= 0_i32 {
                    rem
                } else {
                    rem + num_groups as i32
                }) as u32,
            )
        }
    }
}

use std::env::VarError;

use super::parser::{atom_intern, atom_table_new};

use super::parser::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub(crate) use super::parser::{
    RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
fn context_include_path_append(ctx: &mut XkbContext, path: &str) -> i32 {
    let is_dir = std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false);
    if is_dir {
        ctx.includes.push(path.to_string());
        return 1;
    }
    if !path.is_empty() {
        ctx.failed_includes.push(path.to_string());
    }
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
        Err(_e) => {
            return 0;
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

    let mut ret = 0;
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
        let mut ret: i32 = 0;
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

pub(crate) fn xkb_context_new(flags: u32) -> XkbContext {
    let mut ctx = XkbContext {
        includes: Vec::new(),
        failed_includes: Vec::new(),
        atom_table: atom_table_new(),
        use_environment_names: false,
        use_secure_getenv: false,
        pending_default_includes: false,
    };
    const XKB_CONTEXT_ALL_FLAGS: u32 = XKB_CONTEXT_NO_DEFAULT_INCLUDES
        | XKB_CONTEXT_NO_ENVIRONMENT_NAMES
        | XKB_CONTEXT_NO_SECURE_GETENV;
    if flags & !XKB_CONTEXT_ALL_FLAGS != 0 {
        return ctx;
    }
    ctx.use_environment_names = flags & XKB_CONTEXT_NO_ENVIRONMENT_NAMES == 0;
    ctx.use_secure_getenv = flags & XKB_CONTEXT_NO_SECURE_GETENV == 0;
    ctx.pending_default_includes = flags & XKB_CONTEXT_NO_DEFAULT_INCLUDES == 0;
    ctx
}

// --- Merged from context_priv.rs ---

pub(crate) fn xkb_context_getenv(name: &str) -> Result<String, VarError> {
    std::env::var(name)
}
pub(crate) fn xkb_context_init_includes(ctx: &mut XkbContext) -> bool {
    if ctx.pending_default_includes {
        if ctx.failed_includes.is_empty() {
            if xkb_context_include_path_append_default(ctx) == 0 {
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

pub(crate) fn xkb_context_sanitize_rule_names(ctx: &XkbContext, rmlvo: &mut XkbRuleNames) -> u32 {
    let mut modified: u32 = 0_u32;
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
        modified |= RMLVO_RULES;
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
        modified |= RMLVO_MODEL;
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
        modified |= RMLVO_LAYOUT;
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
        rmlvo.variant = variant;
        modified |= RMLVO_VARIANT;
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
        modified |= RMLVO_OPTIONS;
    }
    modified
}

use super::parser::*;
pub(crate) const CONTROL_NAMES_MIN_V2_INDEX: u32 = 0;
pub(crate) const CONTROL_NAMES_MIN_V1_INDEX: u32 = 7;
pub(crate) const GROUP_LAST_INDEX_NAME: &str = "last";

pub use super::parser::{
    ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK,
    ACTION_TYPE_GROUP_SET, ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET,
    ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT,
    ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT,
    ACTION_TYPE_TERMINATE, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE,
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
pub(crate) static CTRL_MASK_NAMES: [LookupEntry; 25] = [
    LookupEntry {
        name: "Overlay3",
        value: ControlsFlags::OVERLAY3.bits(),
    },
    LookupEntry {
        name: "Overlay4",
        value: ControlsFlags::OVERLAY4.bits(),
    },
    LookupEntry {
        name: "Overlay5",
        value: ControlsFlags::OVERLAY5.bits(),
    },
    LookupEntry {
        name: "Overlay6",
        value: ControlsFlags::OVERLAY6.bits(),
    },
    LookupEntry {
        name: "Overlay7",
        value: ControlsFlags::OVERLAY7.bits(),
    },
    LookupEntry {
        name: "Overlay8",
        value: ControlsFlags::OVERLAY8.bits(),
    },
    LookupEntry {
        name: "all",
        value: ControlsFlags::ALL_BOOLEAN.bits(),
    },
    LookupEntry {
        name: "RepeatKeys",
        value: ControlsFlags::REPEAT.bits(),
    },
    LookupEntry {
        name: "Repeat",
        value: ControlsFlags::REPEAT.bits(),
    },
    LookupEntry {
        name: "AutoRepeat",
        value: ControlsFlags::REPEAT.bits(),
    },
    LookupEntry {
        name: "SlowKeys",
        value: ControlsFlags::SLOW.bits(),
    },
    LookupEntry {
        name: "BounceKeys",
        value: ControlsFlags::DEBOUNCE.bits(),
    },
    LookupEntry {
        name: "StickyKeys",
        value: ControlsFlags::STICKY_KEYS.bits(),
    },
    LookupEntry {
        name: "MouseKeys",
        value: ControlsFlags::MOUSE_KEYS.bits(),
    },
    LookupEntry {
        name: "MouseKeysAccel",
        value: ControlsFlags::MOUSE_KEYS_ACCEL.bits(),
    },
    LookupEntry {
        name: "AccessXKeys",
        value: ControlsFlags::AX.bits(),
    },
    LookupEntry {
        name: "AccessXTimeout",
        value: ControlsFlags::AX_TIMEOUT.bits(),
    },
    LookupEntry {
        name: "AccessXFeedback",
        value: ControlsFlags::AX_FEEDBACK.bits(),
    },
    LookupEntry {
        name: "AudibleBell",
        value: ControlsFlags::BELL.bits(),
    },
    LookupEntry {
        name: "IgnoreGroupLock",
        value: ControlsFlags::IGNORE_GROUP_LOCK.bits(),
    },
    LookupEntry {
        name: "Overlay1",
        value: ControlsFlags::OVERLAY1.bits(),
    },
    LookupEntry {
        name: "Overlay2",
        value: ControlsFlags::OVERLAY2.bits(),
    },
    LookupEntry {
        name: "all",
        value: ControlsFlags::ALL_BOOLEAN_V1.bits(),
    },
    LookupEntry {
        name: "none",
        value: 0,
    },
    LookupEntry { name: "", value: 0 },
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
        value: 0,
    },
    LookupEntry { name: "", value: 0 },
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
        value: 0,
    },
    LookupEntry { name: "", value: 0 },
];

pub(crate) static USE_MOD_MAP_VALUE_NAMES: [LookupEntry; 5] = [
    LookupEntry {
        name: "LevelOne",
        value: 1,
    },
    LookupEntry {
        name: "Level1",
        value: 1,
    },
    LookupEntry {
        name: "AnyLevel",
        value: 0,
    },
    LookupEntry {
        name: "any",
        value: 0,
    },
    LookupEntry { name: "", value: 0 },
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
    LookupEntry { name: "", value: 0 },
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
    LookupEntry { name: "", value: 0 },
];
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
pub fn preprocess_unicode_keysyms(input: &str) -> std::borrow::Cow<'_, str> {
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

pub(crate) fn mod_mask_get_effective(keymap: &XkbKeymap, mods: u32) -> u32 {
    let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
    for i in _XKB_MOD_INDEX_NUM_ENTRIES..keymap.mods.num_mods {
        if mods & 1 << i != 0 {
            mask |= keymap.mods.mods[i as usize].mapping;
        }
    }
    mask
}
