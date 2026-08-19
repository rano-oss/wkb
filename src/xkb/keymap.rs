use std::borrow::Cow;

use arrayvec::ArrayVec;

use crate::xkb::keysym::keysym_to_codepoint;

pub(crate) use super::parser::{
    XkbContext, XkbKeymap, XkbModSet, XkbRuleNames, MOD_REAL, MOD_REAL_MASK_ALL,
};

pub(crate) fn xkb_keymap_new_from_names(
    ctx: XkbContext,
    rmlvo: &XkbRuleNames,
) -> Option<XkbKeymap> {
    let mut rmlvo = rmlvo.clone();
    xkb_context_sanitize_rule_names(&mut rmlvo);
    let mut keymap = xkb_keymap_new(ctx, false);
    let mut components = XkbComponentNames::default();
    let mut matcher = matcher_new_from_names(&mut keymap.ctx, &rmlvo);
    xkb_resolve_rules(
        &rmlvo.rules,
        &mut matcher,
        &mut components,
        &mut keymap.num_groups,
    )
    .then_some(())?;
    keymap.num_groups = keymap.num_groups.min(XKB_MAX_GROUPS);
    let mut file = xkb_file_from_components(&components)?;
    (file.file_type == FileType::Keymap && compile_keymap(&mut file, &mut keymap)).then_some(())?;
    Some(*keymap)
}
pub(crate) fn xkb_keymap_new_from_string(ctx: XkbContext, original: &[u8]) -> Option<XkbKeymap> {
    let source = strip_compat_map(original);
    let bytes = source.as_ref();
    if bytes.is_empty() {
        return None;
    }
    let mut keymap = xkb_keymap_new(ctx, true);
    let mut file = xkb_parse_string(&mut keymap.ctx, bytes, "")?;
    (file.file_type == FileType::Keymap && compile_keymap(&mut file, &mut keymap)).then_some(())?;
    apply_group_action_overrides(&mut keymap, original);
    Some(*keymap)
}

fn find_ascii_case_insensitive(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window.eq_ignore_ascii_case(needle))
}

fn group_action_from_interpret(input: &[u8], name: &[u8]) -> Option<XkbAction> {
    let tail = &input[find_ascii_case_insensitive(input, name)? + name.len()..];
    let open = tail.iter().position(|&b| b == b'{')?;
    let body = &tail[open + 1..];
    let body = &body[..body.iter().position(|&b| b == b'}')?];
    let (constructor, tail): (fn(XkbGroupAction) -> XkbAction, &[u8]) = [
        (XkbAction::GroupSet as _, b"SetGroup".as_slice()),
        (XkbAction::GroupLatch as _, b"LatchGroup".as_slice()),
        (XkbAction::GroupLock as _, b"LockGroup".as_slice()),
    ]
    .into_iter()
    .find_map(|(make, action)| {
        find_ascii_case_insensitive(body, action).map(|pos| (make, &body[pos + action.len()..]))
    })?;
    let tail = &tail[find_ascii_case_insensitive(tail, b"group")? + 5..];
    let mut value = &tail[tail.iter().position(|&b| b == b'=')? + 1..];
    value = &value[value.iter().position(|b| !b.is_ascii_whitespace())?..];
    let relative = matches!(value.first(), Some(b'+' | b'-'));
    let negative = value.first() == Some(&b'-');
    let digits = if relative { &value[1..] } else { value };
    let end = digits
        .iter()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(digits.len());
    let number = std::str::from_utf8(&digits[..end])
        .ok()?
        .parse::<i32>()
        .ok()?;
    let group = if relative {
        number * if negative { -1 } else { 1 }
    } else {
        number - 1
    };
    let action = XkbGroupAction {
        flags: if relative {
            ActionFlags::empty()
        } else {
            ActionFlags::ABSOLUTE_SWITCH
        },
        group,
    };
    Some(constructor(action))
}

