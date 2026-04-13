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
    XKB_LOG_LEVEL_CRITICAL as u32,
    XKB_LOG_LEVEL_ERROR as u32,
    XKB_LOG_LEVEL_WARNING as u32,
    XKB_LOG_LEVEL_INFO as u32,
    XKB_LOG_LEVEL_DEBUG as u32,
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
    XKB_FEATURE_ENUM_FEATURE as u32,
    XKB_FEATURE_ENUM_ERROR_CODE as u32,
    XKB_FEATURE_ENUM_CONTEXT_FLAGS as u32,
    XKB_FEATURE_ENUM_LOG_LEVEL as u32,
    XKB_FEATURE_ENUM_KEYSYM_FLAGS as u32,
    XKB_FEATURE_ENUM_RMLVO_BUILDER_FLAGS as u32,
    XKB_FEATURE_ENUM_KEYMAP_FORMAT as u32,
    XKB_FEATURE_ENUM_KEYMAP_COMPILE_FLAGS as u32,
    XKB_FEATURE_ENUM_KEYMAP_SERIALIZE_FLAGS as u32,
    XKB_FEATURE_ENUM_KEYMAP_KEY_ITERATOR_FLAGS as u32,
    XKB_FEATURE_ENUM_STATE_COMPONENT as u32,
    XKB_FEATURE_ENUM_LAYOUT_OUT_OF_RANGE_POLICY as u32,
    XKB_FEATURE_ENUM_A11Y_FLAGS as u32,
    XKB_FEATURE_ENUM_KEYBOARD_CONTROL_FLAGS as u32,
    XKB_FEATURE_ENUM_STATE_MATCH as u32,
    XKB_FEATURE_ENUM_CONSUMED_MODE as u32,
    XKB_FEATURE_ENUM_EVENT_TYPE as u32,
    XKB_FEATURE_ENUM_KEY_DIRECTION as u32,
    XKB_FEATURE_ENUM_EVENTS_FLAGS as u32,
    XKB_FEATURE_ENUM_COMPOSE_FORMAT as u32,
    XKB_FEATURE_ENUM_COMPOSE_COMPILE_FLAGS as u32,
    XKB_FEATURE_ENUM_COMPOSE_STATUS as u32,
    XKB_FEATURE_ENUM_COMPOSE_STATE_FLAGS as u32,
    XKB_FEATURE_ENUM_COMPOSE_FEED_RESULT as u32,
];

fn is_supported_enum_value_mask(mut values: u32, mut value: u32) -> bool {
    return value < 32 as u32 && values as u32 & (1 as u32) << value != 0;
}
unsafe fn is_supported_enum_value_array(
    mut values: *const u32,
    mut size: usize,
    mut value: u32,
) -> bool {
    unsafe {
        let mut v: usize = 0 as usize;
        while v < size {
            if *values.offset(v as isize) == value {
                return true;
            }
            v = v.wrapping_add(1);
        }
        return false;
    }
}
#[inline]
unsafe fn is_supported_flag_value(mut values: u32, mut accept_zero: bool, mut value: u32) -> bool {
    return (accept_zero as i32 != 0 || value != 0) && values as u32 & value == value;
}
pub unsafe fn xkb_feature_supported(mut feature: xkb_feature, mut value: u32) -> bool {
    unsafe {
        match feature as u32 {
            1 => {
                return is_supported_enum_value_array(
                    &raw const xkb_feature_values as *const u32,
                    (std::mem::size_of::<[u32; 24]>() as usize)
                        .wrapping_div(std::mem::size_of::<u32>() as usize),
                    value,
                );
            }
            1000 => {
                return is_supported_enum_value_array(
                    &raw const xkb_error_code_values as *const u32,
                    (std::mem::size_of::<[u32; 9]>() as usize)
                        .wrapping_div(std::mem::size_of::<u32>() as usize),
                    value,
                );
            }
            3200 => {
                return is_supported_flag_value(XKB_CONTEXT_FLAGS_VALUES, true, value);
            }
            5100 => {
                return is_supported_enum_value_array(
                    &raw const xkb_log_level_values as *const u32,
                    (std::mem::size_of::<[u32; 5]>() as usize)
                        .wrapping_div(std::mem::size_of::<u32>() as usize),
                    value,
                );
            }
            9200 => {
                return is_supported_flag_value(XKB_KEYSYM_FLAGS_VALUES, true, value);
            }
            18200 => {
                return is_supported_flag_value(XKB_RMLVO_BUILDER_FLAGS_VALUES, true, value);
            }
            21000 => return is_supported_enum_value_mask(XKB_KEYMAP_FORMAT_VALUES, value),
            21200 => {
                return is_supported_flag_value(XKB_KEYMAP_COMPILE_FLAGS_VALUES, true, value);
            }
            21400 => {
                return is_supported_flag_value(XKB_KEYMAP_SERIALIZE_FLAGS_VALUES, true, value);
            }
            21600 => {
                return is_supported_flag_value(XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES, true, value);
            }
            24000 => {
                return is_supported_flag_value(XKB_STATE_COMPONENT_VALUES, false, value);
            }
            24020 => {
                return is_supported_enum_value_mask(XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES, value);
            }
            24040 => {
                return is_supported_flag_value(XKB_A11Y_FLAGS_VALUES, true, value);
            }
            24060 => {
                return is_supported_flag_value(XKB_KEYBOARD_CONTROL_FLAGS_VALUES, true, value);
            }
            24820 => {
                return is_supported_flag_value(XKB_STATE_MATCH_VALUES, false, value);
            }
            24840 => return is_supported_enum_value_mask(XKB_CONSUMED_MODE_VALUES, value),
            27000 => return is_supported_enum_value_mask(XKB_EVENT_TYPE_VALUES, value),
            27020 => return is_supported_enum_value_mask(XKB_KEY_DIRECTION_VALUES, value),
            27600 => {
                return is_supported_flag_value(XKB_EVENTS_FLAGS_VALUES, true, value);
            }
            30000 => {
                return is_supported_enum_value_mask(XKB_COMPOSE_FORMAT_VALUES, value);
            }
            30200 => {
                return is_supported_flag_value(XKB_COMPOSE_COMPILE_FLAGS_VALUES, true, value);
            }
            31000 => {
                return is_supported_enum_value_mask(XKB_COMPOSE_STATUS_VALUES, value);
            }
            31200 => {
                return is_supported_flag_value(XKB_COMPOSE_STATE_FLAGS_VALUES, true, value);
            }
            31300 => {
                return is_supported_enum_value_mask(XKB_COMPOSE_FEED_RESULT_VALUES, value);
            }
            _ => return false,
        };
    }
}
