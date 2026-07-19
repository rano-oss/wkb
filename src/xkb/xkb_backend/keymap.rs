use std::rc::Rc;

pub use super::shared_types::XKB_KEYMAP_COMPILE_FLAGS_VALUES;
use super::shared_types::{atom_lookup_ref, atom_text};
pub use super::shared_types::{
    xkb_action, xkb_context, xkb_keymap, xkb_led, xkb_level, xkb_mod_set, xkb_rule_names, MOD_BOTH,
    MOD_REAL, MOD_REAL_MASK_ALL, XKB_ATOM_NONE, XKB_KEYCODE_INVALID, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_INVALID, XKB_LED_INVALID, XKB_MOD_INVALID,
};

pub fn xkb_keymap_new_from_names(
    ctx: xkb_context,
    rmlvo_in: Option<&xkb_rule_names>,
    flags: u32,
) -> Option<Rc<xkb_keymap>> {
    let format = XKB_KEYMAP_FORMAT_TEXT_V2;
    let mut rmlvo: xkb_rule_names = match rmlvo_in {
        Some(r) => r.clone(),
        None => xkb_rule_names {
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
pub fn xkb_keymap_new_from_string(
    ctx: xkb_context,
    string: &std::ffi::CStr,
    format: u32,
    flags: u32,
) -> Option<Rc<xkb_keymap>> {
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

pub fn xkb_keymap_num_mods(keymap: &xkb_keymap) -> u32 {
    keymap.mods.num_mods
}
pub fn xkb_keymap_mod_get_name(keymap: &xkb_keymap, idx: u32) -> Option<&str> {
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
pub fn xkb_keymap_num_layouts(keymap: &xkb_keymap) -> u32 {
    keymap.num_groups
}
pub fn xkb_keymap_layout_get_name(keymap: &xkb_keymap, idx: u32) -> Option<&str> {
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
pub fn xkb_keymap_key_by_name(keymap: &xkb_keymap, name: &str) -> u32 {
    let mut atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom != 0 {
        for alias in keymap.key_aliases.iter() {
            if alias.alias == atom {
                atom = alias.real;
            }
        }
    }
    if atom == 0 {
        return XKB_KEYCODE_INVALID;
    }
    let start_idx = if keymap.num_keys_low == 0_u32 {
        0_u32
    } else {
        keymap.min_key_code
    };
    for ki in start_idx..keymap.num_keys {
        let key = &keymap.keys[ki as usize];
        if key.name == atom {
            return key.keycode;
        }
    }
    XKB_KEYCODE_INVALID
}
pub fn xkb_keymap_key_get_name(keymap: &xkb_keymap, kc: u32) -> Option<&str> {
    let key = keymap.get_key(kc)?;
    let s = atom_text(&keymap.ctx.atom_table, key.name);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub fn xkb_keymap_key_repeats(keymap: &xkb_keymap, kc: u32) -> i32 {
    match keymap.get_key(kc) {
        Some(key) => key.repeats as i32,
        None => 0_i32,
    }
}

pub fn xkb_keymap_min_keycode(keymap: &xkb_keymap) -> u32 {
    keymap.min_key_code
}

pub fn xkb_keymap_max_keycode(keymap: &xkb_keymap) -> u32 {
    keymap.max_key_code
}

pub fn xkb_keymap_num_levels_for_key(keymap: &xkb_keymap, keycode: u32, layout: u32) -> u32 {
    keymap
        .get_key(keycode)
        .and_then(|k| k.groups.get(layout as usize))
        .map(|g| g.levels.len() as u32)
        .unwrap_or(0)
}

pub fn xkb_keymap_mod_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        XKB_MOD_INVALID
    } else {
        XkbModNameToIndex(&keymap.mods, atom, MOD_BOTH)
    }
}

pub fn xkb_keymap_key_get_syms_by_level_ref<'a>(
    keymap: &'a xkb_keymap,
    kc: u32,
    layout: u32,
    level: u32,
) -> &'a [u32] {
    keymap
        .get_key(kc)
        .and_then(|k| k.groups.get(layout as usize))
        .and_then(|g| g.levels.get(level as usize))
        .map(|lvl| lvl.syms.as_slice())
        .unwrap_or(&[])
}

// --- Merged from keymap_priv.rs ---

pub const XKB_MOD_NAME_SHIFT: &str = "Shift";
pub const XKB_MOD_NAME_CAPS: &str = "Lock";
pub const XKB_MOD_NAME_CTRL: &str = "Control";
pub const XKB_MOD_NAME_MOD1: &str = "Mod1";
pub const XKB_MOD_NAME_MOD2: &str = "Mod2";
pub const XKB_MOD_NAME_MOD3: &str = "Mod3";
pub const XKB_MOD_NAME_MOD4: &str = "Mod4";
pub const XKB_MOD_NAME_MOD5: &str = "Mod5";

pub fn xkb_keymap_new(
    ctx: xkb_context,
    func: &str,
    format: u32,
    flags: u32,
) -> Option<Box<xkb_keymap>> {
    static XKB_KEYMAP_COMPILE_FLAGS: u32 = XKB_KEYMAP_COMPILE_FLAGS_VALUES;
    if flags & !XKB_KEYMAP_COMPILE_FLAGS != 0 {
        log::error!(
            "{}: unrecognized keymap compilation flags: 0x{:x}\n",
            func,
            flags & !XKB_KEYMAP_COMPILE_FLAGS
        );
        return None;
    }
    let mut keymap = Box::new(xkb_keymap {
        ctx,
        flags: 0,
        format: 0,
        num_leds: 0,
        leds: [xkb_led::default(); 32],
        min_key_code: 0,
        max_key_code: 0,
        num_keys: 0,
        num_keys_low: 0,
        keys: Vec::new(),
        key_names: Vec::new(),
        key_aliases: Vec::new(),
        types: Vec::new(),
        sym_interprets: Vec::new(),
        mods: xkb_mod_set::default(),
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
        keymap.mods.mods[i].name = xkb_atom_intern_bytes(&mut keymap.ctx, name.as_bytes());
        keymap.mods.mods[i].type_0 = MOD_REAL;
        keymap.mods.mods[i].mapping = 1_u32 << i;
    }
    keymap.mods.num_mods = BUILTIN_MODS.len() as u32;
    keymap.canonical_state_mask = MOD_REAL_MASK_ALL;
    Some(keymap)
}

pub fn xkb_escape_map_name(name: &mut String) {
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

pub fn XkbModNameToIndex(mods: &xkb_mod_set, name: u32, type_0: u32) -> u32 {
    for (i, mod_0) in mods.mods[..mods.num_mods as usize].iter().enumerate() {
        if mod_0.type_0 & type_0 != 0 && name == mod_0.name {
            return i as u32;
        }
    }
    XKB_MOD_INVALID
}
pub fn XkbLevelsSameSyms(a: &xkb_level, b: &xkb_level) -> bool {
    a.syms == b.syms
}
pub fn action_equal(a: &xkb_action, b: &xkb_action) -> bool {
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
pub fn XkbLevelsSameActions(a: &xkb_level, b: &xkb_level) -> bool {
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
pub fn XkbWrapGroupIntoRange(
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
pub fn xkb_keymap_num_layouts_for_key(keymap: &xkb_keymap, kc: u32) -> u32 {
    keymap.get_key(kc).map(|k| k.num_groups as u32).unwrap_or(0)
}

pub fn xkb_keymap_num_leds(keymap: &xkb_keymap) -> u32 {
    keymap.leds.len() as u32
}

pub fn xkb_keymap_led_get_name(keymap: &xkb_keymap, idx: u32) -> Option<&str> {
    keymap
        .leds
        .get(idx as usize)
        .map(|l| atom_text(&keymap.ctx.atom_table, l.name))
}

pub fn xkb_keymap_layout_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
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

pub fn xkb_keymap_led_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
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

use super::shared_types::{
    DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT, DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH, DFLT_XKB_LEGACY_ROOT,
};
pub use super::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use super::shared_types::{XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_LOG_VERBOSITY_DEFAULT};
fn context_include_path_append(ctx: &mut xkb_context, path: &str) -> i32 {
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

pub fn xkb_context_include_path_get_extra_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_EXTRA_PATH") {
        Ok(extra) => extra,
        Err(_) => DFLT_XKB_CONFIG_EXTRA_PATH.to_string(),
    }
}

pub fn xkb_context_include_path_get_unversioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.to_string(),
    }
}

pub fn xkb_context_include_path_get_versioned_extensions_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_VERSIONED_EXTENSIONS_PATH") {
        Ok(ext) => ext,
        Err(_) => DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.to_string(),
    }
}
/// Convert a null-terminated `[i8]` constant to a Rust `String`.
fn add_direct_subdirectories(
    ctx: &mut xkb_context,
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

pub fn xkb_context_include_path_get_system_path() -> String {
    match xkb_context_getenv("XKB_CONFIG_ROOT") {
        Ok(root) => root,
        Err(_) => DFLT_XKB_CONFIG_ROOT.to_string(),
    }
}

pub fn xkb_context_include_path_append_default(ctx: &mut xkb_context) -> i32 {
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

pub fn xkb_context_num_include_paths(ctx: &mut xkb_context) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.includes.len() as u32
    } else {
        0_u32
    }
}
pub fn xkb_context_include_path_get(ctx: &mut xkb_context, idx: u32) -> String {
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
pub fn xkb_context_new(flags: xkb_context_flags) -> xkb_context {
    let mut ctx = xkb_context {
        refcnt: 1,
        log_level: XKB_LOG_LEVEL_ERROR,
        log_verbosity: XKB_LOG_VERBOSITY_DEFAULT,
        names_dflt: xkb_rule_names {
            rules: std::ffi::CString::new("").unwrap(),
            model: std::ffi::CString::new("").unwrap(),
            layout: std::ffi::CString::new("").unwrap(),
            variant: std::ffi::CString::new("").unwrap(),
            options: std::ffi::CString::new("").unwrap(),
        },
        includes: Vec::new(),
        failed_includes: Vec::new(),
        atom_table: atom_table_new(),
        use_environment_names: false,
        use_secure_getenv: false,
        pending_default_includes: false,
    };
    const XKB_CONTEXT_ALL_FLAGS: xkb_context_flags = (XKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
        | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32
        | XKB_CONTEXT_NO_SECURE_GETENV as i32)
        as xkb_context_flags;
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

pub fn xkb_context_get_log_verbosity(ctx: &xkb_context) -> i32 {
    ctx.log_verbosity
}

// --- Merged from context_priv.rs ---

pub fn xkb_context_getenv(name: &str) -> Result<String, VarError> {
    std::env::var(name)
}
pub fn xkb_context_init_includes(ctx: &mut xkb_context) -> bool {
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
pub fn xkb_context_num_failed_include_paths(ctx: &mut xkb_context) -> u32 {
    if xkb_context_init_includes(ctx) {
        ctx.failed_includes.len() as u32
    } else {
        0_u32
    }
}
pub fn xkb_context_failed_include_path_get(ctx: &mut xkb_context, idx: u32) -> String {
    if idx >= xkb_context_num_failed_include_paths(ctx) {
        return "".to_string();
    }
    ctx.failed_includes.get(idx as usize).unwrap().clone()
}

pub fn xkb_atom_intern_bytes(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_intern(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_intern_ref(ctx: &mut xkb_context, bytes: &[u8]) -> u32 {
    atom_intern(&mut ctx.atom_table, bytes, true)
}
pub fn xkb_atom_text(atom_table: &atom_table, atom: u32) -> &str {
    atom_text(atom_table, atom)
}

pub fn xkb_context_sanitize_rule_names(ctx: &xkb_context, rmlvo: &mut xkb_rule_names) -> RMLVO {
    let mut modified: RMLVO = 0 as RMLVO;
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
        modified = (modified as u32 | RMLVO_RULES) as RMLVO;
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
        modified = (modified as u32 | RMLVO_MODEL) as RMLVO;
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
        modified = (modified as u32 | RMLVO_LAYOUT) as RMLVO;
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
        modified = (modified as u32 | RMLVO_VARIANT) as RMLVO;
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
        modified = (modified as u32 | RMLVO_OPTIONS) as RMLVO;
    }
    modified
}

use super::shared_types::*;
// Was in text_h module — now at file level
#[derive(Copy, Clone)]
pub struct LookupEntry {
    pub name: &'static str,
    pub value: u32,
}
pub const CONTROL_NAMES_MIN_V2_INDEX: u32 = 0;
pub const CONTROL_NAMES_MIN_V1_INDEX: u32 = 7;
pub const GROUP_LAST_INDEX_NAME: &str = "last";
#[inline]
pub fn format_control_names_offset(format: u32) -> u8 {
    (if format == XKB_KEYMAP_FORMAT_TEXT_V1 {
        CONTROL_NAMES_MIN_V1_INDEX as i32
    } else {
        CONTROL_NAMES_MIN_V2_INDEX as i32
    }) as u8
}
use super::shared_types::XKB_KEYMAP_FORMAT_TEXT_V1;

pub const XKB_KEYSYM_NAME_MAX_SIZE: i32 = 31;

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
pub fn LookupString(tab: &[LookupEntry], string: &str, value_rtrn: &mut u32) -> bool {
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
pub fn LookupValue(tab: &[LookupEntry], value: u32) -> &'static str {
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
pub static ctrlMaskNames: [LookupEntry; 25] = [
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
pub static modComponentMaskNames: [LookupEntry; 8] = [
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
pub static groupComponentMaskNames: [LookupEntry; 7] = [
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
pub static buttonNames: [LookupEntry; 7] = [
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
pub static useModMapValueNames: [LookupEntry; 5] = [
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
pub static actionTypeNames: [LookupEntry; 43] = [
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
pub static symInterpretMatchMaskNames: [LookupEntry; 6] = [
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
pub fn ModIndexText<'a>(ctx: &'a xkb_context, mods: &xkb_mod_set, ndx: u32) -> &'a str {
    if ndx == XKB_MOD_INVALID {
        return "none";
    }
    if ndx == XKB_MOD_NONE {
        return "None";
    }
    if ndx >= mods.num_mods {
        return "";
    }
    xkb_atom_text(&ctx.atom_table, mods.mods[ndx as usize].name)
}
pub fn ActionTypeText(type_0: u32) -> &'static str {
    let name: &'static str = LookupValue(&actionTypeNames, type_0);
    if !name.is_empty() {
        name
    } else {
        "Private"
    }
}
pub fn KeysymText(sym: u32) -> String {
    xkb_keysym_get_name(sym)
}
pub fn SIMatchText(type_0: u32) -> &'static str {
    LookupValue(&symInterpretMatchMaskNames, type_0)
}
pub fn ModMaskText(ctx: &xkb_context, type_0: u32, mods: &xkb_mod_set, mask: u32) -> String {
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
            result.push_str(xkb_atom_text(&ctx.atom_table, mods.mods[i].name));
        }
        remaining >>= 1_i32;
    }
    result
}

use super::keysym::xkb_keysym_get_name;
