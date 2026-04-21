use std::rc::Rc;

use crate::xkb::atom::{atom_lookup_ref, atom_text};
use crate::xkb::context::{xkb_atom_intern_bytes, xkb_context_sanitize_rule_names};
pub use crate::xkb::messages::{
    XKB_ERROR_ABI_BACKWARD_COMPAT_, XKB_ERROR_ABI_FORWARD_COMPAT_,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CANNOT_RESOLVE_RMLVO,
    XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY, XKB_ERROR_EXPECTED_ARRAY_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD,
    XKB_ERROR_INVALID_COMPOSE_LOCALE, XKB_ERROR_INVALID_COMPOSE_SYNTAX,
    XKB_ERROR_INVALID_EXPRESSION_TYPE, XKB_ERROR_INVALID_FILE_ENCODING,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
    XKB_ERROR_INVALID_INCLUDE_STATEMENT, XKB_ERROR_INVALID_MODMAP_ENTRY,
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_OPERATION, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_RULES_SYNTAX,
    XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT, XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_ERROR_KEYMAP_COMPILATION_FAILED, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_RECURSIVE_INCLUDE, XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE, XKB_LOG_VERBOSITY_DEFAULT,
    XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL, XKB_LOG_VERBOSITY_SILENT,
    XKB_LOG_VERBOSITY_VERBOSE, XKB_WARNING_CANNOT_INFER_KEY_TYPE,
    XKB_WARNING_CONFLICTING_KEY_ACTION, XKB_WARNING_CONFLICTING_KEY_FIELDS,
    XKB_WARNING_CONFLICTING_KEY_NAME, XKB_WARNING_CONFLICTING_KEY_SYMBOL,
    XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS, XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY, XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DEPRECATED_KEYSYM, XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_DUPLICATE_ENTRY,
    XKB_WARNING_EXTRA_SYMBOLS_IGNORED, XKB_WARNING_ILLEGAL_KEYCODE_ALIAS,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_INVALID_ESCAPE_SEQUENCE,
    XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX, XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE,
    XKB_WARNING_NON_BASE_GROUP_NAME, XKB_WARNING_NUMERIC_KEYSYM,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
    XKB_WARNING_UNRECOGNIZED_KEYSYM, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
};
pub use crate::xkb::shared_types::{
    entry_is_active, xkb_action, xkb_action_controls, xkb_action_flags, xkb_controls_action,
    xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action, xkb_key,
    xkb_key_alias, xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led,
    xkb_level, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, KeycodeMatch,
    ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL,
    ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK,
    ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP,
    ACTION_PENDING_COMPUTATION, ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET,
    ACTION_TYPE_GROUP_LATCH, ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL,
    ACTION_TYPE_MOD_LATCH, ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE,
    ACTION_TYPE_PRIVATE, ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK,
    ACTION_TYPE_PTR_MOVE, ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE,
    ACTION_TYPE_UNKNOWN, ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS,
    CONTROL_ALL, CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX,
    CONTROL_AX_FEEDBACK, CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_GROUPS,
    _ACTION_TYPE_NUM_ENTRIES,
};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use crate::xkb::shared_types::{
    XKB_A11Y_FLAGS_VALUES, XKB_COMPOSE_COMPILE_FLAGS_VALUES, XKB_COMPOSE_FEED_RESULT_VALUES,
    XKB_COMPOSE_FORMAT_VALUES, XKB_COMPOSE_STATE_FLAGS_VALUES, XKB_COMPOSE_STATUS_VALUES,
    XKB_CONSUMED_MODE_VALUES, XKB_CONTEXT_FLAGS_VALUES, XKB_EVENTS_FLAGS_VALUES,
    XKB_EVENT_TYPE_VALUES, XKB_KEYBOARD_CONTROL_FLAGS_VALUES, XKB_KEYMAP_COMPILE_FLAGS_VALUES,
    XKB_KEYMAP_FORMAT_VALUES, XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES,
    XKB_KEYMAP_SERIALIZE_FLAGS_VALUES, XKB_KEYSYM_FLAGS_VALUES, XKB_KEY_DIRECTION_VALUES,
    XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES, XKB_RMLVO_BUILDER_FLAGS_VALUES,
    XKB_STATE_COMPONENT_VALUES, XKB_STATE_MATCH_VALUES,
};
pub fn clear_level(leveli: &mut xkb_level) {
    leveli.syms.clear();
    leveli.actions.clear();
}

