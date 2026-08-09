use std::rc::Rc;

use arrayvec::ArrayVec;

use crate::xkb::keysym::keysym_to_codepoint;

pub(crate) use super::parser::{
    XkbAction, XkbContext, XkbKeymap, XkbLed, XkbModSet, XkbRuleNames, MOD_REAL, MOD_REAL_MASK_ALL,
    XKB_KEYMAP_FORMAT_TEXT_V2,
};

pub(crate) fn xkb_keymap_new_from_names(
    ctx: XkbContext,
    rmlvo: &XkbRuleNames,
    flags: u32,
) -> Option<Rc<XkbKeymap>> {
    let format = XKB_KEYMAP_FORMAT_TEXT_V2;
    let mut rmlvo = rmlvo.clone();
    xkb_context_sanitize_rule_names(&ctx, &mut rmlvo);
    let mut keymap = xkb_keymap_new(ctx, format, flags)?;
    let mut components = XkbComponentNames::default();
    xkb_components_from_rules_names(
        &mut keymap.ctx,
        &rmlvo,
        &mut components,
        &mut keymap.num_groups,
    )
    .then_some(())?;
    keymap.num_groups = keymap.num_groups.min(XKB_MAX_GROUPS);
    let mut file = xkb_file_from_components(&components)?;
    (file.file_type == FileType::Keymap && compile_keymap(&mut file, &mut keymap)).then_some(())?;
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
    let mut keymap = xkb_keymap_new(ctx, format, flags)?;
    if length > 0 && bytes[length - 1] == 0 {
        length -= 1;
    }
    let mut file = xkb_parse_string(&mut keymap.ctx, &bytes[..length], "")?;
    (file.file_type == FileType::Keymap && compile_keymap(&mut file, &mut keymap)).then_some(())?;
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
    let utf32 = keysym_to_codepoint(ks).unwrap_or(0);
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
    for item in lhs.split('<').skip(1) {
        let (name, _) = item.split_once('>')?;
        if name.eq_ignore_ascii_case("Multi_key") {
            if multi_key_index.is_none() {
                multi_key_index = Some(keys.len());
            }
        } else {
            keys.push(keysym_name_to_char(name)?);
        }
    }
    (!keys.is_empty()).then_some(())?;
    let output = parse_rhs_value(rhs)?;

    Some(ComposeEntry {
        keys,
        multi_key_index,
        output,
    })
}

