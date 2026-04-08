pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}
pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
}
pub mod stdint_intn_h {
    pub type int8_t = __int8_t;
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: size_t,
    ) -> darray_size_t {
        unsafe {
            if (need as size_t)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as size_t)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }
    use super::__stddef_size_t_h::size_t;
    use super::assert_h::__assert_fail;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_layout_index_t = u32;
    pub type xkb_keycode_t = u32;
    pub type xkb_mod_mask_t = u32;
    pub type xkb_mod_index_t = u32;
    pub type xkb_keysym_t = u32;
    pub type xkb_level_index_t = u32;
    pub type xkb_layout_out_of_range_policy = ::core::ffi::c_uint;
    pub const XKB_LAYOUT_OUT_OF_RANGE_REDIRECT: xkb_layout_out_of_range_policy = 2;
    pub const XKB_LAYOUT_OUT_OF_RANGE_CLAMP: xkb_layout_out_of_range_policy = 1;
    pub const XKB_LAYOUT_OUT_OF_RANGE_WRAP: xkb_layout_out_of_range_policy = 0;
    pub type xkb_state_component = ::core::ffi::c_uint;
    pub const XKB_STATE_CONTROLS: xkb_state_component = 512;
    pub const XKB_STATE_LEDS: xkb_state_component = 256;
    pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
    pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
    pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
    pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
    pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
    pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
    pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
    pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
    pub type xkb_layout_mask_t = u32;
    pub type xkb_led_index_t = u32;
    pub type xkb_keymap_format = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
    pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
    pub type xkb_keymap_compile_flags = ::core::ffi::c_uint;
    pub const XKB_KEYMAP_COMPILE_STRICT_MODE: xkb_keymap_compile_flags = 1;
    pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
    pub type xkb_led_mask_t = u32;
    pub const XKB_KEYCODE_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
    pub const XKB_KEYCODE_MAX: ::core::ffi::c_uint =
        (0xffffffff as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> ::core::ffi::c_int;
    }
}
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub struct xkb_mod_set {
        pub mods: [xkb_mod; 32],
        pub num_mods: xkb_mod_index_t,
        pub explicit_vmods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod {
        pub name: xkb_atom_t,
        pub type_0: mod_type,
        pub mapping: xkb_mod_mask_t,
    }
    pub type mod_type = ::core::ffi::c_uint;
    pub const MOD_BOTH: mod_type = 3;
    pub const MOD_VIRT: mod_type = 2;
    pub const MOD_REAL: mod_type = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
    pub union C2Rust_Unnamed_1 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub struct xkb_internal_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_internal_action_flags,
        pub c2rust_unnamed: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_2 {
        pub clear_latched_mods: xkb_mod_mask_t,
    }
    pub type xkb_internal_action_flags = ::core::ffi::c_uint;
    pub const INTERNAL_BREAKS_MOD_LATCH: xkb_internal_action_flags = 2;
    pub const INTERNAL_BREAKS_GROUP_LATCH: xkb_internal_action_flags = 1;
    pub type xkb_action_type = ::core::ffi::c_uint;
    pub const _ACTION_TYPE_NUM_ENTRIES: xkb_action_type = 21;
    pub const ACTION_TYPE_INTERNAL: xkb_action_type = 20;
    pub const ACTION_TYPE_PRIVATE: xkb_action_type = 19;
    pub const ACTION_TYPE_UNKNOWN: xkb_action_type = 18;
    pub const ACTION_TYPE_UNSUPPORTED_LEGACY: xkb_action_type = 17;
    pub const ACTION_TYPE_REDIRECT_KEY: xkb_action_type = 16;
    pub const ACTION_TYPE_CTRL_LOCK: xkb_action_type = 15;
    pub const ACTION_TYPE_CTRL_SET: xkb_action_type = 14;
    pub const ACTION_TYPE_SWITCH_VT: xkb_action_type = 13;
    pub const ACTION_TYPE_TERMINATE: xkb_action_type = 12;
    pub const ACTION_TYPE_PTR_DEFAULT: xkb_action_type = 11;
    pub const ACTION_TYPE_PTR_LOCK: xkb_action_type = 10;
    pub const ACTION_TYPE_PTR_BUTTON: xkb_action_type = 9;
    pub const ACTION_TYPE_PTR_MOVE: xkb_action_type = 8;
    pub const ACTION_TYPE_GROUP_LOCK: xkb_action_type = 7;
    pub const ACTION_TYPE_GROUP_LATCH: xkb_action_type = 6;
    pub const ACTION_TYPE_GROUP_SET: xkb_action_type = 5;
    pub const ACTION_TYPE_MOD_LOCK: xkb_action_type = 4;
    pub const ACTION_TYPE_MOD_LATCH: xkb_action_type = 3;
    pub const ACTION_TYPE_MOD_SET: xkb_action_type = 2;
    pub const ACTION_TYPE_VOID: xkb_action_type = 1;
    pub const ACTION_TYPE_NONE: xkb_action_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_private_action {
        pub type_0: xkb_action_type,
        pub data: [uint8_t; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_redirect_key_action {
        pub type_0: xkb_action_type,
        pub keycode: xkb_keycode_t,
        pub affect: xkb_mod_mask_t,
        pub mods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_button_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub count: uint8_t,
        pub button: uint8_t,
    }
    pub type xkb_action_flags = ::core::ffi::c_uint;
    pub const ACTION_PENDING_COMPUTATION: xkb_action_flags = 8192;
    pub const ACTION_LATCH_ON_PRESS: xkb_action_flags = 4096;
    pub const ACTION_UNLOCK_ON_PRESS: xkb_action_flags = 2048;
    pub const ACTION_LOCK_ON_RELEASE: xkb_action_flags = 1024;
    pub const ACTION_SAME_SCREEN: xkb_action_flags = 512;
    pub const ACTION_ACCEL: xkb_action_flags = 256;
    pub const ACTION_ABSOLUTE_Y: xkb_action_flags = 128;
    pub const ACTION_ABSOLUTE_X: xkb_action_flags = 64;
    pub const ACTION_ABSOLUTE_SWITCH: xkb_action_flags = 32;
    pub const ACTION_MODS_LOOKUP_MODMAP: xkb_action_flags = 16;
    pub const ACTION_LOCK_NO_UNLOCK: xkb_action_flags = 8;
    pub const ACTION_LOCK_NO_LOCK: xkb_action_flags = 4;
    pub const ACTION_LATCH_TO_LOCK: xkb_action_flags = 2;
    pub const ACTION_LOCK_CLEAR: xkb_action_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub x: int16_t,
        pub y: int16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_switch_screen_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub screen: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_pointer_default_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub value: int8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_controls_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub ctrls: xkb_action_controls,
    }
    pub type xkb_action_controls = ::core::ffi::c_uint;
    pub const CONTROL_ALL_BOOLEAN: xkb_action_controls = 2088447;
    pub const CONTROL_ALL_BOOLEAN_V1: xkb_action_controls = 2087943;
    pub const CONTROL_ALL: xkb_action_controls = 2088959;
    pub const CONTROL_ALL_V1: xkb_action_controls = 2088455;
    pub const CONTROL_IGNORE_GROUP_LOCK: xkb_action_controls = 1048576;
    pub const CONTROL_BELL: xkb_action_controls = 524288;
    pub const CONTROL_AX_FEEDBACK: xkb_action_controls = 262144;
    pub const CONTROL_AX_TIMEOUT: xkb_action_controls = 131072;
    pub const CONTROL_AX: xkb_action_controls = 65536;
    pub const CONTROL_MOUSE_KEYS_ACCEL: xkb_action_controls = 32768;
    pub const CONTROL_MOUSE_KEYS: xkb_action_controls = 16384;
    pub const CONTROL_DEBOUNCE: xkb_action_controls = 4096;
    pub const CONTROL_SLOW: xkb_action_controls = 2048;
    pub const CONTROL_REPEAT: xkb_action_controls = 1024;
    pub const CONTROL_GROUPS_WRAP: xkb_action_controls = 512;
    pub const CONTROL_OVERLAY8: xkb_action_controls = 256;
    pub const CONTROL_OVERLAY7: xkb_action_controls = 128;
    pub const CONTROL_OVERLAY6: xkb_action_controls = 64;
    pub const CONTROL_OVERLAY5: xkb_action_controls = 32;
    pub const CONTROL_OVERLAY4: xkb_action_controls = 16;
    pub const CONTROL_OVERLAY3: xkb_action_controls = 8;
    pub const CONTROL_OVERLAY2: xkb_action_controls = 4;
    pub const CONTROL_OVERLAY1: xkb_action_controls = 2;
    pub const CONTROL_STICKY_KEYS: xkb_action_controls = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_group_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub group: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod_action {
        pub type_0: xkb_action_type,
        pub flags: xkb_action_flags,
        pub mods: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mods {
        pub mods: xkb_mod_mask_t,
        pub mask: xkb_mod_mask_t,
    }
    pub type xkb_action_count_t = uint16_t;
    pub type xkb_match_operation = ::core::ffi::c_uint;
    pub const MATCH_EXACTLY: xkb_match_operation = 4;
    pub const MATCH_ALL: xkb_match_operation = 3;
    pub const MATCH_ANY: xkb_match_operation = 2;
    pub const MATCH_ANY_OR_NONE: xkb_match_operation = 1;
    pub const MATCH_NONE: xkb_match_operation = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
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
    pub struct xkb_key_type_entry {
        pub level: xkb_level_index_t,
        pub mods: xkb_mods,
        pub preserve: xkb_mods,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_3 {
        pub c2rust_unnamed: C2Rust_Unnamed_5,
        pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        pub num_key_aliases: darray_size_t,
        pub key_aliases: *mut xkb_key_alias,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_key_alias {
        pub real: xkb_atom_t,
        pub alias: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        pub num_key_names: darray_size_t,
        pub key_names: *mut KeycodeMatch,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union KeycodeMatch {
        pub c2rust_unnamed: C2Rust_Unnamed_8,
        pub key: C2Rust_Unnamed_7,
        pub alias: C2Rust_Unnamed_6,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_6 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "real", ty = "xkb_atom_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_real: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_7 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "low", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "index", ty = "darray_size_t", bits = "3..=31")]
        pub found_low_is_alias_index: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_8 {
        #[bitfield(name = "found", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "c2rust_unnamed", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "is_alias", ty = "bool", bits = "2..=2")]
        #[bitfield(name = "c2rust_unnamed_0", ty = "darray_size_t", bits = "3..=31")]
        pub found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [u8; 4],
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
    pub union C2Rust_Unnamed_9 {
        pub overlay_key: *const xkb_key,
        pub overlays_keys: *mut *const xkb_key,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
    pub struct xkb_level {
        pub num_syms: xkb_keysym_count_t,
        pub num_actions: xkb_action_count_t,
        pub c2rust_unnamed: C2Rust_Unnamed_12,
        pub s: C2Rust_Unnamed_11,
        pub a: C2Rust_Unnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_10 {
        pub action: xkb_action,
        pub actions: *mut xkb_action,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_11 {
        pub sym: xkb_keysym_t,
        pub syms: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_12 {
        pub upper: xkb_keysym_t,
        pub has_upper: bool,
    }
    pub type xkb_keysym_count_t = uint16_t;
    pub type xkb_overlay_mask_t = uint8_t;
    pub type xkb_explicit_components = ::core::ffi::c_uint;
    pub const EXPLICIT_OVERLAY: xkb_explicit_components = 32;
    pub const EXPLICIT_REPEAT: xkb_explicit_components = 16;
    pub const EXPLICIT_VMODMAP: xkb_explicit_components = 8;
    pub const EXPLICIT_TYPES: xkb_explicit_components = 4;
    pub const EXPLICIT_INTERP: xkb_explicit_components = 2;
    pub const EXPLICIT_SYMBOLS: xkb_explicit_components = 1;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
    pub type xkb_overlay_index_t = uint8_t;
    pub const XKB_MAX_LEDS: xkb_led_index_t = (::core::mem::size_of::<xkb_led_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_led_index_t;
    pub const XKB_KEYCODE_MAX_CONTIGUOUS: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::limits_h::CHAR_BIT;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::xkbcommon_h::{
        xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format, xkb_keysym_t,
        xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy, xkb_led_index_t,
        xkb_led_mask_t, xkb_level_index_t, xkb_mod_index_t, xkb_mod_mask_t, xkb_state_component,
    };
    extern "C" {
        pub fn XkbEscapeMapName(name: *mut ::core::ffi::c_char);
    }
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
    pub type xkb_message_code = ::core::ffi::c_uint;
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
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
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
pub mod ast_h {
    pub type xkb_file_type = ::core::ffi::c_uint;
    pub const FILE_TYPE_INVALID: xkb_file_type = 7;
    pub const _FILE_TYPE_NUM_ENTRIES: xkb_file_type = 7;
    pub const FILE_TYPE_RULES: xkb_file_type = 6;
    pub const FILE_TYPE_KEYMAP: xkb_file_type = 5;
    pub const FILE_TYPE_GEOMETRY: xkb_file_type = 4;
    pub const LAST_KEYMAP_FILE_TYPE: xkb_file_type = 3;
    pub const FIRST_KEYMAP_FILE_TYPE: xkb_file_type = 0;
    pub const FILE_TYPE_SYMBOLS: xkb_file_type = 3;
    pub const FILE_TYPE_COMPAT: xkb_file_type = 2;
    pub const FILE_TYPE_TYPES: xkb_file_type = 1;
    pub const FILE_TYPE_KEYCODES: xkb_file_type = 0;
    pub type stmt_type = ::core::ffi::c_uint;
    pub const _STMT_NUM_VALUES: stmt_type = 37;
    pub const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
    pub const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
    pub const STMT_LED_NAME: stmt_type = 34;
    pub const STMT_LED_MAP: stmt_type = 33;
    pub const STMT_GROUP_COMPAT: stmt_type = 32;
    pub const STMT_MODMAP: stmt_type = 31;
    pub const STMT_SYMBOLS: stmt_type = 30;
    pub const STMT_VMOD: stmt_type = 29;
    pub const STMT_INTERP: stmt_type = 28;
    pub const STMT_TYPE: stmt_type = 27;
    pub const STMT_VAR: stmt_type = 26;
    pub const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
    pub const STMT_EXPR_INVERT: stmt_type = 24;
    pub const STMT_EXPR_NEGATE: stmt_type = 23;
    pub const STMT_EXPR_NOT: stmt_type = 22;
    pub const STMT_EXPR_ASSIGN: stmt_type = 21;
    pub const STMT_EXPR_DIVIDE: stmt_type = 20;
    pub const STMT_EXPR_MULTIPLY: stmt_type = 19;
    pub const STMT_EXPR_SUBTRACT: stmt_type = 18;
    pub const STMT_EXPR_ADD: stmt_type = 17;
    pub const STMT_EXPR_ACTION_LIST: stmt_type = 16;
    pub const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
    pub const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
    pub const STMT_EXPR_ARRAY_REF: stmt_type = 13;
    pub const STMT_EXPR_FIELD_REF: stmt_type = 12;
    pub const STMT_EXPR_ACTION_DECL: stmt_type = 11;
    pub const STMT_EXPR_IDENT: stmt_type = 10;
    pub const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
    pub const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
    pub const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
    pub const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
    pub const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
    pub const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
    pub const STMT_ALIAS: stmt_type = 3;
    pub const STMT_KEYCODE: stmt_type = 2;
    pub const STMT_INCLUDE: stmt_type = 1;
    pub const STMT_UNKNOWN: stmt_type = 0;
    pub type merge_mode = ::core::ffi::c_uint;
    pub const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
    pub const MERGE_REPLACE: merge_mode = 3;
    pub const MERGE_OVERRIDE: merge_mode = 2;
    pub const MERGE_AUGMENT: merge_mode = 1;
    pub const MERGE_DEFAULT: merge_mode = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ParseCommon {
        pub next: *mut _ParseCommon,
        pub type_0: stmt_type,
    }
    pub type ParseCommon = _ParseCommon;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _IncludeStmt {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub stmt: *mut ::core::ffi::c_char,
        pub file: *mut ::core::ffi::c_char,
        pub map: *mut ::core::ffi::c_char,
        pub modifier: *mut ::core::ffi::c_char,
        pub next_incl: *mut _IncludeStmt,
    }
    pub type IncludeStmt = _IncludeStmt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ExprDef {
        pub common: ParseCommon,
        pub ident: ExprIdent,
        pub string: ExprString,
        pub boolean: ExprBoolean,
        pub integer: ExprInteger,
        pub key_name: ExprKeyName,
        pub keysym: ExprKeySym,
        pub binary: ExprBinary,
        pub unary: ExprUnary,
        pub field_ref: ExprFieldRef,
        pub array_ref: ExprArrayRef,
        pub action: ExprAction,
        pub actions: ExprActionList,
        pub keysym_list: ExprKeysymList,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeysymList {
        pub common: ParseCommon,
        pub syms: C2Rust_Unnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_13 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprActionList {
        pub common: ParseCommon,
        pub actions: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprAction {
        pub common: ParseCommon,
        pub name: xkb_atom_t,
        pub args: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprArrayRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
        pub entry: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprFieldRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprUnary {
        pub common: ParseCommon,
        pub child: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBinary {
        pub common: ParseCommon,
        pub left: *mut ExprDef,
        pub right: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeySym {
        pub common: ParseCommon,
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeyName {
        pub common: ParseCommon,
        pub key_name: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprInteger {
        pub common: ParseCommon,
        pub ival: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBoolean {
        pub common: ParseCommon,
        pub set: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprString {
        pub common: ParseCommon,
        pub str: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprIdent {
        pub common: ParseCommon,
        pub ident: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VarDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: *mut ExprDef,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct KeycodeDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct KeyAliasDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub alias: xkb_atom_t,
        pub real: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LedNameDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub virtual_0: bool,
        pub ndx: int64_t,
        pub name: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct UnknownStatement {
        pub common: ParseCommon,
        pub name: *mut ::core::ffi::c_char,
    }
    pub type xkb_map_flags = ::core::ffi::c_uint;
    pub const MAP_IS_ALTGR: xkb_map_flags = 128;
    pub const MAP_HAS_FN: xkb_map_flags = 64;
    pub const MAP_HAS_KEYPAD: xkb_map_flags = 32;
    pub const MAP_HAS_MODIFIER: xkb_map_flags = 16;
    pub const MAP_HAS_ALPHANUMERIC: xkb_map_flags = 8;
    pub const MAP_IS_HIDDEN: xkb_map_flags = 4;
    pub const MAP_IS_PARTIAL: xkb_map_flags = 2;
    pub const MAP_IS_DEFAULT: xkb_map_flags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XkbFile {
        pub common: ParseCommon,
        pub name: *mut ::core::ffi::c_char,
        pub defs: *mut ParseCommon,
        pub file_type: xkb_file_type,
        pub flags: xkb_map_flags,
    }
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::stdint_intn_h::int64_t;
    use super::xkbcommon_h::xkb_keysym_t;
    extern "C" {
        pub fn stmt_type_to_string(type_0: stmt_type) -> *const ::core::ffi::c_char;
    }
}
pub mod text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LookupEntry {
        pub name: *const ::core::ffi::c_char,
        pub value: u32,
    }
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn KeyNameText(ctx: *mut xkb_context, name: xkb_atom_t) -> *const ::core::ffi::c_char;
    }
}
pub mod xkbcomp_priv_h {
    pub type xkb_parser_strict_flags = ::core::ffi::c_uint;
    pub const PARSER_V2_LAX_FLAGS: xkb_parser_strict_flags = 0;
    pub const PARSER_V2_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
    pub const PARSER_V1_LAX_FLAGS: xkb_parser_strict_flags = 16379;
    pub const PARSER_V1_STRICT_FLAGS: xkb_parser_strict_flags = 16383;
    pub const PARSER_NO_ILLEGAL_ACTION_FIELDS: xkb_parser_strict_flags = 8192;
    pub const PARSER_NO_UNKNOWN_ACTION_FIELDS: xkb_parser_strict_flags = 4096;
    pub const PARSER_NO_UNKNOWN_ACTION: xkb_parser_strict_flags = 2048;
    pub const PARSER_NO_UNKNOWN_KEY_FIELDS: xkb_parser_strict_flags = 1024;
    pub const PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS: xkb_parser_strict_flags = 512;
    pub const PARSER_NO_UNKNOWN_LED_FIELDS: xkb_parser_strict_flags = 256;
    pub const PARSER_NO_UNKNOWN_INTERPRET_FIELDS: xkb_parser_strict_flags = 128;
    pub const PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS: xkb_parser_strict_flags = 64;
    pub const PARSER_NO_UNKNOWN_TYPE_FIELDS: xkb_parser_strict_flags = 32;
    pub const PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS: xkb_parser_strict_flags = 16;
    pub const PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS: xkb_parser_strict_flags = 8;
    pub const PARSER_NO_FIELD_VALUE_MISMATCH: xkb_parser_strict_flags = 4;
    pub const PARSER_NO_FIELD_TYPE_MISMATCH: xkb_parser_strict_flags = 2;
    pub const PARSER_NO_UNKNOWN_STATEMENTS: xkb_parser_strict_flags = 1;
    pub const PARSER_NO_STRICT_FLAGS: xkb_parser_strict_flags = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pending_computation {
        pub expr: *mut ExprDef,
        pub computed: bool,
        pub value: u32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct pending_computation_array {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut pending_computation,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_keymap_info {
        pub keymap: xkb_keymap,
        pub strict: xkb_parser_strict_flags,
        pub features: C2Rust_Unnamed_15,
        pub lookup: C2Rust_Unnamed_14,
        pub pending_computations: *mut pending_computation_array,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_14 {
        pub groupIndexNames: [LookupEntry; 3],
        pub groupMaskNames: [LookupEntry; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_15 {
        pub max_groups: xkb_layout_index_t,
        pub max_overlays: xkb_overlay_index_t,
        pub controls_name_offset: uint8_t,
        pub group_lock_on_release: bool,
        pub mods_unlock_on_press: bool,
        pub mods_latch_on_press: bool,
        pub overlapping_overlays: bool,
    }
    #[inline]
    pub unsafe extern "C" fn ReportNotArray(
        mut ctx: *mut xkb_context,
        mut type_0: *const ::core::ffi::c_char,
        mut field: *const ::core::ffi::c_char,
        mut name: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] The %s %s field is not an array; Ignoring illegal assignment in %s\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_WRONG_FIELD_TYPE as ::core::ffi::c_int,
                type_0,
                field,
                name,
            );
            return false_0 != 0;
        }
    }
    #[inline]
    pub unsafe extern "C" fn ReportBadType(
        mut ctx: *mut xkb_context,
        mut code: xkb_message_code,
        mut type_0: *const ::core::ffi::c_char,
        mut field: *const ::core::ffi::c_char,
        mut name: *const ::core::ffi::c_char,
        mut wanted: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] The %s %s field must be a %s; Ignoring illegal assignment in %s\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                code as ::core::ffi::c_uint,
                type_0,
                field,
                wanted,
                name,
            );
            return false_0 != 0;
        }
    }
    #[inline]
    pub unsafe extern "C" fn safe_map_name(mut file: *mut XkbFile) -> *const ::core::ffi::c_char {
        unsafe {
            return if !(*file).name.is_null() {
                (*file).name as *const ::core::ffi::c_char
            } else {
                b"(unnamed map)\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
    }
    use super::ast_h::{ExprDef, XkbFile};
    use super::context_h::{xkb_context, xkb_log};
    use super::darray_h::darray_size_t;
    use super::keymap_h::{xkb_keymap, xkb_overlay_index_t};
    use super::messages_codes_h::{
        xkb_message_code, XKB_ERROR_WRONG_FIELD_TYPE, XKB_LOG_VERBOSITY_MINIMAL,
    };
    use super::stdbool_h::false_0;
    use super::stdint_uintn_h::{u32, uint8_t};
    use super::text_h::LookupEntry;
    use super::xkbcommon_h::{xkb_layout_index_t, XKB_LOG_LEVEL_ERROR};
    extern "C" {
        pub fn FreeXkbFile(file: *mut XkbFile);
    }
}
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
pub mod stdio_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn istreq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            return istrcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn strdup_safe(
        mut s: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        unsafe {
            return if !s.is_null() {
                strdup(s)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        }
    }

    use super::string_h::strdup;
    extern "C" {
        pub fn istrcmp(
            a: *const ::core::ffi::c_char,
            b: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::stdint_intn_h::int64_t;
    extern "C" {
        pub fn ExprResolveLhs(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            elem_rtrn: *mut *const ::core::ffi::c_char,
            field_rtrn: *mut *const ::core::ffi::c_char,
            index_rtrn: *mut *mut ExprDef,
        ) -> bool;
        pub fn ExprResolveInteger(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut int64_t,
        ) -> bool;
        pub fn ExprResolveString(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut xkb_atom_t,
        ) -> bool;
    }
}
pub mod util_mem_h {
    #[inline]
    pub unsafe extern "C" fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
            let mut swapped: *mut ::core::ffi::c_void = *original;
            *original = NULL;
            return swapped;
        }
    }
    use super::__stddef_null_h::NULL;
}
pub mod include_h {
    use super::__stddef_size_t_h::size_t;
    use super::ast_h::{xkb_file_type, IncludeStmt, XkbFile};
    use super::context_h::xkb_context;
    extern "C" {
        pub fn ExceedsIncludeMaxDepth(
            ctx: *mut xkb_context,
            include_depth: ::core::ffi::c_uint,
        ) -> bool;
        pub fn ProcessIncludeFile(
            ctx: *mut xkb_context,
            stmt: *const IncludeStmt,
            file_type: xkb_file_type,
            path: *mut ::core::ffi::c_char,
            path_size: size_t,
        ) -> *mut XkbFile;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_13, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, KeyAliasDef, KeycodeDef, LedNameDef,
    ParseCommon, UnknownStatement, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY,
    FILE_TYPE_INVALID, FILE_TYPE_KEYCODES, FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS,
    FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE, LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC,
    MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER, MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN,
    MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS,
    STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF,
    STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST,
    STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL,
    STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL,
    STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL,
    STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP,
    STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN,
    STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
use self::expr_h::{ExprResolveInteger, ExprResolveLhs, ExprResolveString};
use self::include_h::{ExceedsIncludeMaxDepth, ProcessIncludeFile};
pub use self::internal::{__va_list_tag, __CHAR_BIT__};
pub use self::keymap_h::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_index_t, xkb_overlay_mask_t,
    xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3,
    C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8,
    C2Rust_Unnamed_9, KeycodeMatch, XkbEscapeMapName, _ACTION_TYPE_NUM_ENTRIES,
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
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT,
    XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS,
};
pub use self::limits_h::CHAR_BIT;
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
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::UINT32_MAX;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdio_h::snprintf;
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::{memcpy, memmove, memset};
pub use self::text_h::{KeyNameText, LookupEntry};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, strdup_safe};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t,
    xkb_mod_mask_t, xkb_rule_names, xkb_state_component, XKB_KEYCODE_INVALID, XKB_KEYCODE_MAX,
    XKB_KEYMAP_COMPILE_NO_FLAGS, XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1,
    XKB_KEYMAP_FORMAT_TEXT_V2, XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT,
    XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR,
    XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_strict_flags, C2Rust_Unnamed_14, C2Rust_Unnamed_15, FreeXkbFile, ReportBadType,
    ReportNotArray, PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH,
    PARSER_NO_ILLEGAL_ACTION_FIELDS, PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION,
    PARSER_NO_UNKNOWN_ACTION_FIELDS, PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_INTERPRET_FIELDS, PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_KEY_FIELDS, PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_V1_LAX_FLAGS, PARSER_V1_STRICT_FLAGS,
    PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyNamesInfo {
    pub name: *mut ::core::ffi::c_char,
    pub errorCount: ::core::ffi::c_int,
    pub include_depth: ::core::ffi::c_uint,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: xkb_led_index_t,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedNameInfo {
    pub merge: merge_mode,
    pub name: xkb_atom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeycodeStore {
    pub min: xkb_keycode_t,
    pub low: C2Rust_Unnamed_18,
    pub high: C2Rust_Unnamed_17,
    pub names: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut KeycodeMatch,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut HighKeycodeEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighKeycodeEntry {
    pub keycode: xkb_keycode_t,
    pub name: xkb_atom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_atom_t,
}
#[inline]
unsafe extern "C" fn keycode_store_init(mut store: *mut KeycodeStore) {
    unsafe {
        (*store).low.item = ::core::ptr::null_mut::<xkb_atom_t>();
        (*store).low.size = 0 as darray_size_t;
        (*store).low.alloc = 0 as darray_size_t;
        (*store).high.item = ::core::ptr::null_mut::<HighKeycodeEntry>();
        (*store).high.size = 0 as darray_size_t;
        (*store).high.alloc = 0 as darray_size_t;
        (*store).names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*store).names.size = 0 as darray_size_t;
        (*store).names.alloc = 0 as darray_size_t;
        (*store).min = XKB_KEYCODE_INVALID as xkb_keycode_t;
    }
}
#[inline]
unsafe extern "C" fn keycode_store_free(mut store: *mut KeycodeStore) {
    unsafe {
        free((*store).low.item as *mut ::core::ffi::c_void);
        (*store).low.item = ::core::ptr::null_mut::<xkb_atom_t>();
        (*store).low.size = 0 as darray_size_t;
        (*store).low.alloc = 0 as darray_size_t;
        free((*store).high.item as *mut ::core::ffi::c_void);
        (*store).high.item = ::core::ptr::null_mut::<HighKeycodeEntry>();
        (*store).high.size = 0 as darray_size_t;
        (*store).high.alloc = 0 as darray_size_t;
        free((*store).names.item as *mut ::core::ffi::c_void);
        (*store).names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*store).names.size = 0 as darray_size_t;
        (*store).names.alloc = 0 as darray_size_t;
    }
}
#[inline]
unsafe extern "C" fn keycode_store_update_key(
    mut store: *mut KeycodeStore,
    mut match_0: KeycodeMatch,
    mut name: xkb_atom_t,
) {
    unsafe {
        if (!match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return;
        } else if match_0.key.low() {
            if match_0.key.index() < (*store).low.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->low)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    76 as ::core::ffi::c_uint,
                    b"void keycode_store_update_key(KeycodeStore *, KeycodeMatch, xkb_atom_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            *(*store).low.item.offset(match_0.key.index() as isize) = name;
        } else {
            if match_0.key.index() < (*store).high.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->high)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    80 as ::core::ffi::c_uint,
                    b"void keycode_store_update_key(KeycodeStore *, KeycodeMatch, xkb_atom_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            (*(*store).high.item.offset(match_0.key.index() as isize)).name = name;
        }
        if name >= (*store).names.size {
            let mut __oldSize: darray_size_t = (*store).names.size;
            let mut __newSize: darray_size_t =
                (name as darray_size_t).wrapping_add(1 as darray_size_t);
            (*store).names.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*store).names.alloc {
                    (*store).names.alloc = darray_next_alloc(
                        (*store).names.alloc,
                        __need,
                        ::core::mem::size_of::<KeycodeMatch>() as size_t,
                    );
                    (*store).names.item = realloc(
                        (*store).names.item as *mut ::core::ffi::c_void,
                        ((*store).names.alloc as size_t)
                            .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                    ) as *mut KeycodeMatch;
                }
                memset(
                    (*store).names.item.offset(__oldSize as isize) as *mut KeycodeMatch
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as size_t)
                        .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                );
            }
        }
        *(*store).names.item.offset(name as isize) = match_0;
    }
}
unsafe extern "C" fn keycode_store_insert_key(
    mut store: *mut KeycodeStore,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
) -> bool {
    unsafe {
        if name >= (*store).names.size {
            let mut __oldSize: darray_size_t = (*store).names.size;
            let mut __newSize: darray_size_t =
                (name as darray_size_t).wrapping_add(1 as darray_size_t);
            (*store).names.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*store).names.alloc {
                    (*store).names.alloc = darray_next_alloc(
                        (*store).names.alloc,
                        __need,
                        ::core::mem::size_of::<KeycodeMatch>() as size_t,
                    );
                    (*store).names.item = realloc(
                        (*store).names.item as *mut ::core::ffi::c_void,
                        ((*store).names.alloc as size_t)
                            .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                    ) as *mut KeycodeMatch;
                }
                memset(
                    (*store).names.item.offset(__oldSize as isize) as *mut KeycodeMatch
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as size_t)
                        .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                );
            }
        }
        if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            if kc >= (*store).low.size as xkb_keycode_t {
                let mut __oldSize_0: darray_size_t = (*store).low.size;
                let mut __newSize_0: darray_size_t =
                    (kc as darray_size_t).wrapping_add(1 as darray_size_t);
                (*store).low.size = __newSize_0;
                if __newSize_0 > __oldSize_0 {
                    let mut __need_0: darray_size_t = __newSize_0;
                    if __need_0 > (*store).low.alloc {
                        (*store).low.alloc = darray_next_alloc(
                            (*store).low.alloc,
                            __need_0,
                            ::core::mem::size_of::<xkb_atom_t>() as size_t,
                        );
                        (*store).low.item =
                            realloc(
                                (*store).low.item as *mut ::core::ffi::c_void,
                                ((*store).low.alloc as size_t)
                                    .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as size_t),
                            ) as *mut xkb_atom_t;
                    }
                    memset(
                        (*store).low.item.offset(__oldSize_0 as isize) as *mut xkb_atom_t
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (__newSize_0.wrapping_sub(__oldSize_0) as size_t)
                            .wrapping_mul(::core::mem::size_of::<xkb_atom_t>() as size_t),
                    );
                }
            }
            *(*store).low.item.offset(kc as isize) = name;
            if kc < (*store).min {
                (*store).min = kc;
            }
            *(*store).names.item.offset(name as isize) = KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true_0 != 0);
                    init.set_low(true_0 != 0);
                    init.set_is_alias(false_0 != 0);
                    init.set_index(kc as darray_size_t);
                    init
                },
            };
        } else {
            let idx: darray_size_t = (*store).high.size;
            if idx != 0
                && (*(*store)
                    .high
                    .item
                    .offset(idx.wrapping_sub(1 as darray_size_t) as isize))
                .keycode
                    > kc
            {
                let mut lower: darray_size_t = 0 as darray_size_t;
                let mut upper: darray_size_t = idx;
                while lower < upper {
                    let mid: darray_size_t = lower.wrapping_add(
                        upper
                            .wrapping_sub(1 as darray_size_t)
                            .wrapping_sub(lower)
                            .wrapping_div(2 as darray_size_t),
                    );
                    let entry: *const HighKeycodeEntry =
                        (*store).high.item.offset(mid as isize) as *mut HighKeycodeEntry;
                    if (*entry).keycode < kc {
                        lower = mid.wrapping_add(1 as darray_size_t);
                    } else if (*entry).keycode > kc {
                        upper = mid;
                    } else {
                        if (*entry).keycode != kc {
                        } else {
                            __assert_fail(
                                b"entry->keycode != kc\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../src/xkbcomp/keycodes.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                132 as ::core::ffi::c_uint,
                                b"_Bool keycode_store_insert_key(KeycodeStore *, xkb_keycode_t, xkb_atom_t)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        };
                    }
                }
                if lower < idx {
                } else {
                    __assert_fail(
                        b"lower < idx\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/keycodes.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_uint,
                        b"_Bool keycode_store_insert_key(KeycodeStore *, xkb_keycode_t, xkb_atom_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*(*store).high.item.offset(lower as isize)).keycode > kc {
                } else {
                    __assert_fail(
                        b"darray_item(store->high, lower).keycode > kc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/keycodes.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        136 as ::core::ffi::c_uint,
                        b"_Bool keycode_store_insert_key(KeycodeStore *, xkb_keycode_t, xkb_atom_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                let mut entry_0: *mut HighKeycodeEntry =
                    ::core::ptr::null_mut::<HighKeycodeEntry>();
                if !(*store).high.item.is_null() {
                    entry_0 = (*store).high.item.offset(lower as isize) as *mut HighKeycodeEntry;
                    while entry_0
                        < (*store).high.item.offset((*store).high.size as isize)
                            as *mut HighKeycodeEntry
                    {
                        let ref mut c2rust_fresh4 =
                            (*(*store).names.item.offset((*entry_0).name as isize)).key;
                        (*c2rust_fresh4).set_index((*c2rust_fresh4).index() + 1 as darray_size_t);
                        entry_0 = entry_0.offset(1);
                    }
                }
                let mut __index: darray_size_t = lower;
                (*store).high.size = (*store).high.size.wrapping_add(1 as darray_size_t);
                let mut __need_1: darray_size_t = (*store).high.size;
                if __need_1 > (*store).high.alloc {
                    (*store).high.alloc = darray_next_alloc(
                        (*store).high.alloc,
                        __need_1,
                        ::core::mem::size_of::<HighKeycodeEntry>() as size_t,
                    );
                    (*store).high.item = realloc(
                        (*store).high.item as *mut ::core::ffi::c_void,
                        ((*store).high.alloc as size_t)
                            .wrapping_mul(::core::mem::size_of::<HighKeycodeEntry>() as size_t),
                    ) as *mut HighKeycodeEntry;
                }
                memmove(
                    (*store)
                        .high
                        .item
                        .offset(__index as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    (*store).high.item.offset(__index as isize) as *const ::core::ffi::c_void,
                    ((*store)
                        .high
                        .size
                        .wrapping_sub(__index)
                        .wrapping_sub(1 as darray_size_t) as size_t)
                        .wrapping_mul(::core::mem::size_of::<HighKeycodeEntry>() as size_t),
                );
                *(*store).high.item.offset(__index as isize) = HighKeycodeEntry {
                    keycode: kc,
                    name: name,
                };
                *(*store).names.item.offset(name as isize) = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(lower);
                        init
                    },
                };
            } else {
                (*store).high.size = (*store).high.size.wrapping_add(1 as darray_size_t);
                let mut __need_2: darray_size_t = (*store).high.size;
                if __need_2 > (*store).high.alloc {
                    (*store).high.alloc = darray_next_alloc(
                        (*store).high.alloc,
                        __need_2,
                        ::core::mem::size_of::<HighKeycodeEntry>() as size_t,
                    );
                    (*store).high.item = realloc(
                        (*store).high.item as *mut ::core::ffi::c_void,
                        ((*store).high.alloc as size_t)
                            .wrapping_mul(::core::mem::size_of::<HighKeycodeEntry>() as size_t),
                    ) as *mut HighKeycodeEntry;
                }
                *(*store)
                    .high
                    .item
                    .offset((*store).high.size.wrapping_sub(1 as darray_size_t) as isize) =
                    HighKeycodeEntry {
                        keycode: kc,
                        name: name,
                    };
                *(*store).names.item.offset(name as isize) = KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(idx);
                        init
                    },
                };
            }
            if (*store).low.size == 0 as darray_size_t {
                (*store).min =
                    (*(*store).high.item.offset(0 as ::core::ffi::c_int as isize)).keycode;
            }
        }
        return true_0 != 0;
    }
}
#[inline]
unsafe extern "C" fn keycode_store_insert_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        if alias >= (*store).names.size {
            let mut __oldSize: darray_size_t = (*store).names.size;
            let mut __newSize: darray_size_t =
                (alias as darray_size_t).wrapping_add(1 as darray_size_t);
            (*store).names.size = __newSize;
            if __newSize > __oldSize {
                let mut __need: darray_size_t = __newSize;
                if __need > (*store).names.alloc {
                    (*store).names.alloc = darray_next_alloc(
                        (*store).names.alloc,
                        __need,
                        ::core::mem::size_of::<KeycodeMatch>() as size_t,
                    );
                    (*store).names.item = realloc(
                        (*store).names.item as *mut ::core::ffi::c_void,
                        ((*store).names.alloc as size_t)
                            .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                    ) as *mut KeycodeMatch;
                }
                memset(
                    (*store).names.item.offset(__oldSize as isize) as *mut KeycodeMatch
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (__newSize.wrapping_sub(__oldSize) as size_t)
                        .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
                );
            }
        }
        *(*store).names.item.offset(alias as isize) = KeycodeMatch {
            alias: {
                let mut init = C2Rust_Unnamed_6 {
                    found_c2rust_unnamed_is_alias_real: [0; 4],
                };
                init.set_found(true_0 != 0);
                init.set_c2rust_unnamed(true_0 != 0);
                init.set_is_alias(real != 0);
                init.set_real(real);
                init
            },
        };
        return true_0 != 0;
    }
}
#[inline]
unsafe extern "C" fn keycode_store_update_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        let ref mut c2rust_fresh3 = (*(*store).names.item.offset(alias as isize)).alias;
        (*c2rust_fresh3).set_real(real as xkb_atom_t);
        return true_0 != 0;
    }
}
#[inline]
unsafe extern "C" fn keycode_store_delete_name(
    mut store: *const KeycodeStore,
    mut name: xkb_atom_t,
) {
    unsafe {
        if name < (*store).names.size {
            let ref mut c2rust_fresh5 = (*(*store).names.item.offset(name as isize)).c2rust_unnamed;
            (*c2rust_fresh5).set_found((false_0 != 0) as bool);
        }
    }
}
unsafe extern "C" fn keycode_store_delete_key(mut store: *mut KeycodeStore, match_0: KeycodeMatch) {
    unsafe {
        if (!match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return;
        } else if match_0.key.low() {
            if match_0.key.index() < (*store).low.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->low)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    210 as ::core::ffi::c_uint,
                    b"void keycode_store_delete_key(KeycodeStore *, const KeycodeMatch)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let ref mut c2rust_fresh1 = (*(*store)
                .names
                .item
                .offset(*(*store).low.item.offset(match_0.key.index() as isize) as isize))
            .c2rust_unnamed;
            (*c2rust_fresh1).set_found((false_0 != 0) as bool);
            if match_0.key.index().wrapping_add(1 as ::core::ffi::c_uint) == (*store).low.size {
                if (*store).min == match_0.key.index() as xkb_keycode_t {
                    (*store).low.size = 0 as darray_size_t;
                } else {
                    let mut idx: darray_size_t = match_0.key.index();
                    while idx > 0 as darray_size_t {
                        if *(*store)
                            .low
                            .item
                            .offset(idx.wrapping_sub(1 as darray_size_t) as isize)
                            != XKB_ATOM_NONE as xkb_atom_t
                        {
                            (*store).low.size = idx;
                            break;
                        } else {
                            idx = idx.wrapping_sub(1);
                        }
                    }
                }
            } else {
                *(*store).low.item.offset(match_0.key.index() as isize) =
                    XKB_ATOM_NONE as xkb_atom_t;
            }
        } else {
            if match_0.key.index() < (*store).high.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->high)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_uint,
                    b"void keycode_store_delete_key(KeycodeStore *, const KeycodeMatch)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            let ref mut c2rust_fresh2 = (*(*store)
                .names
                .item
                .offset((*(*store).high.item.offset(match_0.key.index() as isize)).name as isize))
            .c2rust_unnamed;
            (*c2rust_fresh2).set_found((false_0 != 0) as bool);
            let mut __index: darray_size_t = match_0.key.index();
            if __index < (*store).high.size.wrapping_sub(1 as darray_size_t) {
                memmove(
                    (*store).high.item.offset(__index as isize) as *mut HighKeycodeEntry
                        as *mut ::core::ffi::c_void,
                    (*store)
                        .high
                        .item
                        .offset(__index.wrapping_add(1 as darray_size_t) as isize)
                        as *mut HighKeycodeEntry as *const ::core::ffi::c_void,
                    ((*store)
                        .high
                        .size
                        .wrapping_sub(1 as darray_size_t)
                        .wrapping_sub(__index) as size_t)
                        .wrapping_mul(::core::mem::size_of::<HighKeycodeEntry>() as size_t),
                );
            }
            (*store).high.size = (*store).high.size.wrapping_sub(1);
            let mut entry: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
            if !(*store).names.item.is_null() {
                entry = (*store).names.item.offset(0 as ::core::ffi::c_int as isize)
                    as *mut KeycodeMatch;
                while entry
                    < (*store).names.item.offset((*store).names.size as isize) as *mut KeycodeMatch
                {
                    if (*entry).c2rust_unnamed.found() as ::core::ffi::c_int != 0
                        && !(*entry).c2rust_unnamed.is_alias()
                        && !(*entry).key.low()
                        && (*entry).key.index() as ::core::ffi::c_int
                            > match_0.key.index() as ::core::ffi::c_int
                    {
                        (*entry)
                            .key
                            .set_index((*entry).key.index() - 1 as darray_size_t);
                    }
                    entry = entry.offset(1);
                }
            }
        }
        if (*store).low.size == 0 as darray_size_t {
            (*store).min = if (*store).high.size == 0 as darray_size_t {
                XKB_KEYCODE_INVALID as xkb_keycode_t
            } else {
                (*(*store).high.item.offset(0 as ::core::ffi::c_int as isize)).keycode
            };
        } else {
            let mut kc: xkb_keycode_t = (*store).min;
            while kc < (*store).low.size as xkb_keycode_t {
                if *(*store).low.item.offset(kc as isize) != XKB_ATOM_NONE as xkb_atom_t {
                    (*store).min = kc;
                    break;
                } else {
                    kc = kc.wrapping_add(1);
                }
            }
        };
    }
}
#[inline]
unsafe extern "C" fn keycode_store_get_keycode(
    mut store: *const KeycodeStore,
    mut match_0: KeycodeMatch,
) -> xkb_keycode_t {
    unsafe {
        if !match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
        {
            return XKB_KEYCODE_INVALID as xkb_keycode_t;
        } else if match_0.key.low() {
            if match_0.key.index() < (*store).low.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->low)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_uint,
                    b"xkb_keycode_t keycode_store_get_keycode(const KeycodeStore *, KeycodeMatch)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            return match_0.key.index() as xkb_keycode_t;
        } else {
            if match_0.key.index() < (*store).high.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->high)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    272 as ::core::ffi::c_uint,
                    b"xkb_keycode_t keycode_store_get_keycode(const KeycodeStore *, KeycodeMatch)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            return (*(*store).high.item.offset(match_0.key.index() as isize)).keycode;
        };
    }
}
#[inline]
unsafe extern "C" fn keycode_store_get_key_name(
    mut store: *const KeycodeStore,
    mut match_0: KeycodeMatch,
) -> xkb_atom_t {
    unsafe {
        if !match_0.c2rust_unnamed.found()
            || match_0.c2rust_unnamed.is_alias() as ::core::ffi::c_int != 0
        {
            return XKB_ATOM_NONE as xkb_atom_t;
        } else if match_0.key.low() {
            if match_0.key.index() < (*store).low.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->low)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    284 as ::core::ffi::c_uint,
                    b"xkb_atom_t keycode_store_get_key_name(const KeycodeStore *, KeycodeMatch)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            return *(*store).low.item.offset(match_0.key.index() as isize);
        } else {
            if match_0.key.index() < (*store).high.size {
            } else {
                __assert_fail(
                    b"match.key.index < darray_size(store->high)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    288 as ::core::ffi::c_uint,
                    b"xkb_atom_t keycode_store_get_key_name(const KeycodeStore *, KeycodeMatch)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            return (*(*store).high.item.offset(match_0.key.index() as isize)).name;
        };
    }
}
unsafe extern "C" fn keycode_store_lookup_keycode(
    mut store: *const KeycodeStore,
    mut kc: xkb_keycode_t,
) -> KeycodeMatch {
    unsafe {
        if kc < (*store).low.size as xkb_keycode_t {
            return KeycodeMatch {
                key: {
                    let mut init = C2Rust_Unnamed_7 {
                        found_low_is_alias_index: [0; 4],
                    };
                    init.set_found(true_0 != 0);
                    init.set_low(true_0 != 0);
                    init.set_is_alias(false_0 != 0);
                    init.set_index(kc as darray_size_t);
                    init
                },
            };
        } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            return KeycodeMatch {
                c2rust_unnamed: {
                    let mut init = C2Rust_Unnamed_8 {
                        found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                    };
                    init.set_found(false_0 != 0);
                    init.set_c2rust_unnamed(false);
                    init
                },
            };
        }
        let mut lower: darray_size_t = 0 as darray_size_t;
        let mut upper: darray_size_t = (*store).high.size;
        while lower < upper {
            let mid: darray_size_t = lower.wrapping_add(
                upper
                    .wrapping_sub(1 as darray_size_t)
                    .wrapping_sub(lower)
                    .wrapping_div(2 as darray_size_t),
            );
            let entry: *mut HighKeycodeEntry =
                (*store).high.item.offset(mid as isize) as *mut HighKeycodeEntry;
            if (*entry).keycode < kc {
                lower = mid.wrapping_add(1 as darray_size_t);
            } else if (*entry).keycode > kc {
                upper = mid;
            } else {
                return KeycodeMatch {
                    key: {
                        let mut init = C2Rust_Unnamed_7 {
                            found_low_is_alias_index: [0; 4],
                        };
                        init.set_found(true_0 != 0);
                        init.set_low(false_0 != 0);
                        init.set_is_alias(false_0 != 0);
                        init.set_index(mid);
                        init
                    },
                };
            }
        }
        return KeycodeMatch {
            c2rust_unnamed: {
                let mut init = C2Rust_Unnamed_8 {
                    found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                };
                init.set_found(false_0 != 0);
                init.set_c2rust_unnamed(false);
                init
            },
        };
    }
}
unsafe extern "C" fn keycode_store_lookup_name(
    mut store: *const KeycodeStore,
    mut name: xkb_atom_t,
) -> KeycodeMatch {
    unsafe {
        if name >= (*store).names.size {
            return KeycodeMatch {
                c2rust_unnamed: {
                    let mut init = C2Rust_Unnamed_8 {
                        found_c2rust_unnamed_is_alias_c2rust_unnamed_0: [0; 4],
                    };
                    init.set_found(false_0 != 0);
                    init.set_c2rust_unnamed(false_0 != 0);
                    init
                },
            };
        } else {
            return *(*store).names.item.offset(name as isize);
        };
    }
}
unsafe extern "C" fn FindLedByName(
    mut info: *mut KeyNamesInfo,
    mut name: xkb_atom_t,
    mut idx_out: *mut xkb_led_index_t,
) -> *mut LedNameInfo {
    unsafe {
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_led_names {
            let mut ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                .offset(idx as isize)
                as *mut LedNameInfo;
            if (*ledi).name == name {
                *idx_out = idx;
                return ledi;
            }
            idx = idx.wrapping_add(1);
        }
        return ::core::ptr::null_mut::<LedNameInfo>();
    }
}
unsafe extern "C" fn AddLedName(
    mut info: *mut KeyNamesInfo,
    mut same_file: bool,
    mut new: *mut LedNameInfo,
    mut new_idx: xkb_led_index_t,
    mut report: bool,
) -> bool {
    unsafe {
        let mut old_idx: xkb_led_index_t = 0;
        let mut old: *mut LedNameInfo = ::core::ptr::null_mut::<LedNameInfo>();
        let replace: bool = (*new).merge as ::core::ffi::c_uint
            != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
        old = FindLedByName(info, (*new).name, &raw mut old_idx);
        if !old.is_null() {
            if old_idx == new_idx {
                if report {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Multiple indicators named \"%s\"; Identical definitions ignored\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        xkb_atom_text((*info).ctx, (*new).name),
                    );
                }
                return true_0 != 0;
            }
            if report {
                let mut use_0: xkb_led_index_t = if replace as ::core::ffi::c_int != 0 {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                };
                let mut ignore: xkb_led_index_t = if replace as ::core::ffi::c_int != 0 {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                };
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Multiple indicators named %s; Using %u, ignoring %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    xkb_atom_text((*info).ctx, (*new).name),
                    use_0,
                    ignore,
                );
            }
            if replace {
                (*old).name = XKB_ATOM_NONE as xkb_atom_t;
            } else {
                return true_0 != 0;
            }
        }
        if new_idx >= (*info).num_led_names {
            (*info).num_led_names = new_idx.wrapping_add(1 as xkb_led_index_t);
        }
        old = (&raw mut (*info).led_names as *mut LedNameInfo).offset(new_idx as isize)
            as *mut LedNameInfo;
        if (*old).name != XKB_ATOM_NONE as xkb_atom_t {
            if report {
                let use_1: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    (*new).name
                } else {
                    (*old).name
                };
                let ignore_0: xkb_atom_t = if replace as ::core::ffi::c_int != 0 {
                    (*old).name
                } else {
                    (*new).name
                };
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Multiple names for indicator %u; Using %s, ignoring %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    new_idx.wrapping_add(1 as xkb_led_index_t),
                    xkb_atom_text((*info).ctx, use_1),
                    xkb_atom_text((*info).ctx, ignore_0),
                );
            }
            if replace {
                *old = *new;
            }
            return true_0 != 0;
        }
        *old = *new;
        return true_0 != 0;
    }
}
unsafe extern "C" fn ClearKeyNamesInfo(mut info: *mut KeyNamesInfo) {
    unsafe {
        free((*info).name as *mut ::core::ffi::c_void);
        keycode_store_free(&raw mut (*info).keycodes);
    }
}
unsafe extern "C" fn InitKeyNamesInfo(
    mut info: *mut KeyNamesInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: ::core::ffi::c_uint,
) {
    unsafe {
        memset(
            info as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<KeyNamesInfo>() as size_t,
        );
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        keycode_store_init(&raw mut (*info).keycodes);
    }
}
unsafe extern "C" fn AddKeyName(
    mut info: *mut KeyNamesInfo,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
    mut merge: merge_mode,
    mut report: bool,
) -> bool {
    unsafe {
        let mut match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, name);
        if match_name.c2rust_unnamed.found() {
            let clobber: bool = merge as ::core::ffi::c_uint
                != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
            if match_name.c2rust_unnamed.is_alias() {
                if report {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Key name %s already assigned to an alias; Using %s, ignoring %s\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                        KeyNameText((*info).ctx, name),
                        if clobber as ::core::ffi::c_int != 0 {
                            b"key\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"alias\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if clobber as ::core::ffi::c_int != 0 {
                            b"alias\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"key\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                if clobber {
                    keycode_store_delete_name(&raw mut (*info).keycodes, name);
                    match_name.c2rust_unnamed.set_found((false_0 != 0) as bool);
                } else {
                    return true_0 != 0;
                }
            } else {
                let old_kc: xkb_keycode_t =
                    keycode_store_get_keycode(&raw mut (*info).keycodes, match_name)
                        as xkb_keycode_t;
                if old_kc != 0xffffffff as xkb_keycode_t {
                } else {
                    __assert_fail(
                        b"old_kc != XKB_KEYCODE_INVALID\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/keycodes.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        510 as ::core::ffi::c_uint,
                        b"_Bool AddKeyName(KeyNamesInfo *, xkb_keycode_t, xkb_atom_t, enum merge_mode, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if old_kc != kc {
                    if report {
                        let use_0: xkb_keycode_t = if clobber as ::core::ffi::c_int != 0 {
                            kc
                        } else {
                            old_kc
                        };
                        let ignore: xkb_keycode_t = if clobber as ::core::ffi::c_int != 0 {
                            old_kc
                        } else {
                            kc
                        };
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Key name %s assigned to multiple keys; Using %u, ignoring %u\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            KeyNameText((*info).ctx, name),
                            use_0,
                            ignore,
                        );
                    }
                    if clobber {
                        keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                    } else {
                        return true_0 != 0;
                    }
                }
            }
        }
        let match_kc: KeycodeMatch =
            keycode_store_lookup_keycode(&raw mut (*info).keycodes, kc) as KeycodeMatch;
        let old_name: xkb_atom_t =
            keycode_store_get_key_name(&raw mut (*info).keycodes, match_kc) as xkb_atom_t;
        if old_name != XKB_ATOM_NONE as xkb_atom_t {
            if old_name == name {
                if keycode_store_get_keycode(&raw mut (*info).keycodes, match_name) == kc {
                } else {
                    __assert_fail(
                        b"keycode_store_get_keycode(&info->keycodes, match_name) == kc\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/keycodes.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        539 as ::core::ffi::c_uint,
                        b"_Bool AddKeyName(KeyNamesInfo *, xkb_keycode_t, xkb_atom_t, enum merge_mode, _Bool)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if report {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Multiple identical key name definitions; Later occurrences of \"%s = %u\" ignored\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        KeyNameText((*info).ctx, old_name),
                        kc,
                    );
                }
                return true_0 != 0;
            }
            let clobber_0: bool = merge as ::core::ffi::c_uint
                != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
            if report {
                let kname: *const ::core::ffi::c_char =
                    KeyNameText((*info).ctx, name) as *const ::core::ffi::c_char;
                let old_kname: *const ::core::ffi::c_char =
                    KeyNameText((*info).ctx, old_name) as *const ::core::ffi::c_char;
                let use_1: *const ::core::ffi::c_char = if clobber_0 as ::core::ffi::c_int != 0 {
                    kname
                } else {
                    old_kname
                };
                let ignore_0: *const ::core::ffi::c_char = if clobber_0 as ::core::ffi::c_int != 0 {
                    old_kname
                } else {
                    kname
                };
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Multiple names for keycode %u; Using %s, ignoring %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    kc,
                    use_1,
                    ignore_0,
                );
            }
            if clobber_0 {
                keycode_store_delete_name(&raw mut (*info).keycodes, old_name);
                keycode_store_update_key(&raw mut (*info).keycodes, match_kc, name);
            }
        } else if !keycode_store_insert_key(&raw mut (*info).keycodes, kc, name) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot add keycode\n\0".as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_ALLOCATION_ERROR as ::core::ffi::c_int,
            );
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn MergeKeycodeStores(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (*into).keycodes.low.size == 0 as darray_size_t
            && (*into).keycodes.high.size == 0 as darray_size_t
            && (*into).keycodes.names.size == 0 as darray_size_t
        {
            (*into).keycodes = (*from).keycodes;
            (*from).keycodes.low.item = ::core::ptr::null_mut::<xkb_atom_t>();
            (*from).keycodes.low.size = 0 as darray_size_t;
            (*from).keycodes.low.alloc = 0 as darray_size_t;
            (*from).keycodes.high.item = ::core::ptr::null_mut::<HighKeycodeEntry>();
            (*from).keycodes.high.size = 0 as darray_size_t;
            (*from).keycodes.high.alloc = 0 as darray_size_t;
            (*from).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
            (*from).keycodes.names.size = 0 as darray_size_t;
            (*from).keycodes.names.alloc = 0 as darray_size_t;
        } else {
            let mut kc: xkb_keycode_t = (*from).keycodes.min;
            while kc < (*from).keycodes.low.size as xkb_keycode_t {
                let name: xkb_atom_t = *(*from).keycodes.low.item.offset(kc as isize);
                if !(name == XKB_ATOM_NONE as xkb_atom_t) {
                    if !AddKeyName(into, kc, name, merge, report) {
                        (*into).errorCount += 1;
                    }
                }
                kc = kc.wrapping_add(1);
            }
            let mut new: *const HighKeycodeEntry = ::core::ptr::null::<HighKeycodeEntry>();
            if !(*from).keycodes.high.item.is_null() {
                new = (*from)
                    .keycodes
                    .high
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut HighKeycodeEntry;
                while new
                    < (*from)
                        .keycodes
                        .high
                        .item
                        .offset((*from).keycodes.high.size as isize)
                        as *mut HighKeycodeEntry as *const HighKeycodeEntry
                {
                    if (*new).name != 0 as xkb_atom_t {
                    } else {
                        __assert_fail(
                            b"new->name != XKB_ATOM_NONE\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../src/xkbcomp/keycodes.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            609 as ::core::ffi::c_uint,
                            b"void MergeKeycodeStores(KeyNamesInfo *, KeyNamesInfo *, enum merge_mode, _Bool)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    if !AddKeyName(into, (*new).keycode, (*new).name, merge, report) {
                        (*into).errorCount += 1;
                    }
                    new = new.offset(1);
                }
            }
            let mut match_0: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
            let mut alias: xkb_atom_t = 0;
            if !(*from).keycodes.names.item.is_null() {
                alias = 0 as xkb_atom_t;
                match_0 = (*from)
                    .keycodes
                    .names
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut KeycodeMatch;
                while alias < (*from).keycodes.names.size {
                    if !(!(*match_0).c2rust_unnamed.found()
                        || !(*match_0).c2rust_unnamed.is_alias())
                    {
                        let def: KeyAliasDef = KeyAliasDef {
                            common: _ParseCommon {
                                next: ::core::ptr::null_mut::<_ParseCommon>(),
                                type_0: STMT_UNKNOWN,
                            },
                            merge: merge,
                            alias: alias,
                            real: (*match_0).alias.real(),
                        };
                        if !HandleAliasDef(into, &raw const def, report) {
                            (*into).errorCount += 1;
                        }
                    }
                    alias = alias.wrapping_add(1);
                    match_0 = match_0.offset(1);
                }
            }
        };
    }
}
unsafe extern "C" fn MergeIncludedKeycodes(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (*from).errorCount > 0 as ::core::ffi::c_int {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        if (*into).name.is_null() {
            (*into).name = _steal(&raw mut (*from).name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        MergeKeycodeStores(into, from, merge, report);
        if (*into).num_led_names == 0 as xkb_led_index_t {
            memcpy(
                &raw mut (*into).led_names as *mut LedNameInfo as *mut ::core::ffi::c_void,
                &raw mut (*from).led_names as *mut LedNameInfo as *const ::core::ffi::c_void,
                (::core::mem::size_of::<LedNameInfo>() as size_t)
                    .wrapping_mul((*from).num_led_names as size_t),
            );
            (*into).num_led_names = (*from).num_led_names;
            (*from).num_led_names = 0 as xkb_led_index_t;
        } else {
            let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
            while idx < (*from).num_led_names {
                let mut ledi: *mut LedNameInfo = (&raw mut (*from).led_names as *mut LedNameInfo)
                    .offset(idx as isize)
                    as *mut LedNameInfo;
                if !((*ledi).name == XKB_ATOM_NONE as xkb_atom_t) {
                    (*ledi).merge = merge;
                    if !AddLedName(into, false_0 != 0, ledi, idx, report) {
                        (*into).errorCount += 1;
                    }
                }
                idx = idx.wrapping_add(1);
            }
        };
    }
}
unsafe extern "C" fn HandleIncludeKeycodes(
    mut info: *mut KeyNamesInfo,
    mut include: *mut IncludeStmt,
    mut report: bool,
) -> bool {
    unsafe {
        let mut included: KeyNamesInfo = KeyNamesInfo {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_atom_t>(),
                },
                high: C2Rust_Unnamed_17 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                },
                names: C2Rust_Unnamed_16 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeycodeMatch>(),
                },
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as ::core::ffi::c_int;
            return false_0 != 0;
        }
        InitKeyNamesInfo(
            &raw mut included,
            (*info).keymap_info,
            0 as ::core::ffi::c_uint,
        );
        included.name = _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyNamesInfo = KeyNamesInfo {
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                errorCount: 0,
                include_depth: 0,
                keycodes: KeycodeStore {
                    min: 0,
                    low: C2Rust_Unnamed_18 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<xkb_atom_t>(),
                    },
                    high: C2Rust_Unnamed_17 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                    },
                    names: C2Rust_Unnamed_16 {
                        size: 0,
                        alloc: 0,
                        item: ::core::ptr::null_mut::<KeycodeMatch>(),
                    },
                },
                led_names: [LedNameInfo {
                    merge: MERGE_DEFAULT,
                    name: 0,
                }; 32],
                num_led_names: 0,
                ctx: ::core::ptr::null_mut::<xkb_context>(),
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_KEYCODES,
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            );
            if file.is_null() {
                (*info).errorCount += 10 as ::core::ffi::c_int;
                ClearKeyNamesInfo(&raw mut included);
                return false_0 != 0;
            }
            InitKeyNamesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as ::core::ffi::c_uint),
            );
            HandleKeycodesFile(&raw mut next_incl, file);
            MergeIncludedKeycodes(&raw mut included, &raw mut next_incl, (*stmt).merge, report);
            ClearKeyNamesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeycodes(info, &raw mut included, (*include).merge, report);
        ClearKeyNamesInfo(&raw mut included);
        return (*info).errorCount == 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn HandleKeycodeDef(
    mut info: *mut KeyNamesInfo,
    mut stmt: *mut KeycodeDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*stmt).value < 0 as int64_t || (*stmt).value > XKB_KEYCODE_MAX as int64_t {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Illegal keycode %ld: must be between 0..%u; Key ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*stmt).value,
                (0xffffffff as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint),
            );
            return false_0 != 0;
        }
        return AddKeyName(
            info,
            (*stmt).value as xkb_keycode_t,
            (*stmt).name,
            (*stmt).merge,
            report,
        );
    }
}
unsafe extern "C" fn HandleAliasDef(
    mut info: *mut KeyNamesInfo,
    mut def: *const KeyAliasDef,
    mut report: bool,
) -> bool {
    unsafe {
        let match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, (*def).alias) as KeycodeMatch;
        if match_name.c2rust_unnamed.found() {
            let clobber: bool = (*def).merge as ::core::ffi::c_uint
                != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
            if match_name.c2rust_unnamed.is_alias() {
                if (*def).real == match_name.alias.real() {
                    if report {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Alias of %s for %s declared more than once; First definition ignored\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            KeyNameText((*info).ctx, (*def).alias),
                            KeyNameText((*info).ctx, (*def).real),
                        );
                    }
                } else {
                    let use_0: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        (*def).real
                    } else {
                        match_name.alias.real()
                    };
                    let ignore: xkb_atom_t = if clobber as ::core::ffi::c_int != 0 {
                        match_name.alias.real()
                    } else {
                        (*def).real
                    };
                    if report {
                        xkb_log(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"[XKB-%03d] Multiple definitions for alias %s; Using %s, ignoring %s\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                            KeyNameText((*info).ctx, (*def).alias),
                            KeyNameText((*info).ctx, use_0),
                            KeyNameText((*info).ctx, ignore),
                        );
                    }
                    keycode_store_update_alias(&raw mut (*info).keycodes, (*def).alias, use_0);
                }
                return true_0 != 0;
            } else {
                if report {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Alias name %s already assigned to a real key; Using %s, ignoring %s\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_WARNING_CONFLICTING_KEY_NAME as ::core::ffi::c_int,
                        KeyNameText((*info).ctx, (*def).alias),
                        if clobber as ::core::ffi::c_int != 0 {
                            b"alias\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"key\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if clobber as ::core::ffi::c_int != 0 {
                            b"key\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"alias\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                if clobber {
                    keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                } else {
                    return true_0 != 0;
                }
            }
        }
        return keycode_store_insert_alias(&raw mut (*info).keycodes, (*def).alias, (*def).real);
    }
}
unsafe extern "C" fn HandleKeyNameVar(mut info: *mut KeyNamesInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut field: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false_0 != 0;
        }
        if !elem.is_null() {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Cannot set global defaults for \"%s\" element; Assignment to \"%s.%s\" ignored\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as ::core::ffi::c_int,
                elem,
                elem,
                field,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0;
        }
        if !istreq(field, b"minimum\0".as_ptr() as *const ::core::ffi::c_char)
            && !istreq(field, b"maximum\0".as_ptr() as *const ::core::ffi::c_char)
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"[XKB-%03d] Default defined for unknown field \"%s\"; Ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as ::core::ffi::c_int,
                field,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0;
        }
        if !arrayNdx.is_null() {
            ReportNotArray(
                (*info).ctx,
                b"keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                field,
                b"defaults\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0;
        }
        let mut val: int64_t = 0 as int64_t;
        if !ExprResolveInteger((*info).ctx, (*stmt).value, &raw mut val)
            || val < 0 as int64_t
            || val > UINT32_MAX as int64_t
        {
            ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"keycodes\0".as_ptr() as *const ::core::ffi::c_char,
                field,
                b"defaults\0".as_ptr() as *const ::core::ffi::c_char,
                b"integer 0..0xfffffffe\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_FIELD_TYPE_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn HandleLedNameDef(
    mut info: *mut KeyNamesInfo,
    mut def: *mut LedNameDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*def).ndx < 1 as int64_t || (*def).ndx > XKB_MAX_LEDS as int64_t {
            (*info).errorCount += 1;
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Illegal indicator index (%ld) specified; must be between 1 .. %u; Ignored\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*def).ndx,
                (::core::mem::size_of::<xkb_led_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_led_index_t,
            );
            return false_0 != 0;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, (*def).name, &raw mut name) {
            let mut buf: [::core::ffi::c_char; 20] = [0; 20];
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
                b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                (*def).ndx,
            );
            (*info).errorCount += 1;
            return ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"indicator\0".as_ptr() as *const ::core::ffi::c_char,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                b"string\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut ledi: LedNameInfo = LedNameInfo {
            merge: (*def).merge,
            name: name,
        };
        return AddLedName(
            info,
            true_0 != 0,
            &raw mut ledi,
            ((*def).ndx as xkb_led_index_t).wrapping_sub(1 as xkb_led_index_t),
            report,
        );
    }
}
unsafe extern "C" fn HandleKeycodesFile(mut info: *mut KeyNamesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        let report_same_file: bool = verbosity > 0 as ::core::ffi::c_int;
        let report_include: bool = verbosity > 7 as ::core::ffi::c_int;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as ::core::ffi::c_uint {
                1 => {
                    ok = HandleIncludeKeycodes(info, stmt as *mut IncludeStmt, report_include);
                }
                2 => {
                    ok = HandleKeycodeDef(info, stmt as *mut KeycodeDef, report_same_file);
                }
                3 => {
                    ok = HandleAliasDef(info, stmt as *mut KeyAliasDef, report_same_file);
                }
                26 => {
                    ok = HandleKeyNameVar(info, stmt as *mut VarDef);
                }
                34 => {
                    ok = HandleLedNameDef(info, stmt as *mut LedNameDef, report_same_file);
                }
                35 | 36 => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Unsupported keycodes %s statement \"%s\"; Ignoring\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_ERROR_UNKNOWN_STATEMENT as ::core::ffi::c_int,
                        if (*stmt).type_0 as ::core::ffi::c_uint
                            == STMT_UNKNOWN_COMPOUND as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"compound\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"declaration\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        (*(stmt as *mut UnknownStatement)).name,
                    );
                    ok = (*(*info).keymap_info).strict as ::core::ffi::c_uint
                        & PARSER_NO_UNKNOWN_STATEMENTS as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0;
                }
                _ => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Keycode files may define key and indicator names only; Ignoring %s\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false_0 != 0;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as ::core::ffi::c_int {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Abandoning keycodes file \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                    safe_map_name(file),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe extern "C" fn CopyKeyNamesToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        if (*info).keycodes.low.size == 0 as darray_size_t
            && (*info).keycodes.high.size == 0 as darray_size_t
        {
            if (*info).keycodes.min == 0xffffffff as xkb_keycode_t {
            } else {
                __assert_fail(
                    b"info->keycodes.min == XKB_KEYCODE_INVALID\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    931 as ::core::ffi::c_uint,
                    b"_Bool CopyKeyNamesToKeymap(struct xkb_keymap *, KeyNamesInfo *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            (*keymap).min_key_code = 8 as xkb_keycode_t;
            (*keymap).max_key_code = 255 as xkb_keycode_t;
            (*keymap).num_keys_low = (*keymap).max_key_code.wrapping_add(1 as xkb_keycode_t);
            (*keymap).num_keys = (*keymap).num_keys_low;
        } else {
            if (*info).keycodes.min
                <= (0xffffffff as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t)
            {
            } else {
                __assert_fail(
                    b"info->keycodes.min <= XKB_KEYCODE_MAX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                    937 as ::core::ffi::c_uint,
                    b"_Bool CopyKeyNamesToKeymap(struct xkb_keymap *, KeyNamesInfo *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            (*keymap).min_key_code = (*info).keycodes.min;
            (*keymap).max_key_code =
                if (*info).keycodes.high.size == 0 as darray_size_t {
                    ((*info).keycodes.low.size as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t)
                } else {
                    (*(*info).keycodes.high.item.offset(
                        (*info).keycodes.high.size.wrapping_sub(1 as darray_size_t) as isize,
                    ))
                    .keycode
                };
            (*keymap).num_keys_low = (*info).keycodes.low.size as xkb_keycode_t;
            (*keymap).num_keys = (*keymap)
                .num_keys_low
                .wrapping_add((*info).keycodes.high.size as xkb_keycode_t);
        }
        let keys: *mut xkb_key = calloc(
            (*keymap).num_keys as size_t,
            ::core::mem::size_of::<xkb_key>() as size_t,
        ) as *mut xkb_key;
        if keys.is_null() {
            (*keymap).num_keys = 0 as xkb_keycode_t;
            (*keymap).max_key_code = XKB_KEYCODE_INVALID as xkb_keycode_t;
            (*keymap).min_key_code = (*keymap).max_key_code;
            return false_0 != 0;
        }
        let mut kc: xkb_keycode_t = (*keymap).min_key_code;
        while kc < (*keymap).num_keys_low {
            (*keys.offset(kc as isize)).keycode = kc;
            kc = kc.wrapping_add(1);
        }
        let mut kc_0: xkb_keycode_t = (*info).keycodes.min;
        while kc_0 < (*info).keycodes.low.size as xkb_keycode_t {
            (*keys.offset(kc_0 as isize)).name = *(*info).keycodes.low.item.offset(kc_0 as isize);
            kc_0 = kc_0.wrapping_add(1);
        }
        let mut idx: xkb_keycode_t = (*keymap).num_keys_low;
        let mut entry: *const HighKeycodeEntry = ::core::ptr::null::<HighKeycodeEntry>();
        if !(*info).keycodes.high.item.is_null() {
            entry = (*info)
                .keycodes
                .high
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut HighKeycodeEntry;
            while entry
                < (*info)
                    .keycodes
                    .high
                    .item
                    .offset((*info).keycodes.high.size as isize)
                    as *mut HighKeycodeEntry as *const HighKeycodeEntry
            {
                if (*entry).name != 0 as xkb_atom_t {
                } else {
                    __assert_fail(
                        b"entry->name != XKB_ATOM_NONE\0".as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/keycodes.c\0".as_ptr() as *const ::core::ffi::c_char,
                        970 as ::core::ffi::c_uint,
                        b"_Bool CopyKeyNamesToKeymap(struct xkb_keymap *, KeyNamesInfo *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                (*keys.offset(idx as isize)).keycode = (*entry).keycode;
                (*keys.offset(idx as isize)).name = (*entry).name;
                idx = idx.wrapping_add(1);
                entry = entry.offset(1);
            }
        }
        (*keymap).keys = keys;
        return true_0 != 0;
    }
}
unsafe extern "C" fn CopyKeycodeNameLUT(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        let mut match_0: *mut KeycodeMatch = ::core::ptr::null_mut::<KeycodeMatch>();
        let mut name: xkb_atom_t = 0;
        if !(*info).keycodes.names.item.is_null() {
            name = 0 as xkb_atom_t;
            match_0 = (*info)
                .keycodes
                .names
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut KeycodeMatch;
            while name < (*info).keycodes.names.size {
                if (*match_0).c2rust_unnamed.found() {
                    if (*match_0).c2rust_unnamed.is_alias() {
                        let match_real: KeycodeMatch = keycode_store_lookup_name(
                            &raw mut (*info).keycodes,
                            (*match_0).alias.real(),
                        ) as KeycodeMatch;
                        if !match_real.c2rust_unnamed.found() {
                            xkb_log(
                                (*info).ctx,
                                XKB_LOG_LEVEL_WARNING,
                                XKB_LOG_VERBOSITY_DETAILED as ::core::ffi::c_int,
                                b"[XKB-%03d] Attempt to alias %s to non-existent key %s; Ignored\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                XKB_WARNING_UNDEFINED_KEYCODE as ::core::ffi::c_int,
                                KeyNameText((*info).ctx, name),
                                KeyNameText((*info).ctx, (*match_0).alias.real()),
                            );
                            (*match_0).c2rust_unnamed.set_found((false_0 != 0) as bool);
                        } else {
                            if !match_real.c2rust_unnamed.is_alias() {
                            } else {
                                __assert_fail(
                                    b"!match_real.is_alias\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"../src/xkbcomp/keycodes.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1007 as ::core::ffi::c_uint,
                                    b"_Bool CopyKeycodeNameLUT(struct xkb_keymap *, KeyNamesInfo *)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            };
                        }
                    } else if !(*match_0).key.low() {
                        (*match_0).key.set_index(
                            (*match_0).key.index()
                                + (*keymap).num_keys_low as ::core::ffi::c_uint as darray_size_t,
                        );
                    }
                }
                name = name.wrapping_add(1);
                match_0 = match_0.offset(1);
            }
        }
        if (*info).keycodes.names.size > 0 as darray_size_t {
            (*info).keycodes.names.alloc = (*info).keycodes.names.size;
            (*info).keycodes.names.item = realloc(
                (*info).keycodes.names.item as *mut ::core::ffi::c_void,
                ((*info).keycodes.names.alloc as size_t)
                    .wrapping_mul(::core::mem::size_of::<KeycodeMatch>() as size_t),
            ) as *mut KeycodeMatch;
        }
        (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = (*info).keycodes.names.size;
        (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = (*info).keycodes.names.item;
        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
            *(::core::ptr::null_mut::<::core::ffi::c_void>() as *mut darray_size_t) =
                (*info).keycodes.names.size;
        }
        (*info).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*info).keycodes.names.size = 0 as darray_size_t;
        (*info).keycodes.names.alloc = 0 as darray_size_t;
        (*info).keycodes.names.item = ::core::ptr::null_mut::<KeycodeMatch>();
        (*info).keycodes.names.size = 0 as darray_size_t;
        (*info).keycodes.names.alloc = 0 as darray_size_t;
        return true_0 != 0;
    }
}
unsafe extern "C" fn CopyLedNamesToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        (*keymap).num_leds = (*info).num_led_names;
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_led_names {
            let mut ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                .offset(idx as isize)
                as *mut LedNameInfo;
            if !((*ledi).name == XKB_ATOM_NONE as xkb_atom_t) {
                (*keymap).leds[idx as usize].name = (*ledi).name;
            }
            idx = idx.wrapping_add(1);
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn CopyKeyNamesInfoToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        if !CopyKeyNamesToKeymap(keymap, info)
            || !CopyKeycodeNameLUT(keymap, info)
            || !CopyLedNamesToKeymap(keymap, info)
        {
            return false_0 != 0;
        }
        if (*keymap).num_keys == 0 || (*keymap).min_key_code > 0 as xkb_keycode_t {
            (*keymap).redirect_key_auto = 0 as xkb_keycode_t;
        } else {
            let mut keycode: xkb_keycode_t =
                (XKB_KEYCODE_INVALID as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t);
            let mut k: xkb_keycode_t = (*keymap).num_keys;
            loop {
                let c2rust_fresh0 = k;
                k = k.wrapping_sub(1);
                if !(c2rust_fresh0 > (*keymap).num_keys_low) {
                    break;
                }
                if keycode > (*(*keymap).keys.offset(k as isize)).keycode {
                    break;
                }
                keycode = (*(*keymap).keys.offset(k as isize))
                    .keycode
                    .wrapping_sub(1 as xkb_keycode_t);
            }
            (*keymap).redirect_key_auto = keycode;
        }
        (*keymap).keycodes_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).keycodes_section_name);
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompileKeycodes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyNamesInfo = KeyNamesInfo {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: 0,
                low: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_atom_t>(),
                },
                high: C2Rust_Unnamed_17 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<HighKeycodeEntry>(),
                },
                names: C2Rust_Unnamed_16 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<KeycodeMatch>(),
                },
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: ::core::ptr::null_mut::<xkb_context>(),
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
        };
        InitKeyNamesInfo(&raw mut info, keymap_info, 0 as ::core::ffi::c_uint);
        if !file.is_null() {
            HandleKeycodesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as ::core::ffi::c_int) {
            if CopyKeyNamesInfoToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyNamesInfo(&raw mut info);
                return true_0 != 0;
            }
        }
        ClearKeyNamesInfo(&raw mut info);
        return false_0 != 0;
    }
}
