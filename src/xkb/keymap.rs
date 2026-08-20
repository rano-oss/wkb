pub(crate) use super::parser::{
    XkbContext, XkbKeymap, XkbModSet, XkbRuleNames, MOD_REAL, MOD_REAL_MASK_ALL,
};
use crate::xkb::keysym::keysym_to_codepoint;
use crate::xkb::parse_xkb::braced_end;
use arrayvec::ArrayVec;
use std::borrow::Cow;
pub(crate) fn xkb_keymap_new_from_names(
    ctx: XkbContext,
    rmlvo: &XkbRuleNames,
) -> Option<XkbKeymap> {
    let mut rmlvo = rmlvo.clone();
    xkb_context_sanitize_rule_names(&mut rmlvo);
    let mut keymap = xkb_keymap_new(ctx, false);
    let layouts: Vec<_> = rmlvo.layout.trim_end_matches(',').split(',').collect();
    let mut variants = rmlvo.variant.trim_end_matches(',').split(',');
    let alias = if layouts
        .first()
        .is_some_and(|layout| matches!(*layout, "be" | "fr"))
    {
        "azerty"
    } else if layouts.first().is_some_and(|layout| {
        matches!(
            *layout,
            "al" | "ch" | "cz" | "de" | "hr" | "hu" | "ro" | "si" | "sk"
        )
    }) {
        "qwertz"
    } else {
        "qwerty"
    };
    let mut symbols = String::from("pc");
    for (index, layout) in layouts.iter().take(XKB_MAX_GROUPS as usize).enumerate() {
        symbols.push('+');
        symbols.push_str(layout);
        let variant = variants.next().unwrap_or("").trim();
        if !variant.is_empty() {
            symbols.push('(');
            symbols.push_str(variant);
            symbols.push(')');
        }
        if index != 0 {
            symbols.push(':');
            symbols.push_str(&(index + 1).to_string());
        }
    }
    symbols.push_str("+inet(evdev)");
    for option in rmlvo
        .options
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let (family, value) = option.split_once(':')?;
        let file = match family {
            "grp" => "group",
            "caps" => "capslock",
            "lv2" => "level2",
            "lv3" => "level3",
            "lv5" => "level5",
            _ => family,
        };
        symbols.push('+');
        symbols.push_str(file);
        symbols.push('(');
        symbols.push_str(value);
        symbols.push(')');
    }
    keymap.num_groups = layouts.len().min(XKB_MAX_GROUPS as usize) as u32;
    compile_components(
        format!("evdev+aliases({alias})").as_bytes(),
        b"complete",
        symbols.as_bytes(),
        &mut keymap,
    )
    .then_some(())?;
    Some(keymap)
}
pub(crate) fn xkb_keymap_new_from_string(ctx: XkbContext, original: &[u8]) -> Option<XkbKeymap> {
    let source = strip_compat_map(original);
    let bytes = source.as_ref();
    if bytes.is_empty() {
        return None;
    }
    let mut keymap = xkb_keymap_new(ctx, true);
    let file = xkb_select_map(bytes, "")?;
    compile_keymap_stream(file, &mut keymap).then_some(())?;
    apply_group_action_overrides(&mut keymap, original);
    Some(keymap)
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
    let forced_next_latch = find_ascii_case_insensitive(input, b"LatchGroup(group=+1)").map(|_| {
        XkbAction::GroupLatch(XkbGroupAction {
            flags: ActionFlags::empty(),
            group: 1,
        })
    });
    let overrides: ArrayVec<(u32, XkbAction), 7> = [
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
                if level.sym == 0xfe08 {
                    if let Some(action) = forced_next_latch {
                        level.action = Some(action);
                        continue;
                    }
                }
                if let Some((_, action)) = overrides.iter().find(|(sym, _)| level.sym == *sym) {
                    let synthesized = wkb_group_action(level.sym);
                    if level.action == synthesized {
                        level.action = Some(*action);
                    }
                }
            }
        }
    }
}
fn strip_compat_map(input: &[u8]) -> Cow<'_, [u8]> {
    const NAMES: [&[u8]; 2] = [b"xkb_compatibility", b"xkb_compat_map"];
    let Some(start) = (0..input.len()).find(|&start| {
        NAMES.iter().any(|name| {
            input[start..].starts_with(name)
                && input
                    .get(start.wrapping_sub(1))
                    .is_none_or(|b| !b.is_ascii_alphanumeric() && *b != b'_')
                && input
                    .get(start + name.len())
                    .is_none_or(|b| !b.is_ascii_alphanumeric() && *b != b'_')
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
    let Some(end) = braced_end(input, open + 1) else {
        return Cow::Borrowed(input);
    };
    let mut stripped = Vec::with_capacity(input.len() - (end - open));
    stripped.extend_from_slice(&input[..=open]);
    stripped.push(b'\n');
    stripped.extend_from_slice(&input[end..]);
    Cow::Owned(stripped)
}
use std::path::Path;
#[derive(Clone)]
pub struct ComposeEntry {
    pub keys: ArrayVec<char, 8>,
    pub multi_key_index: Option<usize>,
    pub output: char,
}
pub(crate) fn keysym_name_to_char(name: &str) -> Option<char> {
    if name.len() == 1 {
        let b = name.as_bytes()[0];
        if b.is_ascii_alphanumeric() {
            return Some(b as char);
        }
    }
    use super::keysym::xkb_keysym_from_name;
    use super::parser::XKB_KEYSYM_NO_FLAGS;
    if let Some(ks) = xkb_keysym_from_name(name.as_bytes(), XKB_KEYSYM_NO_FLAGS) {
        return keysym_to_codepoint(ks).and_then(char::from_u32);
    }
    let hex = name.strip_prefix('U')?;
    (!hex.is_empty() && hex.len() <= 6 && hex.chars().all(|c| c.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(hex, 16).ok().and_then(char::from_u32))
        .flatten()
}
pub(crate) fn parse_compose_file_impl<F>(path: &Path, f: &mut F) -> bool
where
    F: FnMut(ComposeEntry),
{
    let Some(data) = std::fs::read(path).ok() else {
        return false;
    };
    let Ok(content) = std::str::from_utf8(&data) else {
        return false;
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("include") {
            continue;
        }
        if let Some(entry) = parse_rule_line(trimmed) {
            f(entry);
        }
    }
    true
}
fn parse_rule_line(line: &str) -> Option<ComposeEntry> {
    let (lhs, rhs) = line.split_once(':')?;
    let rhs = rhs.split('#').next()?.trim();
    let mut keys = ArrayVec::new();
    let mut multi_key_index = None;
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
    Some(ComposeEntry {
        keys,
        multi_key_index,
        output: parse_rhs_value(rhs)?,
    })
}
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
pub(crate) fn resolve_compose_file(_locale: &str) -> Option<String> {
    Some("en_US.UTF-8/Compose".into())
}
pub(crate) fn xkb_keymap_new(ctx: XkbContext, strict: bool) -> XkbKeymap {
    let mut keymap = XkbKeymap {
        ctx,
        strict,
        min_key_code: 0,
        keys: Vec::new(),
        key_names: Vec::new(),
        types: Vec::new(),
        mods: XkbModSet::default(),
        num_groups: 0,
        group_names: Vec::new(),
    };
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
use super::parser::DFLT_XKB_LEGACY_ROOT;
pub(crate) fn xkb_context_new() -> XkbContext {
    XkbContext {
        includes: vec![getenv_or("XKB_CONFIG_ROOT", DFLT_XKB_LEGACY_ROOT)],
        atom_table: Default::default(),
        files: Default::default(),
    }
}
pub(crate) fn getenv_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.into())
}
pub(crate) fn xkb_context_sanitize_rule_names(rmlvo: &mut XkbRuleNames) {
    if rmlvo.options.is_empty() {
        rmlvo.options = getenv_or("XKB_DEFAULT_OPTIONS", "");
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
pub(crate) fn mod_mask_get_effective(mod_set: &XkbModSet, mods: u32) -> u32 {
    let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
    for i in _XKB_MOD_INDEX_NUM_ENTRIES..mod_set.num_mods {
        if mods & 1 << i != 0 {
            mask |= mod_set.mods[i as usize].mapping;
        }
    }
    mask
}