/// Parse the RHS value: `"string" [keysym]` or bare `keysym_name`
fn parse_rhs_value(rhs: &str) -> Option<char> {
    let rhs = rhs.trim();
    if let Some(quoted) = rhs.strip_prefix('"') {
        let (s, after) = quoted.split_once('"')?;
        if let Some(ch) = s.chars().next() {
            if !s.starts_with('\\') && !ch.is_ascii_digit() {
                return Some(ch);
            }
        }
        if let Some(name) = after.split_whitespace().next() {
            return keysym_name_to_char(name);
        }
        s.chars().next()
    } else {
        keysym_name_to_char(rhs.split_whitespace().next()?)
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
    if let Some(mapped_locale) = XKB_COMPOSE_MAP
        .iter()
        .find_map(|&(name, mapped)| (name == locale).then_some(mapped))
    {
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

static XKB_COMPOSE_MAP: &[(&str, &str)] = &[
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
];
pub(crate) const XKB_MOD_NAME_SHIFT: &str = "Shift";
pub(crate) const XKB_MOD_NAME_CAPS: &str = "Lock";
pub(crate) const XKB_MOD_NAME_CTRL: &str = "Control";
pub(crate) const XKB_MOD_NAME_MOD1: &str = "Mod1";
pub(crate) const XKB_MOD_NAME_MOD2: &str = "Mod2";
pub(crate) const XKB_MOD_NAME_MOD3: &str = "Mod3";
pub(crate) const XKB_MOD_NAME_MOD4: &str = "Mod4";
pub(crate) const XKB_MOD_NAME_MOD5: &str = "Mod5";

pub(crate) fn xkb_keymap_new(ctx: XkbContext, format: u32, flags: u32) -> Option<Box<XkbKeymap>> {
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
        keymap.mods.mods[i].name = keymap.ctx.atom_intern(name.as_bytes());
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
pub(crate) fn xkb_wrap_group_into_range(
    group: i32,
    num_groups: u32,
    out_of_range_group_policy: u32,
    out_of_range_group_number: u32,
) -> Option<u32> {
    if num_groups == 0 {
        return None;
    }
    if group >= 0 && group < num_groups as i32 {
        return Some(group as u32);
    }
    match out_of_range_group_policy {
        2 => Some(if out_of_range_group_number < num_groups {
            out_of_range_group_number
        } else {
            0
        }),
        1 => Some(if group < 0 { 0 } else { num_groups - 1 }),
        _ => {
            let rem = group % num_groups as i32;
            Some(if rem < 0 {
                (rem + num_groups as i32) as u32
            } else {
                rem as u32
            })
        }
    }
}

use super::parser::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
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

pub(crate) fn xkb_context_include_path_append_default(ctx: &mut XkbContext) -> i32 {
    let mut ret: i32 = 0;
    let home = std::env::var("HOME");
    let xdg = std::env::var("XDG_CONFIG_HOME");
    if let Ok(ref xdg) = xdg {
        ret |= context_include_path_append(ctx, &format!("{}/xkb", xdg));
    } else if let Ok(ref home) = home {
        ret |= context_include_path_append(ctx, &format!("{}/.config/xkb", home));
    }
    if let Ok(ref home) = home {
        ret |= context_include_path_append(ctx, &format!("{}/.xkb", home));
    }
    ret |= context_include_path_append(
        ctx,
        &getenv_or("XKB_CONFIG_EXTRA_PATH", DFLT_XKB_CONFIG_EXTRA_PATH),
    );

    let mut extensions: Vec<String> = Vec::new();
    let versioned_path = getenv_or(
        "XKB_CONFIG_VERSIONED_EXTENSIONS_PATH",
        DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH,
    );
    if !versioned_path.is_empty() {
        ret |= add_direct_subdirectories(ctx, &versioned_path, &mut extensions, 0, 0);
    }
    let unversioned_path = getenv_or(
        "XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH",
        DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    );
    if !unversioned_path.is_empty() {
        let versioned_count = extensions.len();
        ret |= add_direct_subdirectories(
            ctx,
            &unversioned_path,
            &mut extensions,
            versioned_count,
            versioned_path.len(),
        );
    }

    let root = getenv_or("XKB_CONFIG_ROOT", DFLT_XKB_CONFIG_ROOT);
    let has_root = context_include_path_append(ctx, &root) != 0;
    ret |= has_root as i32;
    if !has_root && !root.is_empty() {
        ret |= context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT);
    }
    ret
}

pub(crate) fn xkb_context_num_include_paths(ctx: &mut XkbContext) -> u32 {
    if ctx.pending_default_includes {
        if !ctx.failed_includes.is_empty() || xkb_context_include_path_append_default(ctx) == 0 {
            return 0;
        }
        ctx.pending_default_includes = false;
    }
    ctx.includes.len() as u32
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
        atom_table: Default::default(),
        use_environment_names: false,
        pending_default_includes: false,
    };
    const XKB_CONTEXT_ALL_FLAGS: u32 = XKB_CONTEXT_NO_DEFAULT_INCLUDES
        | XKB_CONTEXT_NO_ENVIRONMENT_NAMES
        | XKB_CONTEXT_NO_SECURE_GETENV;
    if flags & !XKB_CONTEXT_ALL_FLAGS != 0 {
        return ctx;
    }
    ctx.use_environment_names = flags & XKB_CONTEXT_NO_ENVIRONMENT_NAMES == 0;
    ctx.pending_default_includes = flags & XKB_CONTEXT_NO_DEFAULT_INCLUDES == 0;
    ctx
}

pub(crate) fn getenv_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.into())
}
pub(crate) fn xkb_context_sanitize_rule_names(ctx: &XkbContext, rmlvo: &mut XkbRuleNames) {
    for (value, name, default) in [
        (&mut rmlvo.rules, "XKB_DEFAULT_RULES", "evdev"),
        (&mut rmlvo.model, "XKB_DEFAULT_MODEL", "pc105"),
        (&mut rmlvo.options, "XKB_DEFAULT_OPTIONS", ""),
    ] {
        if value.is_empty() {
            *value = if ctx.use_environment_names {
                getenv_or(name, default)
            } else {
                default.into()
            };
        }
    }
    if rmlvo.layout.is_empty() {
        let layout = ctx
            .use_environment_names
            .then(|| std::env::var("XKB_DEFAULT_LAYOUT").ok())
            .flatten();
        rmlvo.variant = layout
            .as_ref()
            .and_then(|_| std::env::var("XKB_DEFAULT_VARIANT").ok())
            .unwrap_or_default();
        rmlvo.layout = layout.unwrap_or_else(|| "us".into());
    }
}

