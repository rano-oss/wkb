use c2rust_bitfields;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
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
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:17"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:17"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
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
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
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
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/context.h:22"]
pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "21:1"]
    pub struct xkb_context {
        pub refcnt: ::core::ffi::c_int,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: ::core::ffi::c_int,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [::core::ffi::c_char; 2048],
        pub text_next: size_t,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:5"]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:5"]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;
    use super::internal::__va_list_tag;
    use super::rmlvo_h::RMLVO;
    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn xkb_atom_lookup(
            ctx: *mut xkb_context,
            string: *const ::core::ffi::c_char,
        ) -> xkb_atom_t;
        #[c2rust::src_loc = "100:1"]
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "106:1"]
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "110:1"]
        pub fn xkb_context_sanitize_rule_names(
            ctx: *mut xkb_context,
            rmlvo: *mut xkb_rule_names,
        ) -> RMLVO;
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/atom.h:20"]
pub mod atom_h {
    #[c2rust::src_loc = "17:1"]
    pub type xkb_atom_t = darray_size_t;
    #[c2rust::src_loc = "19:9"]
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::darray_h::darray_size_t;
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub type atom_table;
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/darray.h:20"]
pub mod darray_h {
    #[c2rust::src_loc = "19:1"]
    pub type darray_size_t = ::core::ffi::c_uint;
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/include/xkbcommon/xkbcommon.h:19"]
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:1"]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "1118:1"]
    pub type xkb_log_level = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1123:5"]
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    #[c2rust::src_loc = "1122:5"]
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    #[c2rust::src_loc = "1121:5"]
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    #[c2rust::src_loc = "1120:5"]
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    #[c2rust::src_loc = "1119:5"]
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    #[c2rust::src_loc = "253:1"]
    pub type xkb_layout_index_t = uint32_t;
    #[c2rust::src_loc = "183:1"]
    pub type xkb_keycode_t = uint32_t;
    #[c2rust::src_loc = "316:1"]
    pub type xkb_mod_mask_t = uint32_t;
    #[c2rust::src_loc = "295:1"]
    pub type xkb_mod_index_t = uint32_t;
    #[c2rust::src_loc = "223:1"]
    pub type xkb_keysym_t = uint32_t;
    #[c2rust::src_loc = "269:1"]
    pub type xkb_level_index_t = uint32_t;
    #[c2rust::src_loc = "3094:1"]
    pub type xkb_layout_out_of_range_policy = ::core::ffi::c_uint;
    #[c2rust::src_loc = "3113:5"]
    pub const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: xkb_layout_out_of_range_policy = 2;
    #[c2rust::src_loc = "3107:5"]
    pub const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: xkb_layout_out_of_range_policy = 1;
    #[c2rust::src_loc = "3100:5"]
    pub const XKB_LAYOUT_OUT_OF_RANGE_WRAP: xkb_layout_out_of_range_policy = 0;
    #[c2rust::src_loc = "2260:1"]
    pub type xkb_state_component = ::core::ffi::c_uint;
    #[c2rust::src_loc = "2344:5"]
    pub const XKB_STATE_CONTROLS: xkb_state_component = 512;
    #[c2rust::src_loc = "2336:5"]
    pub const XKB_STATE_LEDS: xkb_state_component = 256;
    #[c2rust::src_loc = "2330:5"]
    pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
    #[c2rust::src_loc = "2322:5"]
    pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
    #[c2rust::src_loc = "2313:5"]
    pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
    #[c2rust::src_loc = "2304:5"]
    pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
    #[c2rust::src_loc = "2296:5"]
    pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
    #[c2rust::src_loc = "2286:5"]
    pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
    #[c2rust::src_loc = "2277:5"]
    pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
    #[c2rust::src_loc = "2268:5"]
    pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
    #[c2rust::src_loc = "255:1"]
    pub type xkb_layout_mask_t = uint32_t;
    #[c2rust::src_loc = "345:1"]
    pub type xkb_led_index_t = uint32_t;
    #[c2rust::src_loc = "1321:1"]
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1355:5"]
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    #[c2rust::src_loc = "1334:5"]
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    #[c2rust::src_loc = "1220:1"]
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1240:5"]
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    #[c2rust::src_loc = "1222:5"]
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    #[c2rust::src_loc = "1523:1"]
    pub type xkb_keymap_serialize_flags = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1548:5"]
    pub const XKB_KEYMAP_SERIALIZE_EXPLICIT: xkb_keymap_serialize_flags = 4;
    #[c2rust::src_loc = "1541:5"]
    pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
    #[c2rust::src_loc = "1535:5"]
    pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
    #[c2rust::src_loc = "1529:5"]
    pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
    #[c2rust::src_loc = "1644:1"]
    pub type xkb_keymap_key_iterator_flags = ::core::ffi::c_uint;
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
    pub const XKB_KEYCODE_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    #[c2rust::src_loc = "352:9"]
    pub const XKB_LAYOUT_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    #[c2rust::src_loc = "356:9"]
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    #[c2rust::src_loc = "358:9"]
    pub const XKB_LED_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    #[c2rust::src_loc = "1515:9"]
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "983:1"]
        pub fn xkb_context_unref(context: *mut xkb_context);
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/keymap.h:22"]
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "668:1"]
    pub struct xkb_keymap {
        pub ctx: *mut xkb_context,
        pub refcnt: ::core::ffi::c_int,
        pub flags: xkb_keymap_compile_flags,
        pub format: xkb_keymap_format,
        pub num_leds: xkb_led_index_t,
        pub leds: [xkb_led; 32],
        pub min_key_code: xkb_keycode_t,
        pub max_key_code: xkb_keycode_t,
        pub num_keys: xkb_keycode_t,
        pub num_keys_low: xkb_keycode_t,
        pub keys: *mut xkb_key,
        pub c2rust_unnamed: C2Rust_Unnamed_3,
        pub types: *mut xkb_key_type,
        pub num_types: darray_size_t,
        pub num_sym_interprets: darray_size_t,
        pub sym_interprets: *mut xkb_sym_interpret,
        pub mods: xkb_mod_set,
        pub canonical_state_mask: xkb_mod_mask_t,
        pub redirect_key_auto: xkb_keycode_t,
        pub num_groups: xkb_layout_index_t,
        pub num_group_names: xkb_layout_index_t,
        pub group_names: *mut xkb_atom_t,
        pub keycodes_section_name: *mut ::core::ffi::c_char,
        pub symbols_section_name: *mut ::core::ffi::c_char,
        pub types_section_name: *mut ::core::ffi::c_char,
        pub compat_section_name: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "588:1"]
    pub struct xkb_mod_set {
        pub mods: [xkb_mod; 32],
        pub num_mods: xkb_mod_index_t,
        pub explicit_vmods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "582:1"]
    pub struct xkb_mod {
        pub name: xkb_atom_t,
        pub type_0: mod_type,
        pub mapping: xkb_mod_mask_t,
    }
    #[c2rust::src_loc = "67:1"]
    pub type mod_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "73:5"]
    pub const MOD_BOTH: mod_type = 3;
    #[c2rust::src_loc = "71:5"]
    pub const MOD_VIRT: mod_type = 2;
    #[c2rust::src_loc = "69:5"]
    pub const MOD_REAL: mod_type = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "400:1"]
    pub struct xkb_sym_interpret {
        pub sym: xkb_keysym_t,
        pub match_0: xkb_match_operation,
        pub mods: xkb_mod_mask_t,
        pub virtual_mod: xkb_mod_index_t,
        pub level_one_only: bool,
        #[bitfield(name = "repeat", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "required", ty = "bool", bits = "1..=1")]
        pub repeat_required: [u8; 1],
        pub num_actions: xkb_action_count_t,
        pub a: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "409:5"]
    pub union C2Rust_Unnamed_1 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "366:1"]
    pub union xkb_action {
        pub type_0: xkb_action_type,
        pub mods: xkb_mod_action,
        pub group: xkb_group_action,
        pub ctrls: xkb_controls_action,
        pub dflt: xkb_pointer_default_action,
        pub screen: xkb_switch_screen_action,
        pub ptr: xkb_pointer_action,
        pub btn: xkb_pointer_button_action,
        pub redirect: xkb_redirect_key_action,
        pub priv_0: xkb_private_action,
        pub internal: xkb_internal_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "357:1"]
    pub struct xkb_internal_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_internal_action_flags,
        pub c2rust_unnamed: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "360:5"]
    pub union C2Rust_Unnamed_2 {
        pub clear_latched_mods: xkb_mod_mask_t,
    }
    #[c2rust::src_loc = "351:1"]
    pub type xkb_internal_action_flags = ::core::ffi::c_uint;
    #[c2rust::src_loc = "353:5"]
    pub const INTERNAL_BREAKS_MOD_LATCH: xkb_internal_action_flags = 2;
    #[c2rust::src_loc = "352:5"]
    pub const INTERNAL_BREAKS_GROUP_LATCH: xkb_internal_action_flags = 1;
    #[c2rust::src_loc = "91:1"]
    pub type xkb_action_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "113:5"]
    pub const _ACTION_TYPE_NUM_ENTRIES: xkb_action_type = 21;
    #[c2rust::src_loc = "112:5"]
    pub const ACTION_TYPE_INTERNAL: xkb_action_type = 20;
    #[c2rust::src_loc = "111:5"]
    pub const ACTION_TYPE_PRIVATE: xkb_action_type = 19;
    #[c2rust::src_loc = "110:5"]
    pub const ACTION_TYPE_UNKNOWN: xkb_action_type = 18;
    #[c2rust::src_loc = "109:5"]
    pub const ACTION_TYPE_UNSUPPORTED_LEGACY: xkb_action_type = 17;
    #[c2rust::src_loc = "108:5"]
    pub const ACTION_TYPE_REDIRECT_KEY: xkb_action_type = 16;
    #[c2rust::src_loc = "107:5"]
    pub const ACTION_TYPE_CTRL_LOCK: xkb_action_type = 15;
    #[c2rust::src_loc = "106:5"]
    pub const ACTION_TYPE_CTRL_SET: xkb_action_type = 14;
    #[c2rust::src_loc = "105:5"]
    pub const ACTION_TYPE_SWITCH_VT: xkb_action_type = 13;
    #[c2rust::src_loc = "104:5"]
    pub const ACTION_TYPE_TERMINATE: xkb_action_type = 12;
    #[c2rust::src_loc = "103:5"]
    pub const ACTION_TYPE_PTR_DEFAULT: xkb_action_type = 11;
    #[c2rust::src_loc = "102:5"]
    pub const ACTION_TYPE_PTR_LOCK: xkb_action_type = 10;
    #[c2rust::src_loc = "101:5"]
    pub const ACTION_TYPE_PTR_BUTTON: xkb_action_type = 9;
    #[c2rust::src_loc = "100:5"]
    pub const ACTION_TYPE_PTR_MOVE: xkb_action_type = 8;
    #[c2rust::src_loc = "99:5"]
    pub const ACTION_TYPE_GROUP_LOCK: xkb_action_type = 7;
    #[c2rust::src_loc = "98:5"]
    pub const ACTION_TYPE_GROUP_LATCH: xkb_action_type = 6;
    #[c2rust::src_loc = "97:5"]
    pub const ACTION_TYPE_GROUP_SET: xkb_action_type = 5;
    #[c2rust::src_loc = "96:5"]
    pub const ACTION_TYPE_MOD_LOCK: xkb_action_type = 4;
    #[c2rust::src_loc = "95:5"]
    pub const ACTION_TYPE_MOD_LATCH: xkb_action_type = 3;
    #[c2rust::src_loc = "94:5"]
    pub const ACTION_TYPE_MOD_SET: xkb_action_type = 2;
    #[c2rust::src_loc = "93:5"]
    pub const ACTION_TYPE_VOID: xkb_action_type = 1;
    #[c2rust::src_loc = "92:5"]
    pub const ACTION_TYPE_NONE: xkb_action_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "346:1"]
    pub struct xkb_private_action {
        pub type_0: xkb_action_type,
        pub data: [uint8_t; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "327:1"]
    pub struct xkb_redirect_key_action {
        pub type_0: xkb_action_type,
        pub keycode: xkb_keycode_t,
        pub affect: xkb_mod_mask_t,
        pub mods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "320:1"]
    pub struct xkb_pointer_button_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub count: uint8_t,
        pub button: uint8_t,
    }
    #[c2rust::src_loc = "116:1"]
    pub type xkb_action_flags = ::core::ffi::c_uint;
    #[c2rust::src_loc = "130:5"]
    pub const ACTION_PENDING_COMPUTATION: xkb_action_flags = 8192;
    #[c2rust::src_loc = "129:5"]
    pub const ACTION_LATCH_ON_PRESS: xkb_action_flags = 4096;
    #[c2rust::src_loc = "128:5"]
    pub const ACTION_UNLOCK_ON_PRESS: xkb_action_flags = 2048;
    #[c2rust::src_loc = "127:5"]
    pub const ACTION_LOCK_ON_RELEASE: xkb_action_flags = 1024;
    #[c2rust::src_loc = "126:5"]
    pub const ACTION_SAME_SCREEN: xkb_action_flags = 512;
    #[c2rust::src_loc = "125:5"]
    pub const ACTION_ACCEL: xkb_action_flags = 256;
    #[c2rust::src_loc = "124:5"]
    pub const ACTION_ABSOLUTE_Y: xkb_action_flags = 128;
    #[c2rust::src_loc = "123:5"]
    pub const ACTION_ABSOLUTE_X: xkb_action_flags = 64;
    #[c2rust::src_loc = "122:5"]
    pub const ACTION_ABSOLUTE_SWITCH: xkb_action_flags = 32;
    #[c2rust::src_loc = "121:5"]
    pub const ACTION_MODS_LOOKUP_MODMAP: xkb_action_flags = 16;
    #[c2rust::src_loc = "120:5"]
    pub const ACTION_LOCK_NO_UNLOCK: xkb_action_flags = 8;
    #[c2rust::src_loc = "119:5"]
    pub const ACTION_LOCK_NO_LOCK: xkb_action_flags = 4;
    #[c2rust::src_loc = "118:5"]
    pub const ACTION_LATCH_TO_LOCK: xkb_action_flags = 2;
    #[c2rust::src_loc = "117:5"]
    pub const ACTION_LOCK_CLEAR: xkb_action_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "313:1"]
    pub struct xkb_pointer_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub x: int16_t,
        pub y: int16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "307:1"]
    pub struct xkb_switch_screen_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub screen: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct xkb_pointer_default_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub value: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "295:1"]
    pub struct xkb_controls_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub ctrls: xkb_action_controls,
    }
    #[c2rust::src_loc = "141:1"]
    pub type xkb_action_controls = ::core::ffi::c_uint;
    #[c2rust::src_loc = "187:5"]
    pub const CONTROL_ALL_BOOLEAN: xkb_action_controls = 2088447;
    #[c2rust::src_loc = "181:5"]
    pub const CONTROL_ALL_BOOLEAN_V1: xkb_action_controls = 2087943;
    #[c2rust::src_loc = "176:5"]
    pub const CONTROL_ALL: xkb_action_controls = 2088959;
    #[c2rust::src_loc = "170:5"]
    pub const CONTROL_ALL_V1: xkb_action_controls = 2088455;
    #[c2rust::src_loc = "164:5"]
    pub const CONTROL_IGNORE_GROUP_LOCK: xkb_action_controls = 1048576;
    #[c2rust::src_loc = "163:5"]
    pub const CONTROL_BELL: xkb_action_controls = 524288;
    #[c2rust::src_loc = "162:5"]
    pub const CONTROL_AX_FEEDBACK: xkb_action_controls = 262144;
    #[c2rust::src_loc = "161:5"]
    pub const CONTROL_AX_TIMEOUT: xkb_action_controls = 131072;
    #[c2rust::src_loc = "160:5"]
    pub const CONTROL_AX: xkb_action_controls = 65536;
    #[c2rust::src_loc = "159:5"]
    pub const CONTROL_MOUSE_KEYS_ACCEL: xkb_action_controls = 32768;
    #[c2rust::src_loc = "158:5"]
    pub const CONTROL_MOUSE_KEYS: xkb_action_controls = 16384;
    #[c2rust::src_loc = "157:5"]
    pub const CONTROL_DEBOUNCE: xkb_action_controls = 4096;
    #[c2rust::src_loc = "156:5"]
    pub const CONTROL_SLOW: xkb_action_controls = 2048;
    #[c2rust::src_loc = "155:5"]
    pub const CONTROL_REPEAT: xkb_action_controls = 1024;
    #[c2rust::src_loc = "154:5"]
    pub const CONTROL_GROUPS_WRAP: xkb_action_controls = 512;
    #[c2rust::src_loc = "151:5"]
    pub const CONTROL_OVERLAY8: xkb_action_controls = 256;
    #[c2rust::src_loc = "150:5"]
    pub const CONTROL_OVERLAY7: xkb_action_controls = 128;
    #[c2rust::src_loc = "149:5"]
    pub const CONTROL_OVERLAY6: xkb_action_controls = 64;
    #[c2rust::src_loc = "148:5"]
    pub const CONTROL_OVERLAY5: xkb_action_controls = 32;
    #[c2rust::src_loc = "147:5"]
    pub const CONTROL_OVERLAY4: xkb_action_controls = 16;
    #[c2rust::src_loc = "146:5"]
    pub const CONTROL_OVERLAY3: xkb_action_controls = 8;
    #[c2rust::src_loc = "145:5"]
    pub const CONTROL_OVERLAY2: xkb_action_controls = 4;
    #[c2rust::src_loc = "144:5"]
    pub const CONTROL_OVERLAY1: xkb_action_controls = 2;
    #[c2rust::src_loc = "143:5"]
    pub const CONTROL_STICKY_KEYS: xkb_action_controls = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "249:1"]
    pub struct xkb_group_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub group: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "243:1"]
    pub struct xkb_mod_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub mods: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:1"]
    pub struct xkb_mods {
        pub mods: xkb_mod_mask_t,
        pub mask: xkb_mod_mask_t,
    }
    #[c2rust::src_loc = "397:1"]
    pub type xkb_action_count_t = uint16_t;
    #[c2rust::src_loc = "230:1"]
    pub type xkb_match_operation = ::core::ffi::c_uint;
    #[c2rust::src_loc = "235:5"]
    pub const MATCH_EXACTLY: xkb_match_operation = 4;
    #[c2rust::src_loc = "234:5"]
    pub const MATCH_ALL: xkb_match_operation = 3;
    #[c2rust::src_loc = "233:5"]
    pub const MATCH_ANY: xkb_match_operation = 2;
    #[c2rust::src_loc = "232:5"]
    pub const MATCH_ANY_OR_NONE: xkb_match_operation = 1;
    #[c2rust::src_loc = "231:5"]
    pub const MATCH_NONE: xkb_match_operation = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "386:1"]
    pub struct xkb_key_type {
        pub name: xkb_atom_t,
        pub mods: xkb_mods,
        pub required: bool,
        pub num_levels: xkb_level_index_t,
        pub num_level_names: xkb_level_index_t,
        pub level_names: *mut xkb_atom_t,
        pub num_entries: darray_size_t,
        pub entries: *mut xkb_key_type_entry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "380:1"]
    pub struct xkb_key_type_entry {
        pub level: xkb_level_index_t,
        pub mods: xkb_mods,
        pub preserve: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "694:5"]
    pub union C2Rust_Unnamed_3 {
        pub c2rust_unnamed: C2Rust_Unnamed_5,
        pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "715:9"]
    pub struct C2Rust_Unnamed_4 {
        pub num_key_aliases: darray_size_t,
        pub key_aliases: *mut xkb_key_alias,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "433:1"]
    pub struct xkb_key_alias {
        pub real: xkb_atom_t,
        pub alias: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "706:9"]
    pub struct C2Rust_Unnamed_5 {
        pub num_key_names: darray_size_t,
        pub key_names: *mut KeycodeMatch,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "633:9"]
    pub union KeycodeMatch {
        pub c2rust_unnamed: C2Rust_Unnamed_8,
        pub key: C2Rust_Unnamed_7,
        pub alias: C2Rust_Unnamed_6,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "658:5"]
    pub struct C2Rust_Unnamed_6 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "real", ty = "xkb_atom_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_real: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "643:5"]
    pub struct C2Rust_Unnamed_7 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "low", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "index", ty = "darray_size_t", bits = "3..=31")]
        pub found_low_is_alias_index: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "634:5"]
    pub struct C2Rust_Unnamed_8 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "c2rust_unnamed_0", ty = "darray_size_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "539:1"]
    pub struct xkb_key {
        pub keycode: xkb_keycode_t,
        pub name: xkb_atom_t,
        pub explicit: xkb_explicit_components,
        pub modmap: xkb_mod_mask_t,
        pub vmodmap: xkb_mod_mask_t,
        pub overlays: xkb_overlay_mask_t,
        #[bitfield(name = "overlays_inline", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "repeats", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "implicit_actions", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "out_of_range_pending_group", ty = "bool", bits = "3..=3")]
        #[bitfield(
            name = "out_of_range_group_policy",
            ty = "xkb_layout_out_of_range_policy",
            bits = "4..=7"
        )]
        #[bitfield(
            name = "out_of_range_group_number",
            ty = "xkb_layout_index_t",
            bits = "8..=15"
        )]
        #[bitfield(name = "num_groups", ty = "xkb_layout_index_t", bits = "16..=23")]
        pub overlays_inline_repeats_implicit_actions_out_of_range_pending_group_out_of_range_group_policy_out_of_range_group_number_num_groups:
            [u8; 3],
        pub groups: *mut xkb_group,
        pub c2rust_unnamed: C2Rust_Unnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "567:5"]
    pub union C2Rust_Unnamed_9 {
        pub overlay_key: *const xkb_key,
        pub overlays_keys: *mut *const xkb_key,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "495:1"]
    pub struct xkb_group {
        #[bitfield(name = "explicit_symbols", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "explicit_actions", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "implicit_actions", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "explicit_type", ty = "bool", bits = "3..=3")]
        pub explicit_symbols_explicit_actions_implicit_actions_explicit_type: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub type_0: *const xkb_key_type,
        pub levels: *mut xkb_level,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "467:1"]
    pub struct xkb_level {
        pub num_syms: xkb_keysym_count_t,
        pub num_actions: xkb_action_count_t,
        pub c2rust_unnamed: C2Rust_Unnamed_12,
        pub s: C2Rust_Unnamed_11,
        pub a: C2Rust_Unnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "486:5"]
    pub union C2Rust_Unnamed_10 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "481:5"]
    pub union C2Rust_Unnamed_11 {
        pub sym: xkb_keysym_t,
        pub syms: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "473:5"]
    pub union C2Rust_Unnamed_12 {
        pub upper: xkb_keysym_t,
        pub has_upper: bool,
    }
    #[c2rust::src_loc = "463:1"]
    pub type xkb_keysym_count_t = uint16_t;
    #[c2rust::src_loc = "264:1"]
    pub type xkb_overlay_mask_t = uint8_t;
    #[c2rust::src_loc = "454:1"]
    pub type xkb_explicit_components = ::core::ffi::c_uint;
    #[c2rust::src_loc = "460:5"]
    pub const EXPLICIT_OVERLAY: xkb_explicit_components = 32;
    #[c2rust::src_loc = "459:5"]
    pub const EXPLICIT_REPEAT: xkb_explicit_components = 16;
    #[c2rust::src_loc = "458:5"]
    pub const EXPLICIT_VMODMAP: xkb_explicit_components = 8;
    #[c2rust::src_loc = "457:5"]
    pub const EXPLICIT_TYPES: xkb_explicit_components = 4;
    #[c2rust::src_loc = "456:5"]
    pub const EXPLICIT_INTERP: xkb_explicit_components = 2;
    #[c2rust::src_loc = "455:5"]
    pub const EXPLICIT_SYMBOLS: xkb_explicit_components = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "423:1"]
    pub struct xkb_led {
        pub name: xkb_atom_t,
        #[bitfield(name = "which_groups", ty = "xkb_state_component", bits = "0..=30")]
        #[bitfield(name = "pending_groups", ty = "bool", bits = "31..=31")]
        pub which_groups_pending_groups: [u8; 4],
        pub groups: xkb_layout_mask_t,
        pub which_mods: xkb_state_component,
        pub mods: xkb_mods,
        pub ctrls: xkb_action_controls,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "943:1"]
    pub struct xkb_keymap_format_ops {
        pub keymap_new_from_rmlvo:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rmlvo_builder) -> bool>,
        pub keymap_new_from_names:
            Option<unsafe extern "C" fn(*mut xkb_keymap, *const xkb_rule_names) -> bool>,
        pub keymap_new_from_string: Option<
            unsafe extern "C" fn(*mut xkb_keymap, *const ::core::ffi::c_char, size_t) -> bool,
        >,
        pub keymap_new_from_file: Option<unsafe extern "C" fn(*mut xkb_keymap, *mut FILE) -> bool>,
        pub keymap_get_as_string: Option<
            unsafe extern "C" fn(
                *mut xkb_keymap,
                xkb_keymap_format,
                xkb_keymap_serialize_flags,
            ) -> *mut ::core::ffi::c_char,
        >,
    }
    #[c2rust::src_loc = "40:9"]
    pub const XKB_MAX_GROUPS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "855:1"]
    pub unsafe extern "C" fn XkbKey(
        mut keymap: *mut xkb_keymap,
        mut kc: xkb_keycode_t,
    ) -> *const xkb_key {
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
    #[c2rust::src_loc = "883:1"]
    pub unsafe extern "C" fn XkbKeyNumLevels(
        mut key: *const xkb_key,
        mut layout: xkb_layout_index_t,
    ) -> xkb_level_index_t {
        unsafe {
            return (*(*(*key).groups.offset(layout as isize)).type_0).num_levels;
        }
    }
    #[inline]
    #[c2rust::src_loc = "896:1"]
    pub unsafe extern "C" fn entry_is_active(mut entry: *const xkb_key_type_entry) -> bool {
        unsafe {
            return (*entry).mods.mods == 0 as xkb_mod_mask_t
                || (*entry).mods.mask != 0 as xkb_mod_mask_t;
        }
    }
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::rmlvo_h::xkb_rmlvo_builder;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::xkbcommon_h::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keymap_serialize_flags,
        xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
        xkb_led_index_t, xkb_level_index_t, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
        xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    };
    use super::FILE_h::FILE;
    use super::__stddef_null_h::NULL;
    extern "C" {
        #[c2rust::src_loc = "902:1"]
        pub fn xkb_keymap_new(
            ctx: *mut xkb_context,
            func: *const ::core::ffi::c_char,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        #[c2rust::src_loc = "910:1"]
        pub fn XkbModNameToIndex(
            mods: *const xkb_mod_set,
            name: xkb_atom_t,
            type_0: mod_type,
        ) -> xkb_mod_index_t;
        #[c2rust::src_loc = "923:1"]
        pub fn XkbWrapGroupIntoRange(
            group: int32_t,
            num_groups: xkb_layout_index_t,
            out_of_range_group_policy: xkb_layout_out_of_range_policy,
            out_of_range_group_number: xkb_layout_index_t,
        ) -> xkb_layout_index_t;
        #[c2rust::src_loc = "956:1"]
        pub static text_v1_keymap_format_ops: xkb_keymap_format_ops;
    }
}
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/rmlvo.h:22"]
pub mod rmlvo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct xkb_rmlvo_builder {
        pub rules: *mut ::core::ffi::c_char,
        pub model: *mut ::core::ffi::c_char,
        pub layouts: xkb_rmlvo_builder_layouts,
        pub options: xkb_rmlvo_builder_options,
        pub refcnt: ::core::ffi::c_int,
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
        pub option: *mut ::core::ffi::c_char,
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
        pub layout: *mut ::core::ffi::c_char,
        pub variant: *mut ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "14:1"]
    pub type RMLVO = ::core::ffi::c_uint;
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
    pub type xkb_log_verbosity = ::core::ffi::c_int;
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
    pub type xkb_message_code = ::core::ffi::c_uint;
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
    pub type xkb_enumerations_values = ::core::ffi::c_uint;
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
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:20"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:16"]
pub mod assert_h {
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
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
    pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/stdbool.h:22"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
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
    xkb_keymap, xkb_keymap_format_ops, xkb_keymap_new, xkb_keysym_count_t, xkb_led, xkb_level,
    xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbKey, XkbKeyNumLevels, XkbModNameToIndex,
    XkbWrapGroupIntoRange, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y,
    ACTION_ACCEL, ACTION_LATCH_ON_PRESS, ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR,
    ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK, ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP,
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
pub use self::messages_codes_h::{
    xkb_log_verbosity, xkb_message_code, XKB_ERROR_ABI_BACKWARD_COMPAT_,
    XKB_ERROR_ABI_FORWARD_COMPAT_, XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR,
    XKB_ERROR_CANNOT_RESOLVE_RMLVO, XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY,
    XKB_ERROR_EXPECTED_ARRAY_ENTRY, XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE,
    XKB_ERROR_INCLUDED_FILE_NOT_FOUND, XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT,
    XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT, XKB_ERROR_INSUFFICIENT_BUFFER_SIZE,
    XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD, XKB_ERROR_INVALID_COMPOSE_LOCALE,
    XKB_ERROR_INVALID_COMPOSE_SYNTAX, XKB_ERROR_INVALID_EXPRESSION_TYPE,
    XKB_ERROR_INVALID_FILE_ENCODING, XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
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
pub use self::rmlvo_h::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_layouts,
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options, RMLVO, RMLVO_LAYOUT, RMLVO_MODEL,
    RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::INT32_MAX;
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "591:1"]
pub struct xkb_keymap_key_iterator {
    pub increment: int8_t,
    pub skip_unbound: bool,
    pub min: *const xkb_key,
    pub max: *const xkb_key,
    pub next: *const xkb_key,
    pub keymap: *mut xkb_keymap,
}
#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn xkb_keymap_ref(mut keymap: *mut xkb_keymap) -> *mut xkb_keymap {
    unsafe {
        if (*keymap).refcnt > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"keymap->refcnt > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/keymap.c\0".as_ptr() as *const ::core::ffi::c_char,
                29 as ::core::ffi::c_uint,
                b"struct xkb_keymap *xkb_keymap_ref(struct xkb_keymap *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*keymap).refcnt += 1;
        return keymap;
    }
}
#[no_mangle]
#[c2rust::src_loc = "34:1"]
pub unsafe extern "C" fn clear_level(mut leveli: *mut xkb_level) {
    unsafe {
        if (*leveli).num_syms as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            free((*leveli).s.syms as *mut ::core::ffi::c_void);
        }
        if (*leveli).num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            free((*leveli).a.actions as *mut ::core::ffi::c_void);
        }
    }
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn clear_interpret(mut interp: *mut xkb_sym_interpret) {
    unsafe {
        if (*interp).num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            free((*interp).a.actions as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn xkb_keymap_unref(mut keymap: *mut xkb_keymap) {
    unsafe {
        if keymap.is_null() || (*keymap).refcnt > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"!keymap || keymap->refcnt > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/keymap.c\0".as_ptr() as *const ::core::ffi::c_char,
                53 as ::core::ffi::c_uint,
                b"void xkb_keymap_unref(struct xkb_keymap *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if keymap.is_null() || {
            (*keymap).refcnt -= 1;
            (*keymap).refcnt > 0 as ::core::ffi::c_int
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
unsafe extern "C" fn get_keymap_format_ops(
    mut format: xkb_keymap_format,
) -> *const xkb_keymap_format_ops {
    unsafe {
        static mut keymap_format_ops: [*const xkb_keymap_format_ops; 3] = unsafe {
            [
                ::core::ptr::null::<xkb_keymap_format_ops>(),
                &raw const text_v1_keymap_format_ops,
                &raw const text_v1_keymap_format_ops,
            ]
        };
        if (format as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || format as ::core::ffi::c_int
                >= (::core::mem::size_of::<[*const xkb_keymap_format_ops; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const xkb_keymap_format_ops>() as usize)
                    as ::core::ffi::c_int
        {
            return ::core::ptr::null::<xkb_keymap_format_ops>();
        }
        return keymap_format_ops[format as ::core::ffi::c_int as usize];
    }
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn xkb_keymap_new_from_rmlvo(
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
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const ::core::ffi::c_char,
                format as ::core::ffi::c_uint,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            (*rmlvo).ctx,
            b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_new_from_names2(
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
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_names2\0".as_ptr() as *const ::core::ffi::c_char,
                format as ::core::ffi::c_uint,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_names2\0".as_ptr() as *const ::core::ffi::c_char,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: ::core::ptr::null::<::core::ffi::c_char>(),
            model: ::core::ptr::null::<::core::ffi::c_char>(),
            layout: ::core::ptr::null::<::core::ffi::c_char>(),
            variant: ::core::ptr::null::<::core::ffi::c_char>(),
            options: ::core::ptr::null::<::core::ffi::c_char>(),
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
pub unsafe extern "C" fn xkb_keymap_new_from_names(
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
pub unsafe extern "C" fn xkb_keymap_new_from_string(
    mut ctx: *mut xkb_context,
    mut string: *const ::core::ffi::c_char,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        return xkb_keymap_new_from_buffer(ctx, string, strlen(string), format, flags);
    }
}
#[no_mangle]
#[c2rust::src_loc = "188:1"]
pub unsafe extern "C" fn xkb_keymap_new_from_buffer(
    mut ctx: *mut xkb_context,
    mut buffer: *const ::core::ffi::c_char,
    mut length: size_t,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_string.is_none() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_buffer\0".as_ptr() as *const ::core::ffi::c_char,
                format as ::core::ffi::c_uint,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if buffer.is_null() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: no buffer specified\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            format,
            flags,
        );
        if keymap.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if length > 0 as size_t
            && *buffer.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\0' as i32
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
pub unsafe extern "C" fn xkb_keymap_new_from_file(
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
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_file\0".as_ptr() as *const ::core::ffi::c_char,
                format as ::core::ffi::c_uint,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if file.is_null() {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: no file specified\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_new_from_file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_file\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_get_as_string2(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_serialize_flags,
) -> *mut ::core::ffi::c_char {
    unsafe {
        static mut XKB_KEYMAP_SERIALIZE_FLAGS: xkb_keymap_serialize_flags =
            XKB_KEYMAP_SERIALIZE_FLAGS_VALUES as ::core::ffi::c_int as xkb_keymap_serialize_flags;
        if flags as ::core::ffi::c_uint & !(XKB_KEYMAP_SERIALIZE_FLAGS as ::core::ffi::c_uint) != 0
        {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unrecognized serialization flags: %#x\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"xkb_keymap_get_as_string2\0".as_ptr() as *const ::core::ffi::c_char,
                flags as ::core::ffi::c_uint & !(XKB_KEYMAP_SERIALIZE_FLAGS as ::core::ffi::c_uint),
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if format as ::core::ffi::c_uint == XKB_KEYMAP_USE_ORIGINAL_FORMAT as ::core::ffi::c_uint {
            format = (*keymap).format;
        }
        let ops: *const xkb_keymap_format_ops =
            get_keymap_format_ops(format) as *const xkb_keymap_format_ops;
        if ops.is_null() || (*ops).keymap_get_as_string.is_none() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"%s: unsupported keymap format: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"xkb_keymap_get_as_string2\0".as_ptr() as *const ::core::ffi::c_char,
                format as ::core::ffi::c_uint,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return (*ops)
            .keymap_get_as_string
            .expect("non-null function pointer")(keymap, format, flags);
    }
}
#[no_mangle]
#[c2rust::src_loc = "283:1"]
pub unsafe extern "C" fn xkb_keymap_get_as_string(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
) -> *mut ::core::ffi::c_char {
    unsafe {
        return xkb_keymap_get_as_string2(keymap, format, XKB_KEYMAP_SERIALIZE_NO_FLAGS);
    }
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn xkb_keymap_num_mods(mut keymap: *mut xkb_keymap) -> xkb_mod_index_t {
    unsafe {
        return (*keymap).mods.num_mods;
    }
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe extern "C" fn xkb_keymap_mod_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_mod_index_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if idx >= (*keymap).mods.num_mods {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).mods.mods[idx as usize].name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn xkb_keymap_mod_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_mod_get_mask(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_mod_get_mask2(
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
pub unsafe extern "C" fn xkb_keymap_num_layouts(mut keymap: *mut xkb_keymap) -> xkb_layout_index_t {
    unsafe {
        return (*keymap).num_groups;
    }
}
#[no_mangle]
#[c2rust::src_loc = "359:1"]
pub unsafe extern "C" fn xkb_keymap_layout_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_layout_index_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if idx >= (*keymap).num_group_names {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return xkb_atom_text((*keymap).ctx, *(*keymap).group_names.offset(idx as isize));
    }
}
#[no_mangle]
#[c2rust::src_loc = "371:1"]
pub unsafe extern "C" fn xkb_keymap_layout_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_num_layouts_for_key(
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
pub unsafe extern "C" fn xkb_keymap_num_levels_for_key(
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
            layout as int32_t,
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
pub unsafe extern "C" fn xkb_keymap_num_leds(mut keymap: *mut xkb_keymap) -> xkb_led_index_t {
    unsafe {
        return (*keymap).num_leds;
    }
}
#[no_mangle]
#[c2rust::src_loc = "435:1"]
pub unsafe extern "C" fn xkb_keymap_led_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_led_index_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        if idx >= (*keymap).num_leds {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).leds[idx as usize].name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "447:1"]
pub unsafe extern "C" fn xkb_keymap_led_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_key_get_mods_for_level(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut masks_out: *mut xkb_mod_mask_t,
    mut masks_size: size_t,
) -> size_t {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as size_t;
        }
        layout = XkbWrapGroupIntoRange(
            layout as int32_t,
            (*key).num_groups(),
            (*key).out_of_range_group_policy(),
            (*key).out_of_range_group_number(),
        );
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return 0 as size_t;
        }
        if level >= XkbKeyNumLevels(key, layout) {
            return 0 as size_t;
        }
        let mut type_0: *const xkb_key_type = (*(*key).groups.offset(layout as isize)).type_0;
        let mut count: size_t = 0 as size_t;
        if level == 0 as xkb_level_index_t {
            let mut empty_mapped: bool = false_0 != 0;
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*type_0).num_entries && count < masks_size {
                if entry_is_active((*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry)
                    as ::core::ffi::c_int
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
                as ::core::ffi::c_int
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
pub unsafe extern "C" fn xkb_keymap_key_get_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
) -> *mut xkb_level {
    unsafe {
        layout = XkbWrapGroupIntoRange(
            layout as int32_t,
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
pub unsafe extern "C" fn xkb_keymap_key_get_syms_by_level(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut syms_out: *mut *const xkb_keysym_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut leveli: *const xkb_level = ::core::ptr::null::<xkb_level>();
        let mut num_syms: xkb_keysym_count_t = 0;
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if !key.is_null() {
            leveli = xkb_keymap_key_get_level(keymap, key, layout, level);
            if !leveli.is_null() {
                num_syms = (*leveli).num_syms;
                if !(num_syms as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                    if num_syms as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        *syms_out = &raw const (*leveli).s.sym;
                    } else {
                        *syms_out = (*leveli).s.syms;
                    }
                    return num_syms as ::core::ffi::c_int;
                }
            }
        }
        *syms_out = ::core::ptr::null::<xkb_keysym_t>();
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
#[c2rust::src_loc = "579:1"]
pub unsafe extern "C" fn xkb_keymap_min_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).min_key_code;
    }
}
#[no_mangle]
#[c2rust::src_loc = "585:1"]
pub unsafe extern "C" fn xkb_keymap_max_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).max_key_code;
    }
}
#[no_mangle]
#[c2rust::src_loc = "600:1"]
pub unsafe extern "C" fn xkb_keymap_key_iterator_new(
    mut keymap: *mut xkb_keymap,
    mut flags: xkb_keymap_key_iterator_flags,
) -> *mut xkb_keymap_key_iterator {
    unsafe {
        static mut XKB_KEYMAP_KEY_ITERATOR_FLAGS: xkb_keymap_key_iterator_flags =
            (XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER as ::core::ffi::c_int
                | XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND as ::core::ffi::c_int)
                as xkb_keymap_key_iterator_flags;
        if flags as ::core::ffi::c_uint & !(XKB_KEYMAP_KEY_ITERATOR_FLAGS as ::core::ffi::c_uint)
            != 0
        {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"unrecognized keymap iterator flags: %#x\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                flags as ::core::ffi::c_uint
                    & !(XKB_KEYMAP_KEY_ITERATOR_FLAGS as ::core::ffi::c_uint),
            );
            return ::core::ptr::null_mut::<xkb_keymap_key_iterator>();
        }
        let iter: *mut xkb_keymap_key_iterator = calloc(
            1 as size_t,
            ::core::mem::size_of::<xkb_keymap_key_iterator>() as size_t,
        ) as *mut xkb_keymap_key_iterator;
        if iter.is_null() {
            xkb_log(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Could not allocate a keymap key iterator.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
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
        (*iter).skip_unbound = flags as ::core::ffi::c_uint
            & XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0;
        (*iter).increment = (if flags as ::core::ffi::c_uint
            & XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            -1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as int8_t;
        (*iter).min = if (*keymap).num_keys_low != 0 {
            (*(*iter).keymap)
                .keys
                .offset((*keymap).min_key_code as isize) as *mut xkb_key
        } else {
            (*(*iter).keymap)
                .keys
                .offset(0 as ::core::ffi::c_int as isize) as *mut xkb_key
        };
        (*iter).max = (*(*iter).keymap)
            .keys
            .offset((*(*iter).keymap).num_keys.wrapping_sub(1 as xkb_keycode_t) as isize)
            as *mut xkb_key;
        if ((*iter).increment as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*iter).next = (*iter).max;
        } else {
            (*iter).next = (*iter).min;
        }
        return iter;
    }
}
#[no_mangle]
#[c2rust::src_loc = "649:1"]
pub unsafe extern "C" fn xkb_keymap_key_iterator_destroy(mut iter: *mut xkb_keymap_key_iterator) {
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
pub unsafe extern "C" fn xkb_keymap_key_iterator_next(
    mut iter: *mut xkb_keymap_key_iterator,
) -> xkb_keycode_t {
    unsafe {
        if (*iter).next.is_null() {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        }
        let mut next: *const xkb_key = (*iter).next;
        while (*next).name == XKB_ATOM_NONE as xkb_atom_t
            || (*iter).skip_unbound as ::core::ffi::c_int != 0
                && (*next).num_groups() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            next = next.offset((*iter).increment as ::core::ffi::c_int as isize);
            if next < (*iter).min || next > (*iter).max {
                (*iter).next = ::core::ptr::null::<xkb_key>();
                return XKB_KEYCODE_INVALID as xkb_keycode_t;
            }
        }
        let ret: xkb_keycode_t = (*next).keycode;
        next = next.offset((*iter).increment as ::core::ffi::c_int as isize);
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
pub unsafe extern "C" fn xkb_keymap_key_for_each(
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
pub unsafe extern "C" fn xkb_keymap_key_get_name(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return xkb_atom_text((*keymap).ctx, (*key).name);
    }
}
#[no_mangle]
#[c2rust::src_loc = "707:1"]
pub unsafe extern "C" fn xkb_keymap_key_by_name(
    mut keymap: *mut xkb_keymap,
    mut name: *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xkb_keymap_key_repeats(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (*key).repeats() as ::core::ffi::c_int;
    }
}