pub fn xkb_keymap_new_from_names(
    ctx: xkb_context,
    rmlvo_in: Option<&xkb_rule_names>,
    flags: u32,
) -> Option<Rc<xkb_keymap>> {
    let format = XKB_KEYMAP_FORMAT_TEXT_V2;
    let mut keymap = xkb_keymap_new(ctx.clone(), "xkb_keymap_new_from_names2", format, flags)?;
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
    if !crate::xkb::xkbcomp::xkbcomp::text_v1_keymap_new_from_names(&mut keymap, &rmlvo) {
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
    if !crate::xkb::xkbcomp::xkbcomp::text_v1_keymap_new_from_string(&mut keymap, &bytes[..length])
    {
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
pub fn xkb_keymap_num_layouts_for_key(keymap: &xkb_keymap, kc: u32) -> u32 {
    match keymap.get_key(kc) {
        Some(key) => key.num_groups,
        None => 0_u32,
    }
}
pub fn xkb_keymap_num_levels_for_key(keymap: &xkb_keymap, kc: u32, mut layout: u32) -> u32 {
    let key = match keymap.get_key(kc) {
        Some(k) => k,
        None => return 0_u32,
    };
    layout = XkbWrapGroupIntoRange(
        layout as i32,
        key.num_groups,
        key.out_of_range_group_policy,
        key.out_of_range_group_number,
    );
    if layout == XKB_LAYOUT_INVALID {
        return 0_u32;
    }
    keymap.key_num_levels(key, layout)
}
pub fn xkb_keymap_num_leds(keymap: &xkb_keymap) -> u32 {
    keymap.num_leds
}
pub fn xkb_keymap_led_get_name(keymap: &xkb_keymap, idx: u32) -> Option<&str> {
    if idx >= keymap.num_leds {
        return None;
    }
    let s = atom_text(&keymap.ctx.atom_table, keymap.leds[idx as usize].name);
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub fn xkb_keymap_key_get_level<'a>(
    keymap: &'a xkb_keymap,
    key: &'a xkb_key,
    layout: u32,
    level: u32,
) -> Option<&'a xkb_level> {
    keymap.get_key_level(key, layout, level)
}

/// Safe version of `xkb_keymap_mod_get_index` using `&str` name and `&xkb_keymap`.
pub fn xkb_keymap_mod_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        XKB_MOD_INVALID
    } else {
        XkbModNameToIndex(&keymap.mods, atom, MOD_BOTH)
    }
}

/// Safe version of `xkb_keymap_layout_get_index` using `&str` name and `&xkb_keymap`.
pub fn xkb_keymap_layout_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        return XKB_LAYOUT_INVALID;
    }
    for (i, &gn) in keymap.group_names.iter().enumerate() {
        if gn == atom {
            return i as u32;
        }
    }
    XKB_LAYOUT_INVALID
}

/// Safe version of `xkb_keymap_led_get_index` using `&str` name and `&xkb_keymap`.
pub fn xkb_keymap_led_get_index_ref(keymap: &xkb_keymap, name: &str) -> u32 {
    let atom = atom_lookup_ref(&keymap.ctx.atom_table, name.as_bytes());
    if atom == XKB_ATOM_NONE {
        return XKB_LED_INVALID;
    }
    for i in 0..keymap.num_leds {
        if keymap.leds[i as usize].name == atom {
            return i;
        }
    }
    XKB_LED_INVALID
}

/// Safe version: returns a slice of keysyms for a key at a given layout/level.
pub fn xkb_keymap_key_get_syms_by_level_ref(
    keymap: &xkb_keymap,
    kc: u32,
    layout: u32,
    level: u32,
) -> &[u32] {
    if let Some(key) = keymap.get_key(kc) {
        if let Some(leveli) = keymap.get_key_level(key, layout, level) {
            if !leveli.syms.is_empty() {
                return &leveli.syms;
            }
        }
    }
    &[]
}

pub fn xkb_keymap_min_keycode(keymap: &xkb_keymap) -> u32 {
    keymap.min_key_code
}
pub fn xkb_keymap_max_keycode(keymap: &xkb_keymap) -> u32 {
    keymap.max_key_code
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
pub fn xkb_keymap_key_repeats(keymap: &xkb_keymap, kc: u32) -> i32 {
    match keymap.get_key(kc) {
        Some(key) => key.repeats as i32,
        None => 0_i32,
    }
}
use crate::xkb::shared_types::*;

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

pub fn XkbEscapeMapName(name: &mut String) {
    static LEGAL: [u8; 32] = [
        0, 0, 0, 0, 0, 0xa7, 0xff, 0x83, 0xfe, 0xff, 0xff, 0x87, 0xfe, 0xff, 0xff, 0x7, 0, 0, 0, 0,
        0, 0, 0, 0, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0x7f, 0xff,
    ];
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
        0 | _ => {
            let rem: i32 = group % num_groups as i32;
            (if rem >= 0_i32 {
                rem
            } else {
                rem + num_groups as i32
            }) as u32
        }
    }
}
pub fn xkb_keymap_key_get_actions_by_level<'a>(
    keymap: &'a xkb_keymap,
    key: &'a xkb_key,
    layout: u32,
    level: u32,
) -> &'a [xkb_action] {
    let wrapped_layout = XkbWrapGroupIntoRange(
        layout as i32,
        key.num_groups,
        key.out_of_range_group_policy,
        key.out_of_range_group_number,
    );
    if wrapped_layout != XKB_LAYOUT_INVALID && level < keymap.key_num_levels(key, wrapped_layout) {
        let actions = &key.groups[wrapped_layout as usize].levels[level as usize].actions;
        if !actions.is_empty() {
            return actions.as_slice();
        }
    }
    &[]
}