use super::parser::*;
pub(crate) const GROUP_LAST_INDEX_NAME: &str = "last";

pub(crate) fn lookup_string(tab: &[LookupEntry], string: &str) -> Option<u32> {
    (!string.is_empty()).then_some(())?;
    tab.iter()
        .take_while(|entry| !entry.name.is_empty())
        .find(|entry| entry.name.eq_ignore_ascii_case(string))
        .map(|entry| entry.value)
}
pub(crate) static CTRL_MASK_NAMES: [LookupEntry; 25] = [
    lookup_entry("Overlay3", ControlsFlags::OVERLAY3.bits()),
    lookup_entry("Overlay4", ControlsFlags::OVERLAY4.bits()),
    lookup_entry("Overlay5", ControlsFlags::OVERLAY5.bits()),
    lookup_entry("Overlay6", ControlsFlags::OVERLAY6.bits()),
    lookup_entry("Overlay7", ControlsFlags::OVERLAY7.bits()),
    lookup_entry("Overlay8", ControlsFlags::OVERLAY8.bits()),
    lookup_entry("all", ControlsFlags::ALL_BOOLEAN.bits()),
    lookup_entry("RepeatKeys", ControlsFlags::REPEAT.bits()),
    lookup_entry("Repeat", ControlsFlags::REPEAT.bits()),
    lookup_entry("AutoRepeat", ControlsFlags::REPEAT.bits()),
    lookup_entry("SlowKeys", ControlsFlags::SLOW.bits()),
    lookup_entry("BounceKeys", ControlsFlags::DEBOUNCE.bits()),
    lookup_entry("StickyKeys", ControlsFlags::STICKY_KEYS.bits()),
    lookup_entry("MouseKeys", ControlsFlags::MOUSE_KEYS.bits()),
    lookup_entry("MouseKeysAccel", ControlsFlags::MOUSE_KEYS_ACCEL.bits()),
    lookup_entry("AccessXKeys", ControlsFlags::AX.bits()),
    lookup_entry("AccessXTimeout", ControlsFlags::AX_TIMEOUT.bits()),
    lookup_entry("AccessXFeedback", ControlsFlags::AX_FEEDBACK.bits()),
    lookup_entry("AudibleBell", ControlsFlags::BELL.bits()),
    lookup_entry("IgnoreGroupLock", ControlsFlags::IGNORE_GROUP_LOCK.bits()),
    lookup_entry("Overlay1", ControlsFlags::OVERLAY1.bits()),
    lookup_entry("Overlay2", ControlsFlags::OVERLAY2.bits()),
    lookup_entry("all", ControlsFlags::ALL_BOOLEAN_V1.bits()),
    lookup_entry("none", 0),
    lookup_entry("", 0),
];
pub(crate) static MOD_COMPONENT_MASK_NAMES: [LookupEntry; 8] = [
    lookup_entry("base", XKB_STATE_MODS_DEPRESSED),
    lookup_entry("latched", XKB_STATE_MODS_LATCHED),
    lookup_entry("locked", XKB_STATE_MODS_LOCKED),
    lookup_entry("effective", XKB_STATE_MODS_EFFECTIVE),
    lookup_entry("compat", XKB_STATE_MODS_EFFECTIVE),
    lookup_entry("any", XKB_STATE_MODS_EFFECTIVE),
    lookup_entry("none", 0),
    lookup_entry("", 0),
];
pub(crate) static GROUP_COMPONENT_MASK_NAMES: [LookupEntry; 7] = [
    lookup_entry("base", XKB_STATE_LAYOUT_DEPRESSED),
    lookup_entry("latched", XKB_STATE_LAYOUT_LATCHED),
    lookup_entry("locked", XKB_STATE_LAYOUT_LOCKED),
    lookup_entry("effective", XKB_STATE_LAYOUT_EFFECTIVE),
    lookup_entry("any", XKB_STATE_LAYOUT_EFFECTIVE),
    lookup_entry("none", 0),
    lookup_entry("", 0),
];

