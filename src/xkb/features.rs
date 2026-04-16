use crate::xkb::shared_types::*;
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

pub static mut xkb_log_level_values: [u32; 5] = [
    XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_WARNING,
    XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_DEBUG,
];
pub static mut xkb_error_code_values: [u32; 9] = [
    XKB_ERROR_INVALID as u32,
    XKB_SUCCESS as u32,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK as u32,
    XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY as u32,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as u32,
    XKB_ERROR_UNSUPPORTED_A11Y_FLAGS as u32,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE as u32,
    XKB_ERROR_ABI_FORWARD_COMPAT as u32,
    XKB_ERROR_ABI_BACKWARD_COMPAT as u32,
];
pub static mut xkb_feature_values: [u32; 24] = [
    XKB_FEATURE_ENUM_FEATURE,
    XKB_FEATURE_ENUM_ERROR_CODE,
    XKB_FEATURE_ENUM_CONTEXT_FLAGS,
    XKB_FEATURE_ENUM_LOG_LEVEL,
    XKB_FEATURE_ENUM_KEYSYM_FLAGS,
    XKB_FEATURE_ENUM_RMLVO_BUILDER_FLAGS,
    XKB_FEATURE_ENUM_KEYMAP_FORMAT,
    XKB_FEATURE_ENUM_KEYMAP_COMPILE_FLAGS,
    XKB_FEATURE_ENUM_KEYMAP_SERIALIZE_FLAGS,
    XKB_FEATURE_ENUM_KEYMAP_KEY_ITERATOR_FLAGS,
    XKB_FEATURE_ENUM_STATE_COMPONENT,
    XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_FEATURE_ENUM_A11Y_FLAGS,
    XKB_FEATURE_ENUM_KEYBOARD_CONTROL_FLAGS,
    XKB_FEATURE_ENUM_STATE_MATCH,
    XKB_FEATURE_ENUM_CONSUMED_MODE,
    XKB_FEATURE_ENUM_EVENT_TYPE,
    XKB_FEATURE_ENUM_KEY_DIRECTION,
    XKB_FEATURE_ENUM_EVENTS_FLAGS,
    XKB_FEATURE_ENUM_COMPOSE_FORMAT,
    XKB_FEATURE_ENUM_COMPOSE_COMPILE_FLAGS,
    XKB_FEATURE_ENUM_COMPOSE_STATUS,
    XKB_FEATURE_ENUM_COMPOSE_STATE_FLAGS,
    XKB_FEATURE_ENUM_COMPOSE_FEED_RESULT,
];

fn is_supported_enum_value_mask(values: u32, value: u32) -> bool {
    value < 32_u32 && values & 1_u32 << value != 0
}
unsafe fn is_supported_enum_value_array(values: *const u32, size: usize, value: u32) -> bool {
    unsafe {
        let mut v: usize = 0_usize;
        while v < size {
            if *values.add(v) == value {
                return true;
            }
            v = v.wrapping_add(1);
        }
        false
    }
}
fn is_supported_flag_value(values: u32, accept_zero: bool, value: u32) -> bool {
    (accept_zero as i32 != 0 || value != 0) && values & value == value
}
pub unsafe fn xkb_feature_supported(feature: xkb_feature, value: u32) -> bool {
    unsafe {
        match feature {
            1 => is_supported_enum_value_array(
                &raw const xkb_feature_values as *const u32,
                (std::mem::size_of::<[u32; 24]>()).wrapping_div(std::mem::size_of::<u32>()),
                value,
            ),
            1000 => is_supported_enum_value_array(
                &raw const xkb_error_code_values as *const u32,
                (std::mem::size_of::<[u32; 9]>()).wrapping_div(std::mem::size_of::<u32>()),
                value,
            ),
            3200 => is_supported_flag_value(XKB_CONTEXT_FLAGS_VALUES, true, value),
            5100 => is_supported_enum_value_array(
                &raw const xkb_log_level_values as *const u32,
                (std::mem::size_of::<[u32; 5]>()).wrapping_div(std::mem::size_of::<u32>()),
                value,
            ),
            9200 => is_supported_flag_value(XKB_KEYSYM_FLAGS_VALUES, true, value),
            18200 => is_supported_flag_value(XKB_RMLVO_BUILDER_FLAGS_VALUES, true, value),
            21000 => is_supported_enum_value_mask(XKB_KEYMAP_FORMAT_VALUES, value),
            21200 => is_supported_flag_value(XKB_KEYMAP_COMPILE_FLAGS_VALUES, true, value),
            21400 => is_supported_flag_value(XKB_KEYMAP_SERIALIZE_FLAGS_VALUES, true, value),
            21600 => is_supported_flag_value(XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES, true, value),
            24000 => is_supported_flag_value(XKB_STATE_COMPONENT_VALUES, false, value),
            24020 => is_supported_enum_value_mask(XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES, value),
            24040 => is_supported_flag_value(XKB_A11Y_FLAGS_VALUES, true, value),
            24060 => is_supported_flag_value(XKB_KEYBOARD_CONTROL_FLAGS_VALUES, true, value),
            24820 => is_supported_flag_value(XKB_STATE_MATCH_VALUES, false, value),
            24840 => is_supported_enum_value_mask(XKB_CONSUMED_MODE_VALUES, value),
            27000 => is_supported_enum_value_mask(XKB_EVENT_TYPE_VALUES, value),
            27020 => is_supported_enum_value_mask(XKB_KEY_DIRECTION_VALUES, value),
            27600 => is_supported_flag_value(XKB_EVENTS_FLAGS_VALUES, true, value),
            30000 => is_supported_enum_value_mask(XKB_COMPOSE_FORMAT_VALUES, value),
            30200 => is_supported_flag_value(XKB_COMPOSE_COMPILE_FLAGS_VALUES, true, value),
            31000 => is_supported_enum_value_mask(XKB_COMPOSE_STATUS_VALUES, value),
            31200 => is_supported_flag_value(XKB_COMPOSE_STATE_FLAGS_VALUES, true, value),
            31300 => is_supported_enum_value_mask(XKB_COMPOSE_FEED_RESULT_VALUES, value),
            _ => false,
        }
    }
}
