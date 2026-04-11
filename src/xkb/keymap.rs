pub mod internal {
    pub use crate::xkb::shared_types::__va_list_tag;
}
#[c2rust::header_src = "/usr/include/bits/types.h:17"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = i8;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = i16;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = i64;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = i64;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:17"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type i8 = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type i16 = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type i32 = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:17"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:19"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:1"]
    pub struct _IO_FILE {
        pub _flags: i32,
        pub _IO_read_ptr: *mut i8,
        pub _IO_read_end: *mut i8,
        pub _IO_read_base: *mut i8,
        pub _IO_write_base: *mut i8,
        pub _IO_write_ptr: *mut i8,
        pub _IO_write_end: *mut i8,
        pub _IO_buf_base: *mut i8,
        pub _IO_buf_end: *mut i8,
        pub _IO_save_base: *mut i8,
        pub _IO_backup_base: *mut i8,
        pub _IO_save_end: *mut i8,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: i32,
        #[bitfield(name = "_flags2", ty = "i32", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [i8; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [i8; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: i32,
        pub _unused3: i32,
        pub _total_written: __uint64_t,
        pub _unused2: [i8; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:1"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:1"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:19"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod context_h {
    pub use crate::xkb::context_priv::{
        xkb_atom_lookup, xkb_atom_text, xkb_context_sanitize_rule_names,
    };
    pub use crate::xkb::shared_types::{
        atom_table, darray_size_t, xkb_atom_t, xkb_context, xkb_log_level, xkb_rule_names,
        C2Rust_Unnamed, C2Rust_Unnamed_0,
    };
    extern "C" {
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: i32,
            fmt: *const i8,
            ...
        );
    }
}
pub mod atom_h {
    pub use crate::xkb::shared_types::{atom_table, darray_size_t, xkb_atom_t};
    #[c2rust::src_loc = "19:9"]
    pub const XKB_ATOM_NONE: i32 = 0 as i32;
}
pub mod darray_h {
    pub use crate::xkb::shared_types::darray_size_t;
}
pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
        xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE,
        XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_INVALID,
        XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
        XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
        XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID,
        XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED, XKB_STATE_LAYOUT_EFFECTIVE,
        XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
        XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
        XKB_STATE_MODS_LOCKED,
    };
    #[c2rust::src_loc = "1321:1"]
    pub type xkb_keymap_serialize_flags = u32;
    #[c2rust::src_loc = "1548:5"]
    pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
    #[c2rust::src_loc = "1541:5"]
    pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
    #[c2rust::src_loc = "1535:5"]
    pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
    #[c2rust::src_loc = "1529:5"]
    pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
    #[c2rust::src_loc = "1644:1"]
    pub type xkb_keymap_key_iterator_flags = u32;
    #[c2rust::src_loc = "1664:5"]
    pub const XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND: xkb_keymap_key_iterator_flags = 2;
    #[c2rust::src_loc = "1656:5"]
    pub const XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER: xkb_keymap_key_iterator_flags = 1;
    #[c2rust::src_loc = "1650:5"]
    pub const XKB_KEYMAP_KEY_ITERATOR_NO_FLAGS: xkb_keymap_key_iterator_flags = 0;
    #[c2rust::src_loc = "1739:1"]
    pub type xkb_keymap_key_iter_t = Option<
        unsafe extern "C" fn(*mut xkb_keymap, xkb_keycode_t, *mut ::core::ffi::c_void) -> (),
    >;
    #[c2rust::src_loc = "350:9"]
    pub const XKB_KEYCODE_INVALID: u32 = 0xffffffff as u32;
    #[c2rust::src_loc = "358:9"]
    pub const XKB_LED_INVALID: u32 = 0xffffffff as u32;
    #[c2rust::src_loc = "1515:9"]
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    extern "C" {
        #[c2rust::src_loc = "983:1"]
        pub fn xkb_context_unref(context: *mut xkb_context);
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/keymap.h:22"]
pub mod keymap_h {
    use super::rmlvo_h::xkb_rmlvo_builder;
    use super::FILE_h::FILE;
    pub use crate::xkb::shared_types::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "943:1"]
    pub struct xkb_keymap_format_ops {
        pub keymap_new_from_rmlvo:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rmlvo_builder) -> bool>,
        pub keymap_new_from_names:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rule_names) -> bool>,
        pub keymap_new_from_string:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const i8, usize) -> bool>,
        pub keymap_new_from_file: Option<unsafe extern "C" fn(*mut xkb_keymap, *mut FILE) -> bool>,
        pub keymap_get_as_string: Option<
            unsafe extern "C" fn(
                *mut xkb_keymap,
                xkb_keymap_format,
                xkb_keymap_serialize_flags,
            ) -> *mut i8,
        >,
    }
    #[inline]
    #[c2rust::src_loc = "855:1"]
    pub unsafe fn XkbKey(mut keymap: *mut xkb_keymap, mut kc: xkb_keycode_t) -> *const xkb_key {
        unsafe {
            if kc < (*keymap).min_key_code || kc > (*keymap).max_key_code {
                return ::core::ptr::null::<xkb_key>();
            } else if kc < (*keymap).num_keys_low {
                return (*keymap).keys.offset(kc as isize) as *mut xkb_key;
            } else {
                let mut lower: xkb_keycode_t = (*keymap).num_keys_low;
                let mut upper: xkb_keycode_t = (*keymap).num_keys;
                while lower < upper {
                    let mid: xkb_keycode_t = lower.wrapping_add(
                        upper
                            .wrapping_sub(1 as xkb_keycode_t)
                            .wrapping_sub(lower)
                            .wrapping_div(2 as xkb_keycode_t),
                    );
                    let key: *const xkb_key = (*keymap).keys.offset(mid as isize) as *mut xkb_key;
                    if (*key).keycode < kc {
                        lower = mid.wrapping_add(1 as xkb_keycode_t);
                    } else if (*key).keycode > kc {
                        upper = mid;
                    } else {
                        return key;
                    }
                }
                return ::core::ptr::null::<xkb_key>();
            };
        }
    }
    #[inline]
    #[c2rust::src_loc = "896:1"]
    pub unsafe fn entry_is_active(mut entry: *const xkb_key_type_entry) -> bool {
        unsafe {
            return (*entry).mods.mods == 0 as xkb_mod_mask_t
                || (*entry).mods.mask != 0 as xkb_mod_mask_t;
        }
    }
    extern "C" {
        #[c2rust::src_loc = "902:1"]
        #[c2rust::src_loc = "910:1"]
        #[c2rust::src_loc = "923:1"]
        #[c2rust::src_loc = "956:1"]
        #[no_mangle]
        pub static text_v1_keymap_format_ops: xkb_keymap_format_ops;
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/rmlvo.h:22"]
pub mod rmlvo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
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
    #[c2rust::src_loc = "34:9"]
    pub struct xkb_rmlvo_builder_options {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_option,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:1"]
    pub struct xkb_rmlvo_builder_option {
        pub option: *mut i8,
        pub layout: xkb_layout_index_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:9"]
    pub struct xkb_rmlvo_builder_layouts {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_rmlvo_builder_layout,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:1"]
    pub struct xkb_rmlvo_builder_layout {
        pub layout: *mut i8,
        pub variant: *mut i8,
    }
    #[c2rust::src_loc = "14:1"]
    pub type RMLVO = u32;
    #[c2rust::src_loc = "19:5"]
    pub const RMLVO_OPTIONS: RMLVO = 16;
    #[c2rust::src_loc = "18:5"]
    pub const RMLVO_VARIANT: RMLVO = 8;
    #[c2rust::src_loc = "17:5"]
    pub const RMLVO_LAYOUT: RMLVO = 4;
    #[c2rust::src_loc = "16:5"]
    pub const RMLVO_MODEL: RMLVO = 2;
    #[c2rust::src_loc = "15:5"]
    pub const RMLVO_RULES: RMLVO = 1;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::xkbcommon_h::xkb_layout_index_t;
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/messages-codes.h:21"]
pub mod messages_codes_h {
    #[c2rust::src_loc = "43:5"]
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    #[c2rust::src_loc = "154:5"]
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
    #[c2rust::src_loc = "41:1"]
    pub type xkb_log_verbosity = i32;
    #[c2rust::src_loc = "48:5"]
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    #[c2rust::src_loc = "47:5"]
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    #[c2rust::src_loc = "46:5"]
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    #[c2rust::src_loc = "45:5"]
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    #[c2rust::src_loc = "44:5"]
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    #[c2rust::src_loc = "42:5"]
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
    #[c2rust::src_loc = "59:1"]
    pub type xkb_message_code = u32;
    #[c2rust::src_loc = "227:5"]
    pub const _XKB_LOG_MESSAGE_MAX_CODE: xkb_message_code = 971;
    #[c2rust::src_loc = "226:5"]
    pub const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: xkb_message_code = 971;
    #[c2rust::src_loc = "224:5"]
    pub const XKB_ERROR_INVALID_RULES_SYNTAX: xkb_message_code = 967;
    #[c2rust::src_loc = "222:5"]
    pub const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: xkb_message_code = 965;
    #[c2rust::src_loc = "220:5"]
    pub const XKB_ERROR_INVALID_IDENTIFIER: xkb_message_code = 949;
    #[c2rust::src_loc = "218:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_FIELDS: xkb_message_code = 935;
    #[c2rust::src_loc = "216:5"]
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT_: xkb_message_code = 914;
    #[c2rust::src_loc = "214:5"]
    pub const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: xkb_message_code = 903;
    #[c2rust::src_loc = "212:5"]
    pub const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: xkb_message_code = 901;
    #[c2rust::src_loc = "210:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: xkb_message_code = 893;
    #[c2rust::src_loc = "208:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_ACTION: xkb_message_code = 883;
    #[c2rust::src_loc = "206:5"]
    pub const XKB_ERROR_ABI_FORWARD_COMPAT_: xkb_message_code = 876;
    #[c2rust::src_loc = "204:5"]
    pub const XKB_ERROR_UNKNOWN_ACTION_TYPE: xkb_message_code = 844;
    #[c2rust::src_loc = "202:5"]
    pub const XKB_ERROR_KEYMAP_COMPILATION_FAILED: xkb_message_code = 822;
    #[c2rust::src_loc = "200:5"]
    pub const XKB_ERROR_UNKNOWN_FIELD: xkb_message_code = 812;
    #[c2rust::src_loc = "198:5"]
    pub const XKB_WARNING_CONFLICTING_MODMAP: xkb_message_code = 800;
    #[c2rust::src_loc = "196:5"]
    pub const XKB_ERROR_INVALID_VALUE: xkb_message_code = 796;
    #[c2rust::src_loc = "194:5"]
    pub const XKB_ERROR_INVALID_EXPRESSION_TYPE: xkb_message_code = 784;
    #[c2rust::src_loc = "192:5"]
    pub const XKB_WARNING_UNDEFINED_KEYCODE: xkb_message_code = 770;
    #[c2rust::src_loc = "190:5"]
    pub const XKB_ERROR_INVALID_XKB_SYNTAX: xkb_message_code = 769;
    #[c2rust::src_loc = "188:5"]
    pub const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: xkb_message_code = 762;
    #[c2rust::src_loc = "186:5"]
    pub const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: xkb_message_code = 742;
    #[c2rust::src_loc = "184:5"]
    pub const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: xkb_message_code = 711;
    #[c2rust::src_loc = "182:5"]
    pub const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: xkb_message_code = 700;
    #[c2rust::src_loc = "180:5"]
    pub const XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT: xkb_message_code = 693;
    #[c2rust::src_loc = "178:5"]
    pub const XKB_ERROR_INVALID_COMPOSE_SYNTAX: xkb_message_code = 685;
    #[c2rust::src_loc = "176:5"]
    pub const XKB_ERROR_INVALID_COMPOSE_LOCALE: xkb_message_code = 679;
    #[c2rust::src_loc = "174:5"]
    pub const XKB_ERROR_INVALID_INCLUDED_FILE: xkb_message_code = 661;
    #[c2rust::src_loc = "172:5"]
    pub const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: xkb_message_code = 645;
    #[c2rust::src_loc = "170:5"]
    pub const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: xkb_message_code = 639;
    #[c2rust::src_loc = "168:5"]
    pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: xkb_message_code = 632;
    #[c2rust::src_loc = "166:5"]
    pub const XKB_ERROR_INVALID_REAL_MODIFIER: xkb_message_code = 623;
    #[c2rust::src_loc = "164:5"]
    pub const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: xkb_message_code = 607;
    #[c2rust::src_loc = "162:5"]
    pub const XKB_ERROR_CANNOT_RESOLVE_RMLVO: xkb_message_code = 595;
    #[c2rust::src_loc = "160:5"]
    pub const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: xkb_message_code = 588;
    #[c2rust::src_loc = "158:5"]
    pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
    #[c2rust::src_loc = "156:5"]
    pub const XKB_ERROR_INVALID_ACTION_FIELD: xkb_message_code = 563;
    #[c2rust::src_loc = "152:5"]
    pub const XKB_ERROR_INVALID_FILE_ENCODING: xkb_message_code = 542;
    #[c2rust::src_loc = "150:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_NAME: xkb_message_code = 523;
    #[c2rust::src_loc = "148:5"]
    pub const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: xkb_message_code = 516;
    #[c2rust::src_loc = "146:5"]
    pub const XKB_WARNING_NUMERIC_KEYSYM: xkb_message_code = 489;
    #[c2rust::src_loc = "144:5"]
    pub const XKB_ERROR_INVALID_OPERATION: xkb_message_code = 478;
    #[c2rust::src_loc = "142:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_SYMBOL: xkb_message_code = 461;
    #[c2rust::src_loc = "140:5"]
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE_: xkb_message_code = 450;
    #[c2rust::src_loc = "138:5"]
    pub const XKB_WARNING_MISSING_DEFAULT_SECTION: xkb_message_code = 433;
    #[c2rust::src_loc = "136:5"]
    pub const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: xkb_message_code = 428;
    #[c2rust::src_loc = "134:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: xkb_message_code = 407;
    #[c2rust::src_loc = "132:5"]
    pub const XKB_ERROR_RECURSIVE_INCLUDE: xkb_message_code = 386;
    #[c2rust::src_loc = "130:5"]
    pub const XKB_WARNING_DUPLICATE_ENTRY: xkb_message_code = 378;
    #[c2rust::src_loc = "128:5"]
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_: xkb_message_code = 371;
    #[c2rust::src_loc = "126:5"]
    pub const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: xkb_message_code = 362;
    #[c2rust::src_loc = "124:5"]
    pub const XKB_ERROR_OVERLAPPING_OVERLAY: xkb_message_code = 355;
    #[c2rust::src_loc = "122:5"]
    pub const XKB_ERROR_UNKNOWN_OPERATOR: xkb_message_code = 345;
    #[c2rust::src_loc = "120:5"]
    pub const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: xkb_message_code = 338;
    #[c2rust::src_loc = "118:5"]
    pub const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: xkb_message_code = 312;
    #[c2rust::src_loc = "116:5"]
    pub const XKB_WARNING_NON_BASE_GROUP_NAME: xkb_message_code = 305;
    #[c2rust::src_loc = "114:5"]
    pub const XKB_WARNING_DEPRECATED_KEYSYM_NAME: xkb_message_code = 302;
    #[c2rust::src_loc = "112:5"]
    pub const XKB_WARNING_DEPRECATED_KEYSYM: xkb_message_code = 301;
    #[c2rust::src_loc = "110:5"]
    pub const XKB_WARNING_UNDEFINED_KEY_TYPE: xkb_message_code = 286;
    #[c2rust::src_loc = "108:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: xkb_message_code = 266;
    #[c2rust::src_loc = "106:5"]
    pub const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: xkb_message_code = 254;
    #[c2rust::src_loc = "104:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: xkb_message_code = 239;
    #[c2rust::src_loc = "102:5"]
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: xkb_message_code = 237;
    #[c2rust::src_loc = "100:5"]
    pub const XKB_ERROR_UNKNOWN_STATEMENT: xkb_message_code = 222;
    #[c2rust::src_loc = "98:5"]
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_: xkb_message_code = 214;
    #[c2rust::src_loc = "96:5"]
    pub const XKB_ERROR_INVALID_MODMAP_ENTRY: xkb_message_code = 206;
    #[c2rust::src_loc = "94:5"]
    pub const XKB_ERROR_INVALID_INCLUDE_STATEMENT: xkb_message_code = 203;
    #[c2rust::src_loc = "92:5"]
    pub const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: xkb_message_code = 195;
    #[c2rust::src_loc = "90:5"]
    pub const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: xkb_message_code = 193;
    #[c2rust::src_loc = "88:5"]
    pub const XKB_WARNING_CANNOT_INFER_KEY_TYPE: xkb_message_code = 183;
    #[c2rust::src_loc = "86:5"]
    pub const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: xkb_message_code = 172;
    #[c2rust::src_loc = "84:5"]
    pub const XKB_ERROR_INVALID_PATH: xkb_message_code = 161;
    #[c2rust::src_loc = "82:5"]
    pub const XKB_ERROR_WRONG_STATEMENT_TYPE: xkb_message_code = 150;
    #[c2rust::src_loc = "80:5"]
    pub const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: xkb_message_code = 134;
    #[c2rust::src_loc = "78:5"]
    pub const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: xkb_message_code = 123;
    #[c2rust::src_loc = "76:5"]
    pub const XKB_WARNING_UNRECOGNIZED_KEYSYM: xkb_message_code = 107;
    #[c2rust::src_loc = "74:5"]
    pub const XKB_WARNING_ILLEGAL_KEYCODE_ALIAS: xkb_message_code = 101;
    #[c2rust::src_loc = "72:5"]
    pub const XKB_ERROR_INVALID_NUMERIC_KEYSYM: xkb_message_code = 82;
    #[c2rust::src_loc = "70:5"]
    pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
    #[c2rust::src_loc = "68:5"]
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: xkb_message_code = 60;
    #[c2rust::src_loc = "66:5"]
    pub const XKB_ERROR_INTEGER_OVERFLOW: xkb_message_code = 52;
    #[c2rust::src_loc = "64:5"]
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: xkb_message_code = 43;
    #[c2rust::src_loc = "62:5"]
    pub const XKB_ERROR_MALFORMED_NUMBER_LITERAL: xkb_message_code = 34;
    #[c2rust::src_loc = "60:5"]
    pub const _XKB_LOG_MESSAGE_MIN_CODE: xkb_message_code = 34;
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/features/enums.h:21"]
pub mod enums_h {
    #[c2rust::src_loc = "98:5"]
    pub const XKB_KEYMAP_SERIALIZE_FLAGS_VALUES: xkb_enumerations_values = 7;
    #[c2rust::src_loc = "76:1"]
    pub type xkb_enumerations_values = u32;
    #[c2rust::src_loc = "181:5"]
    pub const XKB_COMPOSE_FEED_RESULT_VALUES: xkb_enumerations_values = 3;
    #[c2rust::src_loc = "175:5"]
    pub const XKB_COMPOSE_STATUS_VALUES: xkb_enumerations_values = 15;
    #[c2rust::src_loc = "172:5"]
    pub const XKB_COMPOSE_STATE_FLAGS_VALUES: xkb_enumerations_values = 0;
    #[c2rust::src_loc = "169:5"]
    pub const XKB_COMPOSE_FORMAT_VALUES: xkb_enumerations_values = 2;
    #[c2rust::src_loc = "166:5"]
    pub const XKB_COMPOSE_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 0;
    #[c2rust::src_loc = "162:5"]
    pub const XKB_CONSUMED_MODE_VALUES: xkb_enumerations_values = 3;
    #[c2rust::src_loc = "157:5"]
    pub const XKB_STATE_MATCH_VALUES: xkb_enumerations_values = 65539;
    #[c2rust::src_loc = "152:5"]
    pub const XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES: xkb_enumerations_values = 7;
    #[c2rust::src_loc = "147:5"]
    pub const XKB_KEY_DIRECTION_VALUES: xkb_enumerations_values = 7;
    #[c2rust::src_loc = "142:5"]
    pub const XKB_A11Y_FLAGS_VALUES: xkb_enumerations_values = 3;
    #[c2rust::src_loc = "139:5"]
    pub const XKB_EVENTS_FLAGS_VALUES: xkb_enumerations_values = 0;
    #[c2rust::src_loc = "127:5"]
    pub const XKB_KEYBOARD_CONTROL_FLAGS_VALUES: xkb_enumerations_values = 511;
    #[c2rust::src_loc = "115:5"]
    pub const XKB_STATE_COMPONENT_VALUES: xkb_enumerations_values = 1023;
    #[c2rust::src_loc = "109:5"]
    pub const XKB_EVENT_TYPE_VALUES: xkb_enumerations_values = 30;
    #[c2rust::src_loc = "104:5"]
    pub const XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES: xkb_enumerations_values = 3;
    #[c2rust::src_loc = "94:5"]
    pub const XKB_KEYMAP_FORMAT_VALUES: xkb_enumerations_values = 6;
    #[c2rust::src_loc = "90:5"]
    pub const XKB_KEYMAP_COMPILE_FLAGS_VALUES: xkb_enumerations_values = 1;
    #[c2rust::src_loc = "84:5"]
    pub const XKB_CONTEXT_FLAGS_VALUES: xkb_enumerations_values = 7;
    #[c2rust::src_loc = "80:5"]
    pub const XKB_KEYSYM_FLAGS_VALUES: xkb_enumerations_values = 1;
    #[c2rust::src_loc = "77:5"]
    pub const XKB_RMLVO_BUILDER_FLAGS_VALUES: xkb_enumerations_values = 0;
}
#[c2rust::header_src = "/usr/include/string.h:20"]
pub mod string_h {

    extern "C" {
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const i8) -> usize;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:20"]
pub mod stdlib_h {

    extern "C" {
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:17"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/stdint.h:17"]
pub mod stdint_h {
    #[c2rust::src_loc = "112:10"]
    pub const INT32_MAX: i32 = 2147483647 as i32;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/stdbool.h:22"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: i32 = 1 as i32;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: i32 = 0 as i32;
}
pub use self::__stddef_null_h::NULL;

pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{
    xkb_atom_lookup, xkb_atom_text, xkb_context, xkb_context_sanitize_rule_names, xkb_log,
    C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::darray_size_t;
pub use self::enums_h::{
    xkb_enumerations_values, XKB_A11Y_FLAGS_VALUES, XKB_COMPOSE_COMPILE_FLAGS_VALUES,
    XKB_COMPOSE_FEED_RESULT_VALUES, XKB_COMPOSE_FORMAT_VALUES, XKB_COMPOSE_STATE_FLAGS_VALUES,
    XKB_COMPOSE_STATUS_VALUES, XKB_CONSUMED_MODE_VALUES, XKB_CONTEXT_FLAGS_VALUES,
    XKB_EVENTS_FLAGS_VALUES, XKB_EVENT_TYPE_VALUES, XKB_KEYBOARD_CONTROL_FLAGS_VALUES,
    XKB_KEYMAP_COMPILE_FLAGS_VALUES, XKB_KEYMAP_FORMAT_VALUES,
    XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES, XKB_KEYMAP_SERIALIZE_FLAGS_VALUES,
    XKB_KEYSYM_FLAGS_VALUES, XKB_KEY_DIRECTION_VALUES, XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES,
    XKB_RMLVO_BUILDER_FLAGS_VALUES, XKB_STATE_COMPONENT_VALUES, XKB_STATE_MATCH_VALUES,
};
pub use self::internal::__va_list_tag;
pub use self::keymap_h::{
    entry_is_active, mod_type, text_v1_keymap_format_ops, xkb_action, xkb_action_controls,
    xkb_action_count_t, xkb_action_flags, xkb_action_type, xkb_controls_action,
    xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action,
    xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type, xkb_key_type_entry,
    xkb_keymap, xkb_keymap_format_ops, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t, xkb_pointer_action,
    xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbKey, XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES,
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
};
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
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options, RMLVO, RMLVO_LAYOUT, RMLVO_MODEL,
    RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::INT32_MAX;
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdlib_h::{calloc, free};
use self::string_h::strlen;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{
    __int16_t, __int32_t, __int8_t, __off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use self::xkbcommon_h::{
    xkb_context_unref, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keymap_key_iter_t, xkb_keymap_key_iterator_flags, xkb_keymap_serialize_flags, xkb_keysym_t,
    xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
    xkb_level_index_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
    xkb_state_component, XKB_KEYCODE_INVALID, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER, XKB_KEYMAP_KEY_ITERATOR_NO_FLAGS,
    XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND, XKB_KEYMAP_SERIALIZE_EXPLICIT,
    XKB_KEYMAP_SERIALIZE_KEEP_UNUSED, XKB_KEYMAP_SERIALIZE_NO_FLAGS, XKB_KEYMAP_SERIALIZE_PRETTY,
    XKB_KEYMAP_USE_ORIGINAL_FORMAT, XKB_LAYOUT_INVALID, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LED_INVALID,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::FILE_h::FILE;
pub use crate::xkb::keymap_priv::{xkb_keymap_new, XkbModNameToIndex, XkbWrapGroupIntoRange};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "591:1"]
pub struct xkb_keymap_key_iterator {
    pub increment: i8,
    pub skip_unbound: bool,
    pub min: *const xkb_key,
    pub max: *const xkb_key,
    pub next: *const xkb_key,
    pub keymap: *mut xkb_keymap,
}
#[no_mangle]
#[c2rust::src_loc = "26:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_ref(mut keymap: *mut xkb_keymap) -> *mut xkb_keymap {
    unsafe {
        (*keymap).refcnt += 1;
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "34:1"]
#[no_mangle]
pub unsafe fn clear_level(mut leveli: *mut xkb_level) {
    unsafe {
        if (*leveli).num_syms as i32 > 1 as i32 {
            free((*leveli).s.syms as *mut ::core::ffi::c_void);
        }
        if (*leveli).num_actions as i32 > 1 as i32 {
            free((*leveli).a.actions as *mut ::core::ffi::c_void);
        }
    }
}
#[c2rust::src_loc = "43:1"]
unsafe fn clear_interpret(mut interp: *mut xkb_sym_interpret) {
    unsafe {
        if (*interp).num_actions as i32 > 1 as i32 {
            free((*interp).a.actions as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_unref(mut keymap: *mut xkb_keymap) {
    unsafe {
        if keymap.is_null() || {
            (*keymap).refcnt -= 1;
            (*keymap).refcnt > 0 as i32
        } {
            return;
        }
        if !(*keymap).keys.is_null() {
            let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
            key = (*keymap).keys.offset(
                (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                    0 as xkb_keycode_t
                } else {
                    (*keymap).min_key_code
                }) as isize,
            );
            while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                if !(*key).groups.is_null() {
                    let mut i: xkb_layout_index_t = 0 as xkb_layout_index_t;
                    while i < (*key).num_groups() {
                        if !(*(*key).groups.offset(i as isize)).levels.is_null() {
                            let mut j: xkb_level_index_t = 0 as xkb_level_index_t;
                            while j < XkbKeyNumLevels(key, i) {
                                clear_level(
                                    (*(*key).groups.offset(i as isize))
                                        .levels
                                        .offset(j as isize)
                                        as *mut xkb_level,
                                );
                                j = j.wrapping_add(1);
                            }
                            free(
                                (*(*key).groups.offset(i as isize)).levels
                                    as *mut ::core::ffi::c_void,
                            );
                        }
                        i = i.wrapping_add(1);
                    }
                    free((*key).groups as *mut ::core::ffi::c_void);
                }
                if !(*key).overlays_inline() && !(*key).c2rust_unnamed.overlays_keys.is_null() {
                    free((*key).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
                }
                key = key.offset(1);
            }
            free((*keymap).keys as *mut ::core::ffi::c_void);
        }
        if !(*keymap).types.is_null() {
            let mut i_0: darray_size_t = 0 as darray_size_t;
            while i_0 < (*keymap).num_types {
                free((*(*keymap).types.offset(i_0 as isize)).entries as *mut ::core::ffi::c_void);
                free(
                    (*(*keymap).types.offset(i_0 as isize)).level_names as *mut ::core::ffi::c_void,
                );
                i_0 = i_0.wrapping_add(1);
            }
            free((*keymap).types as *mut ::core::ffi::c_void);
        }
        let mut k: darray_size_t = 0 as darray_size_t;
        while k < (*keymap).num_sym_interprets {
            clear_interpret((*keymap).sym_interprets.offset(k as isize) as *mut xkb_sym_interpret);
            k = k.wrapping_add(1);
        }
        free((*keymap).sym_interprets as *mut ::core::ffi::c_void);
        free((*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases as *mut ::core::ffi::c_void);
        free((*keymap).group_names as *mut ::core::ffi::c_void);
        free((*keymap).keycodes_section_name as *mut ::core::ffi::c_void);
        free((*keymap).symbols_section_name as *mut ::core::ffi::c_void);
        free((*keymap).types_section_name as *mut ::core::ffi::c_void);
        free((*keymap).compat_section_name as *mut ::core::ffi::c_void);
        xkb_context_unref((*keymap).ctx);
        free(keymap as *mut ::core::ffi::c_void);
    }
}
#[c2rust::src_loc = "99:1"]
unsafe fn get_keymap_format_ops(mut format: xkb_keymap_format) -> *const xkb_keymap_format_ops {
    unsafe {
        static mut keymap_format_ops: [*const xkb_keymap_format_ops; 3] = unsafe {
            [
                ::core::ptr::null::<xkb_keymap_format_ops>(),
                &raw const text_v1_keymap_format_ops,
                &raw const text_v1_keymap_format_ops,
            ]
        };
        if (format as i32) < 0 as i32
            || format as i32
                >= (::core::mem::size_of::<[*const xkb_keymap_format_ops; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const xkb_keymap_format_ops>() as usize)
                    as i32
        {
            return ::core::ptr::null::<xkb_keymap_format_ops>();
        }
        return keymap_format_ops[format as i32 as usize];
    }
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_new_from_rmlvo(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_rmlvo.is_none() {
            xkb_log(
                (*rmlvo).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const i8,
                format as u32,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            (*rmlvo).ctx,
            b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if !(*ops)
            .keymap_new_from_rmlvo
            .expect("non-null function pointer")(keymap, rmlvo)
        {
            xkb_keymap_unref(keymap);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_new_from_names2(
    mut ctx: *mut xkb_context,
    mut rmlvo_in: *const xkb_rule_names,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_names.is_none() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_names2\0".as_ptr() as *const i8,
                format as u32,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_names2\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: ::core::ptr::null::<i8>(),
            model: ::core::ptr::null::<i8>(),
            layout: ::core::ptr::null::<i8>(),
            variant: ::core::ptr::null::<i8>(),
            options: ::core::ptr::null::<i8>(),
        };
        if !rmlvo_in.is_null() {
            rmlvo = *rmlvo_in;
        }
        xkb_context_sanitize_rule_names(ctx, &raw mut rmlvo);
        if !(*ops)
            .keymap_new_from_names
            .expect("non-null function pointer")(keymap, &raw mut rmlvo)
        {
            xkb_keymap_unref(keymap);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe fn xkb_keymap_new_from_names(
    mut ctx: *mut xkb_context,
    mut rmlvo_in: *const xkb_rule_names,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        return xkb_keymap_new_from_names2(ctx, rmlvo_in, XKB_KEYMAP_FORMAT_TEXT_V2, flags);
    }
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_new_from_string(
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        return xkb_keymap_new_from_buffer(ctx, string, strlen(string), format, flags);
    }
}
#[no_mangle]
#[c2rust::src_loc = "188:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_new_from_buffer(
    mut ctx: *mut xkb_context,
    mut buffer: *const i8,
    mut length: usize,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_string.is_none() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8,
                format as u32,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if buffer.is_null() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: no buffer specified\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if length > 0 as usize
            && *buffer.offset(length.wrapping_sub(1 as usize) as isize) as i32 == '\0' as i32
        {
            length = length.wrapping_sub(1);
        }
        if !(*ops)
            .keymap_new_from_string
            .expect("non-null function pointer")(keymap, buffer, length)
        {
            xkb_keymap_unref(keymap);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "223:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_new_from_file(
    mut ctx: *mut xkb_context,
    mut file: *mut FILE,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_file.is_none() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_file\0".as_ptr() as *const i8,
                format as u32,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if file.is_null() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: no file specified\n\0".as_ptr() as *const i8,
                b"xkb_keymap_new_from_file\0".as_ptr() as *const i8,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_file\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if !(*ops)
            .keymap_new_from_file
            .expect("non-null function pointer")(keymap, file)
        {
            xkb_keymap_unref(keymap);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "254:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_get_as_string2(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_serialize_flags,
) -> *mut i8 {
    unsafe {
        static mut XKB_KEYMAP_SERIALIZE_FLAGS: xkb_keymap_serialize_flags =
            XKB_KEYMAP_SERIALIZE_FLAGS_VALUES as i32 as xkb_keymap_serialize_flags;
        if flags as u32 & !(XKB_KEYMAP_SERIALIZE_FLAGS as u32) != 0 {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unrecognized serialization flags: %#x\n\0".as_ptr() as *const i8,
                b"xkb_keymap_get_as_string2\0".as_ptr() as *const i8,
                flags as u32 & !(XKB_KEYMAP_SERIALIZE_FLAGS as u32),
            );
            return ::core::ptr::null_mut::<i8>();
        }
        if format as u32 == XKB_KEYMAP_USE_ORIGINAL_FORMAT as u32 {
            format = (*keymap).format;
        }
        let ops: *const xkb_keymap_format_ops =
            get_keymap_format_ops(format) as *const xkb_keymap_format_ops;
        if ops.is_null() || (*ops).keymap_get_as_string.is_none() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const i8,
                b"xkb_keymap_get_as_string2\0".as_ptr() as *const i8,
                format as u32,
            );
            return ::core::ptr::null_mut::<i8>();
        }
        return (*ops)
            .keymap_get_as_string
            .expect("non-null function pointer")(keymap, format, flags);
    }
}
#[no_mangle]
#[c2rust::src_loc = "283:1"]
pub unsafe fn xkb_keymap_get_as_string(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
) -> *mut i8 {
    unsafe {
        return xkb_keymap_get_as_string2(keymap, format, XKB_KEYMAP_SERIALIZE_NO_FLAGS);
    }
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_num_mods(mut keymap: *mut xkb_keymap) -> xkb_mod_index_t {
    unsafe {
        return (*keymap).mods.num_mods;
    }
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe fn xkb_keymap_mod_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_mod_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).mods.num_mods {
            return ::core::ptr::null::<i8>();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).mods.mods[idx as usize].name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "315:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_mod_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_mod_index_t {
    unsafe {
        let atom: xkb_atom_t = xkb_atom_lookup((*keymap).ctx, name) as xkb_atom_t;
        return if atom == XKB_ATOM_NONE as xkb_atom_t {
            XKB_MOD_INVALID as xkb_mod_index_t
        } else {
            XkbModNameToIndex(&raw mut (*keymap).mods, atom, MOD_BOTH)
        };
    }
}
#[no_mangle]
#[c2rust::src_loc = "327:1"]
pub unsafe fn xkb_keymap_mod_get_mask(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_mod_mask_t {
    unsafe {
        let idx: xkb_mod_index_t = xkb_keymap_mod_get_index(keymap, name) as xkb_mod_index_t;
        return if idx >= (*keymap).mods.num_mods {
            0 as xkb_mod_mask_t
        } else {
            (*keymap).mods.mods[idx as usize].mapping
        };
    }
}
#[no_mangle]
#[c2rust::src_loc = "339:1"]
pub unsafe fn xkb_keymap_mod_get_mask2(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_mod_index_t,
) -> xkb_mod_mask_t {
    unsafe {
        return if idx >= (*keymap).mods.num_mods {
            0 as xkb_mod_mask_t
        } else {
            (*keymap).mods.mods[idx as usize].mapping
        };
    }
}
#[no_mangle]
#[c2rust::src_loc = "350:1"]
pub unsafe fn xkb_keymap_num_layouts(mut keymap: *mut xkb_keymap) -> xkb_layout_index_t {
    unsafe {
        return (*keymap).num_groups;
    }
}
#[no_mangle]
#[c2rust::src_loc = "359:1"]
pub unsafe fn xkb_keymap_layout_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_layout_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_group_names {
            return ::core::ptr::null::<i8>();
        }
        return xkb_atom_text((*keymap).ctx, *(*keymap).group_names.offset(idx as isize));
    }
}
#[no_mangle]
#[c2rust::src_loc = "371:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_layout_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_layout_index_t {
    unsafe {
        let mut atom: xkb_atom_t = xkb_atom_lookup((*keymap).ctx, name);
        let mut i: xkb_layout_index_t = 0;
        if atom == XKB_ATOM_NONE as xkb_atom_t {
            return XKB_LAYOUT_INVALID as xkb_layout_index_t;
        }
        i = 0 as xkb_layout_index_t;
        while i < (*keymap).num_group_names {
            if *(*keymap).group_names.offset(i as isize) == atom {
                return i;
            }
            i = i.wrapping_add(1);
        }
        return XKB_LAYOUT_INVALID as xkb_layout_index_t;
    }
}
#[no_mangle]
#[c2rust::src_loc = "390:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_num_layouts_for_key(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
) -> xkb_layout_index_t {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as xkb_layout_index_t;
        }
        return (*key).num_groups();
    }
}
#[no_mangle]
#[c2rust::src_loc = "404:1"]
pub unsafe fn xkb_keymap_num_levels_for_key(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
) -> xkb_level_index_t {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as xkb_level_index_t;
        }
        layout = XkbWrapGroupIntoRange(
            layout as i32,
            (*key).num_groups(),
            (*key).out_of_range_group_policy(),
            (*key).out_of_range_group_number(),
        );
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return 0 as xkb_level_index_t;
        }
        return XkbKeyNumLevels(key, layout);
    }
}
#[no_mangle]
#[c2rust::src_loc = "426:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_num_leds(mut keymap: *mut xkb_keymap) -> xkb_led_index_t {
    unsafe {
        return (*keymap).num_leds;
    }
}
#[no_mangle]
#[c2rust::src_loc = "435:1"]
pub unsafe fn xkb_keymap_led_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_led_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_leds {
            return ::core::ptr::null::<i8>();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).leds[idx as usize].name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "447:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_led_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_led_index_t {
    unsafe {
        let mut atom: xkb_atom_t = xkb_atom_lookup((*keymap).ctx, name);
        let mut i: xkb_led_index_t = 0;
        let mut led: *const xkb_led = ::core::ptr::null::<xkb_led>();
        if atom == XKB_ATOM_NONE as xkb_atom_t {
            return XKB_LED_INVALID as xkb_led_index_t;
        }
        i = 0 as xkb_led_index_t;
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while i < (*keymap).num_leds {
            if (*led).name == atom {
                return i;
            }
            i = i.wrapping_add(1);
            led = led.offset(1);
        }
        return XKB_LED_INVALID as xkb_led_index_t;
    }
}
#[no_mangle]
#[c2rust::src_loc = "464:1"]
pub unsafe fn xkb_keymap_key_get_mods_for_level(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut masks_out: *mut xkb_mod_mask_t,
    mut masks_size: usize,
) -> usize {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as usize;
        }
        layout = XkbWrapGroupIntoRange(
            layout as i32,
            (*key).num_groups(),
            (*key).out_of_range_group_policy(),
            (*key).out_of_range_group_number(),
        );
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return 0 as usize;
        }
        if level >= XkbKeyNumLevels(key, layout) {
            return 0 as usize;
        }
        let mut type_0: *const xkb_key_type = (*(*key).groups.offset(layout as isize)).type_0;
        let mut count: usize = 0 as usize;
        if level == 0 as xkb_level_index_t {
            let mut empty_mapped: bool = false_0 != 0;
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*type_0).num_entries && count < masks_size {
                if entry_is_active((*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry)
                    as i32
                    != 0
                    && (*(*type_0).entries.offset(i as isize)).mods.mask == 0 as xkb_mod_mask_t
                {
                    empty_mapped = true_0 != 0;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if !empty_mapped && count < masks_size {
                let c2rust_fresh0 = count;
                count = count.wrapping_add(1);
                *masks_out.offset(c2rust_fresh0 as isize) = 0 as xkb_mod_mask_t;
            }
        }
        let mut i_0: darray_size_t = 0 as darray_size_t;
        while i_0 < (*type_0).num_entries && count < masks_size {
            if entry_is_active((*type_0).entries.offset(i_0 as isize) as *mut xkb_key_type_entry)
                as i32
                != 0
                && (*(*type_0).entries.offset(i_0 as isize)).level == level
            {
                let c2rust_fresh1 = count;
                count = count.wrapping_add(1);
                *masks_out.offset(c2rust_fresh1 as isize) =
                    (*(*type_0).entries.offset(i_0 as isize)).mods.mask;
            }
            i_0 = i_0.wrapping_add(1);
        }
        return count;
    }
}
#[no_mangle]
#[c2rust::src_loc = "525:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_key_get_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
) -> *mut xkb_level {
    unsafe {
        layout = XkbWrapGroupIntoRange(
            layout as i32,
            (*key).num_groups(),
            (*key).out_of_range_group_policy(),
            (*key).out_of_range_group_number(),
        );
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return ::core::ptr::null_mut::<xkb_level>();
        }
        if level >= XkbKeyNumLevels(key, layout) {
            return ::core::ptr::null_mut::<xkb_level>();
        }
        return (*(*key).groups.offset(layout as isize))
            .levels
            .offset(level as isize) as *mut xkb_level;
    }
}
#[no_mangle]
#[c2rust::src_loc = "545:1"]
#[no_mangle]
pub unsafe fn xkb_keymap_key_get_syms_by_level(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut syms_out: *mut *const xkb_keysym_t,
) -> i32 {
    unsafe {
        let mut leveli: *const xkb_level = ::core::ptr::null::<xkb_level>();
        let mut num_syms: xkb_keysym_count_t = 0;
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if !key.is_null() {
            leveli = xkb_keymap_key_get_level(keymap, key, layout, level);
            if !leveli.is_null() {
                num_syms = (*leveli).num_syms;
                if !(num_syms as i32 == 0 as i32) {
                    if num_syms as i32 == 1 as i32 {
                        *syms_out = &raw const (*leveli).s.sym;
                    } else {
                        *syms_out = (*leveli).s.syms;
                    }
                    return num_syms as i32;
                }
            }
        }
        *syms_out = ::core::ptr::null::<xkb_keysym_t>();
        return 0 as i32;
    }
}
#[no_mangle]
#[c2rust::src_loc = "579:1"]
pub unsafe fn xkb_keymap_min_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).min_key_code;
    }
}
#[no_mangle]
#[c2rust::src_loc = "585:1"]
pub unsafe fn xkb_keymap_max_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).max_key_code;
    }
}
#[no_mangle]
#[c2rust::src_loc = "600:1"]
pub unsafe fn xkb_keymap_key_iterator_new(
    mut keymap: *mut xkb_keymap,
    mut flags: xkb_keymap_key_iterator_flags,
) -> *mut xkb_keymap_key_iterator {
    unsafe {
        static mut XKB_KEYMAP_KEY_ITERATOR_FLAGS: xkb_keymap_key_iterator_flags =
            (XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER as i32
                | XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND as i32)
                as xkb_keymap_key_iterator_flags;
        if flags as u32 & !(XKB_KEYMAP_KEY_ITERATOR_FLAGS as u32) != 0 {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"unrecognized keymap iterator flags: %#x\n\0".as_ptr() as *const i8,
                flags as u32 & !(XKB_KEYMAP_KEY_ITERATOR_FLAGS as u32),
            );
            return ::core::ptr::null_mut::<xkb_keymap_key_iterator>();
        }
        let iter: *mut xkb_keymap_key_iterator = calloc(
            1 as usize,
            ::core::mem::size_of::<xkb_keymap_key_iterator>() as usize,
        ) as *mut xkb_keymap_key_iterator;
        if iter.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                b"[XKB-%03d] Could not allocate a keymap key iterator.\n\0".as_ptr() as *const i8,
                XKB_ERROR_ALLOCATION_ERROR as i32,
            );
            return ::core::ptr::null_mut::<xkb_keymap_key_iterator>();
        }
        (*iter).keymap = xkb_keymap_ref(keymap);
        if (*keymap).num_keys == 0 as xkb_keycode_t {
            (*iter).next = ::core::ptr::null::<xkb_key>();
            (*iter).min = ::core::ptr::null::<xkb_key>();
            (*iter).max = ::core::ptr::null::<xkb_key>();
            return iter;
        }
        (*iter).skip_unbound =
            flags as u32 & XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND as i32 as u32 != 0;
        (*iter).increment =
            (if flags as u32 & XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER as i32 as u32 != 0 {
                -1 as i32
            } else {
                1 as i32
            }) as i8;
        (*iter).min = if (*keymap).num_keys_low != 0 {
            (*(*iter).keymap)
                .keys
                .offset((*keymap).min_key_code as isize) as *mut xkb_key
        } else {
            (*(*iter).keymap).keys.offset(0 as i32 as isize) as *mut xkb_key
        };
        (*iter).max = (*(*iter).keymap)
            .keys
            .offset((*(*iter).keymap).num_keys.wrapping_sub(1 as xkb_keycode_t) as isize)
            as *mut xkb_key;
        if ((*iter).increment as i32) < 0 as i32 {
            (*iter).next = (*iter).max;
        } else {
            (*iter).next = (*iter).min;
        }
        return iter;
    }
}
#[no_mangle]
#[c2rust::src_loc = "649:1"]
pub unsafe fn xkb_keymap_key_iterator_destroy(mut iter: *mut xkb_keymap_key_iterator) {
    unsafe {
        if iter.is_null() {
            return;
        }
        xkb_keymap_unref((*iter).keymap);
        free(iter as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
#[c2rust::src_loc = "659:1"]
pub unsafe fn xkb_keymap_key_iterator_next(
    mut iter: *mut xkb_keymap_key_iterator,
) -> xkb_keycode_t {
    unsafe {
        if (*iter).next.is_null() {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        }
        let mut next: *const xkb_key = (*iter).next;
        while (*next).name == XKB_ATOM_NONE as xkb_atom_t
            || (*iter).skip_unbound as i32 != 0 && (*next).num_groups() as i32 == 0 as i32
        {
            next = next.offset((*iter).increment as i32 as isize);
            if next < (*iter).min || next > (*iter).max {
                (*iter).next = ::core::ptr::null::<xkb_key>();
                return XKB_KEYCODE_INVALID as xkb_keycode_t;
            }
        }
        let ret: xkb_keycode_t = (*next).keycode;
        next = next.offset((*iter).increment as i32 as isize);
        (*iter).next = if next < (*iter).min || next > (*iter).max {
            ::core::ptr::null::<xkb_key>()
        } else {
            next
        };
        return ret;
    }
}
#[no_mangle]
#[c2rust::src_loc = "686:1"]
pub unsafe fn xkb_keymap_key_for_each(
    mut keymap: *mut xkb_keymap,
    mut iter: xkb_keymap_key_iter_t,
    mut data: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            iter.expect("non-null function pointer")(keymap, (*key).keycode, data);
            key = key.offset(1);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "696:1"]
pub unsafe fn xkb_keymap_key_get_name(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
) -> *const i8 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return ::core::ptr::null::<i8>();
        }
        return xkb_atom_text((*keymap).ctx, (*key).name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "707:1"]
pub unsafe fn xkb_keymap_key_by_name(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_keycode_t {
    unsafe {
        let mut key: *mut xkb_key = ::core::ptr::null_mut::<xkb_key>();
        let mut atom: xkb_atom_t = 0;
        atom = xkb_atom_lookup((*keymap).ctx, name);
        if atom != 0 {
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*keymap).c2rust_unnamed.c2rust_unnamed_0.num_key_aliases {
                if (*(*keymap)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .key_aliases
                    .offset(i as isize))
                .alias
                    == atom
                {
                    atom = (*(*keymap)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .key_aliases
                        .offset(i as isize))
                    .real;
                }
                i = i.wrapping_add(1);
            }
        }
        if atom == 0 {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as xkb_keycode_t {
                0 as xkb_keycode_t
            } else {
                (*keymap).min_key_code
            }) as isize,
        );
        while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
            if (*key).name == atom {
                return (*key).keycode;
            }
            key = key.offset(1);
        }
        return XKB_KEYCODE_INVALID as xkb_keycode_t;
    }
}
#[no_mangle]
#[c2rust::src_loc = "733:1"]
pub unsafe fn xkb_keymap_key_repeats(mut keymap: *mut xkb_keymap, mut kc: xkb_keycode_t) -> i32 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as i32;
        }
        return (*key).repeats() as i32;
    }
}
