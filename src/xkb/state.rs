use crate::xkb::utils::{darray_append, darray_free, darray_resize, darray_resize_zero};
use crate::xkb_logf;
use c2rust_bitfields;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_event {
    pub type_0: xkb_event_type,
    pub c2rust_unnamed: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2Rust_Unnamed_17 {
    pub keycode: xkb_keycode_t,
    pub components: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_18 {
    pub components: state_components,
    pub changed: xkb_state_component,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct state_components {
    pub base_group: i32,
    pub latched_group: i32,
    pub locked_group: i32,
    pub group: xkb_layout_index_t,
    pub base_mods: xkb_mod_mask_t,
    pub latched_mods: xkb_mod_mask_t,
    pub locked_mods: xkb_mod_mask_t,
    pub mods: xkb_mod_mask_t,
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
    pub policy: xkb_layout_out_of_range_policy,
    pub redirect: xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_state_components_update_v1 {
    pub size: usize,
    pub components: xkb_state_component,
    pub affect_latched_mods: xkb_mod_mask_t,
    pub latched_mods: xkb_mod_mask_t,
    pub affect_locked_mods: xkb_mod_mask_t,
    pub locked_mods: xkb_mod_mask_t,
    pub latched_layout: i32,
    pub locked_layout: i32,
    pub affect_controls: xkb_keyboard_control_flags,
    pub controls: xkb_keyboard_control_flags,
}
use crate::xkb::shared_types::{
    xkb_action_controls, xkb_event_type, xkb_keyboard_control_flags, xkb_keycode_t,
    xkb_layout_index_t, xkb_layout_out_of_range_policy, xkb_led_mask_t, xkb_mod_mask_t,
    xkb_state_component,
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
pub use crate::xkb::keymap_priv::{
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
pub use crate::xkb::shared_types::darray_size_t;
pub use crate::xkb::shared_types::{
    entry_is_active, format_max_overlays, mod_type, real_mod_index, xkb_action, xkb_action_count_t,
    xkb_action_flags, xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group,
    xkb_group_action, xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias,
    xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level,
    xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    C2Rust_Unnamed_1, C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2,
    C2Rust_Unnamed_23, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbKey,
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
pub use crate::xkb::utils::one_bit_set;
pub use crate::xkb::utils::popcount32;
pub use crate::xkb::utils::xkb_check_versioned_struct_size_;
use libc::{calloc, free, qsort};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_machine {
    pub state: xkb_state,
    pub overlays: C2Rust_Unnamed_13,
    pub config: machine_config,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_config {
    pub modifiers: machine_modifiers_config,
    pub shortcuts: machine_shortcuts_config,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_shortcuts_config {
    pub mask: xkb_mod_mask_t,
    pub targets: *mut xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_modifiers_config {
    pub mask: xkb_mod_mask_t,
    pub mappings_num: darray_size_t,
    pub mappings: *mut machine_mods_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_mods_mapping {
    pub source: xkb_mod_mask_t,
    pub target: xkb_mod_mask_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_13 {
    pub enabled: xkb_overlay_mask_t,
    pub order: u32,
    pub keys: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_14 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_overlaid_key,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_overlaid_key {
    pub old: *const xkb_key,
    pub new: *const xkb_key,
    pub refcnt: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_state {
    pub components: state_components,
    pub controls: machine_controls,
    pub set_mods: xkb_mod_mask_t,
    pub clear_mods: xkb_mod_mask_t,
    pub mod_key_count: [i16; 32],
    pub flags: xkb_a11y_flags,
    pub refcnt: i32,
    pub filters: C2Rust_Unnamed_15,
    pub keymap: *mut xkb_keymap,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_15 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_filter,
}
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
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_events {
    pub next: darray_size_t,
    pub queue: C2Rust_Unnamed_16,
    pub ctx: *mut xkb_context,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_16 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_event,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_controls {
    pub out_of_range_group: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_19 {
    pub policy: xkb_layout_out_of_range_policy,
    pub redirect_group: xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_machine_options {
    pub a11y_affect: xkb_a11y_flags,
    pub a11y_flags: xkb_a11y_flags,
    pub mods: machine_mods_mappings,
    pub shortcuts: xkb_shortcuts_config_options,
    pub ctx: *mut xkb_context,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct xkb_shortcuts_config_options {
    pub mask: xkb_mod_mask_t,
    pub targets: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_20 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_layout_index_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct machine_mods_mappings {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut machine_mods_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_21 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_22,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    #[bitfield(name = "latch", ty = "u32", bits = "0..=1")]
    #[bitfield(name = "group_delta", ty = "i32", bits = "2..=31")]
    pub latch_group_delta: [u8; 4],
}

pub type xkb_key_latch_state = u32;

pub const _KEY_LATCH_STATE_NUM_ENTRIES: xkb_key_latch_state = 3;

pub const LATCH_PENDING: xkb_key_latch_state = 2;

pub const LATCH_KEY_DOWN: xkb_key_latch_state = 1;

pub const NO_LATCH: xkb_key_latch_state = 0;

pub const XKB_STATE_MATCH_FLAGS: C2Rust_Unnamed_24 = 65539;

pub type xkb_filter_result = u32;

pub type C2Rust_Unnamed_24 = u32;
static mut synthetic_key_group_break_group_latch: xkb_group = xkb_group {
    explicit_symbols_explicit_actions_implicit_actions_explicit_type: [0; 1],
    c2rust_padding: [0; 7],
    type_0: std::ptr::null(),
    levels: std::ptr::null_mut(),
};
static mut synthetic_key_break_group_latch: xkb_key = xkb_key {
    keycode: 0,
    name: 0,
    explicit: 0 as xkb_explicit_components,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups: [0; 3],
    groups: std::ptr::null_mut(),
    c2rust_unnamed: C2Rust_Unnamed_9 {
        overlay_key: std::ptr::null(),
    },
};
static mut synthetic_key_level_break_group_latch: xkb_level = xkb_level {
    num_syms: 0 as xkb_keysym_count_t,
    num_actions: 1 as xkb_action_count_t,
    c2rust_unnamed: C2Rust_Unnamed_12 {
        upper: XKB_KEY_NoSymbol as xkb_keysym_t,
    },
    s: C2Rust_Unnamed_11 {
        sym: XKB_KEY_NoSymbol as xkb_keysym_t,
    },
    a: C2Rust_Unnamed_10 {
        action: xkb_action {
            internal: xkb_internal_action {
                type_0: ACTION_TYPE_INTERNAL,
                flags: INTERNAL_BREAKS_GROUP_LATCH,
                c2rust_unnamed: C2Rust_Unnamed_2 {
                    clear_latched_mods: 0 as i32 as xkb_mod_mask_t,
                },
            },
        },
    },
};

unsafe fn get_entry_for_mods(
    mut type_0: *const xkb_key_type,
    mut mods: xkb_mod_mask_t,
) -> *const xkb_key_type_entry {
    unsafe {
        let mut i: darray_size_t = 0 as darray_size_t;
        while i < (*type_0).num_entries {
            if entry_is_active((*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry)
                as i32
                != 0
                && (*(*type_0).entries.offset(i as isize)).mods.mask == mods
            {
                return (*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry;
            }
            i = i.wrapping_add(1);
        }
        return std::ptr::null();
    }
}

unsafe fn get_entry_for_key_state(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut group: xkb_layout_index_t,
) -> *const xkb_key_type_entry {
    unsafe {
        let type_0: *const xkb_key_type = (*(*key).groups.offset(group as isize)).type_0;
        let mut active_mods: xkb_mod_mask_t = (*state).components.mods & (*type_0).mods.mask;
        return get_entry_for_mods(type_0, active_mods);
    }
}
#[inline]

unsafe fn state_key_get_level(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
) -> xkb_level_index_t {
    unsafe {
        if layout >= (*key).num_groups() {
            return XKB_LEVEL_INVALID as xkb_level_index_t;
        }
        let entry: *const xkb_key_type_entry =
            get_entry_for_key_state(state, key, layout) as *const xkb_key_type_entry;
        return if !entry.is_null() {
            (*entry).level
        } else {
            0 as xkb_level_index_t
        };
    }
}

pub unsafe fn xkb_state_key_get_level(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
) -> xkb_level_index_t {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        return if !key.is_null() {
            state_key_get_level(state, key, layout)
        } else {
            XKB_LEVEL_INVALID as xkb_level_index_t
        };
    }
}
#[inline]

unsafe fn state_key_get_layout(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
) -> xkb_layout_index_t {
    unsafe {
        return XkbWrapGroupIntoRange(
            (*state).components.group as i32,
            (*key).num_groups(),
            (*key).out_of_range_group_policy(),
            (*key).out_of_range_group_number(),
        );
    }
}

pub unsafe fn xkb_state_key_get_layout(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
) -> xkb_layout_index_t {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() {
            return XKB_LAYOUT_INVALID as xkb_layout_index_t;
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
) -> xkb_action_count_t {
    unsafe {
        let mut count: xkb_action_count_t = 0;
        let layout: xkb_layout_index_t = state_key_get_layout(state, key) as xkb_layout_index_t;
        let level: xkb_level_index_t = state_key_get_level(state, key, layout) as xkb_level_index_t;
        if !(level == XKB_LEVEL_INVALID as xkb_level_index_t) {
            count =
                xkb_keymap_key_get_actions_by_level((*state).keymap, key, layout, level, actions)
                    as xkb_action_count_t;
            if !(count == 0) {
                return count;
            }
        }
        *actions = &raw const dummy_action;
        return 1 as xkb_action_count_t;
    }
}

unsafe fn xkb_filter_new(mut state: *mut xkb_state) -> *mut xkb_filter {
    unsafe {
        let mut filter: *mut xkb_filter = std::ptr::null_mut();
        let mut iter: *mut xkb_filter = std::ptr::null_mut();
        if !(*state).filters.item.is_null() {
            iter = (*state).filters.item.offset(0 as i32 as isize) as *mut xkb_filter;
            while iter
                < (*state).filters.item.offset((*state).filters.size as isize) as *mut xkb_filter
            {
                if (*iter).func.is_some() {
                    iter = iter.offset(1);
                } else {
                    filter = iter;
                    break;
                }
            }
        }
        if filter.is_null() {
            darray_resize_zero(
                &mut (*state).filters.item,
                &mut (*state).filters.size,
                &mut (*state).filters.alloc,
                (*state).filters.size.wrapping_add(1 as darray_size_t),
            );
            filter = (*state)
                .filters
                .item
                .offset((*state).filters.size.wrapping_sub(1 as darray_size_t) as isize)
                as *mut xkb_filter;
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
        if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0 {
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
        if (*filter).action.group.flags as u32 & ACTION_LOCK_CLEAR as i32 as u32 != 0 {
            (*state).components.locked_group = 0 as i32 as i32;
        }
        (*filter).func = None;
        return XKB_FILTER_CONTINUE as i32 != 0;
    }
}

unsafe fn get_state_component_changes(
    mut a: *const state_components,
    mut b: *const state_components,
) -> xkb_state_component {
    unsafe {
        let mut mask: xkb_state_component = 0 as xkb_state_component;
        if (*a).group != (*b).group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_EFFECTIVE as i32 as u32) as xkb_state_component;
        }
        if (*a).base_group != (*b).base_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_DEPRESSED as i32 as u32) as xkb_state_component;
        }
        if (*a).latched_group != (*b).latched_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_LATCHED as i32 as u32) as xkb_state_component;
        }
        if (*a).locked_group != (*b).locked_group {
            mask = (mask as u32 | XKB_STATE_LAYOUT_LOCKED as i32 as u32) as xkb_state_component;
        }
        if (*a).mods != (*b).mods {
            mask = (mask as u32 | XKB_STATE_MODS_EFFECTIVE as i32 as u32) as xkb_state_component;
        }
        if (*a).base_mods != (*b).base_mods {
            mask = (mask as u32 | XKB_STATE_MODS_DEPRESSED as i32 as u32) as xkb_state_component;
        }
        if (*a).latched_mods != (*b).latched_mods {
            mask = (mask as u32 | XKB_STATE_MODS_LATCHED as i32 as u32) as xkb_state_component;
        }
        if (*a).locked_mods != (*b).locked_mods {
            mask = (mask as u32 | XKB_STATE_MODS_LOCKED as i32 as u32) as xkb_state_component;
        }
        if (*a).leds != (*b).leds {
            mask = (mask as u32 | XKB_STATE_LEDS as i32 as u32) as xkb_state_component;
        }
        if (*a).controls as u32 != (*b).controls as u32 {
            mask = (mask as u32 | XKB_STATE_CONTROLS as i32 as u32) as xkb_state_component;
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
        if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as i32 as u32 != 0 {
            return;
        } else if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0 {
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
            if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as i32 as u32 != 0
                && direction as u32 == XKB_KEY_DOWN as i32 as u32
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
        if (*filter).action.group.flags as u32 & ACTION_LOCK_ON_RELEASE as i32 as u32 != 0 {
            if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0 {
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
    mut mask: xkb_mod_mask_t,
) -> bool {
    unsafe {
        match (*action).type_0 as u32 {
            0 | 1 | 9 | 10 | 14 | 15 | 13 | 12 | 16 => return true,
            20 => {
                return (*action).internal.flags as u32 & flag as u32 != 0
                    && (*action).internal.c2rust_unnamed.clear_latched_mods & mask == mask;
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
            c2rust_unnamed: {
                let mut init = C2Rust_Unnamed_22 {
                    latch_group_delta: [0; 4],
                };
                init.set_latch(LATCH_KEY_DOWN as i32 as u32);
                init.set_group_delta(
                    if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32
                        != 0
                    {
                        (*filter).action.group.group - (*state).components.base_group
                    } else {
                        (*filter).action.group.group
                    },
                );
                init
            },
        };
        (*filter).priv_0 = priv_0.priv_0;
        if (*filter).action.group.flags as u32 & ACTION_ABSOLUTE_SWITCH as i32 as u32 != 0 {
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
        let mut latch: xkb_key_latch_state = priv_0.c2rust_unnamed.latch() as xkb_key_latch_state;
        if direction as u32 == XKB_KEY_DOWN as i32 as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: xkb_action_count_t =
                xkb_key_get_actions(state, key, &raw mut actions) as xkb_action_count_t;
            if latch as u32 == LATCH_KEY_DOWN as i32 as u32 {
                if (*state).flags as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as i32 as u32 != 0 {
                    let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
                    while (k as i32) < count as i32 {
                        if xkb_action_breaks_latch(
                            actions.offset(k as isize) as *const xkb_action,
                            INTERNAL_BREAKS_GROUP_LATCH,
                            0 as xkb_mod_mask_t,
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
            } else if latch as u32 == LATCH_PENDING as i32 as u32 {
                let sticky_keys: bool =
                    (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 != 0;
                let flags: xkb_action_flags = ((*filter).action.group.flags as u32
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: xkb_action_count_t = 0 as xkb_action_count_t;
                while (k_0 as i32) < count as i32 {
                    if (*actions.offset(k_0 as isize)).type_0 as u32
                        == ACTION_TYPE_GROUP_LATCH as i32 as u32
                        && (*actions.offset(k_0 as isize)).group.group
                            == (*filter).action.group.group
                        && (*actions.offset(k_0 as isize)).group.flags as u32
                            == (*filter).action.group.flags as u32
                        || (*actions.offset(k_0 as isize)).type_0 as u32
                            == ACTION_TYPE_GROUP_SET as i32 as u32
                            && sticky_keys as i32 != 0
                            && (*actions.offset(k_0 as isize)).group.flags as u32 == flags as u32
                    {
                        if (*filter).action.group.flags as u32 & ACTION_LATCH_TO_LOCK as i32 as u32
                            != 0
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
                            (*state).components.latched_group -=
                                priv_0.c2rust_unnamed.group_delta();
                            (*filter).key = key;
                            return XKB_FILTER_CONSUME as i32 != 0;
                        }
                    } else if xkb_action_breaks_latch(
                        actions.offset(k_0 as isize) as *const xkb_action,
                        INTERNAL_BREAKS_GROUP_LATCH,
                        0 as xkb_mod_mask_t,
                    ) {
                        (*state).components.latched_group -= priv_0.c2rust_unnamed.group_delta();
                        (*filter).func = None;
                        return XKB_FILTER_CONTINUE as i32 != 0;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            } else {
            }
        } else if !(key != (*filter).key) {
            if direction as u32 == XKB_KEY_REPEATED as i32 as u32 {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else if (*filter).action.group.flags as u32 & ACTION_LOCK_CLEAR as i32 as u32 != 0
                && (*state).components.locked_group != 0
            {
                if latch as u32 == LATCH_PENDING as i32 as u32 {
                    (*state).components.latched_group -= priv_0.c2rust_unnamed.group_delta();
                } else {
                    (*state).components.base_group -= priv_0.c2rust_unnamed.group_delta();
                }
                (*state).components.locked_group = 0 as i32 as i32;
                (*filter).func = None;
            } else if latch as u32 == NO_LATCH as i32 as u32 {
                (*state).components.base_group -= priv_0.c2rust_unnamed.group_delta();
                (*filter).func = None;
            } else if latch as u32 == LATCH_KEY_DOWN as i32 as u32 {
                latch = LATCH_PENDING;
                (*state).components.base_group -= priv_0.c2rust_unnamed.group_delta();
                (*state).components.latched_group += priv_0.c2rust_unnamed.group_delta();
            }
        }
        priv_0.c2rust_unnamed.set_latch(latch as u32 as u32);
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
        if (*filter).action.mods.flags as u32 & unlock as u32 == ACTION_LOCK_CLEAR as i32 as u32 {
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
            && (*filter).action.mods.flags as u32 & ACTION_UNLOCK_ON_PRESS as i32 as u32 != 0
        {
            if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_UNLOCK as i32 as u32 == 0 {
                (*state).components.locked_mods &= !(*filter).priv_0;
            }
            (*filter).func = None;
        } else {
            (*state).set_mods |= (*filter).action.mods.mods.mask;
            if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_LOCK as i32 as u32 == 0 {
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
        if (*filter).action.mods.flags as u32 & ACTION_LOCK_NO_UNLOCK as i32 as u32 == 0 {
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
        if (*filter).action.mods.flags as u32 & ACTION_LOCK_CLEAR as i32 as u32 != 0
            && (*filter).action.mods.flags as u32 & unlockOnPress as u32 != 0
            && (*state).components.locked_mods & (*filter).action.mods.mods.mask
                == (*filter).action.mods.mods.mask
        {
            (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
            (*filter).func = None;
        } else if (*filter).action.mods.flags as u32 & ACTION_LATCH_ON_PRESS as i32 as u32 != 0 {
            (*filter).priv_0 = LATCH_PENDING as i32 as u32;
            (*state).components.latched_mods |= (*filter).action.mods.mods.mask;
        } else {
            (*filter).priv_0 = LATCH_KEY_DOWN as i32 as u32;
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
        if direction as u32 == XKB_KEY_DOWN as i32 as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: xkb_action_count_t =
                xkb_key_get_actions(state, key, &raw mut actions) as xkb_action_count_t;
            if latch as u32 == LATCH_KEY_DOWN as i32 as u32 {
                if (*state).flags as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as i32 as u32 != 0 {
                    let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
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
            } else if latch as u32 == LATCH_PENDING as i32 as u32 {
                let sticky_keys: bool =
                    (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 != 0;
                let flags: xkb_action_flags = ((*filter).action.mods.flags as u32
                    & !(ACTION_LATCH_TO_LOCK as i32) as u32)
                    as xkb_action_flags;
                let mut k_0: xkb_action_count_t = 0 as xkb_action_count_t;
                while (k_0 as i32) < count as i32 {
                    if ((*actions.offset(k_0 as isize)).type_0 as u32
                        == ACTION_TYPE_MOD_LATCH as i32 as u32
                        && (*actions.offset(k_0 as isize)).mods.flags as u32
                            == (*filter).action.mods.flags as u32
                        || (*actions.offset(k_0 as isize)).type_0 as u32
                            == ACTION_TYPE_MOD_SET as i32 as u32
                            && sticky_keys as i32 != 0
                            && (*actions.offset(k_0 as isize)).mods.flags as u32 == flags as u32)
                        && (*actions.offset(k_0 as isize)).mods.mods.mask
                            == (*filter).action.mods.mods.mask
                    {
                        if (*filter).action.mods.flags as u32 & ACTION_LATCH_TO_LOCK as i32 as u32
                            != 0
                        {
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
            if direction as u32 == XKB_KEY_REPEATED as i32 as u32 {
                return XKB_FILTER_CONSUME as i32 != 0;
            } else {
                let unlockOnPress: xkb_action_flags = (ACTION_UNLOCK_ON_PRESS as i32
                    | ACTION_LATCH_ON_PRESS as i32)
                    as xkb_action_flags;
                if (*filter).action.mods.flags as u32 & ACTION_LOCK_CLEAR as i32 as u32 != 0
                    && (*filter).action.mods.flags as u32 & unlockOnPress as u32 == 0
                    && (*state).components.locked_mods & (*filter).action.mods.mods.mask
                        == (*filter).action.mods.mods.mask
                {
                    if latch as u32 == LATCH_PENDING as i32 as u32 {
                        (*state).components.latched_mods &= !(*filter).action.mods.mods.mask;
                    } else {
                        (*state).clear_mods |= (*filter).action.mods.mods.mask;
                    }
                    (*state).components.locked_mods &= !(*filter).action.mods.mods.mask;
                    (*filter).func = None;
                } else if latch as u32 == NO_LATCH as i32 as u32 {
                    (*state).clear_mods |= (*filter).action.mods.mods.mask;
                    (*filter).func = None;
                } else if (*filter).action.mods.flags as u32 & ACTION_LATCH_ON_PRESS as i32 as u32
                    == 0
                {
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
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as i32 as u32 {
            (*filter).priv_0 = (!((*state).components.controls as u32)
                & (*filter).action.ctrls.ctrls as u32) as u32;
        } else {
            (*filter).priv_0 =
                ((*state).components.controls as u32 & (*filter).action.ctrls.ctrls as u32) as u32;
        }
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as i32 as u32
            || (*filter).action.ctrls.flags as u32 & ACTION_LOCK_NO_LOCK as i32 as u32 == 0
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
        if (*filter).action.type_0 as u32 == ACTION_TYPE_CTRL_SET as i32 as u32
            || (*filter).action.ctrls.flags as u32 & ACTION_LOCK_NO_UNLOCK as i32 as u32 == 0
        {
            let old: xkb_action_controls = (*state).components.controls;
            (*state).components.controls = ((*state).components.controls as u32
                & !((*filter).priv_0 as xkb_action_controls as u32))
                as xkb_action_controls;
            if old as u32 & CONTROL_STICKY_KEYS as i32 as u32 != 0
                && (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 == 0
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
        let mut changed: xkb_state_component = 0 as xkb_state_component;
        let mask: xkb_mod_mask_t = (*redirect).affect;
        let mut event: *mut xkb_event = std::ptr::null_mut();
        let mut last_components: state_components = (*state).components;
        if !(*events).queue.item.is_null() && (*events).queue.size != 0 {
            event = (*events)
                .queue
                .item
                .offset((*events).queue.size.wrapping_sub(1 as darray_size_t) as isize)
                as *mut xkb_event;
            while (*events).queue.size > 0 as darray_size_t
                && event >= (*events).queue.item.offset(0 as i32 as isize) as *mut xkb_event
            {
                if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
                    last_components = (*event).c2rust_unnamed.components.components;
                    break;
                } else {
                    event = event.offset(-1);
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
                darray_append(
                    &mut (*events).queue.item,
                    &mut (*events).queue.size,
                    &mut (*events).queue.alloc,
                    xkb_event {
                        type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                        c2rust_unnamed: C2Rust_Unnamed_17 {
                            components: C2Rust_Unnamed_18 {
                                components: new,
                                changed: changed,
                            },
                        },
                    },
                );
            }
        }
        darray_append(
            &mut (*events).queue.item,
            &mut (*events).queue.size,
            &mut (*events).queue.alloc,
            xkb_event {
                type_0: (if direction as u32 == XKB_KEY_UP as i32 as u32 {
                    XKB_EVENT_TYPE_KEY_UP as i32
                } else if direction as u32 == XKB_KEY_REPEATED as i32 as u32 {
                    XKB_EVENT_TYPE_KEY_REPEATED as i32
                } else {
                    XKB_EVENT_TYPE_KEY_DOWN as i32
                }) as xkb_event_type,
                c2rust_unnamed: C2Rust_Unnamed_17 {
                    keycode: (*redirect).keycode,
                },
            },
        );
        if mask != 0 && changed as u32 != 0 {
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        components: C2Rust_Unnamed_18 {
                            components: last_components,
                            changed: changed,
                        },
                    },
                },
            );
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
        if events.is_null()
            || (*filter).action.redirect.keycode == XKB_KEYCODE_INVALID as xkb_keycode_t
        {
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
        if direction as u32 == XKB_KEY_UP as i32 as u32 {
            append_redirect_key_events(
                state,
                events,
                &raw mut (*filter).action.redirect,
                XKB_KEY_UP,
            );
            (*filter).func = None;
            return XKB_FILTER_CONSUME as i32 != 0;
        } else if direction as u32 == XKB_KEY_DOWN as i32 as u32 {
            let mut actions: *const xkb_action = std::ptr::null();
            let count: xkb_action_count_t =
                xkb_key_get_actions(state, key, &raw mut actions) as xkb_action_count_t;
            let mut a: xkb_action_count_t = 0 as xkb_action_count_t;
            while (a as i32) < count as i32 {
                if (*actions.offset(a as isize)).type_0 as u32
                    == ACTION_TYPE_REDIRECT_KEY as i32 as u32
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

static mut filter_action_funcs: [C2Rust_Unnamed_21; 21] = {
    [
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
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
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
            new: None,
            func: None,
        },
        C2Rust_Unnamed_21 {
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
        if !(*state).filters.item.is_null() {
            filter = (*state).filters.item.offset(0 as i32 as isize) as *mut xkb_filter;
            while filter
                < (*state).filters.item.offset((*state).filters.size as isize) as *mut xkb_filter
            {
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
        if consumed as i32 != 0 || direction as u32 == XKB_KEY_UP as i32 as u32 {
            return;
        }
        let mut actions: *const xkb_action = std::ptr::null();
        let count: xkb_action_count_t =
            xkb_key_get_actions(state, key, &raw mut actions) as xkb_action_count_t;
        let mut k: xkb_action_count_t = 0 as xkb_action_count_t;
        while (k as i32) < count as i32 {
            if !((*actions.offset(k as isize)).type_0 as u32
                >= _ACTION_TYPE_NUM_ENTRIES as i32 as u32)
            {
                if !filter_action_funcs[(*actions.offset(k as isize)).type_0 as usize]
                    .new
                    .is_none()
                {
                    filter = xkb_filter_new(state);
                    (*filter).key = key;
                    (*filter).action = *actions.offset(k as isize);
                    if (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 != 0
                    {
                        if (*filter).action.type_0 as u32 == ACTION_TYPE_MOD_SET as i32 as u32 {
                            (*filter).action.type_0 = ACTION_TYPE_MOD_LATCH;
                            if (*state).flags as u32 & XKB_A11Y_LATCH_TO_LOCK as i32 as u32 != 0 {
                                (*filter).action.mods.flags = ((*filter).action.mods.flags as u32
                                    | ACTION_LATCH_TO_LOCK as i32 as u32)
                                    as xkb_action_flags;
                            }
                        } else if (*filter).action.type_0 as u32
                            == ACTION_TYPE_GROUP_SET as i32 as u32
                        {
                            (*filter).action.type_0 = ACTION_TYPE_GROUP_LATCH;
                            if (*state).flags as u32 & XKB_A11Y_LATCH_TO_LOCK as i32 as u32 != 0 {
                                (*filter).action.group.flags = ((*filter).action.group.flags as u32
                                    | ACTION_LATCH_TO_LOCK as i32 as u32)
                                    as xkb_action_flags;
                            }
                        }
                    }
                    if (*filter).action.type_0 as u32 == ACTION_TYPE_REDIRECT_KEY as i32 as u32 {
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
#[inline]

unsafe fn xkb_state_init(
    mut state: *mut xkb_state,
    mut keymap: *mut xkb_keymap,
    mut a11y_affect: xkb_a11y_flags,
    mut a11y_flags: xkb_a11y_flags,
) {
    unsafe {
        (*state).flags = a11y_flags;
        if (*keymap).format as u32 != XKB_KEYMAP_FORMAT_TEXT_V1 as i32 as u32
            && a11y_affect as u32 & XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as i32 as u32 == 0
        {
            (*state).flags = ((*state).flags as u32
                | XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as i32 as u32)
                as xkb_a11y_flags;
        }
        (*state).controls.out_of_range_group.policy = XKB_LAYOUT_OUT_OF_RANGE_WRAP;
        (*state).refcnt = 1 as i32;
        (*state).keymap = xkb_keymap_ref(keymap);
        xkb_state_update_derived(state);
    }
}
pub unsafe fn xkb_state_new(mut keymap: *mut xkb_keymap) -> *mut xkb_state {
    unsafe {
        let state: *mut xkb_state = Box::into_raw(Box::new(std::mem::zeroed::<xkb_state>()));
        xkb_state_init(state, keymap, XKB_A11Y_NO_FLAGS, XKB_A11Y_NO_FLAGS);
        return state;
    }
}

pub unsafe fn xkb_state_ref(mut state: *mut xkb_state) -> *mut xkb_state {
    unsafe {
        (*state).refcnt += 1;
        return state;
    }
}
#[inline]

unsafe fn xkb_state_destroy(mut state: *mut xkb_state) {
    unsafe {
        xkb_keymap_unref((*state).keymap);
        darray_free(
            &mut (*state).filters.item,
            &mut (*state).filters.size,
            &mut (*state).filters.alloc,
        );
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
        xkb_state_destroy(state);
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
        let mut idx: xkb_led_index_t = 0;
        let mut led: *const xkb_led = std::ptr::null();
        (*state).components.leds = 0 as xkb_led_mask_t;
        let mut c2rust_current_block_23: u64;
        idx = 0 as xkb_led_index_t;
        led = &raw mut (*(*state).keymap).leds as *mut xkb_led;
        while idx < (*(*state).keymap).num_leds {
            if (*led).which_mods as u32 != 0 as u32 && (*led).mods.mask != 0 as xkb_mod_mask_t {
                let mut mod_mask: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
                if (*led).which_mods as u32 & XKB_STATE_MODS_EFFECTIVE as i32 as u32 != 0 {
                    mod_mask |= (*state).components.mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_DEPRESSED as i32 as u32 != 0 {
                    mod_mask |= (*state).components.base_mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_LATCHED as i32 as u32 != 0 {
                    mod_mask |= (*state).components.latched_mods;
                }
                if (*led).which_mods as u32 & XKB_STATE_MODS_LOCKED as i32 as u32 != 0 {
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
                    if (*led).which_groups() as i32 != 0 as i32 {
                        if ((*led).groups != 0) as i32 as i64 != 0 as i64 {
                            let mut group_mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
                            if (*led).which_groups() as i32 & XKB_STATE_LAYOUT_EFFECTIVE as i32 != 0
                            {
                                group_mask = (group_mask as u32
                                    | (1 as u32) << (*state).components.group)
                                    as xkb_layout_mask_t;
                            }
                            if (*led).which_groups() as i32 & XKB_STATE_LAYOUT_LOCKED as i32 != 0 {
                                group_mask = (group_mask as u32
                                    | (1 as u32) << (*state).components.locked_group)
                                    as xkb_layout_mask_t;
                            }
                            if (*led).which_groups() as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32 != 0
                                && (*state).components.base_group != 0 as i32
                            {
                                group_mask |= (*led).groups;
                            }
                            if (*led).which_groups() as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
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
                        } else if (*led).which_groups() as i32 & XKB_STATE_LAYOUT_DEPRESSED as i32
                            != 0
                            && (*state).components.base_group == 0 as i32
                            || (*led).which_groups() as i32 & XKB_STATE_LAYOUT_LATCHED as i32 != 0
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
        let mut wrapped: xkb_layout_index_t = 0;
        (*state).components.mods = (*state).components.base_mods
            | (*state).components.latched_mods
            | (*state).components.locked_mods;
        wrapped = XkbWrapGroupIntoRange(
            (*state).components.locked_group,
            (*(*state).keymap).num_groups,
            (*state).controls.out_of_range_group.policy,
            (*state).controls.out_of_range_group.redirect_group,
        );
        (*state).components.locked_group = (if wrapped == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            0 as xkb_layout_index_t
        } else {
            wrapped
        }) as i32;
        wrapped = XkbWrapGroupIntoRange(
            (*state).components.base_group
                + (*state).components.latched_group
                + (*state).components.locked_group,
            (*(*state).keymap).num_groups,
            (*state).controls.out_of_range_group.policy,
            (*state).controls.out_of_range_group.redirect_group,
        );
        (*state).components.group = if wrapped == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            0 as xkb_layout_index_t
        } else {
            wrapped
        };
        xkb_state_led_update_all(state);
    }
}
pub unsafe fn xkb_state_update_key(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut direction: xkb_key_direction,
) -> xkb_state_component {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() || direction as u32 == XKB_KEY_REPEATED as i32 as u32 && !(*key).repeats()
        {
            return 0 as xkb_state_component;
        }
        let prev_components: state_components = (*state).components;
        (*state).set_mods = 0 as xkb_mod_mask_t;
        (*state).clear_mods = 0 as xkb_mod_mask_t;
        xkb_filter_apply_all(state, std::ptr::null_mut(), key, direction);
        let mut i: xkb_mod_index_t = 0;
        let mut bit: xkb_mod_mask_t = 0;
        i = 0 as xkb_mod_index_t;
        bit = 1 as xkb_mod_mask_t;
        while (*state).set_mods != 0 {
            if (*state).set_mods & bit != 0 {
                (*state).mod_key_count[i as usize] += 1;
                (*state).components.base_mods |= bit;
                (*state).set_mods &= !bit;
            }
            i = i.wrapping_add(1);
            bit <<= 1 as i32;
        }
        i = 0 as xkb_mod_index_t;
        bit = 1 as xkb_mod_mask_t;
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
    level: 0 as xkb_level_index_t,
    mods: xkb_mods { mods: 0, mask: 0 },
    preserve: xkb_mods { mods: 0, mask: 0 },
};

static mut synthetic_key_type: xkb_key_type = {
    xkb_key_type {
        name: 0,
        mods: xkb_mods { mods: 0, mask: 0 },
        required: false,
        num_levels: 1 as xkb_level_index_t,
        num_level_names: 0,
        level_names: std::ptr::null_mut(),
        num_entries: 1 as darray_size_t,
        entries: &raw const synthetic_key_level_entry as *mut xkb_key_type_entry,
    }
};

static mut synthetic_key: xkb_key = xkb_key {
    keycode: 0,
    name: 0,
    explicit: 0 as xkb_explicit_components,
    modmap: 0,
    vmodmap: 0,
    overlays: 0,
    overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups: [0; 3],
    groups: std::ptr::null_mut(),
    c2rust_unnamed: C2Rust_Unnamed_9 {
        overlay_key: std::ptr::null(),
    },
};

unsafe fn update_latch_modifiers(
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut mask: xkb_mod_mask_t,
    mut latches: xkb_mod_mask_t,
) {
    unsafe {
        let clear: xkb_mod_mask_t = mask & !latches;
        (*state).components.latched_mods &= !clear;
        let mut synthetic_key_level_break_mod_latch: xkb_level = xkb_level {
            num_syms: 0 as xkb_keysym_count_t,
            num_actions: 1 as xkb_action_count_t,
            c2rust_unnamed: C2Rust_Unnamed_12 {
                upper: XKB_KEY_NoSymbol as xkb_keysym_t,
            },
            s: C2Rust_Unnamed_11 {
                sym: XKB_KEY_NoSymbol as xkb_keysym_t,
            },
            a: C2Rust_Unnamed_10 {
                action: xkb_action {
                    internal: xkb_internal_action {
                        type_0: ACTION_TYPE_INTERNAL,
                        flags: INTERNAL_BREAKS_MOD_LATCH,
                        c2rust_unnamed: C2Rust_Unnamed_2 {
                            clear_latched_mods: clear,
                        },
                    },
                },
            },
        };
        let mut synthetic_key_group_break_mod_latch: xkb_group = {
            let mut init = xkb_group {
                explicit_symbols_explicit_actions_implicit_actions_explicit_type: [0; 1],
                c2rust_padding: [0; 7],
                type_0: &raw mut synthetic_key_type,
                levels: &raw mut synthetic_key_level_break_mod_latch,
            };
            init.set_explicit_symbols(false);
            init.set_explicit_actions(false);
            init.set_implicit_actions(false);
            init.set_explicit_type(false);
            init
        };
        let synthetic_key_break_mod_latch: xkb_key = {
            let mut init = xkb_key {
                overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups: [0; 3],
                keycode: 0,
                name: 0,
                explicit: 0 as xkb_explicit_components,
                modmap: 0,
                vmodmap: 0,
                overlays: 0,
                groups: &raw mut synthetic_key_group_break_mod_latch,
                c2rust_unnamed: C2Rust_Unnamed_9 {
                    overlay_key: std::ptr::null(),
                },
            };
            init.set_overlays_inline(false);
            init.set_repeats(false);
            init.set_implicit_actions(false);
            init.set_out_of_range_pending_group(false);
            init.set_out_of_range_group_policy(XKB_LAYOUT_OUT_OF_RANGE_WRAP);
            init.set_out_of_range_group_number(0);
            init.set_num_groups(1 as xkb_layout_index_t);
            init
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

unsafe fn resolve_to_canonical_mods(
    mut keymap: *mut xkb_keymap,
    mut mods: xkb_mod_mask_t,
) -> xkb_mod_mask_t {
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
        let affect_locked_mods: xkb_mod_mask_t =
            resolve_to_canonical_mods((*state).keymap, (*update).affect_locked_mods)
                as xkb_mod_mask_t;
        if affect_locked_mods != 0 {
            let locked_mods: xkb_mod_mask_t =
                resolve_to_canonical_mods((*state).keymap, (*update).locked_mods) as xkb_mod_mask_t;
            (*state).components.locked_mods &= !affect_locked_mods;
            (*state).components.locked_mods |= locked_mods & affect_locked_mods;
        }
        if (*update).components as u32 & XKB_STATE_LAYOUT_LOCKED as i32 as u32 != 0 {
            (*state).components.locked_group = (*update).locked_layout;
        }
        let affect_latched_mods: xkb_mod_mask_t =
            resolve_to_canonical_mods((*state).keymap, (*update).affect_latched_mods)
                as xkb_mod_mask_t;
        if affect_latched_mods != 0 {
            let latched_mods: xkb_mod_mask_t =
                resolve_to_canonical_mods((*state).keymap, (*update).latched_mods)
                    as xkb_mod_mask_t;
            update_latch_modifiers(state, events, affect_latched_mods, latched_mods);
        }
        if (*update).components as u32 & XKB_STATE_LAYOUT_LATCHED as i32 as u32 != 0 {
            update_latch_group(state, events, (*update).latched_layout);
        }
    }
}

pub unsafe fn xkb_state_update_latched_locked(
    mut state: *mut xkb_state,
    mut affect_latched_mods: xkb_mod_mask_t,
    mut latched_mods: xkb_mod_mask_t,
    mut affect_latched_layout: bool,
    mut latched_layout: i32,
    mut affect_locked_mods: xkb_mod_mask_t,
    mut locked_mods: xkb_mod_mask_t,
    mut affect_locked_layout: bool,
    mut locked_layout: i32,
) -> xkb_state_component {
    unsafe {
        let previous_components: state_components = (*state).components;
        let components: xkb_state_component = ((if affect_latched_mods != 0 || latched_mods != 0 {
            XKB_STATE_MODS_LATCHED as i32
        } else {
            0 as i32
        }) | (if affect_locked_mods != 0 || locked_mods != 0
        {
            XKB_STATE_MODS_LOCKED as i32
        } else {
            0 as i32
        }) | (if affect_latched_layout as i32 != 0 {
            XKB_STATE_LAYOUT_LATCHED as i32
        } else {
            0 as i32
        }) | (if affect_locked_layout as i32 != 0 {
            XKB_STATE_LAYOUT_LOCKED as i32
        } else {
            0 as i32
        })) as xkb_state_component;
        let update: xkb_state_components_update = xkb_state_components_update {
            size: ::core::mem::size_of::<xkb_state_components_update>() as usize,
            components: components,
            affect_latched_mods: affect_latched_mods,
            latched_mods: latched_mods,
            affect_locked_mods: affect_locked_mods,
            locked_mods: locked_mods,
            latched_layout: latched_layout,
            locked_layout: locked_layout,
            affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
            controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
        };
        state_update_latched_locked(
            state,
            &raw const update,
            std::ptr::null_mut(),
        );
        xkb_state_update_derived(state);
        return get_state_component_changes(
            &raw const previous_components,
            &raw mut (*state).components,
        );
    }
}
#[inline]

unsafe fn clear_all_latches_and_locks(mut state: *mut xkb_state, mut events: *mut xkb_events) {
    unsafe {
        static mut components: xkb_state_component = (XKB_STATE_MODS_LATCHED as i32
            | XKB_STATE_MODS_LOCKED as i32
            | XKB_STATE_LAYOUT_LATCHED as i32
            | XKB_STATE_LAYOUT_LOCKED as i32)
            as xkb_state_component;
        let update: xkb_state_components_update = xkb_state_components_update {
            size: ::core::mem::size_of::<xkb_state_components_update>() as usize,
            components: components,
            affect_latched_mods: XKB_MOD_ALL as u32 as xkb_mod_mask_t,
            latched_mods: 0 as xkb_mod_mask_t,
            affect_locked_mods: XKB_MOD_ALL as u32 as xkb_mod_mask_t,
            locked_mods: 0 as xkb_mod_mask_t,
            latched_layout: 0 as i32,
            locked_layout: 0 as i32,
            affect_controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
            controls: XKB_KEYBOARD_CONTROL_NO_FLAGS,
        };
        state_update_latched_locked(state, &raw const update, events);
    }
}

unsafe fn state_update_enabled_controls(
    mut state: *mut xkb_state,
    mut affect: xkb_keyboard_control_flags,
    mut controls: xkb_keyboard_control_flags,
    mut events: *mut xkb_events,
) {
    unsafe {
        let had_sticky_keys: bool =
            (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 != 0;
        affect = (affect as xkb_action_controls as u32 & CONTROL_ALL_BOOLEAN as i32 as u32)
            as xkb_keyboard_control_flags;
        (*state).components.controls =
            ((*state).components.controls as u32 & !(affect as u32)) as xkb_action_controls;
        (*state).components.controls = ((*state).components.controls as u32
            | controls as xkb_action_controls as u32 & affect as u32)
            as xkb_action_controls;
        if had_sticky_keys as i32 != 0
            && (*state).components.controls as u32 & CONTROL_STICKY_KEYS as i32 as u32 == 0
        {
            clear_all_latches_and_locks(state, events);
        }
        xkb_state_update_derived(state);
    }
}

unsafe fn state_update_layout_policy(
    mut state: *mut xkb_state,
    mut update: *const xkb_layout_policy_update,
) -> xkb_error_code {
    unsafe {
        if xkb_feature_supported(
            XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY,
            (*update).policy as i32 as u32,
        ) {
            if (*update).policy as u32 == XKB_LAYOUT_OUT_OF_RANGE_REDIRECT as i32 as u32 {
                if (*update).redirect < (*(*state).keymap).num_groups {
                    (*state).controls.out_of_range_group.redirect_group = (*update).redirect;
                } else {
                    xkb_logf!(
                        (*(*state).keymap).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Layout policy: unsupported layout index {} > {}\n",
                        XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                        (*update).redirect.wrapping_add(1 as xkb_layout_index_t),
                        (*(*state).keymap).num_groups,
                    );
                    return XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX;
                }
            }
            (*state).controls.out_of_range_group.policy = (*update).policy;
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
    unsafe {
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
}

unsafe fn check_state_update_abi_(
    mut ctx: *mut xkb_context,
    mut func: *const i8,
    mut update: *const xkb_state_update,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code = XKB_SUCCESS;
        error = xkb_check_versioned_struct_size_(
            ::core::mem::size_of::<xkb_state_update_v1>() as usize,
            ::core::mem::size_of::<xkb_state_update_v1>() as usize,
            ::core::mem::size_of::<xkb_state_update>() as usize,
            (*update).size,
            update as *const ::core::ffi::c_void,
        );
        if error as i32 != 0
            || !(*update).components.is_null() && {
                error = xkb_check_versioned_struct_size_(
                    ::core::mem::size_of::<xkb_state_update_v1>() as usize,
                    ::core::mem::size_of::<xkb_state_update_v1>() as usize,
                    ::core::mem::size_of::<xkb_state_components_update>() as usize,
                    (*(*update).components).size,
                    (*update).components as *const ::core::ffi::c_void,
                );
                error as i32 != 0
            }
            || !(*update).layout_policy.is_null() && {
                error = xkb_check_versioned_struct_size_(
                    ::core::mem::size_of::<xkb_state_update_v1>() as usize,
                    ::core::mem::size_of::<xkb_state_update_v1>() as usize,
                    ::core::mem::size_of::<xkb_layout_policy_update>() as usize,
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
    mut changed: *mut xkb_state_component,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code = check_state_update_abi_(
            (*(*state).keymap).ctx,
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
            state_update_enabled_controls(
                state,
                (*components).affect_controls,
                (*components).controls,
                std::ptr::null_mut(),
            );
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
    mut base_mods: xkb_mod_mask_t,
    mut latched_mods: xkb_mod_mask_t,
    mut locked_mods: xkb_mod_mask_t,
    mut base_group: xkb_layout_index_t,
    mut latched_group: xkb_layout_index_t,
    mut locked_group: xkb_layout_index_t,
) -> xkb_state_component {
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

unsafe fn should_do_caps_transformation(mut state: *mut xkb_state, mut kc: xkb_keycode_t) -> bool {
    unsafe {
        return xkb_state_mod_index_is_active(
            state,
            XKB_MOD_INDEX_CAPS as i32 as xkb_mod_index_t,
            XKB_STATE_MODS_EFFECTIVE,
        ) > 0 as i32
            && xkb_state_mod_index_is_consumed(
                state,
                kc,
                XKB_MOD_INDEX_CAPS as i32 as xkb_mod_index_t,
            ) == 0 as i32;
    }
}

unsafe fn should_do_ctrl_transformation(mut state: *mut xkb_state, mut kc: xkb_keycode_t) -> bool {
    unsafe {
        return xkb_state_mod_index_is_active(
            state,
            XKB_MOD_INDEX_CTRL as i32 as xkb_mod_index_t,
            XKB_STATE_MODS_EFFECTIVE,
        ) > 0 as i32
            && xkb_state_mod_index_is_consumed(
                state,
                kc,
                XKB_MOD_INDEX_CTRL as i32 as xkb_mod_index_t,
            ) == 0 as i32;
    }
}
pub unsafe fn xkb_state_key_get_syms(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut syms_out: *mut *const xkb_keysym_t,
) -> i32 {
    unsafe {
        let mut level: xkb_level_index_t = 0;
        let mut key: *const xkb_key = std::ptr::null();
        let mut leveli: *const xkb_level = std::ptr::null();
        let mut num_syms: xkb_keysym_count_t = 0;
        let layout: xkb_layout_index_t = xkb_state_key_get_layout(state, kc) as xkb_layout_index_t;
        if !(layout == XKB_LAYOUT_INVALID as xkb_layout_index_t) {
            level = xkb_state_key_get_level(state, kc, layout) as xkb_level_index_t;
            if !(level == XKB_LEVEL_INVALID as xkb_level_index_t) {
                key = XkbKey((*state).keymap, kc) as *const xkb_key;
                if !key.is_null() {
                    leveli = xkb_keymap_key_get_level((*state).keymap, key, layout, level);
                    if !leveli.is_null() {
                        num_syms = (*leveli).num_syms;
                        if !(num_syms as i32 == 0 as i32) {
                            if should_do_caps_transformation(state, kc) {
                                if num_syms as i32 > 1 as i32 {
                                    *syms_out = if (*leveli).c2rust_unnamed.has_upper as i32 != 0 {
                                        (*leveli).s.syms.offset(num_syms as i32 as isize)
                                    } else {
                                        (*leveli).s.syms
                                    };
                                } else {
                                    *syms_out = &raw const (*leveli).c2rust_unnamed.upper;
                                }
                            } else {
                                *syms_out = if num_syms as i32 > 1 as i32 {
                                    (*leveli).s.syms as *const xkb_keysym_t
                                } else {
                                    &raw const (*leveli).s.sym
                                };
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

unsafe fn XkbToControl(mut ch: i8) -> i8 {
    let mut c: i8 = ch;
    if c as i32 >= '@' as i32 && (c as i32) < '\u{7f}' as i32 || c as i32 == ' ' as i32 {
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
    return c;
}

pub unsafe fn xkb_state_key_get_one_sym(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
) -> xkb_keysym_t {
    unsafe {
        let mut syms: *const xkb_keysym_t = std::ptr::null();
        let num_syms: i32 = xkb_state_key_get_syms(state, kc, &raw mut syms) as i32;
        if num_syms != 1 as i32 {
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        } else {
            return *syms.offset(0 as i32 as isize);
        };
    }
}

unsafe fn get_one_sym_for_string(mut state: *mut xkb_state, mut kc: xkb_keycode_t) -> xkb_keysym_t {
    unsafe {
        let layout: xkb_layout_index_t = xkb_state_key_get_layout(state, kc) as xkb_layout_index_t;
        let num_layouts: xkb_layout_index_t =
            xkb_keymap_num_layouts_for_key((*state).keymap, kc) as xkb_layout_index_t;
        let mut level: xkb_level_index_t = xkb_state_key_get_level(state, kc, layout);
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t
            || num_layouts == 0 as xkb_layout_index_t
            || level == XKB_LEVEL_INVALID as xkb_level_index_t
        {
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut syms: *const xkb_keysym_t = std::ptr::null();
        let mut nsyms: i32 =
            xkb_keymap_key_get_syms_by_level((*state).keymap, kc, layout, level, &raw mut syms);
        if nsyms != 1 as i32 {
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut sym: xkb_keysym_t = *syms.offset(0 as i32 as isize);
        if should_do_ctrl_transformation(state, kc) as i32 != 0 && sym > 127 as xkb_keysym_t {
            let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
            while i < num_layouts {
                level = xkb_state_key_get_level(state, kc, i);
                if !(level == XKB_LEVEL_INVALID as xkb_level_index_t) {
                    nsyms = xkb_keymap_key_get_syms_by_level(
                        (*state).keymap,
                        kc,
                        i,
                        level,
                        &raw mut syms,
                    );
                    if nsyms == 1 as i32 && *syms.offset(0 as i32 as isize) <= 127 as xkb_keysym_t {
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
    mut kc: xkb_keycode_t,
    mut buffer: *mut i8,
    mut size: usize,
) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nsyms: i32 = 0;
        let mut syms: *const xkb_keysym_t = std::ptr::null();
        let sym: xkb_keysym_t = get_one_sym_for_string(state, kc) as xkb_keysym_t;
        if sym != XKB_KEY_NoSymbol as xkb_keysym_t {
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
                ::core::mem::size_of::<[i8; 5]>() as usize,
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
                            *buffer.offset(0 as i32 as isize) =
                                XkbToControl(*buffer.offset(0 as i32 as isize));
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

pub unsafe fn xkb_state_key_get_utf32(mut state: *mut xkb_state, mut kc: xkb_keycode_t) -> u32 {
    unsafe {
        let sym: xkb_keysym_t = get_one_sym_for_string(state, kc) as xkb_keysym_t;
        let mut cp: u32 = xkb_keysym_to_utf32(sym);
        if cp <= 127 as u32 && should_do_ctrl_transformation(state, kc) as i32 != 0 {
            cp = XkbToControl(cp as i8) as u32;
        }
        return cp;
    }
}
#[inline]

unsafe fn serialize_mods(
    mut components: *const state_components,
    mut type_0: xkb_state_component,
) -> xkb_mod_mask_t {
    unsafe {
        let mut ret: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if type_0 as u32 & XKB_STATE_MODS_EFFECTIVE as i32 as u32 != 0 {
            return (*components).mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_DEPRESSED as i32 as u32 != 0 {
            ret |= (*components).base_mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_LATCHED as i32 as u32 != 0 {
            ret |= (*components).latched_mods;
        }
        if type_0 as u32 & XKB_STATE_MODS_LOCKED as i32 as u32 != 0 {
            ret |= (*components).locked_mods;
        }
        return ret;
    }
}
pub unsafe fn xkb_state_serialize_mods(
    mut state: *mut xkb_state,
    mut type_0: xkb_state_component,
) -> xkb_mod_mask_t {
    unsafe {
        return serialize_mods(&raw mut (*state).components, type_0);
    }
}
#[inline]

unsafe fn serialize_layout(
    mut components: *const state_components,
    mut type_0: xkb_state_component,
) -> xkb_layout_index_t {
    unsafe {
        let mut ret: xkb_layout_index_t = 0 as xkb_layout_index_t;
        if type_0 as u32 & XKB_STATE_LAYOUT_EFFECTIVE as i32 as u32 != 0 {
            return (*components).group;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_DEPRESSED as i32 as u32 != 0 {
            ret = ret.wrapping_add((*components).base_group as xkb_layout_index_t);
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LATCHED as i32 as u32 != 0 {
            ret = ret.wrapping_add((*components).latched_group as xkb_layout_index_t);
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LOCKED as i32 as u32 != 0 {
            ret = ret.wrapping_add((*components).locked_group as xkb_layout_index_t);
        }
        return ret;
    }
}
pub unsafe fn xkb_state_serialize_layout(
    mut state: *mut xkb_state,
    mut type_0: xkb_state_component,
) -> xkb_layout_index_t {
    unsafe {
        return serialize_layout(&raw mut (*state).components, type_0);
    }
}
#[inline]

unsafe fn serialize_controls(
    mut components: *const state_components,
    mut type_0: xkb_state_component,
) -> xkb_keyboard_control_flags {
    unsafe {
        return (if type_0 as u32 & XKB_STATE_CONTROLS as i32 as u32 != 0 {
            ((*components).controls as u32 & CONTROL_ALL_BOOLEAN as i32 as u32)
                as xkb_keyboard_control_flags as u32
        } else {
            0 as u32
        }) as xkb_keyboard_control_flags;
    }
}

pub unsafe fn xkb_state_serialize_enabled_controls(
    mut state: *const xkb_state,
    mut type_0: xkb_state_component,
) -> xkb_keyboard_control_flags {
    unsafe {
        return serialize_controls(&raw const (*state).components, type_0);
    }
}
pub unsafe fn mod_mask_get_effective(
    mut keymap: *mut xkb_keymap,
    mut mods: xkb_mod_mask_t,
) -> xkb_mod_mask_t {
    unsafe {
        let mut mask: xkb_mod_mask_t = mods & MOD_REAL_MASK_ALL;
        let mut mod_0: *const xkb_mod = std::ptr::null();
        let mut i: xkb_mod_index_t = 0;
        i = _XKB_MOD_INDEX_NUM_ENTRIES as i32 as xkb_mod_index_t;
        mod_0 = (&raw mut (*keymap).mods.mods as *mut xkb_mod)
            .offset(_XKB_MOD_INDEX_NUM_ENTRIES as i32 as isize) as *mut xkb_mod;
        while i < (*keymap).mods.num_mods {
            if mods & (1 as xkb_mod_mask_t) << i != 0 {
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
    mut idx: xkb_mod_index_t,
    mut type_0: xkb_state_component,
) -> i32 {
    unsafe {
        if (idx >= xkb_keymap_num_mods((*state).keymap)) as i32 as i64 != 0 {
            return -1 as i32;
        }
        let mapping: xkb_mod_mask_t = (*(*state).keymap).mods.mods[idx as usize].mapping;
        if mapping == 0 {
            return 0 as i32;
        }
        return (xkb_state_serialize_mods(state, type_0) & mapping == mapping) as i32;
    }
}

unsafe fn match_mod_masks(
    mut state: *mut xkb_state,
    mut type_0: xkb_state_component,
    mut match_0: xkb_state_match,
    mut wanted: xkb_mod_mask_t,
) -> bool {
    unsafe {
        let active: xkb_mod_mask_t = xkb_state_serialize_mods(state, type_0) as xkb_mod_mask_t;
        if match_0 as u32 & XKB_STATE_MATCH_NON_EXCLUSIVE as i32 as u32 == 0
            && active & !wanted != 0
        {
            return false;
        }
        if match_0 as u32 & XKB_STATE_MATCH_ANY as i32 as u32 != 0 {
            return active & wanted != 0;
        }
        return active & wanted == wanted;
    }
}

pub unsafe extern "C" fn xkb_state_mod_indices_are_active(
    mut state: *mut xkb_state,
    mut type_0: xkb_state_component,
    mut match_0: xkb_state_match,
    mut c2rust_args: ...
) -> i32 {
    unsafe {
        if match_0 as u32 & !(XKB_STATE_MATCH_FLAGS as i32 as xkb_state_match as u32) != 0 {
            return -2 as i32;
        }
        let mut ap: ::core::ffi::VaList;
        let mut wanted: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut ret: i32 = 0 as i32;
        let num_mods: xkb_mod_index_t = xkb_keymap_num_mods((*state).keymap) as xkb_mod_index_t;
        ap = c2rust_args.clone();
        loop {
            let mut idx: xkb_mod_index_t = ap.arg::<xkb_mod_index_t>();
            if idx == XKB_MOD_INVALID as xkb_mod_index_t {
                break;
            }
            if (idx >= num_mods) as i32 as i64 != 0 {
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
    mut type_0: xkb_state_component,
) -> i32 {
    unsafe {
        let idx: xkb_mod_index_t =
            xkb_keymap_mod_get_index((*state).keymap, name) as xkb_mod_index_t;
        if idx == XKB_MOD_INVALID as xkb_mod_index_t {
            return -1 as i32;
        }
        return xkb_state_mod_index_is_active(state, idx, type_0);
    }
}

pub unsafe extern "C" fn xkb_state_mod_names_are_active(
    mut state: *mut xkb_state,
    mut type_0: xkb_state_component,
    mut match_0: xkb_state_match,
    mut c2rust_args: ...
) -> i32 {
    unsafe {
        if match_0 as u32 & !(XKB_STATE_MATCH_FLAGS as i32 as xkb_state_match as u32) != 0 {
            return -2 as i32;
        }
        let mut ap: ::core::ffi::VaList;
        let mut wanted: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut ret: i32 = 0 as i32;
        ap = c2rust_args.clone();
        loop {
            let mut str: *const i8 = ap.arg::<*const i8>();
            if str.is_null() {
                break;
            }
            let idx: xkb_mod_index_t =
                xkb_keymap_mod_get_index((*state).keymap, str) as xkb_mod_index_t;
            if idx == XKB_MOD_INVALID as xkb_mod_index_t {
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
    mut idx: xkb_layout_index_t,
    mut type_0: xkb_state_component,
) -> i32 {
    unsafe {
        if idx >= (*(*state).keymap).num_groups {
            return -1 as i32;
        }
        let mut ret: i32 = 0 as i32;
        if type_0 as u32 & XKB_STATE_LAYOUT_EFFECTIVE as i32 as u32 != 0 {
            ret |= ((*state).components.group == idx) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_DEPRESSED as i32 as u32 != 0 {
            ret |= ((*state).components.base_group == idx as i32) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LATCHED as i32 as u32 != 0 {
            ret |= ((*state).components.latched_group == idx as i32) as i32;
        }
        if type_0 as u32 & XKB_STATE_LAYOUT_LOCKED as i32 as u32 != 0 {
            ret |= ((*state).components.locked_group == idx as i32) as i32;
        }
        return ret;
    }
}

pub unsafe fn xkb_state_layout_name_is_active(
    mut state: *mut xkb_state,
    mut name: *const i8,
    mut type_0: xkb_state_component,
) -> i32 {
    unsafe {
        let idx: xkb_layout_index_t =
            xkb_keymap_layout_get_index((*state).keymap, name) as xkb_layout_index_t;
        if idx == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return -1 as i32;
        }
        return xkb_state_layout_index_is_active(state, idx, type_0);
    }
}
pub unsafe fn xkb_state_led_index_is_active(
    mut state: *mut xkb_state,
    mut idx: xkb_led_index_t,
) -> i32 {
    unsafe {
        if idx >= (*(*state).keymap).num_leds
            || (*(*state).keymap).leds[idx as usize].name == XKB_ATOM_NONE as xkb_atom_t
        {
            return -1 as i32;
        }
        return ((*state).components.leds & (1 as xkb_led_mask_t) << idx != 0) as i32;
    }
}

pub unsafe fn xkb_state_led_name_is_active(mut state: *mut xkb_state, mut name: *const i8) -> i32 {
    unsafe {
        let idx: xkb_led_index_t =
            xkb_keymap_led_get_index((*state).keymap, name) as xkb_led_index_t;
        if idx == XKB_LED_INVALID as xkb_led_index_t {
            return -1 as i32;
        }
        return xkb_state_led_index_is_active(state, idx);
    }
}

unsafe fn key_get_consumed(
    mut state: *mut xkb_state,
    mut key: *const xkb_key,
    mut mode: xkb_consumed_mode,
) -> xkb_mod_mask_t {
    unsafe {
        let group: xkb_layout_index_t =
            xkb_state_key_get_layout(state, (*key).keycode) as xkb_layout_index_t;
        if group == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return 0 as xkb_mod_mask_t;
        }
        let mut preserve: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut consumed: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let matching_entry: *const xkb_key_type_entry =
            get_entry_for_key_state(state, key, group) as *const xkb_key_type_entry;
        if !matching_entry.is_null() {
            preserve = (*matching_entry).preserve.mask;
        }
        let type_0: *const xkb_key_type = (*(*key).groups.offset(group as isize)).type_0;
        match mode as u32 {
            0 => {
                consumed = (*type_0).mods.mask;
            }
            1 => {
                let no_mods_entry: *const xkb_key_type_entry =
                    get_entry_for_mods(type_0, 0 as xkb_mod_mask_t) as *const xkb_key_type_entry;
                let no_mods_leveli: xkb_level_index_t = if !no_mods_entry.is_null() {
                    (*no_mods_entry).level
                } else {
                    0 as xkb_level_index_t
                };
                let no_mods_level: *const xkb_level = (*(*key).groups.offset(group as isize))
                    .levels
                    .offset(no_mods_leveli as isize)
                    as *mut xkb_level;
                let mut i: darray_size_t = 0 as darray_size_t;
                while i < (*type_0).num_entries {
                    let entry: *const xkb_key_type_entry =
                        (*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry;
                    if entry_is_active(entry) {
                        let level: *const xkb_level = (*(*key).groups.offset(group as isize))
                            .levels
                            .offset((*entry).level as isize)
                            as *mut xkb_level;
                        if !XkbLevelsSameSyms(level, no_mods_level) {
                            if entry == matching_entry
                                || one_bit_set((*entry).mods.mask as u32) != 0
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
    mut kc: xkb_keycode_t,
    mut idx: xkb_mod_index_t,
    mut mode: xkb_consumed_mode,
) -> i32 {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if (key.is_null() || idx >= xkb_keymap_num_mods((*state).keymap)) as i32 as i64 != 0 {
            return -1 as i32;
        }
        let mapping: xkb_mod_mask_t = (*(*state).keymap).mods.mods[idx as usize].mapping;
        if mapping == 0 {
            return 0 as i32;
        }
        return (mapping & key_get_consumed(state, key, mode) == mapping) as i32;
    }
}

pub unsafe fn xkb_state_mod_index_is_consumed(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut idx: xkb_mod_index_t,
) -> i32 {
    unsafe {
        return xkb_state_mod_index_is_consumed2(state, kc, idx, XKB_CONSUMED_MODE_XKB);
    }
}

pub unsafe fn xkb_state_mod_mask_remove_consumed(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut mask: xkb_mod_mask_t,
) -> xkb_mod_mask_t {
    unsafe {
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() {
            return 0 as xkb_mod_mask_t;
        }
        return resolve_to_canonical_mods((*state).keymap, mask)
            & !key_get_consumed(state, key, XKB_CONSUMED_MODE_XKB);
    }
}

pub unsafe fn xkb_state_key_get_consumed_mods2(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
    mut mode: xkb_consumed_mode,
) -> xkb_mod_mask_t {
    unsafe {
        match mode as u32 {
            0 | 1 => {}
            _ => {
                xkb_logf!(
                    (*(*state).keymap).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "{}: unrecognized consumed modifiers mode: {}\n",
                    crate::xkb::utils::CStrDisplay(
                        b"xkb_state_key_get_consumed_mods2\0".as_ptr() as *const i8
                    ),
                    mode as u32,
                );
                return 0 as xkb_mod_mask_t;
            }
        }
        let key: *const xkb_key = XkbKey((*state).keymap, kc) as *const xkb_key;
        if key.is_null() {
            return 0 as xkb_mod_mask_t;
        }
        return key_get_consumed(state, key, mode);
    }
}

pub unsafe fn xkb_state_key_get_consumed_mods(
    mut state: *mut xkb_state,
    mut kc: xkb_keycode_t,
) -> xkb_mod_mask_t {
    unsafe {
        return xkb_state_key_get_consumed_mods2(state, kc, XKB_CONSUMED_MODE_XKB);
    }
}

static mut default_machine_options: xkb_machine_options = xkb_machine_options {
    a11y_affect: XKB_A11Y_NO_FLAGS,
    a11y_flags: XKB_A11Y_NO_FLAGS,
    mods: machine_mods_mappings {
        size: 0,
        alloc: 0,
        item: std::ptr::null_mut(),
    },
    shortcuts: xkb_shortcuts_config_options {
        mask: 0 as xkb_mod_mask_t,
        targets: C2Rust_Unnamed_20 {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: std::ptr::null_mut(),
        },
    },
    ctx: std::ptr::null_mut(),
};

pub unsafe fn xkb_machine_options_new(mut context: *mut xkb_context) -> *mut xkb_machine_options {
    unsafe {
        let opt: *mut xkb_machine_options =
            Box::into_raw(Box::new(std::mem::zeroed::<xkb_machine_options>()));
        *opt = xkb_machine_options {
            a11y_affect: XKB_A11Y_NO_FLAGS,
            a11y_flags: XKB_A11Y_NO_FLAGS,
            mods: machine_mods_mappings {
                size: 0,
                alloc: 0,
                item: std::ptr::null_mut(),
            },
            shortcuts: xkb_shortcuts_config_options {
                mask: 0 as xkb_mod_mask_t,
                targets: C2Rust_Unnamed_20 {
                    size: 0 as darray_size_t,
                    alloc: 0 as darray_size_t,
                    item: std::ptr::null_mut(),
                },
            },
            ctx: xkb_context_ref(context),
        };
        return opt;
    }
}

pub unsafe fn xkb_machine_options_destroy(mut options: *mut xkb_machine_options) {
    unsafe {
        if options.is_null() {
            return;
        }
        darray_free(
            &mut (*options).shortcuts.targets.item,
            &mut (*options).shortcuts.targets.size,
            &mut (*options).shortcuts.targets.alloc,
        );
        darray_free(
            &mut (*options).mods.item,
            &mut (*options).mods.size,
            &mut (*options).mods.alloc,
        );
        xkb_context_unref((*options).ctx);
        drop(Box::from_raw(options));
    }
}

pub unsafe fn xkb_machine_options_update_a11y_flags(
    mut options: *mut xkb_machine_options,
    mut affect: xkb_a11y_flags,
    mut flags: xkb_a11y_flags,
) -> xkb_error_code {
    unsafe {
        static mut XKB_A11Y_FLAGS: xkb_a11y_flags = (XKB_A11Y_LATCH_TO_LOCK as i32
            | XKB_A11Y_LATCH_SIMULTANEOUS_KEYS as i32)
            as xkb_a11y_flags;
        if affect as u32 & !(XKB_A11Y_FLAGS as u32) != 0 {
            xkb_logf!(
                (*options).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unrecognized state flags: {:#x}\n",
                crate::xkb::utils::CStrDisplay(
                    b"xkb_machine_options_update_a11y_flags\0".as_ptr() as *const i8
                ),
                flags as u32 & !(XKB_A11Y_FLAGS as u32),
            );
            return XKB_ERROR_UNSUPPORTED_A11Y_FLAGS;
        }
        (*options).a11y_affect = ((*options).a11y_affect as u32 | affect as u32) as xkb_a11y_flags;
        (*options).a11y_flags = ((*options).a11y_flags as u32 & !(affect as u32)) as xkb_a11y_flags;
        (*options).a11y_flags =
            ((*options).a11y_flags as u32 | flags as u32 & affect as u32) as xkb_a11y_flags;
        return XKB_SUCCESS;
    }
}

pub unsafe fn xkb_machine_options_remap_mods(
    mut options: *mut xkb_machine_options,
    mut source: xkb_mod_mask_t,
    mut target: xkb_mod_mask_t,
) -> xkb_error_code {
    unsafe {
        if source == 0 {
            if target == 0 {
                (*options).mods.size = 0 as darray_size_t;
                return XKB_SUCCESS;
            } else {
                return XKB_ERROR_UNSUPPORTED_MODIFIER_MASK;
            }
        }
        let mut mapping: *mut machine_mods_mapping =
            std::ptr::null_mut();
        let mut m: darray_size_t = 0 as darray_size_t;
        if !(*options).mods.item.is_null() {
            m = 0 as darray_size_t;
            mapping = (*options).mods.item.offset(0 as i32 as isize) as *mut machine_mods_mapping;
            while m < (*options).mods.size {
                if (*mapping).source == source {
                    if target == 0 {
                        let mut __index: darray_size_t = m;
                        if __index < (*options).mods.size.wrapping_sub(1 as darray_size_t) {
                            std::ptr::copy(
                                (*options)
                                    .mods
                                    .item
                                    .offset(__index.wrapping_add(1 as darray_size_t) as isize),
                                (*options).mods.item.offset(__index as isize),
                                (*options)
                                    .mods
                                    .size
                                    .wrapping_sub(1 as darray_size_t)
                                    .wrapping_sub(__index) as usize,
                            );
                        }
                        (*options).mods.size = (*options).mods.size.wrapping_sub(1);
                    } else {
                        (*mapping).target = target;
                    }
                    return XKB_SUCCESS;
                }
                m = m.wrapping_add(1);
                mapping = mapping.offset(1);
            }
        }
        if target != 0 {
            darray_append(
                &mut (*options).mods.item,
                &mut (*options).mods.size,
                &mut (*options).mods.alloc,
                machine_mods_mapping {
                    source: source,
                    target: target,
                },
            );
        }
        return XKB_SUCCESS;
    }
}

pub unsafe fn xkb_machine_options_update_shortcut_mods(
    mut options: *mut xkb_machine_options,
    mut affect: xkb_mod_mask_t,
    mut mask: xkb_mod_mask_t,
) -> xkb_error_code {
    unsafe {
        (*options).shortcuts.mask &= !affect;
        (*options).shortcuts.mask |= mask & affect;
        return XKB_SUCCESS;
    }
}

pub unsafe fn xkb_machine_options_remap_shortcut_layout(
    mut options: *mut xkb_machine_options,
    mut source: xkb_layout_index_t,
    mut target: xkb_layout_index_t,
) -> xkb_error_code {
    unsafe {
        if source >= XKB_MAX_GROUPS as xkb_layout_index_t
            || target >= XKB_MAX_GROUPS as xkb_layout_index_t
        {
            return XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX;
        }
        if target == source {
            return XKB_SUCCESS;
        }
        let mut config: *mut xkb_shortcuts_config_options = &raw mut (*options).shortcuts;
        if source >= (*config).targets.size as xkb_layout_index_t {
            let mut new: xkb_layout_index_t = (*config).targets.size as xkb_layout_index_t;
            darray_resize(
                &mut (*config).targets.item,
                &mut (*config).targets.size,
                &mut (*config).targets.alloc,
                source.wrapping_add(1 as xkb_layout_index_t) as u32,
            );
            while new < source {
                *(*config).targets.item.offset(new as isize) =
                    XKB_LAYOUT_INVALID as xkb_layout_index_t;
                new = new.wrapping_add(1);
            }
        }
        *(*config).targets.item.offset(source as isize) = if source == target {
            XKB_LAYOUT_INVALID as xkb_layout_index_t
        } else {
            target
        };
        return XKB_SUCCESS;
    }
}

unsafe extern "C" fn cmp_mod_masks(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> i32 {
    unsafe {
        let m1: xkb_mod_mask_t = *(a as *mut xkb_mod_mask_t);
        let m2: xkb_mod_mask_t = *(b as *mut xkb_mod_mask_t);
        if m1 == m2 {
            return 0 as i32;
        }
        if m1 == 0 {
            return 1 as i32;
        }
        if m2 == 0 {
            return -1 as i32;
        }
        let overlap: xkb_mod_mask_t = m1 & m2;
        if overlap == m1 {
            return 1 as i32;
        }
        if overlap == m2 {
            return -1 as i32;
        }
        let count1: u32 = popcount32(m1 as u32) as u32;
        let count2: u32 = popcount32(m2 as u32) as u32;
        if count1 > count2 {
            return -1 as i32;
        }
        if count1 < count2 {
            return 1 as i32;
        }
        return if m1 < m2 { -1 as i32 } else { 1 as i32 };
    }
}

unsafe fn machine_set_mods(
    mut sm: *mut xkb_machine,
    mut raw_mappings: *const machine_mods_mappings,
) -> bool {
    unsafe {
        if !((*raw_mappings).size == 0 as darray_size_t) {
            let mut mappings: machine_mods_mappings = machine_mods_mappings {
                size: 0 as darray_size_t,
                alloc: 0 as darray_size_t,
                item: std::ptr::null_mut(),
            };
            let mut mask: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
            let mut mapping: *const machine_mods_mapping =
                std::ptr::null();
            let invalid: xkb_mod_mask_t = !(*(*sm).state.keymap).canonical_state_mask;
            if !(*raw_mappings).item.is_null() {
                mapping =
                    (*raw_mappings).item.offset(0 as i32 as isize) as *mut machine_mods_mapping;
                while mapping
                    < (*raw_mappings).item.offset((*raw_mappings).size as isize)
                        as *mut machine_mods_mapping
                        as *const machine_mods_mapping
                {
                    if !((*mapping).source == 0
                        || (*mapping).target == 0
                        || (*mapping).source & invalid != 0
                        || (*mapping).target & invalid != 0)
                    {
                        darray_append(
                            &mut mappings.item,
                            &mut mappings.size,
                            &mut mappings.alloc,
                            *mapping,
                        );
                        mask |= (*mapping).source;
                    }
                    mapping = mapping.offset(1);
                }
            }
            qsort(
                mappings.item as *mut ::core::ffi::c_void,
                mappings.size as usize,
                ::core::mem::size_of::<machine_mods_mapping>() as usize,
                Some(
                    cmp_mod_masks
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> i32,
                ),
            );
            (*sm).config.modifiers.mappings = mappings.item as *mut machine_mods_mapping;
            if !(&raw mut (*sm).config.modifiers.mappings_num).is_null() {
                *&raw mut (*sm).config.modifiers.mappings_num = mappings.size;
            }
            mappings.item = std::ptr::null_mut();
            mappings.size = 0 as darray_size_t;
            mappings.alloc = 0 as darray_size_t;
            (*sm).config.modifiers.mask = mask;
        } else {
            (*sm).config.modifiers = machine_modifiers_config {
                mask: 0 as xkb_mod_mask_t,
                mappings_num: 0 as darray_size_t,
                mappings: std::ptr::null_mut(),
            } as machine_modifiers_config;
        }
        return true;
    }
}

unsafe fn machine_set_shortcuts(
    mut sm: *mut xkb_machine,
    mut options: *const xkb_shortcuts_config_options,
) -> bool {
    unsafe {
        if (*options).targets.size == 0 as darray_size_t {
            (*sm).config.shortcuts = machine_shortcuts_config {
                mask: 0 as xkb_mod_mask_t,
                targets: std::ptr::null_mut(),
            } as machine_shortcuts_config;
            return true;
        }
        let keymap: *mut xkb_keymap = (*sm).state.keymap;
        let mut count: xkb_layout_index_t =
            if (*keymap).num_groups < (*options).targets.size as xkb_layout_index_t {
                (*keymap).num_groups
            } else {
                (*options).targets.size as xkb_layout_index_t
            };
        while count > 1 as xkb_layout_index_t {
            if *(*options)
                .targets
                .item
                .offset(count.wrapping_sub(1 as xkb_layout_index_t) as isize)
                < (*keymap).num_groups
            {
                break;
            }
            count = count.wrapping_sub(1);
        }
        if count == 0 {
            return true;
        }
        let mut mask: xkb_mod_mask_t = (*options).mask;
        if mask != 0 {
            mask &= ((1 as u64) << xkb_keymap_num_mods(keymap)).wrapping_sub(1 as u64)
                as xkb_mod_mask_t;
        }
        if mask == 0 {
            return true;
        }
        let targets: *mut xkb_layout_index_t = calloc(
            (*keymap).num_groups as usize,
            ::core::mem::size_of::<xkb_layout_index_t>() as usize,
        ) as *mut xkb_layout_index_t;
        if targets.is_null() {
            return false;
        }
        let mut l: xkb_layout_index_t = 0 as xkb_layout_index_t;
        while l < count {
            *targets.offset(l as isize) =
                if *(*options).targets.item.offset(l as isize) < (*keymap).num_groups {
                    *(*options).targets.item.offset(l as isize)
                } else {
                    XKB_LAYOUT_INVALID as xkb_layout_index_t
                };
            l = l.wrapping_add(1);
        }
        let mut l_0: xkb_layout_index_t = count;
        while l_0 < (*keymap).num_groups {
            *targets.offset(l_0 as isize) = XKB_LAYOUT_INVALID as xkb_layout_index_t;
            l_0 = l_0.wrapping_add(1);
        }
        (*sm).config.shortcuts = machine_shortcuts_config {
            mask: mask,
            targets: targets,
        } as machine_shortcuts_config;
        return true;
    }
}

pub unsafe fn xkb_machine_new(
    mut keymap: *mut xkb_keymap,
    mut options: *const xkb_machine_options,
) -> *mut xkb_machine {
    unsafe {
        let machine: *mut xkb_machine = Box::into_raw(Box::new(std::mem::zeroed::<xkb_machine>()));
        if options.is_null() {
            options = &raw const default_machine_options;
        }
        xkb_state_init(
            &raw mut (*machine).state,
            keymap,
            (*options).a11y_affect,
            (*options).a11y_flags,
        );
        if !machine_set_mods(machine, &raw const (*options).mods)
            || !machine_set_shortcuts(machine, &raw const (*options).shortcuts)
        {
            xkb_machine_unref(machine);
            return std::ptr::null_mut();
        } else {
            (*machine).overlays.keys.item = std::ptr::null_mut();
            (*machine).overlays.keys.size = 0 as darray_size_t;
            (*machine).overlays.keys.alloc = 0 as darray_size_t;
            return machine;
        };
    }
}

pub unsafe fn xkb_machine_ref(mut sm: *mut xkb_machine) -> *mut xkb_machine {
    unsafe {
        (*sm).state.refcnt += 1;
        return sm;
    }
}

pub unsafe fn xkb_machine_unref(mut sm: *mut xkb_machine) {
    unsafe {
        if sm.is_null() || {
            (*sm).state.refcnt -= 1;
            (*sm).state.refcnt > 0 as i32
        } {
            return;
        }
        xkb_state_destroy(&raw mut (*sm).state);
        darray_free(
            &mut (*sm).overlays.keys.item,
            &mut (*sm).overlays.keys.size,
            &mut (*sm).overlays.keys.alloc,
        );
        free((*sm).config.shortcuts.targets as *mut ::core::ffi::c_void);
        free((*sm).config.modifiers.mappings as *mut ::core::ffi::c_void);
        drop(Box::from_raw(sm));
    }
}

pub unsafe fn xkb_machine_get_keymap(mut sm: *const xkb_machine) -> *mut xkb_keymap {
    unsafe {
        return (*sm).state.keymap;
    }
}

pub unsafe fn xkb_machine_get_state(mut sm: *mut xkb_machine) -> *mut xkb_state {
    unsafe {
        return &raw mut (*sm).state;
    }
}

unsafe fn machine_update_overlays(mut sm: *mut xkb_machine) {
    unsafe {
        let mask: xkb_overlay_mask_t = ((*sm).state.components.controls as u32
            >> XKB_OVERLAY1_CONTROLS_OFFSET
            & ((1 as u32) << XKB_OVERLAY_MAX as usize).wrapping_sub(1 as u32))
            as xkb_overlay_mask_t;
        let mut added: xkb_overlay_mask_t =
            (mask as i32 & !((*sm).overlays.enabled as i32)) as xkb_overlay_mask_t;
        let mut order: u32 = (*sm).overlays.order;
        let overlay_max: xkb_overlay_index_t =
            format_max_overlays((*(*sm).state.keymap).format) as xkb_overlay_index_t;
        let mut n: u8 = 0;
        while (n as i32) < overlay_max as i32 {
            let mut overlay_idx: xkb_overlay_index_t =
                (order >> n as i32 * 4 as i32 & 0xf as u32) as xkb_overlay_index_t;
            if overlay_idx == 0 {
                break;
            }
            let overlay_mask: xkb_overlay_mask_t =
                ((1 as u32) << (overlay_idx as u32).wrapping_sub(1 as u32)) as xkb_overlay_mask_t;
            if overlay_mask as i32 & mask as i32 != 0 {
                added = (added as i32 & !(overlay_mask as i32)) as xkb_overlay_mask_t;
                n = n.wrapping_add(1);
            } else {
                let head: u32 = ((1 as u32) << n as i32 * 4 as i32).wrapping_sub(1 as u32);
                order = order & head | order >> 4 as i32 & !head;
            }
        }
        let mut k: xkb_overlay_index_t = 0 as xkb_overlay_index_t;
        while added != 0 {
            if added as i32 & 0x1 != 0 {
                order <<= 4 as i32;
                order = (order as u32 | (k as u32).wrapping_add(1 as u32)) as u32;
            }
            k = k.wrapping_add(1);
            added = (added as i32 >> 1 as i32) as xkb_overlay_mask_t;
        }
        (*sm).overlays.order = order;
        (*sm).overlays.enabled = mask;
    }
}

pub unsafe fn xkb_machine_process_synthetic(
    mut sm: *mut xkb_machine,
    mut update: *const xkb_state_update,
    mut events: *mut xkb_events,
) -> xkb_error_code {
    unsafe {
        let mut error: xkb_error_code = check_state_update_abi_(
            (*(*sm).state.keymap).ctx,
            b"xkb_machine_process_synthetic\0".as_ptr() as *const i8,
            update,
        );
        if error as u64 != 0 {
            return error;
        }
        let state: *mut xkb_state = &raw mut (*sm).state;
        let previous_components: state_components = (*state).components;
        if !(*update).layout_policy.is_null() {
            error = state_update_layout_policy(state, (*update).layout_policy);
            if error as u64 != 0 {
                return error;
            }
        }
        if !(*update).components.is_null() {
            let components: *const xkb_state_components_update = (*update).components;
            state_update_enabled_controls(
                state,
                (*components).affect_controls,
                (*components).controls,
                events,
            );
            state_update_latched_locked(state, components, events);
        }
        xkb_state_update_derived(state);
        let changed: xkb_state_component = get_state_component_changes(
            &raw const previous_components,
            &raw mut (*state).components,
        ) as xkb_state_component;
        if changed as u64 != 0 {
            if changed as u32 & XKB_STATE_CONTROLS as i32 as u32 != 0 {
                machine_update_overlays(sm);
            }
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        components: C2Rust_Unnamed_18 {
                            components: (*state).components,
                            changed: changed,
                        },
                    },
                },
            );
        }
        return XKB_SUCCESS;
    }
}

unsafe fn do_remap_modifiers(
    mut mappings: *const machine_modifiers_config,
    mut state: *mut xkb_state,
    mut events: *mut xkb_events,
    mut key: *const xkb_key,
) -> isize {
    unsafe {
        if (*mappings).mask & (*state).components.mods == 0 {
            return -1 as isize;
        }
        let layout: xkb_layout_index_t = state_key_get_layout(state, key) as xkb_layout_index_t;
        if layout >= (*key).num_groups() {
            return -1 as isize;
        }
        let type_0: *const xkb_key_type = (*(*key).groups.offset(layout as isize)).type_0;
        let mut affect: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut mods: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        let mut m: darray_size_t = 0 as darray_size_t;
        while m < (*mappings).mappings_num {
            let mapping: *const machine_mods_mapping =
                (*mappings).mappings.offset(m as isize) as *mut machine_mods_mapping;
            if (*mapping).source & (*state).components.mods == (*mapping).source
                && (*mapping).source & affect == 0
                && (*mapping).source & (*type_0).mods.mask == 0
            {
                affect |= (*(*mappings).mappings.offset(m as isize)).source;
                mods |= (*(*mappings).mappings.offset(m as isize)).target;
            }
            m = m.wrapping_add(1);
        }
        if affect == 0 {
            return -1 as isize;
        }
        let mut new: xkb_state = *state;
        new.components.base_mods = new.components.base_mods & !affect | mods;
        new.components.latched_mods = new.components.latched_mods & !affect;
        new.components.locked_mods = new.components.locked_mods & !affect;
        xkb_state_update_derived(&raw mut new);
        let mut event_idx: isize = -1 as isize;
        let changed: xkb_state_component =
            get_state_component_changes(&raw mut (*state).components, &raw mut new.components)
                as xkb_state_component;
        if changed as u64 != 0 {
            event_idx = (*events).queue.size as isize;
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        components: C2Rust_Unnamed_18 {
                            components: new.components,
                            changed: changed,
                        },
                    },
                },
            );
        }
        (*state).components.mods = new.components.mods;
        return event_idx;
    }
}

unsafe fn do_shortcuts_tweak(
    mut config: *const machine_shortcuts_config,
    mut state: *mut xkb_state,
    mut previous_components: *const state_components,
    mut events: *mut xkb_events,
    mut remap_event: isize,
) -> isize {
    unsafe {
        if !(*config).targets.is_null()
            && (*state).components.mods & (*config).mask != 0
            && *(*config).targets.offset((*state).components.group as isize)
                != XKB_LAYOUT_INVALID as xkb_layout_index_t
        {
            let mut new: xkb_state = *state;
            if remap_event < 0 as isize {
                remap_event = (*events).queue.size as isize;
                darray_append(
                    &mut (*events).queue.item,
                    &mut (*events).queue.size,
                    &mut (*events).queue.alloc,
                    xkb_event {
                        type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                        c2rust_unnamed: C2Rust_Unnamed_17 {
                            components: C2Rust_Unnamed_18 {
                                components: state_components {
                                    base_group: 0 as i32,
                                    latched_group: 0,
                                    locked_group: 0,
                                    group: 0,
                                    base_mods: 0,
                                    latched_mods: 0,
                                    locked_mods: 0,
                                    mods: 0,
                                    leds: 0,
                                    controls: 0 as xkb_action_controls,
                                },
                                changed: 0 as xkb_state_component,
                            },
                        },
                    },
                );
            } else {
                new.components = (*(*events).queue.item.offset(remap_event as isize))
                    .c2rust_unnamed
                    .components
                    .components;
            }
            new.components.base_group =
                *(*config).targets.offset((*state).components.group as isize) as i32
                    - (*state).components.latched_group
                    - (*state).components.locked_group;
            xkb_state_update_derived(&raw mut new);
            let changed: xkb_state_component =
                get_state_component_changes(previous_components, &raw mut new.components)
                    as xkb_state_component;
            (*(*events).queue.item.offset(remap_event as isize))
                .c2rust_unnamed
                .components
                .changed = changed;
            (*(*events).queue.item.offset(remap_event as isize))
                .c2rust_unnamed
                .components
                .components = new.components;
            (*state).components.group = new.components.group;
        }
        return remap_event;
    }
}

unsafe fn undo_tweaks(
    mut state: *const xkb_state,
    mut previous_components: *const state_components,
    mut events: *mut xkb_events,
) {
    unsafe {
        let mut event: *const xkb_event = std::ptr::null();
        if !(*events).queue.item.is_null() && (*events).queue.size != 0 {
            event = (*events)
                .queue
                .item
                .offset((*events).queue.size.wrapping_sub(1 as darray_size_t) as isize)
                as *mut xkb_event;
            while (*events).queue.size > 0 as darray_size_t
                && event
                    >= (*events).queue.item.offset(0 as i32 as isize) as *mut xkb_event
                        as *const xkb_event
            {
                if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
                    break;
                }
                event = event.offset(-1);
            }
        }
        if event.is_null() {
            return;
        }
        let changed: xkb_state_component = get_state_component_changes(
            previous_components,
            &raw const (*event).c2rust_unnamed.components.components,
        ) as xkb_state_component;
        if changed as u64 != 0 {
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        components: C2Rust_Unnamed_18 {
                            components: *previous_components,
                            changed: changed,
                        },
                    },
                },
            );
        }
    }
}

unsafe fn process_overlayable_key(
    mut sm: *mut xkb_machine,
    mut key: *const xkb_key,
    mut direction: xkb_key_direction,
) -> *const xkb_key {
    unsafe {
        let mut entry: *mut xkb_overlaid_key = std::ptr::null_mut();
        let mut available_entry: *mut xkb_overlaid_key =
            std::ptr::null_mut();
        if !(*sm).overlays.keys.item.is_null() {
            entry = (*sm).overlays.keys.item.offset(0 as i32 as isize) as *mut xkb_overlaid_key;
            while entry
                < (*sm)
                    .overlays
                    .keys
                    .item
                    .offset((*sm).overlays.keys.size as isize)
                    as *mut xkb_overlaid_key
            {
                if (*entry).old == key {
                    match direction as u32 {
                        1 => {
                            (*entry).refcnt += 1;
                        }
                        2 => {}
                        0 | _ => {
                            (*entry).refcnt -= 1;
                        }
                    }
                    if (*entry).refcnt <= 0 as i32 {
                        (*entry).old = std::ptr::null();
                    }
                    return (*entry).new;
                } else if (*entry).old.is_null() && available_entry.is_null() {
                    available_entry = entry;
                }
                entry = entry.offset(1);
            }
        }
        if direction as u32 == XKB_KEY_DOWN as i32 as u32 {
            let mut new: *const xkb_key = key;
            if (*key).overlays as i32 & (*sm).overlays.enabled as i32 != 0 {
                let mut stack: u32 = (*sm).overlays.order;
                while stack != 0 {
                    let overlay: xkb_overlay_index_t =
                        (stack & 0xf as u32).wrapping_sub(1 as u32) as xkb_overlay_index_t;
                    let mask: xkb_overlay_mask_t =
                        ((1 as u32) << overlay as i32) as xkb_overlay_mask_t;
                    if (*key).overlays as i32 & mask as i32 != 0 {
                        if (*key).overlays_inline() {
                            new = (*key).c2rust_unnamed.overlay_key;
                        } else {
                            let low: xkb_overlay_mask_t = ((*key).overlays as i32
                                & (mask as u32).wrapping_sub(1 as u32) as xkb_overlay_mask_t as i32)
                                as xkb_overlay_mask_t;
                            let index: xkb_overlay_index_t =
                                popcount32(low as u32) as xkb_overlay_index_t;
                            new = *(*key).c2rust_unnamed.overlays_keys.offset(index as isize);
                        }
                        break;
                    } else {
                        stack >>= 4 as i32;
                    }
                }
            }
            if !available_entry.is_null() {
                entry = available_entry;
            } else {
                let idx: darray_size_t = (*sm).overlays.keys.size;
                darray_append(
                    &mut (*sm).overlays.keys.item,
                    &mut (*sm).overlays.keys.size,
                    &mut (*sm).overlays.keys.alloc,
                    core::mem::zeroed::<xkb_overlaid_key>(),
                );
                entry = (*sm).overlays.keys.item.offset(idx as isize) as *mut xkb_overlaid_key;
            }
            (*entry).old = key;
            (*entry).new = new;
            (*entry).refcnt = 1 as i32;
            return new;
        }
        return key;
    }
}
pub unsafe fn xkb_machine_process_key(
    mut sm: *mut xkb_machine,
    mut kc: xkb_keycode_t,
    mut direction: xkb_key_direction,
    mut events: *mut xkb_events,
) -> xkb_error_code {
    unsafe {
        (*events).queue.size = 0 as darray_size_t;
        (*events).next = 0 as darray_size_t;
        let state: *mut xkb_state = &raw mut (*sm).state;
        let mut key: *const xkb_key = XkbKey((*state).keymap, kc);
        if key.is_null() || direction as u32 == XKB_KEY_REPEATED as i32 as u32 && !(*key).repeats()
        {
            return XKB_SUCCESS;
        }
        let previous_components: state_components = (*state).components;
        if (*key).overlays != 0 {
            key = process_overlayable_key(sm, key, direction);
        }
        let mut remap_event: isize =
            do_remap_modifiers(&raw mut (*sm).config.modifiers, state, events, key);
        remap_event = do_shortcuts_tweak(
            &raw mut (*sm).config.shortcuts,
            state,
            &raw const previous_components,
            events,
            remap_event,
        );
        (*state).set_mods = 0 as xkb_mod_mask_t;
        (*state).clear_mods = 0 as xkb_mod_mask_t;
        xkb_filter_apply_all(state, events, key, direction);
        let mut i: xkb_mod_index_t = 0;
        let mut bit: xkb_mod_mask_t = 0;
        i = 0 as xkb_mod_index_t;
        bit = 1 as xkb_mod_mask_t;
        while (*state).set_mods != 0 {
            if (*state).set_mods & bit != 0 {
                (*state).mod_key_count[i as usize] += 1;
                (*state).components.base_mods |= bit;
                (*state).set_mods &= !bit;
            }
            i = i.wrapping_add(1);
            bit <<= 1 as i32;
        }
        i = 0 as xkb_mod_index_t;
        bit = 1 as xkb_mod_mask_t;
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
        let mut has_key_event: bool = false;
        let mut event: *const xkb_event = std::ptr::null();
        if !(*events).queue.item.is_null() {
            event = (*events).queue.item.offset(0 as i32 as isize) as *mut xkb_event;
            while event
                < (*events).queue.item.offset((*events).queue.size as isize) as *mut xkb_event
                    as *const xkb_event
            {
                match (*event).type_0 as u32 {
                    1 | 2 | 3 => {
                        has_key_event = true;
                    }
                    _ => {}
                }
                event = event.offset(1);
            }
        }
        if !has_key_event {
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: (if direction as u32 == XKB_KEY_UP as i32 as u32 {
                        XKB_EVENT_TYPE_KEY_UP as i32
                    } else if direction as u32 == XKB_KEY_REPEATED as i32 as u32 {
                        XKB_EVENT_TYPE_KEY_REPEATED as i32
                    } else {
                        XKB_EVENT_TYPE_KEY_DOWN as i32
                    }) as xkb_event_type,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        keycode: (*key).keycode,
                    },
                },
            );
        }
        if remap_event >= 0 as isize {
            undo_tweaks(state, &raw const previous_components, events);
        }
        let changed: xkb_state_component = get_state_component_changes(
            &raw const previous_components,
            &raw mut (*state).components,
        ) as xkb_state_component;
        if changed as u64 != 0 {
            if changed as u32 & XKB_STATE_CONTROLS as i32 as u32 != 0 {
                machine_update_overlays(sm);
            }
            darray_append(
                &mut (*events).queue.item,
                &mut (*events).queue.size,
                &mut (*events).queue.alloc,
                xkb_event {
                    type_0: XKB_EVENT_TYPE_COMPONENTS_CHANGE,
                    c2rust_unnamed: C2Rust_Unnamed_17 {
                        components: C2Rust_Unnamed_18 {
                            components: (*state).components,
                            changed: changed,
                        },
                    },
                },
            );
        }
        return XKB_SUCCESS;
    }
}

pub unsafe fn xkb_events_new_batch(
    mut context: *mut xkb_context,
    mut flags: xkb_events_flags,
) -> *mut xkb_events {
    unsafe {
        static mut XKB_EVENTS_FLAGS: xkb_events_flags = XKB_EVENTS_NO_FLAGS;
        if flags as u32 & !(XKB_EVENTS_FLAGS as u32) != 0 {
            xkb_logf!(
                context,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unrecognized events batch flags: {:#x}\n",
                crate::xkb::utils::CStrDisplay(b"xkb_events_new_batch\0".as_ptr() as *const i8),
                flags as u32 & !(XKB_EVENTS_FLAGS as u32),
            );
            return std::ptr::null_mut();
        }
        let mut events: *mut xkb_events = Box::into_raw(Box::new(std::mem::zeroed::<xkb_events>()));
        (*events).queue.item = std::ptr::null_mut();
        (*events).queue.size = 0 as darray_size_t;
        (*events).queue.alloc = 0 as darray_size_t;
        (*events).next = 0 as darray_size_t;
        (*events).ctx = xkb_context_ref(context);
        return events;
    }
}

pub unsafe fn xkb_events_destroy(mut events: *mut xkb_events) {
    unsafe {
        if events.is_null() {
            return;
        }
        darray_free(
            &mut (*events).queue.item,
            &mut (*events).queue.size,
            &mut (*events).queue.alloc,
        );
        xkb_context_unref((*events).ctx);
        drop(Box::from_raw(events));
    }
}
pub unsafe fn xkb_events_next(mut events: *mut xkb_events) -> *const xkb_event {
    unsafe {
        if (*events).next < (*events).queue.size {
            let c2rust_fresh0 = (*events).next;
            (*events).next = (*events).next.wrapping_add(1);
            let index: darray_size_t = c2rust_fresh0;
            return (*events).queue.item.offset(index as isize) as *mut xkb_event;
        } else {
            return std::ptr::null();
        };
    }
}
pub unsafe fn xkb_event_get_type(mut event: *const xkb_event) -> xkb_event_type {
    unsafe {
        return (*event).type_0;
    }
}
pub unsafe fn xkb_event_get_keycode(mut event: *const xkb_event) -> xkb_keycode_t {
    unsafe {
        match (*event).type_0 as u32 {
            1 | 2 | 3 => return (*event).c2rust_unnamed.keycode,
            _ => return XKB_KEYCODE_INVALID as xkb_keycode_t,
        };
    }
}

pub unsafe fn xkb_event_get_changed_components(mut event: *const xkb_event) -> xkb_state_component {
    unsafe {
        return (if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
            (*event).c2rust_unnamed.components.changed as u32
        } else {
            0 as u32
        }) as xkb_state_component;
    }
}

pub unsafe fn xkb_event_serialize_enabled_controls(
    mut event: *const xkb_event,
    mut components: xkb_state_component,
) -> xkb_keyboard_control_flags {
    unsafe {
        return (if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
            serialize_controls(
                &raw const (*event).c2rust_unnamed.components.components,
                components,
            ) as u32
        } else {
            0 as u32
        }) as xkb_keyboard_control_flags;
    }
}

pub unsafe fn xkb_event_serialize_mods(
    mut event: *const xkb_event,
    mut components: xkb_state_component,
) -> xkb_mod_mask_t {
    unsafe {
        return if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
            serialize_mods(
                &raw const (*event).c2rust_unnamed.components.components,
                components,
            )
        } else {
            0 as xkb_mod_mask_t
        };
    }
}

pub unsafe fn xkb_event_serialize_layout(
    mut event: *const xkb_event,
    mut components: xkb_state_component,
) -> xkb_layout_index_t {
    unsafe {
        return if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
            serialize_layout(
                &raw const (*event).c2rust_unnamed.components.components,
                components,
            )
        } else {
            XKB_LAYOUT_INVALID as xkb_layout_index_t
        };
    }
}
pub unsafe fn xkb_state_update_event(
    mut state: *mut xkb_state,
    mut event: *const xkb_event,
) -> xkb_state_component {
    unsafe {
        if (*event).type_0 as u32 == XKB_EVENT_TYPE_COMPONENTS_CHANGE as i32 as u32 {
            let prev_components: state_components = (*state).components;
            (*state).components = (*event).c2rust_unnamed.components.components;
            return get_state_component_changes(
                &raw const prev_components,
                &raw mut (*state).components,
            );
        } else {
            return 0 as xkb_state_component;
        };
    }
}
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        synthetic_key_group_break_group_latch = {
            let mut init = xkb_group {
                explicit_symbols_explicit_actions_implicit_actions_explicit_type: [0; 1],
                c2rust_padding: [0; 7],
                type_0: &raw mut synthetic_key_type,
                levels: &raw mut synthetic_key_level_break_group_latch,
            };
            init.set_explicit_symbols(false);
            init.set_explicit_actions(false);
            init.set_implicit_actions(false);
            init.set_explicit_type(false);
            init
        };
        synthetic_key_break_group_latch = {
            let mut init = xkb_key {
                overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups: [0; 3],
                keycode: 0,
                name: 0,
                explicit: 0 as xkb_explicit_components,
                modmap: 0,
                vmodmap: 0,
                overlays: 0,
                groups: &raw mut synthetic_key_group_break_group_latch,
                c2rust_unnamed: C2Rust_Unnamed_9 {
                    overlay_key: std::ptr::null(),
                },
            };
            init.set_overlays_inline(false);
            init.set_repeats(false);
            init.set_implicit_actions(false);
            init.set_out_of_range_pending_group(false);
            init.set_out_of_range_group_policy(XKB_LAYOUT_OUT_OF_RANGE_WRAP);
            init.set_out_of_range_group_number(0);
            init.set_num_groups(1 as xkb_layout_index_t);
            init
        };
        synthetic_key = {
            let mut init = xkb_key {
                overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups: [0; 3],
                keycode: 0 as xkb_keycode_t,
                name: 0,
                explicit: 0 as xkb_explicit_components,
                modmap: 0,
                vmodmap: 0,
                overlays: 0,
                groups: std::ptr::null_mut(),
                c2rust_unnamed: C2Rust_Unnamed_9 {
                    overlay_key: std::ptr::null(),
                },
            };
            init.set_overlays_inline(false);
            init.set_repeats(false);
            init.set_implicit_actions(false);
            init.set_out_of_range_pending_group(false);
            init.set_out_of_range_group_policy(XKB_LAYOUT_OUT_OF_RANGE_WRAP);
            init.set_out_of_range_group_number(0);
            init.set_num_groups(0);
            init
        };
    }
}
use crate::xkb::context::{xkb_context_ref, xkb_context_unref};
use crate::xkb::keymap::{
    xkb_keymap_key_get_syms_by_level, xkb_keymap_layout_get_index, xkb_keymap_led_get_index,
    xkb_keymap_mod_get_index, xkb_keymap_num_layouts_for_key, xkb_keymap_num_mods, xkb_keymap_ref,
    xkb_keymap_unref,
};
use crate::xkb::keysym_case_mappings::xkb_keysym_to_upper;
use crate::xkb::keysym_utf::{xkb_keysym_to_utf32, xkb_keysym_to_utf8};
use crate::xkb::shared_types::*;
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