pub(crate) static USE_MOD_MAP_VALUE_NAMES: [LookupEntry; 5] = [
    lookup_entry("LevelOne", 1),
    lookup_entry("Level1", 1),
    lookup_entry("AnyLevel", 0),
    lookup_entry("any", 0),
    lookup_entry("", 0),
];

pub static ACTION_TYPE_NAMES: [LookupEntry; 43] = [
    lookup_entry("NoAction", ACTION_TYPE_NONE),
    lookup_entry("VoidAction", ACTION_TYPE_VOID),
    lookup_entry("SetMods", ACTION_TYPE_MOD_SET),
    lookup_entry("LatchMods", ACTION_TYPE_MOD_LATCH),
    lookup_entry("LockMods", ACTION_TYPE_MOD_LOCK),
    lookup_entry("SetGroup", ACTION_TYPE_GROUP_SET),
    lookup_entry("LatchGroup", ACTION_TYPE_GROUP_LATCH),
    lookup_entry("LockGroup", ACTION_TYPE_GROUP_LOCK),
    lookup_entry("MovePtr", ACTION_TYPE_PTR_MOVE),
    lookup_entry("MovePointer", ACTION_TYPE_PTR_MOVE),
    lookup_entry("PtrBtn", ACTION_TYPE_PTR_BUTTON),
    lookup_entry("PointerButton", ACTION_TYPE_PTR_BUTTON),
    lookup_entry("LockPtrBtn", ACTION_TYPE_PTR_LOCK),
    lookup_entry("LockPtrButton", ACTION_TYPE_PTR_LOCK),
    lookup_entry("LockPointerButton", ACTION_TYPE_PTR_LOCK),
    lookup_entry("LockPointerBtn", ACTION_TYPE_PTR_LOCK),
    lookup_entry("SetPtrDflt", ACTION_TYPE_PTR_DEFAULT),
    lookup_entry("SetPointerDefault", ACTION_TYPE_PTR_DEFAULT),
    lookup_entry("Terminate", ACTION_TYPE_TERMINATE),
    lookup_entry("TerminateServer", ACTION_TYPE_TERMINATE),
    lookup_entry("SwitchScreen", ACTION_TYPE_SWITCH_VT),
    lookup_entry("SetControls", ACTION_TYPE_CTRL_SET),
    lookup_entry("LockControls", ACTION_TYPE_CTRL_LOCK),
    lookup_entry("RedirectKey", ACTION_TYPE_REDIRECT_KEY),
    lookup_entry("Redirect", ACTION_TYPE_REDIRECT_KEY),
    lookup_entry("Private", ACTION_TYPE_PRIVATE),
    lookup_entry("ISOLock", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("ActionMessage", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("MessageAction", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("Message", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DeviceBtn", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DevBtn", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DevButton", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DeviceButton", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("LockDeviceBtn", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("LockDevBtn", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("LockDevButton", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("LockDeviceButton", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DeviceValuator", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DevVal", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DeviceVal", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("DevValuator", ACTION_TYPE_UNSUPPORTED_LEGACY),
    lookup_entry("", 0),
];
pub(crate) static SYM_INTERPRET_MATCH_MASK_NAMES: [LookupEntry; 6] = [
    lookup_entry("NoneOf", MATCH_NONE),
    lookup_entry("AnyOfOrNone", MATCH_ANY_OR_NONE),
    lookup_entry("AnyOf", MATCH_ANY),
    lookup_entry("AllOf", MATCH_ALL),
    lookup_entry("Exactly", MATCH_EXACTLY),
    lookup_entry("", 0),
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

pub(crate) fn mod_mask_get_effective(mod_set: &XkbModSet, mods: u32) -> u32 {
    let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
    for i in _XKB_MOD_INDEX_NUM_ENTRIES..mod_set.num_mods {
        if mods & 1 << i != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
    }
    mask
}
