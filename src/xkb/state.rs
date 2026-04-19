use std::rc::Rc;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_event {
    pub type_0: xkb_event_type,
    pub data: xkb_event_data,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union xkb_event_data {
    pub keycode: u32,
    pub components: xkb_event_components,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_event_components {
    pub components: state_components,
    pub changed: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct state_components {
    pub base_group: i32,
    pub latched_group: i32,
    pub locked_group: i32,
    pub group: u32,
    pub base_mods: u32,
    pub latched_mods: u32,
    pub locked_mods: u32,
    pub mods: u32,
    pub leds: xkb_led_mask_t,
    pub controls: xkb_action_controls,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_state_update_v1 {
    pub size: usize,
    pub components: *const xkb_state_components_update_v1,
    pub layout_policy: *const xkb_layout_policy_update_v1,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_layout_policy_update_v1 {
    pub size: usize,
    pub policy: u32,
    pub redirect: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_state_components_update_v1 {
    pub size: usize,
    pub components: u32,
    pub affect_latched_mods: u32,
    pub latched_mods: u32,
    pub affect_locked_mods: u32,
    pub locked_mods: u32,
    pub latched_layout: i32,
    pub locked_layout: i32,
    pub affect_controls: xkb_keyboard_control_flags,
    pub controls: xkb_keyboard_control_flags,
}
use crate::xkb::shared_types::{
    xkb_action_controls, xkb_event_type, xkb_keyboard_control_flags, xkb_led_mask_t,
};

pub type xkb_feature = u32;

pub const XKB_FEATURE_ENUM_COMPOSE_FEED_RESULT: xkb_feature = 31300;

pub const XKB_FEATURE_ENUM_COMPOSE_STATE_FLAGS: xkb_feature = 31200;

pub const XKB_FEATURE_ENUM_COMPOSE_STATUS: xkb_feature = 31000;

pub const XKB_FEATURE_ENUM_COMPOSE_COMPILE_FLAGS: xkb_feature = 30200;

pub const XKB_FEATURE_ENUM_COMPOSE_FORMAT: xkb_feature = 30000;

pub const XKB_FEATURE_ENUM_EVENTS_FLAGS: xkb_feature = 27600;

pub const XKB_FEATURE_ENUM_KEY_DIRECTION: xkb_feature = 27020;

pub const XKB_FEATURE_ENUM_EVENT_TYPE: xkb_feature = 27000;

pub const XKB_FEATURE_ENUM_CONSUMED_MODE: xkb_feature = 24840;

pub const XKB_FEATURE_ENUM_STATE_MATCH: xkb_feature = 24820;

pub const XKB_FEATURE_ENUM_KEYBOARD_CONTROL_FLAGS: xkb_feature = 24060;

pub const XKB_FEATURE_ENUM_A11Y_FLAGS: xkb_feature = 24040;

pub const XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY: xkb_feature = 24020;

pub const XKB_FEATURE_ENUM_STATE_COMPONENT: xkb_feature = 24000;

pub const XKB_FEATURE_ENUM_KEYMAP_KEY_ITERATOR_FLAGS: xkb_feature = 21600;

pub const XKB_FEATURE_ENUM_KEYMAP_SERIALIZE_FLAGS: xkb_feature = 21400;

pub const XKB_FEATURE_ENUM_KEYMAP_COMPILE_FLAGS: xkb_feature = 21200;

pub const XKB_FEATURE_ENUM_KEYMAP_FORMAT: xkb_feature = 21000;

pub const XKB_FEATURE_ENUM_RMLVO_BUILDER_FLAGS: xkb_feature = 18200;

pub const XKB_FEATURE_ENUM_KEYSYM_FLAGS: xkb_feature = 9200;

pub const XKB_FEATURE_ENUM_LOG_LEVEL: xkb_feature = 5100;

pub const XKB_FEATURE_ENUM_CONTEXT_FLAGS: xkb_feature = 3200;

pub const XKB_FEATURE_ENUM_ERROR_CODE: xkb_feature = 1000;

pub const XKB_FEATURE_ENUM_FEATURE: xkb_feature = 1;
pub use crate::xkb::features::xkb_feature_supported;

pub mod utf8_h {

    /// Native Rust UTF-8 validation (replaces C FFI)
    #[inline]
    pub fn is_valid_utf8(ss: *const i8, len: usize) -> bool {
        if ss.is_null() {
            return false;
        }
        unsafe {
            let slice = std::slice::from_raw_parts(ss as *const u8, len);
            std::str::from_utf8(slice).is_ok()
        }
    }
}

use self::utf8_h::is_valid_utf8;
pub use crate::xkb::keymap::xkb_keymap_key_get_level;
pub use crate::xkb::keymap::{
    xkb_keymap_key_get_actions_by_level, XkbLevelsSameSyms, XkbWrapGroupIntoRange,
};
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
    entry_is_active, format_max_overlays, real_mod_index, xkb_action, xkb_action_flags,
    xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action,
    xkb_key, xkb_key_alias, xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t,
    xkb_led, xkb_level, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    KeycodeMatch, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL,
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
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT,
    XKB_MAX_GROUPS, XKB_MOD_ALL, XKB_MOD_INDEX_CAPS, XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1,
    XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3, XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5,
    XKB_MOD_INDEX_SHIFT, XKB_OVERLAY1_CONTROLS_OFFSET, XKB_OVERLAY_MAX, XKB_OVERLAY_MAX_X11,
    _ACTION_TYPE_NUM_ENTRIES, _XKB_MOD_INDEX_NUM_ENTRIES,
};
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
use crate::xkb::utils::xkb_check_versioned_struct_size_;

fn vec_resize_zero<T>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        // SAFETY: we just reserved enough capacity, and zeroed memory is
        // valid for the repr(C) types stored in these vectors (xkb_filter).
        unsafe {
            let ptr = v.as_mut_ptr().add(old_len);
            std::ptr::write_bytes(ptr, 0, new_len - old_len);
            v.set_len(new_len);
        }
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

#[derive(Clone)]

pub struct xkb_state {
    pub components: state_components,
    pub controls: machine_controls,
    pub set_mods: u32,
    pub clear_mods: u32,
    pub mod_key_count: [i16; 32],
    pub flags: xkb_a11y_flags,
    pub refcnt: i32,
    pub filters: Vec<xkb_filter>,
    pub keymap: Rc<xkb_keymap>,
}

impl xkb_state {
    /// Safe accessor for the keymap reference.
    #[inline]
    pub fn keymap(&self) -> &xkb_keymap {
        &self.keymap
    }
}
// C2Rust_Unnamed_15 removed: replaced by Vec<xkb_filter>
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_filter {
    pub action: xkb_action,
    pub key: *const xkb_key,
    pub func: Option<
        unsafe fn(
            *mut xkb_state,
            *mut xkb_events,
            *mut xkb_filter,
            *const xkb_key,
            xkb_key_direction,
        ) -> bool,
    >,
    pub priv_0: u32,
    pub refcnt: i32,
}
#[derive(Clone)]

pub struct xkb_events {
    pub next: u32,
    pub queue: Vec<xkb_event>,
    pub ctx: xkb_context,
}
// C2Rust_Unnamed_16 removed: replaced by Vec<xkb_event>
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_controls {
    pub out_of_range_group_policy: u32,
    pub out_of_range_redirect_group: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct FilterActionFuncs {
    pub new: Option<unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> ()>,
    pub func: Option<
        unsafe fn(
            *mut xkb_state,
            *mut xkb_events,
            *mut xkb_filter,
            *const xkb_key,
            xkb_key_direction,
        ) -> bool,
    >,
}

pub const XKB_FILTER_CONSUME: xkb_filter_result = 0;

pub const XKB_FILTER_CONTINUE: xkb_filter_result = 1;
#[derive(Copy, Clone)]
#[repr(C)]

pub union group_latch_priv {
    pub priv_0: u32,
    pub latch_state: GroupLatchState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupLatchState {
    pub latch: u32,
    pub group_delta: i32,
}

pub type xkb_key_latch_state = u32;

pub const _KEY_LATCH_STATE_NUM_ENTRIES: xkb_key_latch_state = 3;

pub const LATCH_PENDING: xkb_key_latch_state = 2;

pub const LATCH_KEY_DOWN: xkb_key_latch_state = 1;

pub const NO_LATCH: xkb_key_latch_state = 0;

pub const XKB_STATE_MATCH_FLAGS: u32 = 65539;

pub type xkb_filter_result = u32;

static mut SYNTHETIC_KEY_BREAK_GROUP_LATCH: xkb_key = xkb_key {
    keycode: 0,
    name: 0,
    explicit: 0 as xkb_explicit_components,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    repeats: false,
    implicit_actions: false,
    out_of_range_pending_group: false,
    out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    out_of_range_group_number: 0,
    num_groups: 1,
    groups: Vec::new(),
    overlay_keys: Vec::new(),
};

fn get_entry_for_mods(type_0: &xkb_key_type, mods: u32) -> Option<&xkb_key_type_entry> {
    for entry in &type_0.entries {
        if entry_is_active(entry) && entry.mods.mask == mods {
            return Some(entry);
        }
    }
    None
}

fn get_entry_for_key_state<'a>(
    state: &'a xkb_state,
    key: &xkb_key,
    group: u32,
) -> Option<&'a xkb_key_type_entry> {
    let keymap = state.keymap();
    let type_0 = &keymap.types[key.groups[group as usize].type_idx as usize];
    let active_mods: u32 = state.components.mods & type_0.mods.mask;
    get_entry_for_mods(type_0, active_mods)
}
#[inline]

fn state_key_get_level(state: &xkb_state, key: &xkb_key, layout: u32) -> u32 {
    if layout >= key.num_groups {
        return XKB_LEVEL_INVALID;
    }
    match get_entry_for_key_state(state, key, layout) {
        Some(entry) => entry.level,
        None => 0_u32,
    }
}

pub fn xkb_state_key_get_level(state: &xkb_state, kc: u32, layout: u32) -> u32 {
    match state.keymap().get_key(kc) {
        Some(key) => state_key_get_level(state, key, layout),
        None => XKB_LEVEL_INVALID,
    }
}
#[inline]

fn state_key_get_layout(state: &xkb_state, key: &xkb_key) -> u32 {
    XkbWrapGroupIntoRange(
        state.components.group as i32,
        key.num_groups,
        key.out_of_range_group_policy,
        key.out_of_range_group_number,
    )
}

pub fn xkb_state_key_get_layout(state: &xkb_state, kc: u32) -> u32 {
    match state.keymap().get_key(kc) {
        Some(key) => state_key_get_layout(state, key),
        None => XKB_LAYOUT_INVALID,
    }
}

static DUMMY_ACTION: xkb_action = xkb_action::None;

fn xkb_key_get_actions<'a>(state: &'a xkb_state, key: &'a xkb_key) -> &'a [xkb_action] {
    let layout: u32 = state_key_get_layout(state, key);
    let level: u32 = state_key_get_level(state, key, layout);
    if level != XKB_LEVEL_INVALID {
        let wrapped_layout = XkbWrapGroupIntoRange(
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

fn xkb_filter_new(state: *mut xkb_state) -> *mut xkb_filter {
    unsafe {
        let mut filter: *mut xkb_filter = std::ptr::null_mut();
        let filters = &mut (*state).filters;
        for i in 0..filters.len() {
            if filters[i].func.is_none() {
                filter = &mut filters[i] as *mut xkb_filter;
                break;
            }
        }
        if filter.is_null() {
            let new_len = (*state).filters.len().wrapping_add(1);
            vec_resize_zero(&mut (*state).filters, new_len);
            let last = (*state).filters.len() - 1;
            filter = (*state).filters.as_mut_ptr().add(last);
        }
        (*filter).refcnt = 1_i32;
        filter
    }
}

fn xkb_filter_group_set_new(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        (*filter).priv_0 = (*state).components.base_group as u32;
        if (*filter).action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
            (*state).components.base_group = (*filter).action.as_group().group;
        } else {
            (*state).components.base_group += (*filter).action.as_group().group;
        };
    }
}

fn xkb_filter_group_set_func(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            (*filter).action.as_group_mut().flags = ((*filter).action.as_group().flags
                & !(ACTION_LOCK_CLEAR as i32) as u32)
                as xkb_action_flags;
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_38: {
            match direction {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0_i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_38;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        (*state).components.base_group = (*filter).priv_0 as i32;
        if (*filter).action.as_group().flags & ACTION_LOCK_CLEAR != 0 {
            (*state).components.locked_group = 0_i32;
        }
        (*filter).func = None;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn get_state_component_changes(a: &state_components, b: &state_components) -> u32 {
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
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        if (*filter).action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0 {
        } else if (*filter).action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
            (*state).components.locked_group = (*filter).action.as_group().group;
        } else {
            (*state).components.locked_group += (*filter).action.as_group().group;
        }
    }
}

fn xkb_filter_group_lock_func(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            if (*filter).action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0
                && direction == XKB_KEY_DOWN
            {
                (*filter).action.as_group_mut().flags = ((*filter).action.as_group().flags
                    & !(ACTION_LOCK_ON_RELEASE as i32) as u32)
                    as xkb_action_flags;
            }
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_47: {
            match direction {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0_i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_47;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        if (*filter).action.as_group().flags & ACTION_LOCK_ON_RELEASE != 0 {
            if (*filter).action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
                (*state).components.locked_group = (*filter).action.as_group().group;
            } else {
                (*state).components.locked_group += (*filter).action.as_group().group;
            }
        }
        (*filter).func = None;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn xkb_action_breaks_latch(action: &xkb_action, flag: u32, mask: u32) -> bool {
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
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        let priv_0: group_latch_priv = group_latch_priv {
            latch_state: GroupLatchState {
                latch: LATCH_KEY_DOWN,
                group_delta: if (*filter).action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
                    (*filter).action.as_group().group - (*state).components.base_group
                } else {
                    (*filter).action.as_group().group
                },
            },
        };
        (*filter).priv_0 = priv_0.priv_0;
        if (*filter).action.as_group().flags & ACTION_ABSOLUTE_SWITCH != 0 {
            (*state).components.base_group = (*filter).action.as_group().group;
        } else {
            (*state).components.base_group += (*filter).action.as_group().group;
        };
    }
}

fn xkb_filter_group_latch_func(
    state: *mut xkb_state,
    events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut priv_0: group_latch_priv = group_latch_priv {
            priv_0: (*filter).priv_0,
        };
        let mut latch: xkb_key_latch_state = priv_0.latch_state.latch as xkb_key_latch_state;
        if direction == XKB_KEY_DOWN {
            let actions = xkb_key_get_actions(&*state, &*key);
            if latch as u32 == LATCH_KEY_DOWN {
                if (*state).flags & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS != 0 {
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
                let sticky_keys: bool = (*state).components.controls & CONTROL_STICKY_KEYS != 0;
                let flags: xkb_action_flags = ((*filter).action.as_group().flags
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: u16 = 0_u16;
                while (k_0 as usize) < actions.len() {
                    if actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_GROUP_LATCH
                        && actions[k_0 as usize].as_group().group
                            == (*filter).action.as_group().group
                        && actions[k_0 as usize].as_group().flags as u32
                            == (*filter).action.as_group().flags
                        || actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_GROUP_SET
                            && sticky_keys as i32 != 0
                            && actions[k_0 as usize].as_group().flags as u32 == flags as u32
                    {
                        if (*filter).action.as_group().flags & ACTION_LATCH_TO_LOCK != 0
                            && (*filter).action.as_group().group != 0_i32
                        {
                            let group_data = *(*filter).action.as_group();
                            (*filter).action = xkb_action::GroupLock(group_data);
                            (*filter).func = Some(xkb_filter_group_lock_func);
                            xkb_filter_group_lock_new(state, events, filter);
                            (*state).components.latched_group -= priv_0.latch_state.group_delta;
                            (*filter).key = key;
                            return XKB_FILTER_CONSUME as i32 != 0;
                        }
                    } else if xkb_action_breaks_latch(
                        &actions[k_0 as usize],
                        INTERNAL_BREAKS_GROUP_LATCH,
                        0_u32,
                    ) {
                        (*state).components.latched_group -= priv_0.latch_state.group_delta;
                        (*filter).func = None;
                        return XKB_FILTER_CONTINUE as i32 != 0;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
        } else if key == (*filter).key {
            if direction == XKB_KEY_REPEATED {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else if (*filter).action.as_group().flags & ACTION_LOCK_CLEAR != 0
                && (*state).components.locked_group != 0
            {
                if latch as u32 == LATCH_PENDING {
                    (*state).components.latched_group -= priv_0.latch_state.group_delta;
                } else {
                    (*state).components.base_group -= priv_0.latch_state.group_delta;
                }
                (*state).components.locked_group = 0_i32;
                (*filter).func = None;
            } else if latch as u32 == NO_LATCH {
                (*state).components.base_group -= priv_0.latch_state.group_delta;
                (*filter).func = None;
            } else if latch as u32 == LATCH_KEY_DOWN {
                latch = LATCH_PENDING;
                (*state).components.base_group -= priv_0.latch_state.group_delta;
                (*state).components.latched_group += priv_0.latch_state.group_delta;
            }
        }
        priv_0.latch_state.latch = latch as u32;
        (*filter).priv_0 = priv_0.priv_0;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn xkb_filter_mod_set_new(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        let unlock: xkb_action_flags =
            (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LOCK_CLEAR as i32) as xkb_action_flags;
        if (*filter).action.as_mods().flags & unlock as u32 == unlock as u32 {
            (*filter).priv_0 =
                (*filter).action.as_mods().mods.mask & !(*state).components.locked_mods;
            (*state).components.locked_mods &= !(*filter).action.as_mods().mods.mask;
        } else {
            (*filter).priv_0 = (*filter).action.as_mods().mods.mask;
        }
        (*state).set_mods |= (*filter).priv_0;
    }
}

fn xkb_filter_mod_set_func(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            (*filter).action.as_mods_mut().flags = ((*filter).action.as_mods().flags
                & !(ACTION_LOCK_CLEAR as i32) as u32)
                as xkb_action_flags;
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_38: {
            match direction {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0_i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_38;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        (*state).clear_mods |= (*filter).priv_0;
        let unlock: xkb_action_flags =
            (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LOCK_CLEAR as i32) as xkb_action_flags;
        if (*filter).action.as_mods().flags & unlock as u32 == ACTION_LOCK_CLEAR {
            (*state).components.locked_mods &= !(*filter).action.as_mods().mods.mask;
        }
        (*filter).func = None;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn xkb_filter_mod_lock_new(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        (*filter).priv_0 = (*state).components.locked_mods & (*filter).action.as_mods().mods.mask;
        if (*filter).priv_0 != 0 && (*filter).action.as_mods().flags & ACTION_UNLOCK_ON_PRESS != 0 {
            if (*filter).action.as_mods().flags & ACTION_LOCK_NO_UNLOCK == 0 {
                (*state).components.locked_mods &= !(*filter).priv_0;
            }
            (*filter).func = None;
        } else {
            (*state).set_mods |= (*filter).action.as_mods().mods.mask;
            if (*filter).action.as_mods().flags & ACTION_LOCK_NO_LOCK == 0 {
                (*state).components.locked_mods |= (*filter).action.as_mods().mods.mask;
            }
        };
    }
}

fn xkb_filter_mod_lock_func(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_32: {
            match direction {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0_i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_32;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        (*state).clear_mods |= (*filter).action.as_mods().mods.mask;
        if (*filter).action.as_mods().flags & ACTION_LOCK_NO_UNLOCK == 0 {
            (*state).components.locked_mods &= !(*filter).priv_0;
        }
        (*filter).func = None;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn xkb_filter_mod_latch_new(
    state: *mut xkb_state,
    _events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        let unlockOnPress: xkb_action_flags =
            (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LATCH_ON_PRESS as i32) as xkb_action_flags;
        if (*filter).action.as_mods().flags & ACTION_LOCK_CLEAR != 0
            && (*filter).action.as_mods().flags & unlockOnPress as u32 != 0
            && (*state).components.locked_mods & (*filter).action.as_mods().mods.mask
                == (*filter).action.as_mods().mods.mask
        {
            (*state).components.locked_mods &= !(*filter).action.as_mods().mods.mask;
            (*filter).func = None;
        } else if (*filter).action.as_mods().flags & ACTION_LATCH_ON_PRESS != 0 {
            (*filter).priv_0 = LATCH_PENDING;
            (*state).components.latched_mods |= (*filter).action.as_mods().mods.mask;
        } else {
            (*filter).priv_0 = LATCH_KEY_DOWN;
            (*state).set_mods |= (*filter).action.as_mods().mods.mask;
        };
    }
}

fn xkb_filter_mod_latch_func(
    state: *mut xkb_state,
    events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut latch: xkb_key_latch_state = (*filter).priv_0 as xkb_key_latch_state;
        if direction == XKB_KEY_DOWN {
            let actions = xkb_key_get_actions(&*state, &*key);
            if latch as u32 == LATCH_KEY_DOWN {
                if (*state).flags & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS != 0 {
                    let mut k: u16 = 0_u16;
                    while (k as usize) < actions.len() {
                        if xkb_action_breaks_latch(
                            &actions[k as usize],
                            INTERNAL_BREAKS_MOD_LATCH,
                            (*filter).action.as_mods().mods.mask,
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
                let sticky_keys: bool = (*state).components.controls & CONTROL_STICKY_KEYS != 0;
                let flags: xkb_action_flags = ((*filter).action.as_mods().flags
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: u16 = 0_u16;
                while (k_0 as usize) < actions.len() {
                    if (actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_MOD_LATCH
                        && actions[k_0 as usize].as_mods().flags as u32
                            == (*filter).action.as_mods().flags
                        || actions[k_0 as usize].action_type() as u32 == ACTION_TYPE_MOD_SET
                            && sticky_keys as i32 != 0
                            && actions[k_0 as usize].as_mods().flags as u32 == flags as u32)
                        && actions[k_0 as usize].as_mods().mods.mask
                            == (*filter).action.as_mods().mods.mask
                    {
                        if (*filter).action.as_mods().flags & ACTION_LATCH_TO_LOCK != 0 {
                            let mods_data = *(*filter).action.as_mods();
                            (*filter).action = xkb_action::ModLock(mods_data);
                            (*filter).func = Some(xkb_filter_mod_lock_func);
                            xkb_filter_mod_lock_new(state, events, filter);
                        } else {
                            let mods_data = *(*filter).action.as_mods();
                            (*filter).action = xkb_action::ModSet(mods_data);
                            (*filter).func = Some(xkb_filter_mod_set_func);
                            xkb_filter_mod_set_new(state, events, filter);
                        }
                        (*filter).key = key;
                        (*state).components.latched_mods &= !(*filter).action.as_mods().mods.mask;
                        return XKB_FILTER_CONSUME as i32 != 0;
                    } else if xkb_action_breaks_latch(
                        &actions[k_0 as usize],
                        INTERNAL_BREAKS_MOD_LATCH,
                        (*filter).action.as_mods().mods.mask,
                    ) {
                        (*state).components.latched_mods &= !(*filter).action.as_mods().mods.mask;
                        (*filter).func = None;
                        return XKB_FILTER_CONTINUE as i32 != 0;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
        } else if key == (*filter).key {
            if direction == XKB_KEY_REPEATED {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else {
                let unlockOnPress: xkb_action_flags = (ACTION_UNLOCK_ON_PRESS as i32
                    | ACTION_LATCH_ON_PRESS as i32)
                    as xkb_action_flags;
                if (*filter).action.as_mods().flags & ACTION_LOCK_CLEAR != 0
                    && (*filter).action.as_mods().flags & unlockOnPress as u32 == 0
                    && (*state).components.locked_mods & (*filter).action.as_mods().mods.mask
                        == (*filter).action.as_mods().mods.mask
                {
                    if latch as u32 == LATCH_PENDING {
                        (*state).components.latched_mods &= !(*filter).action.as_mods().mods.mask;
                    } else {
                        (*state).clear_mods |= (*filter).action.as_mods().mods.mask;
                    }
                    (*state).components.locked_mods &= !(*filter).action.as_mods().mods.mask;
                    (*filter).func = None;
                } else if latch as u32 == NO_LATCH {
                    (*state).clear_mods |= (*filter).action.as_mods().mods.mask;
                    (*filter).func = None;
                } else if (*filter).action.as_mods().flags & ACTION_LATCH_ON_PRESS == 0 {
                    latch = LATCH_PENDING;
                    (*state).clear_mods |= (*filter).action.as_mods().mods.mask;
                    (*state).components.latched_mods |= (*filter).action.as_mods().mods.mask;
                }
            }
        }
        (*filter).priv_0 = latch as u32;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn xkb_filter_ctrls_new(state: *mut xkb_state, _events: *mut xkb_events, filter: *mut xkb_filter) {
    unsafe {
        if (*filter).action.action_type() == ACTION_TYPE_CTRL_SET {
            (*filter).priv_0 = !(*state).components.controls & (*filter).action.as_ctrls().ctrls;
        } else {
            (*filter).priv_0 = (*state).components.controls & (*filter).action.as_ctrls().ctrls;
        }
        if (*filter).action.action_type() == ACTION_TYPE_CTRL_SET
            || (*filter).action.as_ctrls().flags & ACTION_LOCK_NO_LOCK == 0
        {
            (*state).components.controls = ((*state).components.controls
                | (*filter).action.as_ctrls().ctrls)
                as xkb_action_controls;
        }
    }
}

fn xkb_filter_ctrls_func(
    state: *mut xkb_state,
    events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_32: {
            match direction {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0_i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_32;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        if (*filter).action.action_type() == ACTION_TYPE_CTRL_SET
            || (*filter).action.as_ctrls().flags & ACTION_LOCK_NO_UNLOCK == 0
        {
            let old: xkb_action_controls = (*state).components.controls;
            (*state).components.controls = ((*state).components.controls
                & !((*filter).priv_0 as xkb_action_controls))
                as xkb_action_controls;
            if old as u32 & CONTROL_STICKY_KEYS != 0
                && (*state).components.controls & CONTROL_STICKY_KEYS == 0
            {
                clear_all_latches_and_locks(state, events);
            }
        }
        (*filter).func = None;
        XKB_FILTER_CONTINUE as i32 != 0
    }
}

fn append_redirect_key_events(
    state: *mut xkb_state,
    events: *mut xkb_events,
    redirect: *const xkb_redirect_key_action,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut changed: u32 = 0_u32;
        let mask: u32 = (*redirect).affect;
        let mut last_components: state_components = (*state).components;
        {
            let queue = &(*events).queue;
            if !queue.is_empty() {
                let mut idx = queue.len() - 1;
                loop {
                    if queue[idx].type_0 == XKB_EVENT_TYPE_COMPONENTS_CHANGE {
                        last_components = queue[idx].data.components.components;
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
            let mut new: state_components = last_components;
            new.base_mods = new.base_mods & !mask | (*redirect).mods;
            new.latched_mods = new.latched_mods & !mask | (*redirect).mods;
            new.locked_mods = new.locked_mods & !mask | (*redirect).mods;
            new.mods = new.mods & !mask | (*redirect).mods;
            changed = get_state_component_changes(&last_components, &new);
            if changed as u64 != 0 {
                (*events).queue.push(xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    data: xkb_event_data {
                        components: xkb_event_components {
                            components: new,
                            changed,
                        },
                    },
                });
            }
        }
        (*events).queue.push(xkb_event {
            type_0: (if direction == XKB_KEY_UP {
                XKB_EVENT_TYPE_KEY_UP as i32
            } else if direction == XKB_KEY_REPEATED {
                XKB_EVENT_TYPE_KEY_REPEATED as i32
            } else {
                XKB_EVENT_TYPE_KEY_DOWN as i32
            }) as xkb_event_type,
            data: xkb_event_data {
                keycode: (*redirect).keycode,
            },
        });
        if mask != 0 && changed != 0 {
            (*events).queue.push(xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                data: xkb_event_data {
                    components: xkb_event_components {
                        components: last_components,
                        changed,
                    },
                },
            });
        }
        true
    }
}

fn xkb_filter_redirect_key_new(
    state: *mut xkb_state,
    events: *mut xkb_events,
    filter: *mut xkb_filter,
) {
    unsafe {
        if events.is_null() || (*filter).action.as_redirect().keycode == XKB_KEYCODE_INVALID {
            (*filter).func = None;
            return;
        }
        append_redirect_key_events(
            state,
            events,
            (*filter).action.as_redirect() as *const xkb_redirect_key_action,
            XKB_KEY_DOWN,
        );
    }
}

fn xkb_filter_redirect_key_func(
    state: *mut xkb_state,
    events: *mut xkb_events,
    filter: *mut xkb_filter,
    key: *const xkb_key,
    direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        if direction == XKB_KEY_UP {
            append_redirect_key_events(
                state,
                events,
                (*filter).action.as_redirect() as *const xkb_redirect_key_action,
                XKB_KEY_UP,
            );
            (*filter).func = None;
            return XKB_FILTER_CONSUME as i32 != 0;
        } else if direction == XKB_KEY_DOWN {
            let actions = xkb_key_get_actions(&*state, &*key);
            let mut a: u16 = 0_u16;
            while (a as usize) < actions.len() {
                if actions[a as usize].action_type() as u32 == ACTION_TYPE_REDIRECT_KEY
                    && actions[a as usize].as_redirect().keycode
                        != (*filter).action.as_redirect().keycode
                {
                    append_redirect_key_events(
                        state,
                        events,
                        (*filter).action.as_redirect() as *const xkb_redirect_key_action,
                        XKB_KEY_UP,
                    );
                    (*filter).func = None;
                    return XKB_FILTER_CONTINUE as i32 != 0;
                }
                a = a.wrapping_add(1);
            }
        }
        append_redirect_key_events(
            state,
            events,
            (*filter).action.as_redirect() as *const xkb_redirect_key_action,
            direction,
        );
        XKB_FILTER_CONSUME as i32 != 0
    }
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
    state: *mut xkb_state,
    events: *mut xkb_events,
    key: *const xkb_key,
    direction: xkb_key_direction,
) {
    unsafe {
        let mut consumed: bool = false;
        let mut filter: *mut xkb_filter;
        let filters_len = (*state).filters.len();
        if filters_len > 0 {
            filter = (*state).filters.as_mut_ptr();
            let end = filter.add(filters_len);
            while filter < end {
                if (*filter).func.is_some()
                    && (*filter).func.expect("non-null function pointer")(
                        state, events, filter, key, direction,
                    ) as i32
                        == XKB_FILTER_CONSUME as i32
                {
                    consumed = true;
                }
                filter = filter.offset(1);
            }
        }
        if consumed as i32 != 0 || direction == XKB_KEY_UP {
            return;
        }
        let actions = xkb_key_get_actions(&*state, &*key);
        let mut k: u16 = 0_u16;
        while (k as usize) < actions.len() {
            if ((actions[k as usize].action_type() as u32) < _ACTION_TYPE_NUM_ENTRIES)
                && FILTER_ACTION_FUNCS[actions[k as usize].action_type() as usize]
                    .new
                    .is_some()
            {
                filter = xkb_filter_new(state);
                (*filter).key = key;
                (*filter).action = actions[k as usize];
                if (*state).components.controls & CONTROL_STICKY_KEYS != 0 {
                    if (*filter).action.action_type() == ACTION_TYPE_MOD_SET {
                        let mods_data = *(*filter).action.as_mods();
                        (*filter).action = xkb_action::ModLatch(mods_data);
                        if (*state).flags & XKB_A11Y_LATCH_TO_LOCK != 0 {
                            (*filter).action.as_mods_mut().flags =
                                ((*filter).action.as_mods().flags | ACTION_LATCH_TO_LOCK)
                                    as xkb_action_flags;
                        }
                    } else if (*filter).action.action_type() == ACTION_TYPE_GROUP_SET {
                        let group_data = *(*filter).action.as_group();
                        (*filter).action = xkb_action::GroupLatch(group_data);
                        if (*state).flags & XKB_A11Y_LATCH_TO_LOCK != 0 {
                            (*filter).action.as_group_mut().flags =
                                ((*filter).action.as_group().flags | ACTION_LATCH_TO_LOCK)
                                    as xkb_action_flags;
                        }
                    }
                }
                if (*filter).action.action_type() == ACTION_TYPE_REDIRECT_KEY {
                    let km = (*state).keymap();
                    (*filter).action.as_redirect_mut().affect =
                        mod_mask_get_effective(km, (*filter).action.as_redirect().affect);
                    (*filter).action.as_redirect_mut().mods =
                        mod_mask_get_effective(km, (*filter).action.as_redirect().mods);
                }
                (*filter).func = FILTER_ACTION_FUNCS[(*filter).action.action_type() as usize].func;
                FILTER_ACTION_FUNCS[(*filter).action.action_type() as usize]
                    .new
                    .expect("non-null function pointer")(state, events, filter);
            }
            k = k.wrapping_add(1);
        }
    }
}
pub fn xkb_state_new(keymap: Rc<xkb_keymap>) -> *mut xkb_state {
    let state: *mut xkb_state = Box::into_raw(Box::new(xkb_state {
        components: unsafe { std::mem::zeroed() },
        controls: unsafe { std::mem::zeroed() },
        set_mods: 0,
        clear_mods: 0,
        mod_key_count: [0; 32],
        flags: unsafe { std::mem::zeroed() },
        refcnt: 0,
        filters: Vec::new(),
        keymap: keymap.clone(),
    }));
    unsafe {
        (*state).flags = XKB_A11Y_NO_FLAGS;
        if keymap.format != XKB_KEYMAP_FORMAT_TEXT_V1
            && XKB_A11Y_NO_FLAGS & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS == 0
        {
            (*state).flags =
                ((*state).flags as u32 | XKB_A11Y_LATCH_SIMULTANEOUS_KEYS) as xkb_a11y_flags;
        }
        (*state).controls.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP;
        (*state).refcnt = 1_i32;
        xkb_state_update_derived(&mut *state);
    }
    state
}

pub unsafe fn xkb_state_unref(state: *mut xkb_state) {
    if state.is_null() || {
        (*state).refcnt -= 1;
        (*state).refcnt > 0_i32
    } {
        return;
    }
    // xkb_state_destroy inlined — Rc<xkb_keymap> drops automatically
    // Vec<xkb_filter> will be dropped when the owning struct is dropped
    drop(Box::from_raw(state));
}
pub fn xkb_state_get_keymap(state: &xkb_state) -> &xkb_keymap {
    &state.keymap
}

fn xkb_state_led_update_all(state: &mut xkb_state) {
    let keymap = &*state.keymap;
    state.components.leds = 0 as xkb_led_mask_t;
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
                state.components.leds = (state.components.leds | 1_u32 << idx) as xkb_led_mask_t;
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
                            (state.components.leds | 1_u32 << idx) as xkb_led_mask_t;
                        set_led = true;
                    }
                } else if led.which_groups as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32 != 0
                    && state.components.base_group == 0_i32
                    || led.which_groups as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
                        && state.components.latched_group == 0_i32
                {
                    state.components.leds =
                        (state.components.leds | 1_u32 << idx) as xkb_led_mask_t;
                    set_led = true;
                }
            }
            if !set_led {
                if led.ctrls & state.components.controls != 0 {
                    state.components.leds =
                        (state.components.leds | 1_u32 << idx) as xkb_led_mask_t;
                }
            }
        }
        idx = idx.wrapping_add(1);
    }
}

fn xkb_state_update_derived(state: &mut xkb_state) {
    let mut wrapped: u32;
    state.components.mods =
        state.components.base_mods | state.components.latched_mods | state.components.locked_mods;
    // SAFETY: raw pointer deref avoids borrowing `state` (needed for mutation below)
    let keymap = &*state.keymap;
    wrapped = XkbWrapGroupIntoRange(
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
    wrapped = XkbWrapGroupIntoRange(
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
pub fn xkb_state_update_key(state: &mut xkb_state, kc: u32, direction: xkb_key_direction) -> u32 {
    // Use Rc::as_ptr to avoid creating a borrow on `state` (needed for mutation below)
    let keymap = unsafe { &*Rc::as_ptr(&state.keymap) };
    let key_ref = match keymap.get_key(kc) {
        Some(k) => k,
        None => return 0_u32,
    };
    if direction == XKB_KEY_REPEATED && !key_ref.repeats {
        return 0_u32;
    }
    let prev_components: state_components = state.components;
    state.set_mods = 0_u32;
    state.clear_mods = 0_u32;
    let state_ptr = state as *mut xkb_state;
    // SAFETY: xkb_filter_apply_all needs raw ptrs due to aliasing between state and its filters
    unsafe {
        xkb_filter_apply_all(
            state_ptr,
            std::ptr::null_mut(),
            key_ref as *const xkb_key,
            direction,
        );
    }
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

static mut SYNTHETIC_KEY_LEVEL_ENTRY: xkb_key_type_entry = xkb_key_type_entry {
    level: 0_u32,
    mods: xkb_mods { mods: 0, mask: 0 },
    preserve: xkb_mods { mods: 0, mask: 0 },
};

static mut SYNTHETIC_KEY_TYPE: xkb_key_type = {
    xkb_key_type {
        name: 0,
        mods: xkb_mods { mods: 0, mask: 0 },
        required: false,
        num_levels: 1_u32,
        level_names: Vec::new(),
        entries: Vec::new(), // populated in c2rust_run_static_initializers
    }
};

static mut SYNTHETIC_KEY: xkb_key = xkb_key {
    keycode: 0,
    name: 0,
    explicit: 0 as xkb_explicit_components,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    repeats: false,
    implicit_actions: false,
    out_of_range_pending_group: false,
    out_of_range_group_policy: 0,
    out_of_range_group_number: 0,
    num_groups: 1,
    groups: Vec::new(),
    overlay_keys: Vec::new(),
};

fn update_latch_modifiers(state: *mut xkb_state, events: *mut xkb_events, mask: u32, latches: u32) {
    unsafe {
        let clear: u32 = mask & !latches;
        (*state).components.latched_mods &= !clear;
        let synthetic_key_level_break_mod_latch: xkb_level = xkb_level {
            upper: XKB_KEY_NoSymbol as u32,
            has_upper: false,
            syms: Vec::new(),
            actions: vec![xkb_action::Internal(xkb_internal_action {
                type_0: ACTION_TYPE_INTERNAL,
                flags: INTERNAL_BREAKS_MOD_LATCH,
                clear_latched_mods: clear,
            })],
        };
        let synthetic_key_group_break_mod_latch: xkb_group = xkb_group {
            explicit_symbols: false,
            explicit_actions: false,
            implicit_actions: false,
            explicit_type: false,
            type_idx: 0,
            levels: vec![synthetic_key_level_break_mod_latch],
        };
        let synthetic_key_break_mod_latch = xkb_key {
            keycode: 0,
            name: 0,
            explicit: 0 as xkb_explicit_components,
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
        xkb_filter_apply_all(
            state,
            events,
            &raw const synthetic_key_break_mod_latch,
            XKB_KEY_DOWN,
        );
        let key: *const xkb_key = &raw const SYNTHETIC_KEY;
        let latch_mods: xkb_action = xkb_action::ModLatch(xkb_mod_action {
            type_0: ACTION_TYPE_MOD_LATCH,
            flags: 0 as xkb_action_flags,
            mods: xkb_mods {
                mods: 0,
                mask: mask & latches,
            },
        });
        let filter: *mut xkb_filter = xkb_filter_new(state) as *mut xkb_filter;
        (*filter).key = key;
        (*filter).func = Some(xkb_filter_mod_latch_func);
        (*filter).action = latch_mods;
        xkb_filter_mod_latch_new(state, events, filter);
        xkb_filter_mod_latch_func(state, events, filter, key, XKB_KEY_UP);
    }
}

fn update_latch_group(state: *mut xkb_state, events: *mut xkb_events, group: i32) {
    unsafe {
        xkb_filter_apply_all(
            state,
            events,
            &raw const SYNTHETIC_KEY_BREAK_GROUP_LATCH,
            XKB_KEY_DOWN,
        );
        let key: *const xkb_key = &raw const SYNTHETIC_KEY;
        let latch_group: xkb_action = xkb_action::GroupLatch(xkb_group_action {
            type_0: ACTION_TYPE_GROUP_LATCH,
            flags: ACTION_ABSOLUTE_SWITCH,
            group,
        });
        let filter: *mut xkb_filter = xkb_filter_new(state) as *mut xkb_filter;
        (*filter).key = key;
        (*filter).func = Some(xkb_filter_group_latch_func);
        (*filter).action = latch_group;
        xkb_filter_group_latch_new(state, events, filter);
        xkb_filter_group_latch_func(state, events, filter, key, XKB_KEY_UP);
    }
}
#[inline]

fn resolve_to_canonical_mods(keymap: &xkb_keymap, mods: u32) -> u32 {
    mods & keymap.canonical_state_mask
        | mod_mask_get_effective(keymap, mods & !keymap.canonical_state_mask)
}

fn state_update_latched_locked(
    state: *mut xkb_state,
    update: *const xkb_state_components_update,
    events: *mut xkb_events,
) {
    unsafe {
        let keymap = (*state).keymap();
        let affect_locked_mods: u32 =
            resolve_to_canonical_mods(keymap, (*update).affect_locked_mods);
        if affect_locked_mods != 0 {
            let locked_mods: u32 = resolve_to_canonical_mods(keymap, (*update).locked_mods);
            (*state).components.locked_mods &= !affect_locked_mods;
            (*state).components.locked_mods |= locked_mods & affect_locked_mods;
        }
        if (*update).components & XKB_STATE_LAYOUT_LOCKED != 0 {
            (*state).components.locked_group = (*update).locked_layout;
        }
        let affect_latched_mods: u32 =
            resolve_to_canonical_mods(keymap, (*update).affect_latched_mods);
        if affect_latched_mods != 0 {
            let latched_mods: u32 = resolve_to_canonical_mods(keymap, (*update).latched_mods);
            update_latch_modifiers(state, events, affect_latched_mods, latched_mods);
        }
        if (*update).components & XKB_STATE_LAYOUT_LATCHED != 0 {
            update_latch_group(state, events, (*update).latched_layout);
        }
    }
}

#[inline]

fn clear_all_latches_and_locks(state: *mut xkb_state, events: *mut xkb_events) {
    static COMPONENTS: u32 = (XKB_STATE_MODS_LATCHED as i32
        | XKB_STATE_MODS_LOCKED as i32
        | XKB_STATE_LAYOUT_LATCHED as i32
        | XKB_STATE_LAYOUT_LOCKED as i32) as u32;
    let update: xkb_state_components_update = xkb_state_components_update {
        size: std::mem::size_of::<xkb_state_components_update>(),
        components: COMPONENTS,
        affect_latched_mods: XKB_MOD_ALL,
        latched_mods: 0_u32,
        affect_locked_mods: XKB_MOD_ALL,
        locked_mods: 0_u32,
        latched_layout: 0_i32,
        locked_layout: 0_i32,
        affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
        controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
    };
    state_update_latched_locked(state, &raw const update, events);
}

fn state_update_layout_policy(
    state: *mut xkb_state,
    update: *const xkb_layout_policy_update,
) -> xkb_error_code {
    unsafe {
        let keymap = (*state).keymap();
        if xkb_feature_supported(
            XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY,
            (*update).policy,
        ) {
            if (*update).policy == XKB_LAYOUT_OUT_OF_RANGE_REDIRECT {
                if (*update).redirect < keymap.num_groups {
                    (*state).controls.out_of_range_redirect_group = (*update).redirect;
                } else {
                    log::error!(
                        "[XKB-{:03}] Layout policy: unsupported layout index {} > {}\n",
                        { XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX },
                        (*update).redirect.wrapping_add(1_u32),
                        keymap.num_groups
                    );
                    return XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX;
                }
            }
            (*state).controls.out_of_range_group_policy = (*update).policy;
            XKB_SUCCESS
        } else {
            log::error!(
                "[XKB-{:03}] Unsupported layout policy: {}\n",
                { XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY },
                { (*update).policy }
            );
            XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY
        }
    }
}

fn log_abi_error(_ctx: *mut xkb_context, func: &str, error: xkb_error_code) {
    match error {
        450 => {
            log::error!(
                "[XKB-{:03}] {}: ABI error: unsupported versioned struct\n",
                { XKB_ERROR_ABI_INVALID_STRUCT_SIZE },
                func
            );
        }
        914 => {
            log::error!(
                "[XKB-{:03}] {}: ABI version mismatch: missing newer required fields\n",
                { XKB_ERROR_ABI_BACKWARD_COMPAT },
                func
            );
        }
        876 => {
            log::error!(
                "[XKB-{:03}] {}: ABI version mismatch: cannot use newer fields\n",
                { XKB_ERROR_ABI_FORWARD_COMPAT },
                func
            );
        }
        _ => {}
    };
}

fn check_state_update_abi_(
    ctx: *mut xkb_context,
    func: &str,
    update: *const xkb_state_update,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code;
        error = xkb_check_versioned_struct_size_(
            std::mem::size_of::<xkb_state_update_v1>(),
            std::mem::size_of::<xkb_state_update_v1>(),
            std::mem::size_of::<xkb_state_update>(),
            (*update).size,
            update as *const ::core::ffi::c_void,
        );
        if error as i32 != 0
            || !(*update).components.is_null() && {
                error = xkb_check_versioned_struct_size_(
                    std::mem::size_of::<xkb_state_update_v1>(),
                    std::mem::size_of::<xkb_state_update_v1>(),
                    std::mem::size_of::<xkb_state_components_update>(),
                    (*(*update).components).size,
                    (*update).components as *const ::core::ffi::c_void,
                );
                error as i32 != 0
            }
            || !(*update).layout_policy.is_null() && {
                error = xkb_check_versioned_struct_size_(
                    std::mem::size_of::<xkb_state_update_v1>(),
                    std::mem::size_of::<xkb_state_update_v1>(),
                    std::mem::size_of::<xkb_layout_policy_update>(),
                    (*(*update).layout_policy).size,
                    (*update).layout_policy as *const ::core::ffi::c_void,
                );
                error as i32 != 0
            }
        {
            log_abi_error(ctx, func, error);
        }
        error
    }
}

pub unsafe fn xkb_state_update_synthetic(
    state: *mut xkb_state,
    update: *const xkb_state_update,
    changed: *mut u32,
) -> xkb_error_code {
    let mut error: xkb_error_code = check_state_update_abi_(
        &raw mut (*(Rc::as_ptr(&(*state).keymap) as *mut xkb_keymap)).ctx,
        "xkb_state_update_synthetic",
        update,
    );
    if error as u64 != 0 {
        return error;
    }
    let previous_components: state_components = (*state).components;
    if !(*update).layout_policy.is_null() {
        error = state_update_layout_policy(state, (*update).layout_policy);
        if error as u64 != 0 {
            return error;
        }
    }
    if !(*update).components.is_null() {
        let components: *const xkb_state_components_update = (*update).components;
        // state_update_enabled_controls inlined
        {
            let had_sticky_keys: bool = (*state).components.controls & CONTROL_STICKY_KEYS != 0;
            let affect = ((*components).affect_controls as xkb_action_controls
                & CONTROL_ALL_BOOLEAN) as xkb_keyboard_control_flags;
            (*state).components.controls =
                ((*state).components.controls & !(affect as u32)) as xkb_action_controls;
            (*state).components.controls = ((*state).components.controls
                | (*components).controls as xkb_action_controls & affect as u32)
                as xkb_action_controls;
            if had_sticky_keys as i32 != 0
                && (*state).components.controls & CONTROL_STICKY_KEYS == 0
            {
                clear_all_latches_and_locks(state, std::ptr::null_mut());
            }
            xkb_state_update_derived(&mut *state);
        }
        state_update_latched_locked(state, components, std::ptr::null_mut());
    }
    xkb_state_update_derived(&mut *state);
    if !changed.is_null() {
        *changed = get_state_component_changes(&previous_components, &(*state).components);
    }
    XKB_SUCCESS
}

pub fn xkb_state_update_mask(
    state: &mut xkb_state,
    base_mods: u32,
    latched_mods: u32,
    locked_mods: u32,
    base_group: u32,
    latched_group: u32,
    locked_group: u32,
) -> u32 {
    let prev_components: state_components = state.components;
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

fn should_do_caps_transformation(state: &xkb_state, kc: u32) -> bool {
    xkb_state_mod_index_is_active(
        state,
        XKB_MOD_INDEX_CAPS as i32 as u32,
        XKB_STATE_MODS_EFFECTIVE,
    ) > 0_i32
        && xkb_state_mod_index_is_consumed(state, kc, XKB_MOD_INDEX_CAPS as i32 as u32) == 0_i32
}

fn should_do_ctrl_transformation(state: &xkb_state, kc: u32) -> bool {
    xkb_state_mod_index_is_active(
        state,
        XKB_MOD_INDEX_CTRL as i32 as u32,
        XKB_STATE_MODS_EFFECTIVE,
    ) > 0_i32
        && xkb_state_mod_index_is_consumed(state, kc, XKB_MOD_INDEX_CTRL as i32 as u32) == 0_i32
}
pub fn xkb_state_key_get_syms(state: &xkb_state, kc: u32) -> &[u32] {
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

pub fn xkb_state_key_get_one_sym(state: &xkb_state, kc: u32) -> u32 {
    let syms = xkb_state_key_get_syms(state, kc);
    if syms.len() != 1 {
        XKB_KEY_NoSymbol as u32
    } else {
        syms[0]
    }
}

fn get_one_sym_for_string(state: &xkb_state, kc: u32) -> u32 {
    let layout = xkb_state_key_get_layout(state, kc);
    let keymap = state.keymap();
    let num_layouts = xkb_keymap_num_layouts_for_key(keymap, kc);
    let mut level = xkb_state_key_get_level(state, kc, layout);
    if layout == XKB_LAYOUT_INVALID || num_layouts == 0_u32 || level == XKB_LEVEL_INVALID {
        return XKB_KEY_NoSymbol as u32;
    }
    let syms = xkb_keymap_key_get_syms_by_level_ref(keymap, kc, layout, level);
    if syms.len() != 1 {
        return XKB_KEY_NoSymbol as u32;
    }
    let mut sym = syms[0];
    if should_do_ctrl_transformation(state, kc) && sym > 127_u32 {
        for i in 0..num_layouts {
            level = xkb_state_key_get_level(state, kc, i);
            if level != XKB_LEVEL_INVALID {
                let syms_i = xkb_keymap_key_get_syms_by_level_ref(keymap, kc, i, level);
                if syms_i.len() == 1 && syms_i[0] <= 127_u32 {
                    sym = syms_i[0];
                    break;
                }
            }
        }
    }
    if should_do_caps_transformation(state, kc) {
        sym = unsafe { xkb_keysym_to_upper(sym) };
    }
    sym
}

pub fn xkb_state_key_get_utf8(state: &xkb_state, kc: u32) -> String {
    let syms: &[u32];
    let sym: u32 = get_one_sym_for_string(state, kc);
    let sym_slice: [u32; 1];
    if sym != XKB_KEY_NoSymbol as u32 {
        sym_slice = [sym];
        syms = &sym_slice;
    } else {
        syms = xkb_state_key_get_syms(state, kc);
    }
    let mut result = Vec::new();
    let mut tmp: [i8; 5] = [0; 5];
    for &s in syms {
        let ret = unsafe {
            xkb_keysym_to_utf8(s, &raw mut tmp as *mut i8, std::mem::size_of::<[i8; 5]>())
        };
        if ret <= 0 {
            return String::new();
        }
        let len = (ret - 1) as usize;
        for j in 0..len {
            result.push(tmp[j] as u8);
        }
    }
    // Validate UTF-8
    if let Ok(s) = std::str::from_utf8(&result) {
        let mut s = s.to_string();
        if s.len() == 1 && s.as_bytes()[0] <= 127 && should_do_ctrl_transformation(state, kc) {
            let c = s.as_bytes()[0] as i8;
            let ctrl = if c as i32 >= '@' as i32 && (c as i32) < '\u{7f}' as i32
                || c as i32 == ' ' as i32
            {
                (c as i32 & 0x1f_i32) as u8
            } else if c as i32 == '2' as i32 {
                0u8
            } else if c as i32 >= '3' as i32 && c as i32 <= '7' as i32 {
                (c as i32 - ('3' as i32 - '\u{1b}' as i32)) as u8
            } else if c as i32 == '8' as i32 {
                '\u{7f}' as u8
            } else if c as i32 == '/' as i32 {
                ('_' as i32 & 0x1f_i32) as u8
            } else {
                c as u8
            };
            s = String::from(ctrl as char);
        }
        s
    } else {
        String::new()
    }
}

#[inline]

fn serialize_mods(components: &state_components, type_0: u32) -> u32 {
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
pub fn xkb_state_serialize_mods(state: &xkb_state, type_0: u32) -> u32 {
    serialize_mods(&state.components, type_0)
}
#[inline]

fn serialize_layout(components: &state_components, type_0: u32) -> u32 {
    let mut ret: u32 = 0_u32;
    if type_0 & XKB_STATE_LAYOUT_EFFECTIVE != 0 {
        return components.group;
    }
    if type_0 & XKB_STATE_LAYOUT_DEPRESSED != 0 {
        ret = ret.wrapping_add(components.base_group as u32);
    }
    if type_0 & XKB_STATE_LAYOUT_LATCHED != 0 {
        ret = ret.wrapping_add(components.latched_group as u32);
    }
    if type_0 & XKB_STATE_LAYOUT_LOCKED != 0 {
        ret = ret.wrapping_add(components.locked_group as u32);
    }
    ret
}
pub fn xkb_state_serialize_layout(state: &xkb_state, type_0: u32) -> u32 {
    serialize_layout(&state.components, type_0)
}

pub fn mod_mask_get_effective(keymap: &xkb_keymap, mods: u32) -> u32 {
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

pub fn xkb_state_mod_index_is_active(state: &xkb_state, idx: u32, type_0: u32) -> i32 {
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

fn match_mod_masks(state: &xkb_state, type_0: u32, match_0: xkb_state_match, wanted: u32) -> bool {
    let active: u32 = xkb_state_serialize_mods(state, type_0);
    if match_0 & XKB_STATE_MATCH_NON_EXCLUSIVE == 0 && active & !wanted != 0 {
        return false;
    }
    if match_0 & XKB_STATE_MATCH_ANY != 0 {
        return active & wanted != 0;
    }
    active & wanted == wanted
}

pub fn xkb_state_mod_name_is_active(state: &xkb_state, name: &str, type_0: u32) -> i32 {
    let idx = xkb_keymap_mod_get_index_ref(state.keymap(), name);
    if idx == XKB_MOD_INVALID {
        return -1_i32;
    }
    xkb_state_mod_index_is_active(state, idx, type_0)
}

pub fn xkb_state_layout_index_is_active(state: &xkb_state, idx: u32, type_0: u32) -> i32 {
    if idx >= state.keymap().num_groups {
        return -1_i32;
    }
    let mut ret: i32 = 0_i32;
    if type_0 & XKB_STATE_LAYOUT_EFFECTIVE != 0 {
        ret |= (state.components.group == idx) as i32;
    }
    if type_0 & XKB_STATE_LAYOUT_DEPRESSED != 0 {
        ret |= (state.components.base_group == idx as i32) as i32;
    }
    if type_0 & XKB_STATE_LAYOUT_LATCHED != 0 {
        ret |= (state.components.latched_group == idx as i32) as i32;
    }
    if type_0 & XKB_STATE_LAYOUT_LOCKED != 0 {
        ret |= (state.components.locked_group == idx as i32) as i32;
    }
    ret
}

pub fn xkb_state_layout_name_is_active(state: &xkb_state, name: &str, type_0: u32) -> i32 {
    let idx = xkb_keymap_layout_get_index_ref(state.keymap(), name);
    if idx == XKB_LAYOUT_INVALID {
        return -1_i32;
    }
    xkb_state_layout_index_is_active(state, idx, type_0)
}

pub fn xkb_state_led_index_is_active(state: &xkb_state, idx: u32) -> i32 {
    let keymap = state.keymap();
    if idx >= keymap.num_leds || keymap.leds[idx as usize].name == XKB_ATOM_NONE {
        return -1_i32;
    }
    (state.components.leds & (1 as xkb_led_mask_t) << idx != 0) as i32
}

pub fn xkb_state_led_name_is_active(state: &xkb_state, name: &str) -> i32 {
    let idx = xkb_keymap_led_get_index_ref(state.keymap(), name);
    if idx == XKB_LED_INVALID {
        return -1_i32;
    }
    xkb_state_led_index_is_active(state, idx)
}

fn key_get_consumed(state: &xkb_state, key: &xkb_key, mode: xkb_consumed_mode) -> u32 {
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
            let no_mods_level: &xkb_level =
                &key.groups[group as usize].levels[no_mods_leveli as usize];
            for entry in &type_0.entries {
                if entry_is_active(entry) {
                    let level: &xkb_level =
                        &key.groups[group as usize].levels[entry.level as usize];
                    if !XkbLevelsSameSyms(level, no_mods_level)
                        && (matching_entry.map_or(false, |m| std::ptr::eq(entry, m))
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

pub fn xkb_state_mod_index_is_consumed2(
    state: &xkb_state,
    kc: u32,
    idx: u32,
    mode: xkb_consumed_mode,
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

pub fn xkb_state_mod_index_is_consumed(state: &xkb_state, kc: u32, idx: u32) -> i32 {
    xkb_state_mod_index_is_consumed2(state, kc, idx, XKB_CONSUMED_MODE_XKB)
}

fn c2rust_run_static_initializers() {
    unsafe {
        SYNTHETIC_KEY_TYPE.entries = vec![SYNTHETIC_KEY_LEVEL_ENTRY];
        let synthetic_key_group_break_group_latch = xkb_group {
            explicit_symbols: false,
            explicit_actions: false,
            implicit_actions: false,
            explicit_type: false,
            type_idx: 0,
            levels: vec![xkb_level {
                upper: XKB_KEY_NoSymbol as u32,
                has_upper: false,
                syms: Vec::new(),
                actions: vec![xkb_action::Internal(xkb_internal_action {
                    type_0: ACTION_TYPE_INTERNAL,
                    flags: INTERNAL_BREAKS_GROUP_LATCH,
                    clear_latched_mods: 0,
                })],
            }],
        };
        SYNTHETIC_KEY_BREAK_GROUP_LATCH = xkb_key {
            keycode: 0,
            name: 0,
            explicit: 0 as xkb_explicit_components,
            modmap: 0,
            vmodmap: 0,
            overlays: 0,
            repeats: false,
            implicit_actions: false,
            out_of_range_pending_group: false,
            out_of_range_group_policy: XKB_LAYOUT_OUT_OF_RANGE_WRAP,
            out_of_range_group_number: 0,
            num_groups: 1,
            groups: vec![synthetic_key_group_break_group_latch],
            overlay_keys: Vec::new(),
        };
        SYNTHETIC_KEY = xkb_key {
            keycode: 0_u32,
            name: 0,
            explicit: 0 as xkb_explicit_components,
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
        };
    }
}

use crate::xkb::keymap::{
    xkb_keymap_key_get_syms_by_level_ref, xkb_keymap_layout_get_index_ref,
    xkb_keymap_led_get_index_ref, xkb_keymap_mod_get_index_ref, xkb_keymap_num_layouts_for_key,
    xkb_keymap_num_mods,
};
use crate::xkb::keysym_case_mappings::xkb_keysym_to_upper;
use crate::xkb::keysym_utf::xkb_keysym_to_utf8;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
