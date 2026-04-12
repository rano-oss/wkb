use crate::xkb_logf;
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod xkbcommon_errors_h {
    pub type xkb_error_code = i32;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT: xkb_error_code = 914;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT: xkb_error_code = 876;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE: xkb_error_code = 450;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS: xkb_error_code = 371;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX: xkb_error_code = 237;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY: xkb_error_code = 214;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK: xkb_error_code = 60;
    pub const XKB_SUCCESS: xkb_error_code = 0;
    pub const XKB_ERROR_INVALID: xkb_error_code = -1;
}
pub mod context_h {
    pub use crate::xkb::shared_types::*;
}
pub mod atom_h {
    pub use crate::xkb::shared_types::*;

    extern "C" {
        pub type atom_table;
    }
}

pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;

    pub type xkb_rmlvo_builder_flags = u32;
    pub const XKB_RMLVO_BUILDER_NO_FLAGS: xkb_rmlvo_builder_flags = 0;
    extern "C" {
        pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;
        pub fn xkb_context_unref(context: *mut xkb_context);
    }
}
pub mod rmlvo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder {
        pub rules: *mut i8,
        pub model: *mut i8,
        pub layouts: xkb_rmlvo_builder_layouts,
        pub options: xkb_rmlvo_builder_options,
        pub refcnt: i32,
        pub ctx: *mut xkb_context,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_options {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_option,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_option {
        pub option: *mut i8,
        pub layout: xkb_layout_index_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_layouts {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_layout,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rmlvo_builder_layout {
        pub layout: *mut i8,
        pub variant: *mut i8,
    }
    use super::context_h::xkb_context;
    use super::xkbcommon_h::xkb_layout_index_t;
    use crate::xkb::shared_types::darray_size_t;
}
pub mod messages_codes_h {
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub type xkb_log_verbosity = i32;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
    pub type xkb_message_code = u32;
    pub const _XKB_LOG_MESSAGE_MAX_CODE: xkb_message_code = 971;
    pub const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: xkb_message_code = 971;
    pub const XKB_ERROR_INVALID_RULES_SYNTAX: xkb_message_code = 967;
    pub const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: xkb_message_code = 965;
    pub const XKB_ERROR_INVALID_IDENTIFIER: xkb_message_code = 949;
    pub const XKB_WARNING_CONFLICTING_KEY_FIELDS: xkb_message_code = 935;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT_: xkb_message_code = 914;
    pub const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: xkb_message_code = 903;
    pub const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: xkb_message_code = 901;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: xkb_message_code = 893;
    pub const XKB_WARNING_CONFLICTING_KEY_ACTION: xkb_message_code = 883;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT_: xkb_message_code = 876;
    pub const XKB_ERROR_UNKNOWN_ACTION_TYPE: xkb_message_code = 844;
    pub const XKB_ERROR_KEYMAP_COMPILATION_FAILED: xkb_message_code = 822;
    pub const XKB_ERROR_UNKNOWN_FIELD: xkb_message_code = 812;
    pub const XKB_WARNING_CONFLICTING_MODMAP: xkb_message_code = 800;
    pub const XKB_ERROR_INVALID_VALUE: xkb_message_code = 796;
    pub const XKB_ERROR_INVALID_EXPRESSION_TYPE: xkb_message_code = 784;
    pub const XKB_WARNING_UNDEFINED_KEYCODE: xkb_message_code = 770;
    pub const XKB_ERROR_INVALID_XKB_SYNTAX: xkb_message_code = 769;
    pub const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: xkb_message_code = 762;
    pub const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: xkb_message_code = 742;
    pub const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: xkb_message_code = 711;
    pub const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: xkb_message_code = 700;
    pub const XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT: xkb_message_code = 693;
    pub const XKB_ERROR_INVALID_COMPOSE_SYNTAX: xkb_message_code = 685;
    pub const XKB_ERROR_INVALID_COMPOSE_LOCALE: xkb_message_code = 679;
    pub const XKB_ERROR_INVALID_INCLUDED_FILE: xkb_message_code = 661;
    pub const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: xkb_message_code = 645;
    pub const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: xkb_message_code = 639;
    pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: xkb_message_code = 632;
    pub const XKB_ERROR_INVALID_REAL_MODIFIER: xkb_message_code = 623;
    pub const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: xkb_message_code = 607;
    pub const XKB_ERROR_CANNOT_RESOLVE_RMLVO: xkb_message_code = 595;
    pub const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: xkb_message_code = 588;
    pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
    pub const XKB_ERROR_INVALID_ACTION_FIELD: xkb_message_code = 563;
    pub const XKB_ERROR_INVALID_FILE_ENCODING: xkb_message_code = 542;
    pub const XKB_WARNING_CONFLICTING_KEY_NAME: xkb_message_code = 523;
    pub const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: xkb_message_code = 516;
    pub const XKB_WARNING_NUMERIC_KEYSYM: xkb_message_code = 489;
    pub const XKB_ERROR_INVALID_OPERATION: xkb_message_code = 478;
    pub const XKB_WARNING_CONFLICTING_KEY_SYMBOL: xkb_message_code = 461;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE_: xkb_message_code = 450;
    pub const XKB_WARNING_MISSING_DEFAULT_SECTION: xkb_message_code = 433;
    pub const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: xkb_message_code = 428;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: xkb_message_code = 407;
    pub const XKB_ERROR_RECURSIVE_INCLUDE: xkb_message_code = 386;
    pub const XKB_WARNING_DUPLICATE_ENTRY: xkb_message_code = 378;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_: xkb_message_code = 371;
    pub const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: xkb_message_code = 362;
    pub const XKB_ERROR_OVERLAPPING_OVERLAY: xkb_message_code = 355;
    pub const XKB_ERROR_UNKNOWN_OPERATOR: xkb_message_code = 345;
    pub const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: xkb_message_code = 338;
    pub const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: xkb_message_code = 312;
    pub const XKB_WARNING_NON_BASE_GROUP_NAME: xkb_message_code = 305;
    pub const XKB_WARNING_DEPRECATED_KEYSYM_NAME: xkb_message_code = 302;
    pub const XKB_WARNING_DEPRECATED_KEYSYM: xkb_message_code = 301;
    pub const XKB_WARNING_UNDEFINED_KEY_TYPE: xkb_message_code = 286;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: xkb_message_code = 266;
    pub const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: xkb_message_code = 254;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: xkb_message_code = 239;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: xkb_message_code = 237;
    pub const XKB_ERROR_UNKNOWN_STATEMENT: xkb_message_code = 222;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_: xkb_message_code = 214;
    pub const XKB_ERROR_INVALID_MODMAP_ENTRY: xkb_message_code = 206;
    pub const XKB_ERROR_INVALID_INCLUDE_STATEMENT: xkb_message_code = 203;
    pub const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: xkb_message_code = 195;
    pub const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: xkb_message_code = 193;
    pub const XKB_WARNING_CANNOT_INFER_KEY_TYPE: xkb_message_code = 183;
    pub const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: xkb_message_code = 172;
    pub const XKB_ERROR_INVALID_PATH: xkb_message_code = 161;
    pub const XKB_ERROR_WRONG_STATEMENT_TYPE: xkb_message_code = 150;
    pub const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: xkb_message_code = 134;
    pub const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: xkb_message_code = 123;
    pub const XKB_WARNING_UNRECOGNIZED_KEYSYM: xkb_message_code = 107;
    pub const XKB_WARNING_ILLEGAL_KEYCODE_ALIAS: xkb_message_code = 101;
    pub const XKB_ERROR_INVALID_NUMERIC_KEYSYM: xkb_message_code = 82;
    pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: xkb_message_code = 60;
    pub const XKB_ERROR_INTEGER_OVERFLOW: xkb_message_code = 52;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: xkb_message_code = 43;
    pub const XKB_ERROR_MALFORMED_NUMBER_LITERAL: xkb_message_code = 34;
    pub const _XKB_LOG_MESSAGE_MIN_CODE: xkb_message_code = 34;
}
pub mod utils_h {
    #[inline]
    pub unsafe fn strdup_safe(mut s: *const i8) -> *mut i8 {
        unsafe { cstr_dup(s) }
    }

    use crate::xkb::utils::cstr_dup;
}
pub mod keymap_h {
    pub const XKB_MAX_GROUPS: i32 = 32 as i32;
}
pub mod rules_h {
    pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
}

pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::internal::__va_list_tag;
pub use self::keymap_h::XKB_MAX_GROUPS;
pub use self::messages_codes_h::{
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
pub use self::rmlvo_h::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_layouts,
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options,
};
pub use self::rules_h::OPTIONS_GROUP_SPECIFIER_PREFIX;
pub use self::utils_h::strdup_safe;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_context_ref, xkb_context_unref, xkb_layout_index_t, xkb_log_level, xkb_rmlvo_builder_flags,
    xkb_rule_names, XKB_LAYOUT_INVALID, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_RMLVO_BUILDER_NO_FLAGS,
};
pub use crate::xkb::shared_types::darray_size_t;
use crate::xkb::utils::cstr_cmp;
use crate::xkb::utils::{darray_append, darray_free};
use libc::{calloc, free};
pub unsafe fn xkb_rmlvo_builder_new(
    mut context: *mut xkb_context,
    mut rules: *const i8,
    mut model: *const i8,
    mut flags: xkb_rmlvo_builder_flags,
) -> *mut xkb_rmlvo_builder {
    unsafe {
        static mut XKB_RMLVO_BUILDER_FLAGS: xkb_rmlvo_builder_flags = XKB_RMLVO_BUILDER_NO_FLAGS;
        if flags as u32 & !(XKB_RMLVO_BUILDER_FLAGS as u32) != 0 {
            xkb_logf!(
                context,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Unsupported RMLVO flags: 0x{:x}\n",
                flags as u32 & !(XKB_RMLVO_BUILDER_FLAGS as u32),
            );
            return ::core::ptr::null_mut::<xkb_rmlvo_builder>();
        }
        let builder: *mut xkb_rmlvo_builder = calloc(
            1 as usize,
            ::core::mem::size_of::<xkb_rmlvo_builder>() as usize,
        ) as *mut xkb_rmlvo_builder;
        if !builder.is_null() {
            (*builder).refcnt = 1 as i32;
            (*builder).ctx = xkb_context_ref(context);
            (*builder).rules = strdup_safe(rules);
            if !((*builder).rules.is_null() && !rules.is_null()) {
                (*builder).model = strdup_safe(model);
                if !((*builder).model.is_null() && !model.is_null()) {
                    (*builder).layouts.item = ::core::ptr::null_mut::<xkb_rmlvo_builder_layout>();
                    (*builder).layouts.size = 0 as darray_size_t;
                    (*builder).layouts.alloc = 0 as darray_size_t;
                    (*builder).options.item = ::core::ptr::null_mut::<xkb_rmlvo_builder_option>();
                    (*builder).options.size = 0 as darray_size_t;
                    (*builder).options.alloc = 0 as darray_size_t;
                    return builder;
                }
            }
        }
        xkb_logf!(
            context,
            XKB_LOG_LEVEL_ERROR,
            XKB_LOG_VERBOSITY_MINIMAL as i32,
            "[XKB-{:03}] Cannot allocate a RMLVO builder.\n",
            XKB_ERROR_ALLOCATION_ERROR as i32,
        );
        xkb_rmlvo_builder_unref(builder);
        return ::core::ptr::null_mut::<xkb_rmlvo_builder>();
    }
}
pub unsafe fn xkb_rmlvo_builder_append_layout(
    mut rmlvo: *mut xkb_rmlvo_builder,
    mut layout: *const i8,
    mut variant: *const i8,
    mut options: *const *const i8,
    mut options_len: usize,
) -> bool {
    unsafe {
        let idx: xkb_layout_index_t = (*rmlvo).layouts.size as xkb_layout_index_t;
        if idx >= XKB_MAX_GROUPS as xkb_layout_index_t {
            xkb_logf!(
                (*rmlvo).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Maximum layout count reached: {}; cannot add layout \"{}({})\" to the RMLVO builder.\n",
                XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX as i32,
                32 as i32,
                crate::xkb::utils::CStrDisplay(layout),
                crate::xkb::utils::CStrDisplay(if !variant.is_null() {
                    variant
                } else {
                    b"\0".as_ptr() as *const i8
                }),
            );
            return false;
        }
        let new: xkb_rmlvo_builder_layout = xkb_rmlvo_builder_layout {
            layout: strdup_safe(layout),
            variant: strdup_safe(variant),
        };
        if new.layout.is_null() || new.variant.is_null() && !variant.is_null() {
            free(new.layout as *mut ::core::ffi::c_void);
            free(new.variant as *mut ::core::ffi::c_void);
            xkb_logf!(
                (*rmlvo).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot allocate layout \"{}({})\" to the RMLVO builder.\n",
                XKB_ERROR_ALLOCATION_ERROR as i32,
                crate::xkb::utils::CStrDisplay(layout),
                crate::xkb::utils::CStrDisplay(if !variant.is_null() {
                    variant
                } else {
                    b"\0".as_ptr() as *const i8
                }),
            );
            return false;
        }
        darray_append(
            &mut (*rmlvo).layouts.item,
            &mut (*rmlvo).layouts.size,
            &mut (*rmlvo).layouts.alloc,
            new,
        );
        if options.is_null() {
            options_len = 0 as usize;
        }
        let mut k: usize = 0 as usize;
        while k < options_len {
            let option: xkb_rmlvo_builder_option = xkb_rmlvo_builder_option {
                option: strdup_safe(*options.offset(k as isize)),
                layout: idx,
            };
            if option.option.is_null() {
                xkb_logf!(
                    (*rmlvo).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot allocate option \"{}\" of layout \"{}({})\" to the RMLVO builder.\n",
                    XKB_ERROR_ALLOCATION_ERROR as i32,
                    crate::xkb::utils::CStrDisplay(*options.offset(k as isize)),
                    crate::xkb::utils::CStrDisplay(layout),
                    crate::xkb::utils::CStrDisplay(if !variant.is_null() {
                        variant
                    } else {
                        b"\0".as_ptr() as *const i8
                    }),
                );
                return false;
            }
            darray_append(
                &mut (*rmlvo).options.item,
                &mut (*rmlvo).options.size,
                &mut (*rmlvo).options.alloc,
                option,
            );
            k = k.wrapping_add(1);
        }
        return true;
    }
}
pub unsafe fn xkb_rmlvo_builder_append_option(
    mut rmlvo: *mut xkb_rmlvo_builder,
    mut option: *const i8,
) -> bool {
    unsafe {
        if option.is_null() {
            return false;
        }
        let mut prev: *const xkb_rmlvo_builder_option =
            ::core::ptr::null::<xkb_rmlvo_builder_option>();
        if !(*rmlvo).options.item.is_null() {
            prev = (*rmlvo).options.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_option;
            while prev
                < (*rmlvo).options.item.offset((*rmlvo).options.size as isize)
                    as *mut xkb_rmlvo_builder_option
                    as *const xkb_rmlvo_builder_option
            {
                if (*prev).layout == XKB_LAYOUT_INVALID as xkb_layout_index_t
                    && cstr_cmp((*prev).option, option) == 0 as i32
                {
                    return true;
                }
                prev = prev.offset(1);
            }
        }
        let new: xkb_rmlvo_builder_option = xkb_rmlvo_builder_option {
            option: strdup_safe(option),
            layout: XKB_LAYOUT_INVALID as xkb_layout_index_t,
        };
        if new.option.is_null() {
            xkb_logf!(
                (*rmlvo).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot allocate option \"{}\" to the RMLVO builder.\n",
                XKB_ERROR_ALLOCATION_ERROR as i32,
                crate::xkb::utils::CStrDisplay(option),
            );
            return false;
        }
        darray_append(
            &mut (*rmlvo).options.item,
            &mut (*rmlvo).options.size,
            &mut (*rmlvo).options.alloc,
            new,
        );
        return true;
    }
}

pub unsafe fn xkb_rmlvo_builder_ref(mut rmlvo: *mut xkb_rmlvo_builder) -> *mut xkb_rmlvo_builder {
    unsafe {
        (*rmlvo).refcnt += 1;
        return rmlvo;
    }
}
pub unsafe fn xkb_rmlvo_builder_unref(mut rmlvo: *mut xkb_rmlvo_builder) {
    unsafe {
        if rmlvo.is_null() || {
            (*rmlvo).refcnt -= 1;
            (*rmlvo).refcnt > 0 as i32
        } {
            return;
        }
        free((*rmlvo).rules as *mut ::core::ffi::c_void);
        free((*rmlvo).model as *mut ::core::ffi::c_void);
        let mut layout: *const xkb_rmlvo_builder_layout =
            ::core::ptr::null::<xkb_rmlvo_builder_layout>();
        if !(*rmlvo).layouts.item.is_null() {
            layout =
                (*rmlvo).layouts.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_layout;
            while layout
                < (*rmlvo).layouts.item.offset((*rmlvo).layouts.size as isize)
                    as *mut xkb_rmlvo_builder_layout
                    as *const xkb_rmlvo_builder_layout
            {
                free((*layout).layout as *mut ::core::ffi::c_void);
                free((*layout).variant as *mut ::core::ffi::c_void);
                layout = layout.offset(1);
            }
        }
        darray_free(
            &mut (*rmlvo).layouts.item,
            &mut (*rmlvo).layouts.size,
            &mut (*rmlvo).layouts.alloc,
        );
        let mut option: *const xkb_rmlvo_builder_option =
            ::core::ptr::null::<xkb_rmlvo_builder_option>();
        if !(*rmlvo).options.item.is_null() {
            option =
                (*rmlvo).options.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_option;
            while option
                < (*rmlvo).options.item.offset((*rmlvo).options.size as isize)
                    as *mut xkb_rmlvo_builder_option
                    as *const xkb_rmlvo_builder_option
            {
                free((*option).option as *mut ::core::ffi::c_void);
                option = option.offset(1);
            }
        }
        darray_free(
            &mut (*rmlvo).options.item,
            &mut (*rmlvo).options.size,
            &mut (*rmlvo).options.alloc,
        );
        xkb_context_unref((*rmlvo).ctx);
        free(rmlvo as *mut ::core::ffi::c_void);
    }
}
pub unsafe fn xkb_rmlvo_builder_to_rules_names(
    mut builder: *const xkb_rmlvo_builder,
    mut rmlvo: *mut xkb_rule_names,
    mut buf: *mut i8,
    mut buf_size: usize,
) -> bool {
    unsafe {
        (*rmlvo).rules = (*builder).rules;
        (*rmlvo).model = (*builder).model;
        let mut start: *mut i8 = buf;
        (*rmlvo).layout = start;
        let mut k: darray_size_t = 0;
        let mut layout: *const xkb_rmlvo_builder_layout =
            ::core::ptr::null::<xkb_rmlvo_builder_layout>();
        if !(*builder).layouts.item.is_null() {
            k = 0 as darray_size_t;
            layout =
                (*builder).layouts.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_layout;
            while k < (*builder).layouts.size {
                let (count, trunc) = crate::xkb::utils::snprintf_args(
                    start,
                    buf_size,
                    format_args!(
                        "{}{}",
                        if k > 0 as darray_size_t { "," } else { "" },
                        crate::xkb::utils::CStrDisplay((*layout).layout),
                    ),
                );
                if trunc || (count == 0 && k > 0) {
                    return false;
                }
                buf_size = buf_size.wrapping_sub(count);
                start = start.offset(count as isize);
                k = k.wrapping_add(1);
                layout = layout.offset(1);
            }
        }
        if buf_size <= 1 as usize {
            return false;
        }
        *start = '\0' as i32 as i8;
        start = start.offset(1);
        buf_size = buf_size.wrapping_sub(1);
        (*rmlvo).variant = start;
        if !(*builder).layouts.item.is_null() {
            k = 0 as darray_size_t;
            layout =
                (*builder).layouts.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_layout;
            while k < (*builder).layouts.size {
                let (count_0, trunc) = crate::xkb::utils::snprintf_args(
                    start,
                    buf_size,
                    format_args!(
                        "{}{}",
                        if k > 0 as darray_size_t { "," } else { "" },
                        crate::xkb::utils::CStrDisplay(if !(*layout).variant.is_null() {
                            (*layout).variant as *const i8
                        } else {
                            b"\0".as_ptr() as *const i8
                        }),
                    ),
                );
                if trunc || (count_0 == 0 && k > 0) {
                    return false;
                }
                buf_size = buf_size.wrapping_sub(count_0);
                start = start.offset(count_0 as isize);
                k = k.wrapping_add(1);
                layout = layout.offset(1);
            }
        }
        if buf_size <= 1 as usize {
            return false;
        }
        *start = '\0' as i32 as i8;
        start = start.offset(1);
        buf_size = buf_size.wrapping_sub(1);
        (*rmlvo).options = start;
        let mut option: *const xkb_rmlvo_builder_option =
            ::core::ptr::null::<xkb_rmlvo_builder_option>();
        if !(*builder).options.item.is_null() {
            k = 0 as darray_size_t;
            option =
                (*builder).options.item.offset(0 as i32 as isize) as *mut xkb_rmlvo_builder_option;
            while k < (*builder).options.size {
                let mut count_1: i32 = {
                    let (written, trunc) = crate::xkb::utils::snprintf_args(
                        start,
                        buf_size,
                        format_args!(
                            "{}{}",
                            if k > 0 as darray_size_t { "," } else { "" },
                            crate::xkb::utils::CStrDisplay((*option).option),
                        ),
                    );
                    if trunc {
                        -1
                    } else {
                        written as i32
                    }
                };
                if count_1 < 0 as i32 || count_1 as usize >= buf_size {
                    return false;
                }
                buf_size = buf_size.wrapping_sub(count_1 as usize);
                start = start.offset(count_1 as isize);
                if (*option).layout != XKB_LAYOUT_INVALID as xkb_layout_index_t {
                    let (written, trunc) = crate::xkb::utils::snprintf_args(
                        start,
                        buf_size,
                        format_args!(
                            "{}{}",
                            (OPTIONS_GROUP_SPECIFIER_PREFIX as u8 as char),
                            (*option).layout
                        ),
                    );
                    count_1 = if trunc { -1 } else { written as i32 };
                    if count_1 < 0 as i32 || count_1 as usize >= buf_size {
                        return false;
                    }
                    buf_size = buf_size.wrapping_sub(count_1 as usize);
                    start = start.offset(count_1 as isize);
                }
                k = k.wrapping_add(1);
                option = option.offset(1);
            }
        }
        if buf_size == 0 as usize {
            return false;
        }
        *start = '\0' as i32 as i8;
        return true;
    }
}
