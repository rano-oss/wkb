use crate::xkb_logf;

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
    xkb_log_verbosity, xkb_message_code, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
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
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};
pub use crate::xkb::shared_types::{
    entry_is_active, format_max_overlays, real_mod_index, xkb_action, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_mod,
    xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, KeycodeMatch, XkbKey,
    _ACTION_TYPE_NUM_ENTRIES, _XKB_MOD_INDEX_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
    ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS,
    ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK,
    ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION,
    ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH,
    ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH,
    ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE,
    ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
    CONTROL_IGNORE_GROUP_LOCK, CONTROL_MOUSE_KEYS, CONTROL_MOUSE_KEYS_ACCEL, CONTROL_OVERLAY1,
    CONTROL_OVERLAY2, CONTROL_OVERLAY3, CONTROL_OVERLAY4, CONTROL_OVERLAY5, CONTROL_OVERLAY6,
    CONTROL_OVERLAY7, CONTROL_OVERLAY8, CONTROL_REPEAT, CONTROL_SLOW, CONTROL_STICKY_KEYS,
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT,
    XKB_MAX_GROUPS, XKB_MOD_ALL, XKB_MOD_INDEX_CAPS, XKB_MOD_INDEX_CTRL, XKB_MOD_INDEX_MOD1,
    XKB_MOD_INDEX_MOD2, XKB_MOD_INDEX_MOD3, XKB_MOD_INDEX_MOD4, XKB_MOD_INDEX_MOD5,
    XKB_MOD_INDEX_SHIFT, XKB_OVERLAY1_CONTROLS_OFFSET, XKB_OVERLAY_MAX, XKB_OVERLAY_MAX_X11,
};
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
use crate::xkb::utils::xkb_check_versioned_struct_size_;

unsafe fn vec_resize_zero<T>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
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
    pub keymap: *mut xkb_keymap,
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

static mut synthetic_key_group_break_group_latch: xkb_group = xkb_group {
    explicit_symbols: false,
    explicit_actions: false,
    implicit_actions: false,
    explicit_type: false,
    type_idx: 0,
    levels: Vec::new(),
};
static mut synthetic_key_break_group_latch: xkb_key = xkb_key {
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
static mut synthetic_key_level_break_group_latch: xkb_level = xkb_level {
    upper: XKB_KEY_NoSymbol as u32,
    has_upper: false,
    syms: Vec::new(),
    actions: Vec::new(),
};

unsafe fn get_entry_for_mods(
    mut type_0: *const xkb_key_type,
    mut mods: u32,
) -> *const xkb_key_type_entry {
    unsafe {
        let mut i: u32 = 0 as u32;
        while i < (&(*type_0).entries).len() as u32 {
            if entry_is_active(&(&(*type_0).entries)[i as usize] as *const xkb_key_type_entry)
                as i32
                != 0
                && (&(*type_0).entries)[i as usize].mods.mask == mods
            {
                return &(&(*type_0).entries)[i as usize] as *const xkb_key_type_entry;
            }
            i = i.wrapping_add(1);
        }
        return std::ptr::null();
    }
}

unsafe fn get_entry_for_key_state(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut group: u32,
) -> *const xkb_key_type_entry {
    unsafe {
        let type_0: *const xkb_key_type =
            &(&(*(*state).keymap).types)[(&(*key).groups)[group as usize].type_idx as usize];
        let mut active_mods: u32 = (*state).components.mods & (*type_0).mods.mask;
        return get_entry_for_mods(type_0, active_mods);
    }
}
#[inline]

unsafe fn state_key_get_level(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut layout: u32,
) -> u32 {
    unsafe {
        if layout >= (*key).num_groups {
            return XKB_LEVEL_INVALID as u32;
        }
        let entry: *const xkb_key_type_entry =
            get_entry_for_key_state(state, key, layout) as *const xkb_key_type_entry;
        return if !entry.is_null() {
            (*entry).level
        } else {
            0 as u32
        };
    }
}

pub unsafe fn xkb_state_key_get_level(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut layout: u32,
) -> u32 {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        return if !key.is_null() {
            state_key_get_level(state, key, layout)
        } else {
            XKB_LEVEL_INVALID as u32
        };
    }
}
#[inline]

unsafe fn state_key_get_layout(mut state: *mut xkb_state, mut key: *const xkb_key) -> u32 {
    unsafe {
        return XkbWrapGroupIntoRange(
            (*state).components.group as i32,
            (*key).num_groups,
            (*key).out_of_range_group_policy,
            (*key).out_of_range_group_number,
        );
    }
}

pub unsafe fn xkb_state_key_get_layout(mut state: *mut xkb_state, mut kc: u32) -> u32 {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() {
            return XKB_LAYOUT_INVALID as u32;
        }
        return state_key_get_layout(state, key);
    }
}

static mut dummy_action: xkb_action = xkb_action {
    type_0: ACTION_TYPE_NONE,
};

unsafe fn xkb_key_get_actions(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut actions: *mut *const xkb_action,
) -> u16 {
    unsafe {
        let mut count: u16 = 0;
        let layout: u32 = state_key_get_layout(state, key) as u32;
        let level: u32 = state_key_get_level(state, key, layout) as u32;
        if !(level == XKB_LEVEL_INVALID as u32) {
            count =
                xkb_keymap_key_get_actions_by_level((*state).keymap, key, layout, level, actions)
                    as u16;
            if !(count == 0) {
                return count;
            }
        }
        *actions = &raw const dummy_action;
        return 1 as u16;
    }
}

unsafe fn xkb_filter_new(mut state: *mut xkb_state) -> *mut xkb_filter {
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
            let new_len = (&(*state).filters).len().wrapping_add(1);
            vec_resize_zero(&mut (*state).filters, new_len);
            let last = (&(*state).filters).len() - 1;
            filter = (&mut (*state).filters).as_mut_ptr().add(last);
        }
        (*filter).refcnt = 1 as i32;
        return filter;
    }
}

unsafe fn xkb_filter_group_set_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        (*filter).priv_0 = (*state).components.base_group as u32;
        if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0 {
            (*state).components.base_group = (*filter).action.group.group;
        } else {
            (*state).components.base_group += (*filter).action.group.group;
        };
    }
}

unsafe fn xkb_filter_group_set_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            (*filter).action.group.flags = ((*filter).action.group.flags as u32
                & !(ACTION_LOCK_CLEAR as i32) as u32)
                as xkb_action_flags;
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_38: {
            match direction as u32 {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0 as i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_38;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        (*state).components.base_group = (*filter).priv_0 as i32;
        if (*filter).action.group.flags as u32 & ACTION_LOCK_CLEAR as u32 != 0 {
            (*state).components.locked_group = 0 as i32 as i32;
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn get_state_component_changes(
    mut a: *const state_components,
    mut b: *const state_components,
) -> u32 {
    unsafe {
        let mut mask: u32 = 0 as u32;
        if (*a).group != (*b).group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_EFFECTIVE as u32) as u32;
        }
        if (*a).base_group != (*b).base_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_DEPRESSED as u32) as u32;
        }
        if (*a).latched_group != (*b).latched_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_LATCHED as u32) as u32;
        }
        if (*a).locked_group != (*b).locked_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_LOCKED as u32) as u32;
        }
        if (*a).mods != (*b).mods {
            mask = (mask as u32 | XKB_STATE_MODS_EFFECTIVE as u32) as u32;
        }
        if (*a).base_mods != (*b).base_mods {
            mask = (mask as u32 | XKB_STATE_MODS_DEPRESSED as u32) as u32;
        }
        if (*a).latched_mods != (*b).latched_mods {
            mask = (mask as u32 | XKB_STATE_MODS_LATCHED as u32) as u32;
        }
        if (*a).locked_mods != (*b).locked_mods {
            mask = (mask as u32 | XKB_STATE_MODS_LOCKED as u32) as u32;
        }
        if (*a).leds != (*b).leds {
            mask = (mask as u32 | XKB_STATE_LEDS as u32) as u32;
        }
        if (*a).controls as u32 != (*b).controls as u32 {
            mask = (mask as u32 | XKB_STATE_CONTROLS as u32) as u32;
        }
        return mask;
    }
}

unsafe fn xkb_filter_group_lock_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as u32 != 0 {
            return;
        } else if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0 {
            (*state).components.locked_group = (*filter).action.group.group;
        } else {
            (*state).components.locked_group += (*filter).action.group.group;
        };
    }
}