fn apply_group_action_overrides(keymap: &mut XkbKeymap, input: &[u8]) {
    // xkbcommon versions differ in whether the generated symbols section also
    // contains the interpreted action. An explicit relative NextGroup latch in
    // the source must win over either representation.
    let forced_next_latch = find_ascii_case_insensitive(input, b"LatchGroup(group=+1)").map(|_| {
        XkbAction::GroupLatch(XkbGroupAction {
            flags: ActionFlags::empty(),
            group: 1,
        })
    });
    let overrides: Vec<(u32, XkbAction)> = [
        (0xff7e, b"Mode_switch".as_slice()),
        (0xff2d, b"Kana_Lock".as_slice()),
        (0xfe06, b"ISO_Group_Latch".as_slice()),
        (0xfe08, b"ISO_Next_Group".as_slice()),
        (0xfe0a, b"ISO_Prev_Group".as_slice()),
        (0xfe0c, b"ISO_First_Group".as_slice()),
        (0xfe0e, b"ISO_Last_Group".as_slice()),
    ]
    .into_iter()
    .filter_map(|(sym, name)| group_action_from_interpret(input, name).map(|action| (sym, action)))
    .collect();
    if overrides.is_empty() && forced_next_latch.is_none() {
        return;
    }
    for key in &mut keymap.keys {
        for group in &mut key.groups {
            for level in &mut group.levels {
                if level.syms.contains(&0xfe08) {
                    if let Some(action) = forced_next_latch {
                        level.action = Some(action);
                        continue;
                    }
                }
                if let Some((_, action)) =
                    overrides.iter().find(|(sym, _)| level.syms.contains(sym))
                {
                    let synthesized = level.syms.iter().copied().find_map(wkb_group_action);
                    if level.action == synthesized {
                        level.action = Some(*action);
                    }
                }
            }
        }
    }
}

/// Remove the xkb_compat body before parsing a complete keymap.
///
/// Compatibility maps describe the xkb runtime state machine. WKB derives the
/// few observable pieces it needs directly from symbols, so constructing this
/// AST is pure setup overhead.
fn strip_compat_map(input: &[u8]) -> Cow<'_, [u8]> {
    const NAMES: [&[u8]; 2] = [b"xkb_compatibility", b"xkb_compat_map"];
    let Some(start) = (0..input.len()).find(|&start| {
        NAMES.iter().any(|name| {
            input[start..].starts_with(name)
                && input
                    .get(start.wrapping_sub(1))
                    .map_or(true, |b| !b.is_ascii_alphanumeric() && *b != b'_')
                && input
                    .get(start + name.len())
                    .map_or(true, |b| !b.is_ascii_alphanumeric() && *b != b'_')
        })
    }) else {
        return Cow::Borrowed(input);
    };
    let Some(open) = input[start..]
        .iter()
        .position(|&byte| byte == b'{')
        .map(|i| start + i)
    else {
        return Cow::Borrowed(input);
    };

    let mut depth = 1;
    for pos in open + 1..input.len() {
        depth += usize::from(input[pos] == b'{');
        depth -= usize::from(input[pos] == b'}');
        if depth == 0 {
            let mut stripped = Vec::with_capacity(input.len() - (pos - open));
            stripped.extend_from_slice(&input[..=open]);
            stripped.push(b'\n');
            stripped.extend_from_slice(&input[pos..]);
            return Cow::Owned(stripped);
        }
    }
    Cow::Borrowed(input)
}

use std::{fs, path::Path};

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
pub(crate) fn keysym_name_to_char(name: &str) -> Option<char> {
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
    fs::read_to_string(Path::new(LOCALE_DIR).join(filename))
        .ok()?
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .find_map(|line| {
            (line.split_whitespace().nth(match_index) == Some(locale))
                .then(|| line.split_whitespace().nth(return_index).map(str::to_owned))
                .flatten()
        })
}

fn lookup_compose_dir(locale: &str) -> Option<String> {
    lookup_locale_file("compose.dir", 1, 0, locale)
}

/// Resolve a locale name to the compose file sub-path (relative to
/// `/usr/share/X11/locale/`) that should be used.
pub(crate) fn resolve_compose_file(locale: &str) -> Option<String> {
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

    if locale.len() >= 2 && locale.len() <= 5 && locale.chars().all(|c| c.is_ascii_lowercase()) {
        #[rustfmt::skip]
        const LANGUAGES: &[(&str, &str)] = &[("us","en"),("gb","en"),("au","en"),("nz","en"),("za","en"),("bw","en"),("no","nb"),("dk","da"),("se","sv"),("at","de"),("ch","de"),("cz","cs"),("gr","el"),("rs","sr"),("me","sr"),("al","sq"),("ba","bs"),("by","be"),("ge","ka"),("ua","uk"),("jp","ja"),("kr","ko"),("cn","zh"),("tw","zh"),("kh","km"),("vn","vi"),("in","hi"),("bd","bn"),("lk","si"),("np","ne"),("pk","ur"),("il","he"),("ara","ar"),("iq","ar"),("sy","ar"),("eg","ar"),("dz","ar"),("ma","ar"),("ir","fa"),("kg","ky"),("kz","kk"),("tj","tg"),("la","lo"),("my","ms"),("ie","ga"),("epo","eo"),("latam","es")];
        let language = LANGUAGES
            .iter()
            .find_map(|&(name, language)| (name == locale).then_some(language))
            .unwrap_or(locale);
        let country = [("ara", "SA"), ("epo", "XX"), ("latam", "MX")]
            .into_iter()
            .find_map(|(name, country)| (name == locale).then_some(country.to_owned()))
            .unwrap_or_else(|| locale.to_ascii_uppercase());
        let candidate = format!("{language}_{country}.UTF-8");
        if let Some(compose_file) = lookup_compose_dir(&candidate) {
            return Some(compose_file);
        }
    }

    lookup_compose_dir("en_US.UTF-8")
}
pub(crate) fn xkb_keymap_new(ctx: XkbContext, strict: bool) -> Box<XkbKeymap> {
    let mut keymap = Box::new(XkbKeymap {
        ctx,
        strict,
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
    #[rustfmt::skip]
    static BUILTIN_MODS: [&str; 8] = ["Shift", "Lock", "Control", "Mod1", "Mod2", "Mod3", "Mod4", "Mod5"];
    for (i, name) in BUILTIN_MODS.iter().enumerate() {
        keymap.mods.mods[i].name = keymap.ctx.atom_intern(name.as_bytes());
        keymap.mods.mods[i].type_0 = MOD_REAL;
        keymap.mods.mods[i].mapping = 1_u32 << i;
    }
    keymap.mods.num_mods = BUILTIN_MODS.len() as u32;
    keymap
}

pub(crate) fn xkb_mod_name_to_index(mods: &XkbModSet, name: u32, type_0: u32) -> Option<u32> {
    for (i, mod_0) in mods.mods[..mods.num_mods as usize].iter().enumerate() {
        if mod_0.type_0 & type_0 != 0 && name == mod_0.name {
            return Some(i as u32);
        }
    }
    None
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

use super::parser::{DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_LEGACY_ROOT};
fn context_include_path_append(ctx: &mut XkbContext, path: &str) -> bool {
    if std::fs::metadata(path).is_ok_and(|metadata| metadata.is_dir()) {
        ctx.includes.push(path.to_string());
        true
    } else {
        false
    }
}

fn xkb_context_include_path_append_default(ctx: &mut XkbContext) -> bool {
    let home = std::env::var("HOME");
    let xdg = std::env::var("XDG_CONFIG_HOME");
    if let Ok(ref xdg) = xdg {
        context_include_path_append(ctx, &format!("{}/xkb", xdg));
    } else if let Ok(ref home) = home {
        context_include_path_append(ctx, &format!("{}/.config/xkb", home));
    }
    if let Ok(ref home) = home {
        context_include_path_append(ctx, &format!("{}/.xkb", home));
    }
    context_include_path_append(
        ctx,
        &getenv_or("XKB_CONFIG_EXTRA_PATH", DFLT_XKB_CONFIG_EXTRA_PATH),
    );

    let root = getenv_or("XKB_CONFIG_ROOT", DFLT_XKB_CONFIG_ROOT);
    if !context_include_path_append(ctx, &root) && !root.is_empty() {
        context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT);
    }
    !ctx.includes.is_empty()
}

pub(crate) fn xkb_context_new() -> XkbContext {
    let mut ctx = XkbContext {
        includes: Vec::new(),
        atom_table: Default::default(),
    };
    xkb_context_include_path_append_default(&mut ctx);
    ctx
}

pub(crate) fn getenv_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.into())
}
pub(crate) fn xkb_context_sanitize_rule_names(rmlvo: &mut XkbRuleNames) {
    for (value, name, default) in [
        (&mut rmlvo.rules, "XKB_DEFAULT_RULES", "evdev"),
        (&mut rmlvo.model, "XKB_DEFAULT_MODEL", "pc105"),
        (&mut rmlvo.options, "XKB_DEFAULT_OPTIONS", ""),
    ] {
        if value.is_empty() {
            *value = getenv_or(name, default);
        }
    }
    if rmlvo.layout.is_empty() {
        let layout = std::env::var("XKB_DEFAULT_LAYOUT").ok();
        rmlvo.variant = layout
            .as_ref()
            .and_then(|_| std::env::var("XKB_DEFAULT_VARIANT").ok())
            .unwrap_or_default();
        rmlvo.layout = layout.unwrap_or_else(|| "us".into());
    }
}

use super::parser::*;
pub(crate) const GROUP_LAST_INDEX_NAME: &str = "last";

pub(crate) fn mod_mask_get_effective(mod_set: &XkbModSet, mods: u32) -> u32 {
    let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
    for i in _XKB_MOD_INDEX_NUM_ENTRIES..mod_set.num_mods {
        if mods & 1 << i != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
    }
    mask
}