unsafe fn xkb_filter_group_lock_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as u32 != 0
                && direction as u32 == XKB_KEY_DOWN as u32
            {
                (*filter).action.group.flags = ((*filter).action.group.flags as u32
                    & !(ACTION_LOCK_ON_RELEASE as i32) as u32)
                    as xkb_action_flags;
            }
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_47: {
            match direction as u32 {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0 as i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_47;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as u32 != 0 {
            if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0 {
                (*state).components.locked_group = (*filter).action.group.group;
            } else {
                (*state).components.locked_group += (*filter).action.group.group;
            }
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn xkb_action_breaks_latch(
    mut action: *const xkb_action,
    mut flag: xkb_internal_action_flags,
    mut mask: u32,
) -> bool {
    unsafe {
        match (*action).type_0 as u32 {
            0 | 1 | 9 | 10 | 14 | 15 | 13 | 12 | 16 => return true,
            20 => {
                return (*action).internal.flags as u32 & flag as u32 != 0
                    && (*action).internal.clear_latched_mods & mask == mask;
            }
            _ => return false,
        };
    }
}

unsafe fn xkb_filter_group_latch_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        let priv_0: group_latch_priv = group_latch_priv {
            latch_state: GroupLatchState {
                latch: LATCH_KEY_DOWN as u32,
                group_delta: if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32
                    != 0
                {
                    (*filter).action.group.group - (*state).components.base_group
                } else {
                    (*filter).action.group.group
                },
            },
        };
        (*filter).priv_0 = priv_0.priv_0;
        if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as u32 != 0 {
            (*state).components.base_group = (*filter).action.group.group;
        } else {
            (*state).components.base_group += (*filter).action.group.group;
        };
    }
}

unsafe fn xkb_filter_group_latch_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut priv_0: group_latch_priv = group_latch_priv {
            priv_0: (*filter).priv_0,
        };
        let mut latch: xkb_key_latch_state = priv_0.latch_state.latch as xkb_key_latch_state;
        if direction as u32 == XKB_KEY_DOWN as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: u16 = xkb_key_get_actions(state, key, &raw mut actions) as u16;
            if latch as u32 == LATCH_KEY_DOWN as u32 {
                if (*state).flags as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as u32 != 0 {
                    let mut k: u16 = 0 as u16;
                    while (k as i32) < count as i32 {
                        if xkb_action_breaks_latch(
                            actions.offset(k as isize) as *const xkb_action,
                            INTERNAL_BREAKS_GROUP_LATCH,
                            0 as u32,
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
            } else if latch as u32 == LATCH_PENDING as u32 {
                let sticky_keys: bool =
                    (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 != 0;
                let flags: xkb_action_flags = ((*filter).action.group.flags as u32
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: u16 = 0 as u16;
                while (k_0 as i32) < count as i32 {
                    if (*actions.offset(k_0 as isize)).type_0 as u32
                        == ACTION_TYPE_GROUP_LATCH as u32
                        && (*actions.offset(k_0 as isize)).group.group
                            == (*filter).action.group.group
                        && (*actions.offset(k_0 as isize)).group.flags as u32
                            == (*filter).action.group.flags as u32
                        || (*actions.offset(k_0 as isize)).type_0 as u32
                            == ACTION_TYPE_GROUP_SET as u32
                            && sticky_keys as i32 != 0
                            && (*actions.offset(k_0 as isize)).group.flags as u32 == flags as u32
                    {
                        if (*filter).action.group.flags as u32 & ACTION_LATCH_TO_LOCK as u32 != 0
                            && (*filter).action.group.group != 0 as i32
                        {
                            (*filter).action.type_0 = ACTION_TYPE_GROUP_LOCK;
                            (*filter).func = Some(
                                xkb_filter_group_lock_func
                                    as unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                            )
                                as Option<
                                    unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                                >;
                            xkb_filter_group_lock_new(state, events, filter);
                            (*state).components.latched_group -= priv_0.latch_state.group_delta;
                            (*filter).key = key;
                            return XKB_FILTER_CONSUME as i32 != 0;
                        }
                    } else if xkb_action_breaks_latch(
                        actions.offset(k_0 as isize) as *const xkb_action,
                        INTERNAL_BREAKS_GROUP_LATCH,
                        0 as u32,
                    ) {
                        (*state).components.latched_group -= priv_0.latch_state.group_delta;
                        (*filter).func = None;
                        return XKB_FILTER_CONTINUE as i32 != 0;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            } else {
            }
        } else if !(key != (*filter).key) {
            if direction as u32 == XKB_KEY_REPEATED as u32 {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else if (*filter).action.group.flags as u32 & ACTION_LOCK_CLEAR as u32 != 0
                && (*state).components.locked_group != 0
            {
                if latch as u32 == LATCH_PENDING as u32 {
                    (*state).components.latched_group -= priv_0.latch_state.group_delta;
                } else {
                    (*state).components.base_group -= priv_0.latch_state.group_delta;
                }
                (*state).components.locked_group = 0 as i32 as i32;
                (*filter).func = None;
            } else if latch as u32 == NO_LATCH as u32 {
                (*state).components.base_group -= priv_0.latch_state.group_delta;
                (*filter).func = None;
            } else if latch as u32 == LATCH_KEY_DOWN as u32 {
                latch = LATCH_PENDING;
                (*state).components.base_group -= priv_0.latch_state.group_delta;
                (*state).components.latched_group += priv_0.latch_state.group_delta;
            }
        }
        priv_0.latch_state.latch = latch as u32;
        (*filter).priv_0 = priv_0.priv_0;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn xkb_filter_mod_set_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        let unlock: xkb_action_flags =
            (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LOCK_CLEAR as i32) as xkb_action_flags;
        if (*filter).action.mods.flags as u32 & unlock as u32 == unlock as u32 {
            (*filter).priv_0 =
                ((*filter).action.mods.mods.mask & !(*state).components.locked_mods) as u32;
            (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
        } else {
            (*filter).priv_0 = (*filter).action.mods.mods.mask as u32;
        }
        (*state).set_mods |= (*filter).priv_0;
    }
}

unsafe fn xkb_filter_mod_set_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            (*filter).action.mods.flags = ((*filter).action.mods.flags as u32
                & !(ACTION_LOCK_CLEAR as i32) as u32)
                as xkb_action_flags;
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_38: {
            match direction as u32 {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0 as i32 {
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
        if (*filter).action.mods.flags as u32 & unlock as u32 == ACTION_LOCK_CLEAR as u32 {
            (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn xkb_filter_mod_lock_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        (*filter).priv_0 =
            ((*state).components.locked_mods & (*filter).action.mods.mods.mask) as u32;
        if (*filter).priv_0 != 0
            && (*filter).action.mods.flags as u32 & ACTION_UNLOCK_ON_PRESS as u32 != 0
        {
            if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_UNLOCK as u32 == 0 {
                (*state).components.locked_mods &= !(*filter).priv_0;
            }
            (*filter).func = None;
        } else {
            (*state).set_mods |= (*filter).action.mods.mods.mask;
            if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_LOCK as u32 == 0 {
                (*state).components.locked_mods |= (*filter).action.mods.mods.mask;
            }
        };
    }
}

unsafe fn xkb_filter_mod_lock_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_32: {
            match direction as u32 {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0 as i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_32;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        (*state).clear_mods |= (*filter).action.mods.mods.mask;
        if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_UNLOCK as u32 == 0 {
            (*state).components.locked_mods &= !(*filter).priv_0;
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn xkb_filter_mod_latch_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        let unlockOnPress: xkb_action_flags =
            (ACTION_UNLOCK_ON_PRESS as i32 | ACTION_LATCH_ON_PRESS as i32) as xkb_action_flags;
        if (*filter).action.mods.flags as u32 & ACTION_LOCK_CLEAR as u32 != 0
            && (*filter).action.mods.flags as u32 & unlockOnPress as u32 != 0
            && (*state).components.locked_mods & (*filter).action.mods.mods.mask
                == (*filter).action.mods.mods.mask
        {
            (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
            (*filter).func = None;
        } else if (*filter).action.mods.flags as u32 & ACTION_LATCH_ON_PRESS as u32 != 0 {
            (*filter).priv_0 = LATCH_PENDING as u32;
            (*state).components.latched_mods |= (*filter).action.mods.mods.mask;
        } else {
            (*filter).priv_0 = LATCH_KEY_DOWN as u32;
            (*state).set_mods |= (*filter).action.mods.mods.mask;
        };
    }
}

unsafe fn xkb_filter_mod_latch_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut latch: xkb_key_latch_state = (*filter).priv_0 as xkb_key_latch_state;
        if direction as u32 == XKB_KEY_DOWN as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: u16 = xkb_key_get_actions(state, key, &raw mut actions) as u16;
            if latch as u32 == LATCH_KEY_DOWN as u32 {
                if (*state).flags as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as u32 != 0 {
                    let mut k: u16 = 0 as u16;
                    while (k as i32) < count as i32 {
                        if xkb_action_breaks_latch(
                            actions.offset(k as isize) as *const xkb_action,
                            INTERNAL_BREAKS_MOD_LATCH,
                            (*filter).action.mods.mods.mask,
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
            } else if latch as u32 == LATCH_PENDING as u32 {
                let sticky_keys: bool =
                    (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 != 0;
                let flags: xkb_action_flags = ((*filter).action.mods.flags as u32
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: u16 = 0 as u16;
                while (k_0 as i32) < count as i32 {
                    if ((*actions.offset(k_0 as isize)).type_0 as u32
                        == ACTION_TYPE_MOD_LATCH as u32
                        && (*actions.offset(k_0 as isize)).mods.flags as u32
                            == (*filter).action.mods.flags as u32
                        || (*actions.offset(k_0 as isize)).type_0 as u32
                            == ACTION_TYPE_MOD_SET as u32
                            && sticky_keys as i32 != 0
                            && (*actions.offset(k_0 as isize)).mods.flags as u32 == flags as u32)
                        && (*actions.offset(k_0 as isize)).mods.mods.mask
                            == (*filter).action.mods.mods.mask
                    {
                        if (*filter).action.mods.flags as u32 & ACTION_LATCH_TO_LOCK as u32 != 0 {
                            (*filter).action.type_0 = ACTION_TYPE_MOD_LOCK;
                            (*filter).func = Some(
                                xkb_filter_mod_lock_func
                                    as unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                            )
                                as Option<
                                    unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                                >;
                            xkb_filter_mod_lock_new(state, events, filter);
                        } else {
                            (*filter).action.type_0 = ACTION_TYPE_MOD_SET;
                            (*filter).func = Some(
                                xkb_filter_mod_set_func
                                    as unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                            )
                                as Option<
                                    unsafe fn(
                                        *mut xkb_state,
                                        *mut xkb_events,
                                        *mut xkb_filter,
                                        *const xkb_key,
                                        xkb_key_direction,
                                    ) -> bool,
                                >;
                            xkb_filter_mod_set_new(state, events, filter);
                        }
                        (*filter).key = key;
                        (*state).components.latched_mods &= !(*filter).action.mods.mods.mask;
                        return XKB_FILTER_CONSUME as i32 != 0;
                    } else if xkb_action_breaks_latch(
                        actions.offset(k_0 as isize) as *const xkb_action,
                        INTERNAL_BREAKS_MOD_LATCH,
                        (*filter).action.mods.mods.mask,
                    ) {
                        (*state).components.latched_mods &= !(*filter).action.mods.mods.mask;
                        (*filter).func = None;
                        return XKB_FILTER_CONTINUE as i32 != 0;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            } else {
            }
        } else if !(key != (*filter).key) {
            if direction as u32 == XKB_KEY_REPEATED as u32 {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else {
                let unlockOnPress: xkb_action_flags = (ACTION_UNLOCK_ON_PRESS as i32
                    | ACTION_LATCH_ON_PRESS as i32)
                    as xkb_action_flags;
                if (*filter).action.mods.flags as u32 & ACTION_LOCK_CLEAR as u32 != 0
                    && (*filter).action.mods.flags as u32 & unlockOnPress as u32 == 0
                    && (*state).components.locked_mods & (*filter).action.mods.mods.mask
                        == (*filter).action.mods.mods.mask
                {
                    if latch as u32 == LATCH_PENDING as u32 {
                        (*state).components.latched_mods &= !(*filter).action.mods.mods.mask;
                    } else {
                        (*state).clear_mods |= (*filter).action.mods.mods.mask;
                    }
                    (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
                    (*filter).func = None;
                } else if latch as u32 == NO_LATCH as u32 {
                    (*state).clear_mods |= (*filter).action.mods.mods.mask;
                    (*filter).func = None;
                } else if (*filter).action.mods.flags as u32 & ACTION_LATCH_ON_PRESS as u32 == 0 {
                    latch = LATCH_PENDING;
                    (*state).clear_mods |= (*filter).action.mods.mods.mask;
                    (*state).components.latched_mods |= (*filter).action.mods.mods.mask;
                }
            }
        }
        (*filter).priv_0 = latch as u32;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn xkb_filter_ctrls_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as u32 {
            (*filter).priv_0 = (!((*state).components.controls as u32)
                & (*filter).action.ctrls.ctrls as u32) as u32;
        } else {
            (*filter).priv_0 =
                ((*state).components.controls as u32 & (*filter).action.ctrls.ctrls as u32) as u32;
        }
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as u32
            || (*filter).action.ctrls.flags as u32 & ACTION_LOCK_NO_LOCK as u32 == 0
        {
            (*state).components.controls = ((*state).components.controls as u32
                | (*filter).action.ctrls.ctrls as u32)
                as xkb_action_controls;
        }
    }
}

unsafe fn xkb_filter_ctrls_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        's_32: {
            match direction as u32 {
                1 => {
                    (*filter).refcnt += 1;
                }
                2 => {}
                _ => {
                    (*filter).refcnt -= 1;
                    if (*filter).refcnt > 0 as i32 {
                        return XKB_FILTER_CONSUME as i32 != 0;
                    }
                    break 's_32;
                }
            }
            return XKB_FILTER_CONSUME as i32 != 0;
        }
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as u32
            || (*filter).action.ctrls.flags as u32 & ACTION_LOCK_NO_UNLOCK as u32 == 0
        {
            let old: xkb_action_controls = (*state).components.controls;
            (*state).components.controls = ((*state).components.controls as u32
                & !((*filter).priv_0 as xkb_action_controls as u32))
                as xkb_action_controls;
            if old as u32 & CONTROL_STICKY_KEYS as u32 != 0
                && (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 == 0
            {
                clear_all_latches_and_locks(state, events);
            }
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn append_redirect_key_events(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut redirect: *const xkb_redirect_key_action,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        let mut changed: u32 = 0 as u32;
        let mask: u32 = (*redirect).affect;
        let mut last_components: state_components = (*state).components;
        {
            let queue = &(*events).queue;
            if !queue.is_empty() {
                let mut idx = queue.len() - 1;
                loop {
                    if queue[idx].type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as u32 {
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
            changed = get_state_component_changes(&raw mut last_components, &raw mut new);
            if changed as u64 != 0 {
                (&mut (*events).queue).push(xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    data: xkb_event_data {
                        components: xkb_event_components {
                            components: new,
                            changed: changed,
                        },
                    },
                });
            }
        }
        (&mut (*events).queue).push(xkb_event {
            type_0: (if direction as u32 == XKB_KEY_UP as u32 {
                XKB_EVENT_TYPE_KEY_UP as i32
            } else if direction as u32 == XKB_KEY_REPEATED as u32 {
                XKB_EVENT_TYPE_KEY_REPEATED as i32
            } else {
                XKB_EVENT_TYPE_KEY_DOWN as i32
            }) as xkb_event_type,
            data: xkb_event_data {
                keycode: (*redirect).keycode,
            },
        });
        if mask != 0 && changed as u32 != 0 {
            (&mut (*events).queue).push(xkb_event {
                type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                data: xkb_event_data {
                    components: xkb_event_components {
                        components: last_components,
                        changed: changed,
                    },
                },
            });
        }
        return true;
    }
}

unsafe fn xkb_filter_redirect_key_new(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
) {
    unsafe {
        if events.is_null() || (*filter).action.redirect.keycode == XKB_KEYCODE_INVALID as u32 {
            (*filter).func = None;
            return;
        }
        append_redirect_key_events(
            state,
            events,
            &raw mut (*filter).action.redirect,
            XKB_KEY_DOWN,
        );
    }
}

unsafe fn xkb_filter_redirect_key_func(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut filter: *mut xkb_filter,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> bool {
    unsafe {
        if key != (*filter).key {
            return XKB_FILTER_CONTINUE as i32 != 0;
        }
        if direction as u32 == XKB_KEY_UP as u32 {
            append_redirect_key_events(
                state,
                events,
                &raw mut (*filter).action.redirect,
                XKB_KEY_UP,
            );
            (*filter).func = None;
            return XKB_FILTER_CONSUME as i32 != 0;
        } else if direction as u32 == XKB_KEY_DOWN as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: u16 = xkb_key_get_actions(state, key, &raw mut actions) as u16;
            let mut a: u16 = 0 as u16;
            while (a as i32) < count as i32 {
                if (*actions.offset(a as isize)).type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32
                    && (*actions.offset(a as isize)).redirect.keycode
                        != (*filter).action.redirect.keycode
                {
                    append_redirect_key_events(
                        state,
                        events,
                        &raw mut (*filter).action.redirect,
                        XKB_KEY_UP,
                    );
                    (*filter).func = None;
                    return XKB_FILTER_CONTINUE as i32 != 0;
                }
                a = a.wrapping_add(1);
            }
        }
        append_redirect_key_events(state, events, &raw mut (*filter).action.redirect, direction);
        return XKB_FILTER_CONSUME as i32 != 0;
    }
}

static mut filter_action_funcs: [FilterActionFuncs; 21] = {
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
            new: Some(
                xkb_filter_mod_set_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_mod_set_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_mod_latch_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_mod_latch_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_mod_lock_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_mod_lock_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_group_set_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_group_set_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_group_latch_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_group_latch_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_group_lock_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_group_lock_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
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
            new: Some(
                xkb_filter_ctrls_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_ctrls_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_ctrls_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_ctrls_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
        },
        FilterActionFuncs {
            new: Some(
                xkb_filter_redirect_key_new
                    as unsafe fn(*mut xkb_state, *mut xkb_events, *mut xkb_filter) -> (),
            ),
            func: Some(
                xkb_filter_redirect_key_func
                    as unsafe fn(
                        *mut xkb_state,
                        *mut xkb_events,
                        *mut xkb_filter,
                        *const xkb_key,
                        xkb_key_direction,
                    ) -> bool,
            ),
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

unsafe fn xkb_filter_apply_all(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) {
    unsafe {
        let mut consumed: bool = false;
        let mut filter: *mut xkb_filter = std::ptr::null_mut();
        let filters_len = (&(*state).filters).len();
        if filters_len > 0 {
            filter = (*state).filters.as_mut_ptr();
            let end = filter.add(filters_len);
            while filter < end {
                if !(*filter).func.is_none() {
                    if (*filter).func.expect("non-null function pointer")(
                        state, events, filter, key, direction,
                    ) as i32
                        == XKB_FILTER_CONSUME as i32
                    {
                        consumed = true;
                    }
                }
                filter = filter.offset(1);
            }
        }
        if consumed as i32 != 0 || direction as u32 == XKB_KEY_UP as u32 {
            return;
        }
        let mut actions: *const xkb_action = std::ptr::null();
        let count: u16 = xkb_key_get_actions(state, key, &raw mut actions) as u16;
        let mut k: u16 = 0 as u16;
        while (k as i32) < count as i32 {
            if !((*actions.offset(k as isize)).type_0 as u32 >= _ACTION_TYPE_NUM_ENTRIES as u32) {
                if !filter_action_funcs[(*actions.offset(k as isize)).type_0 as usize]
                    .new
                    .is_none()
                {
                    filter = xkb_filter_new(state);
                    (*filter).key = key;
                    (*filter).action = *actions.offset(k as isize);
                    if (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 != 0 {
                        if (*filter).action.type_0 as u32 == ACTION_TYPE_MOD_SET as u32 {
                            (*filter).action.type_0 = ACTION_TYPE_MOD_LATCH;
                            if (*state).flags as u32 & XKB_A11Y_LATCH_TO_LOCK as u32 != 0 {
                                (*filter).action.mods.flags = ((*filter).action.mods.flags as u32
                                    | ACTION_LATCH_TO_LOCK as u32)
                                    as xkb_action_flags;
                            }
                        } else if (*filter).action.type_0 as u32 == ACTION_TYPE_GROUP_SET as u32 {
                            (*filter).action.type_0 = ACTION_TYPE_GROUP_LATCH;
                            if (*state).flags as u32 & XKB_A11Y_LATCH_TO_LOCK as u32 != 0 {
                                (*filter).action.group.flags = ((*filter).action.group.flags as u32
                                    | ACTION_LATCH_TO_LOCK as u32)
                                    as xkb_action_flags;
                            }
                        }
                    }
                    if (*filter).action.type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as u32 {
                        (*filter).action.redirect.affect = mod_mask_get_effective(
                            (*state).keymap,
                            (*filter).action.redirect.affect,
                        );
                        (*filter).action.redirect.mods =
                            mod_mask_get_effective((*state).keymap, (*filter).action.redirect.mods);
                    }
                    (*filter).func = filter_action_funcs[(*filter).action.type_0 as usize].func;
                    filter_action_funcs[(*filter).action.type_0 as usize]
                        .new
                        .expect("non-null function pointer")(
                        state, events, filter
                    );
                }
            }
            k = k.wrapping_add(1);
        }
    }
}
pub unsafe fn xkb_state_new(mut keymap: *mut xkb_keymap) -> *mut xkb_state {
    unsafe {
        let state: *mut xkb_state = Box::into_raw(Box::new(xkb_state {
            components: std::mem::zeroed(),
            controls: std::mem::zeroed(),
            set_mods: 0,
            clear_mods: 0,
            mod_key_count: [0; 32],
            flags: std::mem::zeroed(),
            refcnt: 0,
            filters: Vec::new(),
            keymap: std::ptr::null_mut(),
        }));
        // xkb_state_init inlined
        (*state).flags = XKB_A11Y_NO_FLAGS;
        if (*keymap).format as u32 != XKB_KEYMAP_FORMAT_TEXT_V1 as u32
            && XKB_A11Y_NO_FLAGS as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as u32 == 0
        {
            (*state).flags =
                ((*state).flags as u32 | XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as u32) as xkb_a11y_flags;
        }
        (*state).controls.out_of_range_group_policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP;
        (*state).refcnt = 1 as i32;
        (*state).keymap = xkb_keymap_ref(keymap);
        xkb_state_update_derived(state);
        return state;
    }
}

pub unsafe fn xkb_state_unref(mut state: *mut xkb_state) {
    unsafe {
        if state.is_null() || {
            (*state).refcnt -= 1;
            (*state).refcnt > 0 as i32
        } {
            return;
        }
        // xkb_state_destroy inlined
        xkb_keymap_unref((*state).keymap);
        // Vec<xkb_filter> will be dropped when the owning struct is dropped
        drop(Box::from_raw(state));
    }
}
pub unsafe fn xkb_state_get_keymap(mut state: *mut xkb_state) -> *mut xkb_keymap {
    unsafe {
        return (*state).keymap;
    }
}

unsafe fn xkb_state_led_update_all(mut state: *mut xkb_state) {
    unsafe {
        let mut idx: u32 = 0;
        let mut led: *const xkb_led = std::ptr::null();
        (*state).components.leds = 0 as xkb_led_mask_t;
        let mut c2rust_current_block_23: u64;
        idx = 0 as u32;
        led = &raw mut (*(*state).keymap).leds as *mut xkb_led;
        while idx < (*(*state).keymap).num_leds {
            if (*led).which_mods as u32 != 0 as u32 && (*led).mods.mask != 0 as u32 {
                let mut mod_mask: u32 = 0 as u32;
                if (*led).which_mods as u32 & XKB_STATE_MODS_EFFECTIVE as u32 != 0 {
                    mod_mask |= (*state).components.mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_DEPRESSED as u32 != 0 {
                    mod_mask |= (*state).components.base_mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_LATCHED as u32 != 0 {
                    mod_mask |= (*state).components.latched_mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_LOCKED as u32 != 0 {
                    mod_mask |= (*state).components.locked_mods;
                }
                if (*led).mods.mask & mod_mask != 0 {
                    (*state).components.leds =
                        ((*state).components.leds as u32 | (1 as u32) << idx) as xkb_led_mask_t;
                    c2rust_current_block_23 = 16559507199688588974;
                } else {
                    c2rust_current_block_23 = 13586036798005543211;
                }
            } else {
                c2rust_current_block_23 = 13586036798005543211;
            }
            match c2rust_current_block_23 {
                13586036798005543211 => {
                    if (*led).which_groups as i32 != 0 as i32 {
                        if ((*led).groups != 0) as i64 != 0 as i64 {
                            let mut group_mask: u32 = 0 as u32;
                            if (*led).which_groups as i32 & XKB_STATE_LAYOUT_EFFECTIVE as i32 != 0 {
                                group_mask = (group_mask as u32
                                    | (1 as u32) << (*state).components.group)
                                    as u32;
                            }
                            if (*led).which_groups as i32 & XKB_STATE_LAYOUT_LOCKED as i32 != 0 {
                                group_mask = (group_mask as u32
                                    | (1 as u32) << (*state).components.locked_group)
                                    as u32;
                            }
                            if (*led).which_groups as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32 != 0
                                && (*state).components.base_group != 0 as i32
                            {
                                group_mask |= (*led).groups;
                            }
                            if (*led).which_groups as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
                                && (*state).components.latched_group != 0 as i32
                            {
                                group_mask |= (*led).groups;
                            }
                            if (*led).groups & group_mask != 0 {
                                (*state).components.leds = ((*state).components.leds as u32
                                    | (1 as u32) << idx)
                                    as xkb_led_mask_t;
                                c2rust_current_block_23 = 16559507199688588974;
                            } else {
                                c2rust_current_block_23 = 14359455889292382949;
                            }
                        } else if (*led).which_groups as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32
                            != 0
                            && (*state).components.base_group == 0 as i32
                            || (*led).which_groups as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
                                && (*state).components.latched_group == 0 as i32
                        {
                            (*state).components.leds = ((*state).components.leds as u32
                                | (1 as u32) << idx)
                                as xkb_led_mask_t;
                            c2rust_current_block_23 = 16559507199688588974;
                        } else {
                            c2rust_current_block_23 = 14359455889292382949;
                        }
                    } else {
                        c2rust_current_block_23 = 14359455889292382949;
                    }
                    match c2rust_current_block_23 {
                        16559507199688588974 => {}
                        _ => {
                            if (*led).ctrls as u32 & (*state).components.controls as u32 != 0 {
                                (*state).components.leds = ((*state).components.leds as u32
                                    | (1 as u32) << idx)
                                    as xkb_led_mask_t;
                            }
                        }
                    }
                }
                _ => {}
            }
            idx = idx.wrapping_add(1);
            led = led.offset(1);
        }
    }
}

unsafe fn xkb_state_update_derived(mut state: *mut xkb_state) {
    unsafe {
        let mut wrapped: u32 = 0;
        (*state).components.mods = (*state).components.base_mods
            | (*state).components.latched_mods
            | (*state).components.locked_mods;
        wrapped = XkbWrapGroupIntoRange(
            (*state).components.locked_group,
            (*(*state).keymap).num_groups,
            (*state).controls.out_of_range_group_policy,
            (*state).controls.out_of_range_redirect_group,
        );
        (*state).components.locked_group = (if wrapped == XKB_LAYOUT_INVALID as u32 {
            0 as u32
        } else {
            wrapped
        }) as i32;
        wrapped = XkbWrapGroupIntoRange(
            (*state).components.base_group
                + (*state).components.latched_group
                + (*state).components.locked_group,
            (*(*state).keymap).num_groups,
            (*state).controls.out_of_range_group_policy,
            (*state).controls.out_of_range_redirect_group,
        );
        (*state).components.group = if wrapped == XKB_LAYOUT_INVALID as u32 {
            0 as u32
        } else {
            wrapped
        };
        xkb_state_led_update_all(state);
    }
}
pub unsafe fn xkb_state_update_key(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut direction: xkb_key_direction,
) -> u32 {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() || direction as u32 == XKB_KEY_REPEATED as u32 && !(*key).repeats {
            return 0 as u32;
        }
        let prev_components: state_components = (*state).components;
        (*state).set_mods = 0 as u32;
        (*state).clear_mods = 0 as u32;
        xkb_filter_apply_all(state, std::ptr::null_mut(), key, direction);
        let mut i: u32 = 0;
        let mut bit: u32 = 0;
        i = 0 as u32;
        bit = 1 as u32;
        while (*state).set_mods != 0 {
            if (*state).set_mods & bit != 0 {
                (*state).mod_key_count[i as usize] += 1;
                (*state).components.base_mods |= bit;
                (*state).set_mods &= !bit;
            }
            i = i.wrapping_add(1);
            bit <<= 1 as i32;
        }
        i = 0 as u32;
        bit = 1 as u32;
        while (*state).clear_mods != 0 {
            if (*state).clear_mods & bit != 0 {
                (*state).mod_key_count[i as usize] -= 1;
                if (*state).mod_key_count[i as usize] as i32 <= 0 as i32 {
                    (*state).components.base_mods &= !bit;
                    (*state).mod_key_count[i as usize] = 0 as i16;
                }
                (*state).clear_mods &= !bit;
            }
            i = i.wrapping_add(1);
            bit <<= 1 as i32;
        }
        xkb_state_update_derived(state);
        return get_state_component_changes(
            &raw const prev_components,
            &raw mut (*state).components,
        );
    }
}

static mut synthetic_key_level_entry: xkb_key_type_entry = xkb_key_type_entry {
    level: 0 as u32,
    mods: xkb_mods { mods: 0, mask: 0 },
    preserve: xkb_mods { mods: 0, mask: 0 },
};

static mut synthetic_key_type: xkb_key_type = {
    xkb_key_type {
        name: 0,
        mods: xkb_mods { mods: 0, mask: 0 },
        required: false,
        num_levels: 1 as u32,
        level_names: Vec::new(),
        entries: Vec::new(), // populated in c2rust_run_static_initializers
    }
};

static mut synthetic_key: xkb_key = xkb_key {
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

unsafe fn update_latch_modifiers(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut mask: u32,
    mut latches: u32,
) {
    unsafe {
        let clear: u32 = mask & !latches;
        (*state).components.latched_mods &= !clear;
        let mut synthetic_key_level_break_mod_latch: xkb_level = xkb_level {
            upper: XKB_KEY_NoSymbol as u32,
            has_upper: false,
            syms: Vec::new(),
            actions: vec![xkb_action {
                internal: xkb_internal_action {
                    type_0: ACTION_TYPE_INTERNAL,
                    flags: INTERNAL_BREAKS_MOD_LATCH,
                    clear_latched_mods: clear,
                },
            }],
        };
        let mut synthetic_key_group_break_mod_latch: xkb_group = xkb_group {
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
        let key: *const xkb_key = &raw const synthetic_key;
        let latch_mods: xkb_action = xkb_action {
            mods: xkb_mod_action {
                type_0: ACTION_TYPE_MOD_LATCH,
                flags: 0 as xkb_action_flags,
                mods: xkb_mods {
                    mods: 0,
                    mask: mask & latches,
                },
            },
        };
        let filter: *mut xkb_filter = xkb_filter_new(state) as *mut xkb_filter;
        (*filter).key = key;
        (*filter).func = Some(
            xkb_filter_mod_latch_func
                as unsafe fn(
                    *mut xkb_state,
                    *mut xkb_events,
                    *mut xkb_filter,
                    *const xkb_key,
                    xkb_key_direction,
                ) -> bool,
        )
            as Option<
                unsafe fn(
                    *mut xkb_state,
                    *mut xkb_events,
                    *mut xkb_filter,
                    *const xkb_key,
                    xkb_key_direction,
                ) -> bool,
            >;
        (*filter).action = latch_mods;
        xkb_filter_mod_latch_new(state, events, filter);
        xkb_filter_mod_latch_func(state, events, filter, key, XKB_KEY_UP);
    }
}

unsafe fn update_latch_group(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut group: i32,
) {
    unsafe {
        xkb_filter_apply_all(
            state,
            events,
            &raw const synthetic_key_break_group_latch,
            XKB_KEY_DOWN,
        );
        let key: *const xkb_key = &raw const synthetic_key;
        let latch_group: xkb_action = xkb_action {
            group: xkb_group_action {
                type_0: ACTION_TYPE_GROUP_LATCH,
                flags: ACTION_ABSOLUTE_SWITCH,
                group: group,
            },
        };
        let filter: *mut xkb_filter = xkb_filter_new(state) as *mut xkb_filter;
        (*filter).key = key;
        (*filter).func = Some(
            xkb_filter_group_latch_func
                as unsafe fn(
                    *mut xkb_state,
                    *mut xkb_events,
                    *mut xkb_filter,
                    *const xkb_key,
                    xkb_key_direction,
                ) -> bool,
        )
            as Option<
                unsafe fn(
                    *mut xkb_state,
                    *mut xkb_events,
                    *mut xkb_filter,
                    *const xkb_key,
                    xkb_key_direction,
                ) -> bool,
            >;
        (*filter).action = latch_group;
        xkb_filter_group_latch_new(state, events, filter);
        xkb_filter_group_latch_func(state, events, filter, key, XKB_KEY_UP);
    }
}
#[inline]

unsafe fn resolve_to_canonical_mods(mut keymap: *mut xkb_keymap, mut mods: u32) -> u32 {
    unsafe {
        return mods & (*keymap).canonical_state_mask
            | mod_mask_get_effective(keymap, mods & !(*keymap).canonical_state_mask);
    }
}

unsafe fn state_update_latched_locked(
    mut state: *mut xkb_state,
    mut update: *const xkb_state_components_update,
    mut events: *mut xkb_events,
) {
    unsafe {
        let affect_locked_mods: u32 =
            resolve_to_canonical_mods((*state).keymap, (*update).affect_locked_mods) as u32;
        if affect_locked_mods != 0 {
            let locked_mods: u32 =
                resolve_to_canonical_mods((*state).keymap, (*update).locked_mods) as u32;
            (*state).components.locked_mods &= !affect_locked_mods;
            (*state).components.locked_mods |= locked_mods & affect_locked_mods;
        }
        if (*update).components as u32 & XKB_STATE_LAYOUT_LOCKED as u32 != 0 {
            (*state).components.locked_group = (*update).locked_layout;
        }
        let affect_latched_mods: u32 =
            resolve_to_canonical_mods((*state).keymap, (*update).affect_latched_mods) as u32;
        if affect_latched_mods != 0 {
            let latched_mods: u32 =
                resolve_to_canonical_mods((*state).keymap, (*update).latched_mods) as u32;
            update_latch_modifiers(state, events, affect_latched_mods, latched_mods);
        }
        if (*update).components as u32 & XKB_STATE_LAYOUT_LATCHED as u32 != 0 {
            update_latch_group(state, events, (*update).latched_layout);
        }
    }
}

#[inline]

unsafe fn clear_all_latches_and_locks(mut state: *mut xkb_state, mut events: *mut xkb_events) {
    unsafe {
        static mut components: u32 = (XKB_STATE_MODS_LATCHED as i32
            | XKB_STATE_MODS_LOCKED as i32
            | XKB_STATE_LAYOUT_LATCHED as i32
            | XKB_STATE_LAYOUT_LOCKED as i32) as u32;
        let update: xkb_state_components_update = xkb_state_components_update {
            size: std::mem::size_of::<xkb_state_components_update>(),
            components: components,
            affect_latched_mods: XKB_MOD_ALL as u32 as u32,
            latched_mods: 0 as u32,
            affect_locked_mods: XKB_MOD_ALL as u32 as u32,
            locked_mods: 0 as u32,
            latched_layout: 0 as i32,
            locked_layout: 0 as i32,
            affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
            controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
        };
        state_update_latched_locked(state, &raw const update, events);
    }
}

unsafe fn state_update_layout_policy(
    mut state: *mut xkb_state,
    mut update: *const xkb_layout_policy_update,
) -> xkb_error_code {
    unsafe {
        if xkb_feature_supported(
            XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY,
            (*update).policy as u32,
        ) {
            if (*update).policy as u32 == XKB_LAYOUT_OUT_OF_RANGE_REDIRECT as u32 {
                if (*update).redirect < (*(*state).keymap).num_groups {
                    (*state).controls.out_of_range_redirect_group = (*update).redirect;
                } else {
                    xkb_logf!(
                        (*(*state).keymap).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Layout policy: unsupported layout index {} > {}\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        (*update).redirect.wrapping_add(1 as u32),
                        (*(*state).keymap).num_groups,
                    );
                    return XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX;
                }
            }
            (*state).controls.out_of_range_group_policy = (*update).policy;
            return XKB_SUCCESS;
        } else {
            xkb_logf!(
                (*(*state).keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Unsupported layout policy: {}\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY as i32,
                (*update).policy as u32,
            );
            return XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY;
        };
    }
}

unsafe fn log_abi_error(mut ctx: *mut xkb_context, mut func: *const i8, mut error: xkb_error_code) {
    match error as i32 {
        450 => {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}: ABI error: unsupported versioned struct\n",
                XKB_ERROR_ABI_INVALID_STRUCT_SIZE as i32,
                crate::xkb::utils::CStrDisplay(func),
            );
        }
        914 => {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}: ABI version mismatch: missing newer required fields\n",
                XKB_ERROR_ABI_BACKWARD_COMPAT as i32,
                crate::xkb::utils::CStrDisplay(func),
            );
        }
        876 => {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] {}: ABI version mismatch: cannot use newer fields\n",
                XKB_ERROR_ABI_FORWARD_COMPAT as i32,
                crate::xkb::utils::CStrDisplay(func),
            );
        }
        _ => {}
    };
}

unsafe fn check_state_update_abi_(
    mut ctx: *mut xkb_context,
    mut func: *const i8,
    mut update: *const xkb_state_update,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code = XKB_SUCCESS;
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
        return error;
    }
}

pub unsafe fn xkb_state_update_synthetic(
    mut state: *mut xkb_state,
    mut update: *const xkb_state_update,
    mut changed: *mut u32,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code = check_state_update_abi_(
            &raw mut (*(*state).keymap).ctx,
            b"xkb_state_update_synthetic\0".as_ptr() as *const i8,
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
                let had_sticky_keys: bool =
                    (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 != 0;
                let mut affect = ((*components).affect_controls as xkb_action_controls as u32
                    & CONTROL_ALL_BOOLEAN as u32)
                    as xkb_keyboard_control_flags;
                (*state).components.controls =
                    ((*state).components.controls as u32 & !(affect as u32)) as xkb_action_controls;
                (*state).components.controls = ((*state).components.controls as u32
                    | (*components).controls as xkb_action_controls as u32 & affect as u32)
                    as xkb_action_controls;
                if had_sticky_keys as i32 != 0
                    && (*state).components.controls as u32 & CONTROL_STICKY_KEYS as u32 == 0
                {
                    clear_all_latches_and_locks(state, std::ptr::null_mut());
                }
                xkb_state_update_derived(state);
            }
            state_update_latched_locked(state, components, std::ptr::null_mut());
        }
        xkb_state_update_derived(state);
        if !changed.is_null() {
            *changed = get_state_component_changes(
                &raw const previous_components,
                &raw mut (*state).components,
            );
        }
        return XKB_SUCCESS;
    }
}

pub unsafe fn xkb_state_update_mask(
    mut state: *mut xkb_state,
    mut base_mods: u32,
    mut latched_mods: u32,
    mut locked_mods: u32,
    mut base_group: u32,
    mut latched_group: u32,
    mut locked_group: u32,
) -> u32 {
    unsafe {
        let prev_components: state_components = (*state).components;
        (*state).components.base_mods = resolve_to_canonical_mods((*state).keymap, base_mods);
        (*state).components.latched_mods = resolve_to_canonical_mods((*state).keymap, latched_mods);
        (*state).components.locked_mods = resolve_to_canonical_mods((*state).keymap, locked_mods);
        (*state).components.base_group = base_group as i32;
        (*state).components.latched_group = latched_group as i32;
        (*state).components.locked_group = locked_group as i32;
        xkb_state_update_derived(state);
        return get_state_component_changes(
            &raw const prev_components,
            &raw mut (*state).components,
        );
    }
}

unsafe fn should_do_caps_transformation(mut state: *mut xkb_state, mut kc: u32) -> bool {
    unsafe {
        return xkb_state_mod_index_is_active(
            state,
            XKB_MOD_INDEX_CAPS as i32 as u32,
            XKB_STATE_MODS_EFFECTIVE,
        ) > 0 as i32
            && xkb_state_mod_index_is_consumed(state, kc, XKB_MOD_INDEX_CAPS as i32 as u32)
                == 0 as i32;
    }
}

unsafe fn should_do_ctrl_transformation(mut state: *mut xkb_state, mut kc: u32) -> bool {
    unsafe {
        return xkb_state_mod_index_is_active(
            state,
            XKB_MOD_INDEX_CTRL as i32 as u32,
            XKB_STATE_MODS_EFFECTIVE,
        ) > 0 as i32
            && xkb_state_mod_index_is_consumed(state, kc, XKB_MOD_INDEX_CTRL as i32 as u32)
                == 0 as i32;
    }
}
pub unsafe fn xkb_state_key_get_syms(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut syms_out: *mut *const u32,
) -> i32 {
    unsafe {
        let mut level: u32 = 0;
        let mut key: *const xkb_key = std::ptr::null();
        let mut leveli: *const xkb_level = std::ptr::null();
        let layout: u32 = xkb_state_key_get_layout(state, kc) as u32;
        if !(layout == XKB_LAYOUT_INVALID as u32) {
            level = xkb_state_key_get_level(state, kc, layout) as u32;
            if !(level == XKB_LEVEL_INVALID as u32) {
                key = XkbKey((*state).keymap, kc) as *const xkb_key;
                if !key.is_null() {
                    leveli = xkb_keymap_key_get_level((*state).keymap, key, layout, level);
                    if !leveli.is_null() {
                        let num_syms = (*leveli).syms.len();
                        if num_syms > 0 {
                            if should_do_caps_transformation(state, kc) {
                                if num_syms > 1 {
                                    *syms_out = if (*leveli).has_upper as i32 != 0 {
                                        (*leveli).syms.as_ptr().offset(num_syms as isize)
                                    } else {
                                        (*leveli).syms.as_ptr()
                                    };
                                } else {
                                    *syms_out = &raw const (*leveli).upper;
                                }
                            } else {
                                *syms_out = (*leveli).syms.as_ptr();
                            }
                            return num_syms as i32;
                        }
                    }
                }
            }
        }
        *syms_out = std::ptr::null();
        return 0 as i32;
    }
}

pub unsafe fn xkb_state_key_get_one_sym(mut state: *mut xkb_state, mut kc: u32) -> u32 {
    unsafe {
        let mut syms: *const u32 = std::ptr::null();
        let num_syms: i32 = xkb_state_key_get_syms(state, kc, &raw mut syms) as i32;
        if num_syms != 1 as i32 {
            return XKB_KEY_NoSymbol as u32;
        } else {
            return *syms.offset(0 as i32 as isize);
        };
    }
}

unsafe fn get_one_sym_for_string(mut state: *mut xkb_state, mut kc: u32) -> u32 {
    unsafe {
        let layout: u32 = xkb_state_key_get_layout(state, kc) as u32;
        let num_layouts: u32 = xkb_keymap_num_layouts_for_key((*state).keymap, kc) as u32;
        let mut level: u32 = xkb_state_key_get_level(state, kc, layout);
        if layout == XKB_LAYOUT_INVALID as u32
            || num_layouts == 0 as u32
            || level == XKB_LEVEL_INVALID as u32
        {
            return XKB_KEY_NoSymbol as u32;
        }
        let mut syms: *const u32 = std::ptr::null();
        let mut nsyms: i32 =
            xkb_keymap_key_get_syms_by_level((*state).keymap, kc, layout, level, &raw mut syms);
        if nsyms != 1 as i32 {
            return XKB_KEY_NoSymbol as u32;
        }
        let mut sym: u32 = *syms.offset(0 as i32 as isize);
        if should_do_ctrl_transformation(state, kc) as i32 != 0 && sym > 127 as u32 {
            let mut i: u32 = 0 as u32;
            while i < num_layouts {
                level = xkb_state_key_get_level(state, kc, i);
                if !(level == XKB_LEVEL_INVALID as u32) {
                    nsyms = xkb_keymap_key_get_syms_by_level(
                        (*state).keymap,
                        kc,
                        i,
                        level,
                        &raw mut syms,
                    );
                    if nsyms == 1 as i32 && *syms.offset(0 as i32 as isize) <= 127 as u32 {
                        sym = *syms.offset(0 as i32 as isize);
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if should_do_caps_transformation(state, kc) {
            sym = xkb_keysym_to_upper(sym);
        }
        return sym;
    }
}

pub unsafe fn xkb_state_key_get_utf8(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut buffer: *mut i8,
    mut size: usize,
) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nsyms: i32 = 0;
        let mut syms: *const u32 = std::ptr::null();
        let sym: u32 = get_one_sym_for_string(state, kc) as u32;
        if sym != XKB_KEY_NoSymbol as u32 {
            nsyms = 1 as i32;
            syms = &raw const sym;
        } else {
            nsyms = xkb_state_key_get_syms(state, kc, &raw mut syms);
        }
        let mut offset: i32 = 0 as i32;
        let mut tmp: [i8; 5] = [0; 5];
        let mut i: i32 = 0 as i32;
        loop {
            if !(i < nsyms) {
                c2rust_current_block = 11050875288958768710;
                break;
            }
            let mut ret: i32 = xkb_keysym_to_utf8(
                *syms.offset(i as isize),
                &raw mut tmp as *mut i8,
                std::mem::size_of::<[i8; 5]>(),
            );
            if ret <= 0 as i32 {
                c2rust_current_block = 17545813786824981435;
                break;
            }
            ret -= 1;
            if (offset as usize).wrapping_add(ret as usize) <= size {
                std::ptr::copy_nonoverlapping(
                    &raw mut tmp as *const u8,
                    buffer.offset(offset as isize) as *mut u8,
                    ret as usize,
                );
            }
            offset += ret;
            i += 1;
        }
        match c2rust_current_block {
            11050875288958768710 => {
                if offset as usize >= size {
                    if size > 0 as usize {
                        *buffer.offset(size.wrapping_sub(1 as usize) as isize) = '\0' as i32 as i8;
                    }
                    return offset;
                } else {
                    *buffer.offset(offset as isize) = '\0' as i32 as i8;
                    if is_valid_utf8(buffer, offset as usize) {
                        if offset == 1 as i32
                            && *buffer.offset(0 as i32 as isize) as u32 <= 127 as u32
                            && should_do_ctrl_transformation(state, kc) as i32 != 0
                        {
                            // XkbToControl inlined
                            *buffer.offset(0 as i32 as isize) = {
                                let mut c: i8 = *buffer.offset(0 as i32 as isize);
                                if c as i32 >= '@' as i32 && (c as i32) < '\u{7f}' as i32
                                    || c as i32 == ' ' as i32
                                {
                                    c = (c as i32 & 0x1f as i32) as i8;
                                } else if c as i32 == '2' as i32 {
                                    c = '\0' as i32 as i8;
                                } else if c as i32 >= '3' as i32 && c as i32 <= '7' as i32 {
                                    c = (c as i32 - ('3' as i32 - '\u{1b}' as i32)) as i8;
                                } else if c as i32 == '8' as i32 {
                                    c = '\u{7f}' as i32 as i8;
                                } else if c as i32 == '/' as i32 {
                                    c = ('_' as i32 & 0x1f as i32) as i8;
                                }
                                c
                            };
                        }
                        return offset;
                    }
                }
            }
            _ => {}
        }
        if size > 0 as usize {
            *buffer.offset(0 as i32 as isize) = '\0' as i32 as i8;
        }
        return 0 as i32;
    }
}

#[inline]

unsafe fn serialize_mods(mut components: *const state_components, mut type_0: u32) -> u32 {
    unsafe {
        let mut ret: u32 = 0 as u32;
        if type_0 as u32 & XKB_STATE_MODS_EFFECTIVE as u32 != 0 {
            return (*components).mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_DEPRESSED as u32 != 0 {
            ret |= (*components).base_mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_LATCHED as u32 != 0 {
            ret |= (*components).latched_mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_LOCKED as u32 != 0 {
            ret |= (*components).locked_mods;
        }
        return ret;
    }
}
pub unsafe fn xkb_state_serialize_mods(mut state: *mut xkb_state, mut type_0: u32) -> u32 {
    unsafe {
        return serialize_mods(&raw mut (*state).components, type_0);
    }
}
#[inline]

unsafe fn serialize_layout(mut components: *const state_components, mut type_0: u32) -> u32 {
    unsafe {
        let mut ret: u32 = 0 as u32;
        if type_0 as u32 & XKB_STATE_LAYOUT_EFFECTIVE as u32 != 0 {
            return (*components).group;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_DEPRESSED as u32 != 0 {
            ret = ret.wrapping_add((*components).base_group as u32);
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LATCHED as u32 != 0 {
            ret = ret.wrapping_add((*components).latched_group as u32);
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LOCKED as u32 != 0 {
            ret = ret.wrapping_add((*components).locked_group as u32);
        }
        return ret;
    }
}
pub unsafe fn xkb_state_serialize_layout(mut state: *mut xkb_state, mut type_0: u32) -> u32 {
    unsafe {
        return serialize_layout(&raw mut (*state).components, type_0);
    }
}
#[inline]

unsafe fn serialize_controls(
    mut components: *const state_components,
    mut type_0: u32,
) -> xkb_keyboard_control_flags {
    unsafe {
        return (if type_0 as u32 & XKB_STATE_CONTROLS as u32 != 0 {
            ((*components).controls as u32 & CONTROL_ALL_BOOLEAN as u32)
                as xkb_keyboard_control_flags as u32
        } else {
            0 as u32
        }) as xkb_keyboard_control_flags;
    }
}

pub unsafe fn mod_mask_get_effective(mut keymap: *mut xkb_keymap, mut mods: u32) -> u32 {
    unsafe {
        let mut mask: u32 = mods & MOD_REAL_MASK_ALL;
        let mut mod_0: *const xkb_mod = std::ptr::null();
        let mut i: u32 = 0;
        i = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as u32;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize) as *mut xkb_mod;
        while i < (*keymap).mods.num_mods {
            if mods & (1 as u32) << i != 0 {
                mask |= (*mod_0).mapping;
            }
            i = i.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return mask;
    }
}

pub unsafe fn xkb_state_mod_index_is_active(
    mut state: *mut xkb_state,
    mut idx: u32,
    mut type_0: u32,
) -> i32 {
    unsafe {
        if (idx >= xkb_keymap_num_mods((*state).keymap)) as i64 != 0 {
            return -1 as i32;
        }
        let mapping: u32 = (*(*state).keymap).mods.mods[idx as usize].mapping;
        if mapping == 0 {
            return 0 as i32;
        }
        return (xkb_state_serialize_mods(state, type_0) & mapping == mapping) as i32;
    }
}

unsafe fn match_mod_masks(
    mut state: *mut xkb_state,
    mut type_0: u32,
    mut match_0: xkb_state_match,
    mut wanted: u32,
) -> bool {
    unsafe {
        let active: u32 = xkb_state_serialize_mods(state, type_0) as u32;
        if match_0 as u32 & XKB_STATE_MATCH_NON_EXCLUSIVE as u32 == 0 && active & !wanted != 0 {
            return false;
        }
        if match_0 as u32 & XKB_STATE_MATCH_ANY as u32 != 0 {
            return active & wanted != 0;
        }
        return active & wanted == wanted;
    }
}

pub unsafe extern "C" fn xkb_state_mod_indices_are_active(
    mut state: *mut xkb_state,
    mut type_0: u32,
    mut match_0: xkb_state_match,
    mut c2rust_args: ...
) -> i32 {
    unsafe {
        if match_0 as u32 & !(XKB_STATE_MATCH_FLAGS as i32 as xkb_state_match as u32) != 0 {
            return -2 as i32;
        }
        let mut ap: ::core::ffi::VaList;
        let mut wanted: u32 = 0 as u32;
        let mut ret: i32 = 0 as i32;
        let num_mods: u32 = xkb_keymap_num_mods((*state).keymap) as u32;
        ap = c2rust_args.clone();
        loop {
            let mut idx: u32 = ap.arg::<u32>();
            if idx == XKB_MOD_INVALID as u32 {
                break;
            }
            if (idx >= num_mods) as i64 != 0 {
                ret = -1 as i32;
                break;
            } else {
                wanted |= (*(*state).keymap).mods.mods[idx as usize].mapping;
            }
        }
        if ret == -1 as i32 {
            return ret;
        }
        if wanted == 0 {
            return 0 as i32;
        }
        return match_mod_masks(state, type_0, match_0, wanted) as i32;
    }
}

pub unsafe fn xkb_state_mod_name_is_active(
    mut state: *mut xkb_state,
    mut name: *const i8,
    mut type_0: u32,
) -> i32 {
    unsafe {
        let idx: u32 = xkb_keymap_mod_get_index((*state).keymap, name) as u32;
        if idx == XKB_MOD_INVALID as u32 {
            return -1 as i32;
        }
        return xkb_state_mod_index_is_active(state, idx, type_0);
    }
}

pub unsafe extern "C" fn xkb_state_mod_names_are_active(
    mut state: *mut xkb_state,
    mut type_0: u32,
    mut match_0: xkb_state_match,
    mut c2rust_args: ...
) -> i32 {
    unsafe {
        if match_0 as u32 & !(XKB_STATE_MATCH_FLAGS as i32 as xkb_state_match as u32) != 0 {
            return -2 as i32;
        }
        let mut ap: ::core::ffi::VaList;
        let mut wanted: u32 = 0 as u32;
        let mut ret: i32 = 0 as i32;
        ap = c2rust_args.clone();
        loop {
            let mut str: *const i8 = ap.arg::<*const i8>();
            if str.is_null() {
                break;
            }
            let idx: u32 = xkb_keymap_mod_get_index((*state).keymap, str) as u32;
            if idx == XKB_MOD_INVALID as u32 {
                ret = -1 as i32;
                break;
            } else {
                wanted |= (*(*state).keymap).mods.mods[idx as usize].mapping;
            }
        }
        if ret == -1 as i32 {
            return ret;
        }
        if wanted == 0 {
            return 0 as i32;
        }
        return match_mod_masks(state, type_0, match_0, wanted) as i32;
    }
}

pub unsafe fn xkb_state_layout_index_is_active(
    mut state: *mut xkb_state,
    mut idx: u32,
    mut type_0: u32,
) -> i32 {
    unsafe {
        if idx >= (*(*state).keymap).num_groups {
            return -1 as i32;
        }
        let mut ret: i32 = 0 as i32;
        if type_0 as u32 & XKB_STATE_LAYOUT_EFFECTIVE as u32 != 0 {
            ret |= ((*state).components.group == idx) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_DEPRESSED as u32 != 0 {
            ret |= ((*state).components.base_group == idx as i32) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LATCHED as u32 != 0 {
            ret |= ((*state).components.latched_group == idx as i32) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LOCKED as u32 != 0 {
            ret |= ((*state).components.locked_group == idx as i32) as i32;
        }
        return ret;
    }
}

pub unsafe fn xkb_state_layout_name_is_active(
    mut state: *mut xkb_state,
    mut name: *const i8,
    mut type_0: u32,
) -> i32 {
    unsafe {
        let idx: u32 = xkb_keymap_layout_get_index((*state).keymap, name) as u32;
        if idx == XKB_LAYOUT_INVALID as u32 {
            return -1 as i32;
        }
        return xkb_state_layout_index_is_active(state, idx, type_0);
    }
}
pub unsafe fn xkb_state_led_index_is_active(mut state: *mut xkb_state, mut idx: u32) -> i32 {
    unsafe {
        if idx >= (*(*state).keymap).num_leds
            || (*(*state).keymap).leds[idx as usize].name == XKB_ATOM_NONE as u32
        {
            return -1 as i32;
        }
        return ((*state).components.leds & (1 as xkb_led_mask_t) << idx != 0) as i32;
    }
}

pub unsafe fn xkb_state_led_name_is_active(mut state: *mut xkb_state, mut name: *const i8) -> i32 {
    unsafe {
        let idx: u32 = xkb_keymap_led_get_index((*state).keymap, name) as u32;
        if idx == XKB_LED_INVALID as u32 {
            return -1 as i32;
        }
        return xkb_state_led_index_is_active(state, idx);
    }
}

unsafe fn key_get_consumed(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut mode: xkb_consumed_mode,
) -> u32 {
    unsafe {
        let group: u32 = xkb_state_key_get_layout(state, (*key).keycode) as u32;
        if group == XKB_LAYOUT_INVALID as u32 {
            return 0 as u32;
        }
        let mut preserve: u32 = 0 as u32;
        let mut consumed: u32 = 0 as u32;
        let matching_entry: *const xkb_key_type_entry =
            get_entry_for_key_state(state, key, group) as *const xkb_key_type_entry;
        if !matching_entry.is_null() {
            preserve = (*matching_entry).preserve.mask;
        }
        let type_0: *const xkb_key_type =
            &(&(*(*state).keymap).types)[(&(*key).groups)[group as usize].type_idx as usize];
        match mode as u32 {
            0 => {
                consumed = (*type_0).mods.mask;
            }
            1 => {
                let no_mods_entry: *const xkb_key_type_entry =
                    get_entry_for_mods(type_0, 0 as u32) as *const xkb_key_type_entry;
                let no_mods_leveli: u32 = if !no_mods_entry.is_null() {
                    (*no_mods_entry).level
                } else {
                    0 as u32
                };
                let no_mods_level: *const xkb_level = &(&(*key).groups)[group as usize].levels
                    [no_mods_leveli as usize]
                    as *const xkb_level;
                let mut i: u32 = 0 as u32;
                while i < (&(*type_0).entries).len() as u32 {
                    let entry: *const xkb_key_type_entry =
                        &(&(*type_0).entries)[i as usize] as *const xkb_key_type_entry;
                    if entry_is_active(entry) {
                        let level: *const xkb_level = &(&(*key).groups)[group as usize].levels
                            [(*entry).level as usize]
                            as *const xkb_level;
                        if !XkbLevelsSameSyms(level, no_mods_level) {
                            if entry == matching_entry
                                || ((*entry).mods.mask as u32 != 0
                                    && ((*entry).mods.mask as u32).is_power_of_two())
                            {
                                consumed |= (*entry).mods.mask & !(*entry).preserve.mask;
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
        return consumed & !preserve;
    }
}

pub unsafe fn xkb_state_mod_index_is_consumed2(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut idx: u32,
    mut mode: xkb_consumed_mode,
) -> i32 {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if (key.is_null() || idx >= xkb_keymap_num_mods((*state).keymap)) as i64 != 0 {
            return -1 as i32;
        }
        let mapping: u32 = (*(*state).keymap).mods.mods[idx as usize].mapping;
        if mapping == 0 {
            return 0 as i32;
        }
        return (mapping & key_get_consumed(state, key, mode) == mapping) as i32;
    }
}

pub unsafe fn xkb_state_mod_index_is_consumed(
    mut state: *mut xkb_state,
    mut kc: u32,
    mut idx: u32,
) -> i32 {
    unsafe {
        return xkb_state_mod_index_is_consumed2(state, kc, idx, XKB_CONSUMED_MODE_XKB);
    }
}

unsafe fn c2rust_run_static_initializers() {
    unsafe {
        synthetic_key_type.entries = vec![synthetic_key_level_entry];
        synthetic_key_group_break_group_latch = xkb_group {
            explicit_symbols: false,
            explicit_actions: false,
            implicit_actions: false,
            explicit_type: false,
            type_idx: 0,
            levels: vec![xkb_level {
                upper: XKB_KEY_NoSymbol as u32,
                has_upper: false,
                syms: Vec::new(),
                actions: vec![xkb_action {
                    internal: xkb_internal_action {
                        type_0: ACTION_TYPE_INTERNAL,
                        flags: INTERNAL_BREAKS_GROUP_LATCH,
                        clear_latched_mods: 0,
                    },
                }],
            }],
        };
        synthetic_key_break_group_latch = xkb_key {
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
            groups: vec![synthetic_key_group_break_group_latch.clone()],
            overlay_keys: Vec::new(),
        };
        synthetic_key = xkb_key {
            keycode: 0 as u32,
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
    xkb_keymap_key_get_syms_by_level, xkb_keymap_layout_get_index, xkb_keymap_led_get_index,
    xkb_keymap_mod_get_index, xkb_keymap_num_layouts_for_key, xkb_keymap_num_mods, xkb_keymap_ref,
    xkb_keymap_unref,
};
use crate::xkb::keysym_case_mappings::xkb_keysym_to_upper;
use crate::xkb::keysym_utf::xkb_keysym_to_utf8;
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
