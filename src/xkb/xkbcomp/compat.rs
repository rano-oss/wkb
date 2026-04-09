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
        pub text_next: usize,
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

    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
        pub fn xkb_context_get_buffer(
            ctx: *mut xkb_context,
            size: usize,
        ) -> *mut ::core::ffi::c_char;
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
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if (need as usize)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as usize)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, usize)\0"
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
    pub const XKB_MOD_INVALID: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
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
    pub type C2Rust_Unnamed_13 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_13 = 0;
    pub const DEFAULT_INTERPRET_KEY_REPEAT: C2Rust_Unnamed_13 = 1;
    pub const DEFAULT_KEY_REPEAT: C2Rust_Unnamed_13 = 0;
    pub type C2Rust_Unnamed_14 = ::core::ffi::c_uint;
    pub const FALLBACK_INTERPRET_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const DEFAULT_INTERPRET_VMOD: C2Rust_Unnamed_14 = 4294967295;
    pub const DEFAULT_KEY_VMODMAP: C2Rust_Unnamed_14 = 0;
    pub const XKB_MAX_LEDS: xkb_led_index_t = (::core::mem::size_of::<xkb_led_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_led_index_t;
    pub const MOD_REAL_MASK_ALL: xkb_mod_mask_t = 0xff as ::core::ffi::c_int as xkb_mod_mask_t;
    pub const MAX_ACTIONS_PER_LEVEL: ::core::ffi::c_int = UINT16_MAX;
    use super::atom_h::xkb_atom_t;
    use super::context_h::xkb_context;
    use super::darray_h::darray_size_t;
    use super::limits_h::CHAR_BIT;
    use super::stdint_h::UINT16_MAX;
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
        pub syms: C2Rust_Unnamed_15,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_15 {
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
    pub struct VModDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct InterpDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub sym: xkb_keysym_t,
        pub match_0: *mut ExprDef,
        pub def: *mut VarDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct LedMapDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub body: *mut VarDef,
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
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_match_operation, xkb_mod_set};
    use super::stdint_uintn_h::u32;
    use super::xkbcommon_h::{xkb_keysym_t, xkb_mod_mask_t};
    extern "C" {
        pub fn LookupString(
            tab: *const LookupEntry,
            string: *const ::core::ffi::c_char,
            value_rtrn: *mut ::core::ffi::c_uint,
        ) -> bool;
        pub static ctrlMaskNames: [LookupEntry; 0];
        pub static modComponentMaskNames: [LookupEntry; 0];
        pub static groupComponentMaskNames: [LookupEntry; 0];
        pub static useModMapValueNames: [LookupEntry; 0];
        pub static symInterpretMatchMaskNames: [LookupEntry; 0];
        pub fn ModMaskText(
            ctx: *mut xkb_context,
            type_0: mod_type,
            mods: *const xkb_mod_set,
            mask: xkb_mod_mask_t,
        ) -> *const ::core::ffi::c_char;
        pub fn KeysymText(ctx: *mut xkb_context, sym: xkb_keysym_t) -> *const ::core::ffi::c_char;
        pub fn SIMatchText(type_0: xkb_match_operation) -> *const ::core::ffi::c_char;
    }
}
pub mod xkbcomp_priv_h {
    pub type xkb_parser_error = ::core::ffi::c_uint;
    pub const PARSER_FATAL_ERROR: xkb_parser_error = 2;
    pub const PARSER_RECOVERABLE_ERROR: xkb_parser_error = 1;
    pub const PARSER_SUCCESS: xkb_parser_error = 0;
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
        pub features: C2Rust_Unnamed_17,
        pub lookup: C2Rust_Unnamed_16,
        pub pending_computations: *mut pending_computation_array,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_16 {
        pub groupIndexNames: [LookupEntry; 3],
        pub groupMaskNames: [LookupEntry; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_17 {
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
    pub unsafe extern "C" fn ReportBadField(
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
                b"Unknown %s field \"%s\" in %s; Ignoring assignment to unknown field in %s\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                type_0,
                field,
                name,
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
pub mod action_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ActionsInfo {
        pub actions: [xkb_action; 21],
    }
    use super::ast_h::{merge_mode, ExprDef};

    use super::keymap_h::{xkb_action, xkb_keymap, xkb_mod_set};

    use super::xkbcomp_priv_h::{xkb_keymap_info, xkb_parser_error};

    extern "C" {
        pub fn InitActionsInfo(keymap: *const xkb_keymap, info: *mut ActionsInfo);
        pub fn HandleActionDef(
            keymap_info: *const xkb_keymap_info,
            info: *mut ActionsInfo,
            mods: *const xkb_mod_set,
            def: *mut ExprDef,
            action: *mut xkb_action,
        ) -> xkb_parser_error;
        pub fn SetDefaultActionField(
            keymap_info: *const xkb_keymap_info,
            info: *mut ActionsInfo,
            mods: *mut xkb_mod_set,
            elem: *const ::core::ffi::c_char,
            field: *const ::core::ffi::c_char,
            array_ndx: *mut ExprDef,
            value_ptr: *mut *mut ExprDef,
            merge: merge_mode,
        ) -> xkb_parser_error;
    }
}
pub mod stdio_h {

    extern "C" {
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
pub mod string_h {

    extern "C" {
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub mod vmod_h {

    use super::ast_h::{merge_mode, VModDef};
    use super::context_h::xkb_context;
    use super::keymap_h::xkb_mod_set;

    extern "C" {
        pub fn InitVMods(info: *mut xkb_mod_set, mods: *const xkb_mod_set, reset: bool);
        pub fn MergeModSets(
            ctx: *mut xkb_context,
            into: *mut xkb_mod_set,
            from: *const xkb_mod_set,
            merge: merge_mode,
        );
        pub fn HandleVModDef(
            ctx: *mut xkb_context,
            mods: *mut xkb_mod_set,
            stmt: *mut VModDef,
        ) -> bool;
    }
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_mod_set};
    use super::stdint_uintn_h::u32;
    use super::text_h::LookupEntry;
    use super::xkbcommon_h::{xkb_layout_mask_t, xkb_mod_index_t, xkb_mod_mask_t};
    use super::xkbcomp_priv_h::xkb_keymap_info;
    extern "C" {
        pub fn ExprResolveLhs(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            elem_rtrn: *mut *const ::core::ffi::c_char,
            field_rtrn: *mut *const ::core::ffi::c_char,
            index_rtrn: *mut *mut ExprDef,
        ) -> bool;
        pub fn ExprResolveModMask(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            mod_type: mod_type,
            mods: *const xkb_mod_set,
            mask_rtrn: *mut xkb_mod_mask_t,
        ) -> bool;
        pub fn ExprResolveMod(
            ctx: *mut xkb_context,
            def: *const ExprDef,
            mod_type: mod_type,
            mods: *const xkb_mod_set,
            ndx_rtrn: *mut xkb_mod_index_t,
        ) -> bool;
        pub fn ExprResolveBoolean(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            set_rtrn: *mut bool,
        ) -> bool;
        pub fn ExprResolveGroupMask(
            keymap_info: *const xkb_keymap_info,
            expr: *const ExprDef,
            group_rtrn: *mut xkb_layout_mask_t,
            pending_rtrn: *mut bool,
        ) -> bool;
        pub fn ExprResolveEnum(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            val_rtrn: *mut u32,
            values: *const LookupEntry,
        ) -> bool;
        pub fn ExprResolveMask(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            mask_rtrn: *mut u32,
            values: *const LookupEntry,
        ) -> bool;
    }
}
pub mod util_mem_h {
    #[inline]
    pub unsafe extern "C" fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
            let mut swapped: *mut ::core::ffi::c_void = *original;
            *original = NULL_0;
            return swapped;
        }
    }
    use super::__stddef_null_h::NULL_0;
}
pub mod include_h {

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
            path_size: usize,
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
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
    pub const NULL_0: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::{NULL, NULL_0};

pub use self::action_h::{ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField};
use self::assert_h::__assert_fail;
pub use self::ast_h::{
    _IncludeStmt, _ParseCommon, merge_mode, stmt_type, stmt_type_to_string, xkb_file_type,
    xkb_map_flags, C2Rust_Unnamed_15, ExprAction, ExprActionList, ExprArrayRef, ExprBinary,
    ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger, ExprKeyName, ExprKeySym,
    ExprKeysymList, ExprString, ExprUnary, IncludeStmt, InterpDef, LedMapDef, ParseCommon,
    UnknownStatement, VModDef, VarDef, XkbFile, _FILE_TYPE_NUM_ENTRIES, _MERGE_MODE_NUM_ENTRIES,
    _STMT_NUM_VALUES, FILE_TYPE_COMPAT, FILE_TYPE_GEOMETRY, FILE_TYPE_INVALID, FILE_TYPE_KEYCODES,
    FILE_TYPE_KEYMAP, FILE_TYPE_RULES, FILE_TYPE_SYMBOLS, FILE_TYPE_TYPES, FIRST_KEYMAP_FILE_TYPE,
    LAST_KEYMAP_FILE_TYPE, MAP_HAS_ALPHANUMERIC, MAP_HAS_FN, MAP_HAS_KEYPAD, MAP_HAS_MODIFIER,
    MAP_IS_ALTGR, MAP_IS_DEFAULT, MAP_IS_HIDDEN, MAP_IS_PARTIAL, MERGE_AUGMENT, MERGE_DEFAULT,
    MERGE_OVERRIDE, MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST,
    STMT_EXPR_ADD, STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL,
    STMT_EXPR_DIVIDE, STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL,
    STMT_EXPR_IDENT, STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL,
    STMT_EXPR_KEYSYM_LIST, STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE,
    STMT_EXPR_NOT, STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS,
    STMT_GROUP_COMPAT, STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME,
    STMT_MODMAP, STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND,
    STMT_UNKNOWN_DECLARATION, STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t, XKB_ATOM_NONE};
pub use self::context_h::{
    xkb_atom_text, xkb_context, xkb_context_get_buffer, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0,
};
pub use self::darray_h::{darray_next_alloc, darray_size_t};
use self::expr_h::{
    ExprResolveBoolean, ExprResolveEnum, ExprResolveGroupMask, ExprResolveLhs, ExprResolveMask,
    ExprResolveMod, ExprResolveModMask,
};
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
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_13, C2Rust_Unnamed_14,
    C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6,
    C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbEscapeMapName,
    _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y,
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
    DEFAULT_INTERPRET_KEY_REPEAT, DEFAULT_INTERPRET_VMOD, DEFAULT_INTERPRET_VMODMAP,
    DEFAULT_KEY_REPEAT, DEFAULT_KEY_VMODMAP, EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT,
    EXPLICIT_SYMBOLS, EXPLICIT_TYPES, EXPLICIT_VMODMAP, FALLBACK_INTERPRET_KEY_REPEAT,
    FALLBACK_INTERPRET_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL,
    MATCH_ANY, MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MAX_ACTIONS_PER_LEVEL, MOD_BOTH,
    MOD_REAL, MOD_REAL_MASK_ALL, MOD_VIRT, XKB_MAX_LEDS,
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
pub use self::stdint_h::UINT16_MAX;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
use self::stdio_h::snprintf;
use self::stdlib_h::{free, realloc};
use self::string_h::{memcpy, memset};
pub use self::text_h::{
    ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
    useModMapValueNames, KeysymText, LookupEntry, LookupString, ModMaskText, SIMatchText,
};
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{istrcmp, istreq, strdup_safe};
use self::vmod_h::{HandleVModDef, InitVMods, MergeModSets};
pub use self::xkbcommon_h::{
    xkb_context_get_log_verbosity, xkb_keycode_t, xkb_keymap_compile_flags, xkb_keymap_format,
    xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t, xkb_layout_out_of_range_policy,
    xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t, xkb_log_level, xkb_mod_index_t,
    xkb_mod_mask_t, xkb_rule_names, xkb_state_component, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_LAYOUT_OUT_OF_RANGE_CLAMP, XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING, XKB_MOD_INVALID, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::xkbcomp_priv_h::{
    pending_computation, pending_computation_array, safe_map_name, xkb_keymap_info,
    xkb_parser_error, xkb_parser_strict_flags, C2Rust_Unnamed_16, C2Rust_Unnamed_17, FreeXkbFile,
    ReportBadField, ReportBadType, ReportNotArray, PARSER_FATAL_ERROR,
    PARSER_NO_FIELD_TYPE_MISMATCH, PARSER_NO_FIELD_VALUE_MISMATCH, PARSER_NO_ILLEGAL_ACTION_FIELDS,
    PARSER_NO_STRICT_FLAGS, PARSER_NO_UNKNOWN_ACTION, PARSER_NO_UNKNOWN_ACTION_FIELDS,
    PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_INTERPRET_FIELDS,
    PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_KEY_FIELDS,
    PARSER_NO_UNKNOWN_LED_FIELDS, PARSER_NO_UNKNOWN_STATEMENTS,
    PARSER_NO_UNKNOWN_SYMBOLS_GLOBAL_FIELDS, PARSER_NO_UNKNOWN_TYPES_GLOBAL_FIELDS,
    PARSER_NO_UNKNOWN_TYPE_FIELDS, PARSER_RECOVERABLE_ERROR, PARSER_SUCCESS, PARSER_V1_LAX_FLAGS,
    PARSER_V1_STRICT_FLAGS, PARSER_V2_LAX_FLAGS, PARSER_V2_STRICT_FLAGS,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompatInfo {
    pub name: *mut ::core::ffi::c_char,
    pub errorCount: ::core::ffi::c_int,
    pub include_depth: ::core::ffi::c_uint,
    pub default_interp: SymInterpInfo,
    pub interps: C2Rust_Unnamed_18,
    pub default_led: LedInfo,
    pub leds: [LedInfo; 32],
    pub num_leds: xkb_led_index_t,
    pub default_actions: ActionsInfo,
    pub mods: xkb_mod_set,
    pub keymap_info: *const xkb_keymap_info,
    pub ctx: *mut xkb_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedInfo {
    pub defined: led_field,
    pub merge: merge_mode,
    pub led: xkb_led,
}
pub type led_field = ::core::ffi::c_uint;
pub const LED_FIELD_CTRLS: led_field = 4;
pub const LED_FIELD_GROUPS: led_field = 2;
pub const LED_FIELD_MODS: led_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut SymInterpInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymInterpInfo {
    pub defined: si_field,
    pub merge: merge_mode,
    pub interp: xkb_sym_interpret,
}
pub type si_field = ::core::ffi::c_uint;
pub const SI_FIELD_LEVEL_ONE_ONLY: si_field = 8;
pub const SI_FIELD_AUTO_REPEAT: si_field = 4;
pub const SI_FIELD_ACTION: si_field = 2;
pub const SI_FIELD_VIRTUAL_MOD: si_field = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_sym_interpret,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collect {
    pub sym_interprets: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut xkb_action,
}
unsafe extern "C" fn siText(
    mut si: *mut SymInterpInfo,
    mut info: *mut CompatInfo,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buf: *mut ::core::ffi::c_char = xkb_context_get_buffer((*info).ctx, 128 as usize);
        if si == &raw mut (*info).default_interp {
            return b"default\0".as_ptr() as *const ::core::ffi::c_char;
        }
        snprintf(
            buf,
            128 as usize,
            b"%s+%s(%s)\0".as_ptr() as *const ::core::ffi::c_char,
            KeysymText((*info).ctx, (*si).interp.sym),
            SIMatchText((*si).interp.match_0),
            ModMaskText(
                (*info).ctx,
                MOD_BOTH,
                &raw mut (*info).mods,
                (*si).interp.mods,
            ),
        );
        return buf;
    }
}
#[inline]
unsafe extern "C" fn ReportSINotArray(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        return ReportNotArray(
            (*info).ctx,
            b"symbol interpretation\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            siText(si, info),
        );
    }
}
#[inline]
unsafe extern "C" fn ReportSIBadType(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const ::core::ffi::c_char,
    mut wanted: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"symbol interpretation\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            siText(si, info),
            wanted,
        );
    }
}
unsafe extern "C" fn LEDText(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
) -> *const ::core::ffi::c_char {
    unsafe {
        if ledi == &raw mut (*info).default_led {
            if xkb_atom_text((*info).ctx, (*ledi).led.name).is_null() {
            } else {
                __assert_fail(
                    b"xkb_atom_text(info->ctx, ledi->led.name) == NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/compat.c\0".as_ptr() as *const ::core::ffi::c_char,
                    107 as ::core::ffi::c_uint,
                    b"const char *LEDText(CompatInfo *, LedInfo *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return b"default\0".as_ptr() as *const ::core::ffi::c_char;
        } else {
            if !xkb_atom_text((*info).ctx, (*ledi).led.name).is_null() {
            } else {
                __assert_fail(
                    b"xkb_atom_text(info->ctx, ledi->led.name) != NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/xkbcomp/compat.c\0".as_ptr() as *const ::core::ffi::c_char,
                    110 as ::core::ffi::c_uint,
                    b"const char *LEDText(CompatInfo *, LedInfo *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return xkb_atom_text((*info).ctx, (*ledi).led.name);
        };
    }
}
#[inline]
unsafe extern "C" fn ReportLedBadType(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const ::core::ffi::c_char,
    mut wanted: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"indicator map\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            LEDText(info, ledi),
            wanted,
        );
    }
}
#[inline]
unsafe extern "C" fn ReportLedNotArray(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        return ReportNotArray(
            (*info).ctx,
            b"indicator map\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            LEDText(info, ledi),
        );
    }
}
#[inline]
unsafe extern "C" fn InitInterp(mut info: *mut SymInterpInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
        (*info).interp.virtual_mod = XKB_MOD_INVALID as xkb_mod_index_t;
    }
}
#[inline]
unsafe extern "C" fn InitLED(mut info: *mut LedInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
    }
}
unsafe extern "C" fn InitCompatInfo(
    mut info: *mut CompatInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: ::core::ffi::c_uint,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        memset(
            info as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<CompatInfo>() as usize,
        );
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitActionsInfo(
            &raw const (*keymap_info).keymap,
            &raw mut (*info).default_actions,
        );
        InitVMods(
            &raw mut (*info).mods,
            mods,
            include_depth > 0 as ::core::ffi::c_uint,
        );
        InitInterp(&raw mut (*info).default_interp);
        InitLED(&raw mut (*info).default_led);
    }
}
unsafe extern "C" fn ClearCompatInfo(mut info: *mut CompatInfo) {
    unsafe {
        free((*info).name as *mut ::core::ffi::c_void);
        free((*info).interps.item as *mut ::core::ffi::c_void);
        (*info).interps.item = ::core::ptr::null_mut::<SymInterpInfo>();
        (*info).interps.size = 0 as darray_size_t;
        (*info).interps.alloc = 0 as darray_size_t;
    }
}
unsafe extern "C" fn FindMatchingInterp(
    mut info: *mut CompatInfo,
    mut new: *mut SymInterpInfo,
) -> *mut SymInterpInfo {
    unsafe {
        let mut old: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
        if !(*info).interps.item.is_null() {
            old = (*info)
                .interps
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut SymInterpInfo;
            while old
                < (*info).interps.item.offset((*info).interps.size as isize) as *mut SymInterpInfo
            {
                if (*old).interp.sym == (*new).interp.sym
                    && (*old).interp.mods == (*new).interp.mods
                    && (*old).interp.match_0 as ::core::ffi::c_uint
                        == (*new).interp.match_0 as ::core::ffi::c_uint
                {
                    return old;
                }
                old = old.offset(1);
            }
        }
        return ::core::ptr::null_mut::<SymInterpInfo>();
    }
}
unsafe extern "C" fn UseNewInterpField(
    mut field: si_field,
    mut old: si_field,
    mut new: si_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut si_field,
) -> bool {
    unsafe {
        if old as ::core::ffi::c_uint & field as ::core::ffi::c_uint == 0 {
            return new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0;
        }
        if new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0 {
            if report {
                *collide =
                    (*collide as ::core::ffi::c_uint | field as ::core::ffi::c_uint) as si_field;
            }
            return clobber;
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn MergeInterp(
    mut info: *mut CompatInfo,
    mut old: *mut SymInterpInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let clobber: bool = (*new).merge as ::core::ffi::c_uint
            != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        let report: bool = same_file as ::core::ffi::c_int != 0
            && verbosity > 0 as ::core::ffi::c_int
            || verbosity > 9 as ::core::ffi::c_int;
        let mut collide: si_field = 0 as si_field;
        if (*new).merge as ::core::ffi::c_uint
            == MERGE_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if report {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Multiple definitions for \"%s\"; Earlier interpretation ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    siText(new, info),
                );
            }
            *old = *new;
            return true_0 != 0;
        }
        if UseNewInterpField(
            SI_FIELD_VIRTUAL_MOD,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.virtual_mod = (*new).interp.virtual_mod;
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | SI_FIELD_VIRTUAL_MOD as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_ACTION,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            if (*old).interp.num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                free((*old).interp.a.actions as *mut ::core::ffi::c_void);
            }
            (*old).interp.num_actions = (*new).interp.num_actions;
            if (*new).interp.num_actions as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*old).interp.a.actions = (*new).interp.a.actions;
                (*new).interp.a.action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                (*new).interp.num_actions = 0 as xkb_action_count_t;
            } else {
                (*old).interp.a.action = (*new).interp.a.action;
            }
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | SI_FIELD_ACTION as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_AUTO_REPEAT,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.set_repeat((*new).interp.repeat() as bool);
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | SI_FIELD_AUTO_REPEAT as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_LEVEL_ONE_ONLY,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.level_one_only = (*new).interp.level_one_only;
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | SI_FIELD_LEVEL_ONE_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        }
        if collide as u64 != 0 {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Multiple interpretations of \"%s\"; Using %s definition for duplicate fields\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                siText(old, info),
                if clobber as ::core::ffi::c_int != 0 {
                    b"last\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"first\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn AddInterp(
    mut info: *mut CompatInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut old: *mut SymInterpInfo = FindMatchingInterp(info, new);
        if !old.is_null() {
            return MergeInterp(info, old, new, same_file);
        }
        (*info).interps.size = (*info).interps.size.wrapping_add(1 as darray_size_t);
        let mut __need: darray_size_t = (*info).interps.size;
        if __need > (*info).interps.alloc {
            (*info).interps.alloc = darray_next_alloc(
                (*info).interps.alloc,
                __need,
                ::core::mem::size_of::<SymInterpInfo>() as usize,
            );
            (*info).interps.item = realloc(
                (*info).interps.item as *mut ::core::ffi::c_void,
                ((*info).interps.alloc as usize)
                    .wrapping_mul(::core::mem::size_of::<SymInterpInfo>() as usize),
            ) as *mut SymInterpInfo;
        }
        *(*info)
            .interps
            .item
            .offset((*info).interps.size.wrapping_sub(1 as darray_size_t) as isize) = *new;
        return true_0 != 0;
    }
}
unsafe extern "C" fn ResolveStateAndPredicate(
    mut expr: *mut ExprDef,
    mut pred_rtrn: *mut xkb_match_operation,
    mut mods_rtrn: *mut xkb_mod_mask_t,
    mut info: *mut CompatInfo,
) -> bool {
    unsafe {
        if expr.is_null() {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true_0 != 0;
        }
        *pred_rtrn = MATCH_EXACTLY;
        if (*expr).common.type_0 as ::core::ffi::c_uint
            == STMT_EXPR_ACTION_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut pred_txt: *const ::core::ffi::c_char =
                xkb_atom_text((*info).ctx, (*expr).action.name);
            let mut pred: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            if !LookupString(
                &raw const symInterpretMatchMaskNames as *const LookupEntry,
                pred_txt,
                &raw mut pred,
            ) || (*expr).action.args.is_null()
                || !(*(*expr).action.args).common.next.is_null()
            {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Illegal modifier predicate \"%s\"; Ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pred_txt,
                );
                return false_0 != 0;
            }
            *pred_rtrn = pred as xkb_match_operation;
            expr = (*expr).action.args as *mut ExprDef;
        } else if (*expr).common.type_0 as ::core::ffi::c_uint
            == STMT_EXPR_IDENT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut pred_txt_0: *const ::core::ffi::c_char =
                xkb_atom_text((*info).ctx, (*expr).ident.ident);
            if !pred_txt_0.is_null()
                && istreq(pred_txt_0, b"any\0".as_ptr() as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0
            {
                *pred_rtrn = MATCH_ANY;
                *mods_rtrn = MOD_REAL_MASK_ALL;
                return true_0 != 0;
            }
        }
        return ExprResolveModMask(
            (*info).ctx,
            expr,
            MOD_REAL,
            &raw mut (*info).mods,
            mods_rtrn,
        );
    }
}
unsafe extern "C" fn UseNewLEDField(
    mut field: led_field,
    mut old: led_field,
    mut new: led_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut led_field,
) -> bool {
    unsafe {
        if old as ::core::ffi::c_uint & field as ::core::ffi::c_uint == 0 {
            return new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0;
        }
        if new as ::core::ffi::c_uint & field as ::core::ffi::c_uint != 0 {
            if report {
                *collide =
                    (*collide as ::core::ffi::c_uint | field as ::core::ffi::c_uint) as led_field;
            }
            return clobber;
        }
        return false_0 != 0;
    }
}
unsafe extern "C" fn MergeLedMap(
    mut info: *mut CompatInfo,
    mut old: *mut LedInfo,
    mut new: *mut LedInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut collide: led_field = 0 as led_field;
        let clobber: bool = (*new).merge as ::core::ffi::c_uint
            != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
        let verbosity: ::core::ffi::c_int =
            xkb_context_get_log_verbosity((*info).ctx) as ::core::ffi::c_int;
        let report: bool = same_file as ::core::ffi::c_int != 0
            && verbosity > 0 as ::core::ffi::c_int
            || verbosity > 9 as ::core::ffi::c_int;
        if (*old).led.mods.mods == (*new).led.mods.mods
            && (*old).led.pending_groups() as ::core::ffi::c_int
                == (*new).led.pending_groups() as ::core::ffi::c_int
            && (*old).led.groups == (*new).led.groups
            && (*old).led.ctrls as ::core::ffi::c_uint == (*new).led.ctrls as ::core::ffi::c_uint
            && (*old).led.which_mods as ::core::ffi::c_uint
                == (*new).led.which_mods as ::core::ffi::c_uint
            && (*old).led.which_groups() as ::core::ffi::c_int
                == (*new).led.which_groups() as ::core::ffi::c_int
        {
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | (*new).defined as ::core::ffi::c_uint) as led_field;
            return true_0 != 0;
        }
        if (*new).merge as ::core::ffi::c_uint
            == MERGE_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if report {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Map for indicator %s redefined; Earlier definition ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LEDText(info, old),
                );
            }
            *old = *new;
            return true_0 != 0;
        }
        collide = 0 as led_field;
        if UseNewLEDField(
            LED_FIELD_MODS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).led.which_mods = (*new).led.which_mods;
            (*old).led.mods = (*new).led.mods;
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | LED_FIELD_MODS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        }
        if UseNewLEDField(
            LED_FIELD_GROUPS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old)
                .led
                .set_which_groups((*new).led.which_groups() as xkb_state_component);
            (*old).led.groups = (*new).led.groups;
            (*old)
                .led
                .set_pending_groups((*new).led.pending_groups() as bool);
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | LED_FIELD_GROUPS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        }
        if UseNewLEDField(
            LED_FIELD_CTRLS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).led.ctrls = (*new).led.ctrls;
            (*old).defined = ((*old).defined as ::core::ffi::c_uint
                | LED_FIELD_CTRLS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        }
        if collide as u64 != 0 {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Map for indicator %s redefined; Using %s definition for duplicate fields\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LEDText(info, old),
                if clobber as ::core::ffi::c_int != 0 {
                    b"last\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"first\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn AddLedMap(
    mut info: *mut CompatInfo,
    mut new: *mut LedInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut i: xkb_led_index_t = 0 as xkb_led_index_t;
        while i < (*info).num_leds {
            let mut old: *mut LedInfo =
                (&raw mut (*info).leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
            if (*old).led.name != (*new).led.name {
                i = i.wrapping_add(1);
            } else {
                return MergeLedMap(info, old, new, same_file);
            }
        }
        if (*info).num_leds >= XKB_MAX_LEDS {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Too many LEDs defined (maximum %u)\n\0".as_ptr() as *const ::core::ffi::c_char,
                (::core::mem::size_of::<xkb_led_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_led_index_t,
            );
            return false_0 != 0;
        }
        let c2rust_fresh1 = (*info).num_leds;
        (*info).num_leds = (*info).num_leds.wrapping_add(1);
        (*info).leds[c2rust_fresh1 as usize] = *new;
        return true_0 != 0;
    }
}
unsafe extern "C" fn MergeIncludedCompatMaps(
    mut into: *mut CompatInfo,
    mut from: *mut CompatInfo,
    mut merge: merge_mode,
) {
    unsafe {
        if (*from).errorCount > 0 as ::core::ffi::c_int {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        MergeModSets(
            (*into).ctx,
            &raw mut (*into).mods,
            &raw mut (*from).mods,
            merge,
        );
        if (*into).name.is_null() {
            (*into).name = _steal(&raw mut (*from).name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if (*into).interps.size == 0 as darray_size_t {
            (*into).interps = (*from).interps;
            (*from).interps.item = ::core::ptr::null_mut::<SymInterpInfo>();
            (*from).interps.size = 0 as darray_size_t;
            (*from).interps.alloc = 0 as darray_size_t;
        } else {
            let mut si: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
            if !(*from).interps.item.is_null() {
                si = (*from)
                    .interps
                    .item
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut SymInterpInfo;
                while si
                    < (*from).interps.item.offset((*from).interps.size as isize)
                        as *mut SymInterpInfo
                {
                    (*si).merge = merge;
                    if !AddInterp(into, si, false_0 != 0) {
                        (*into).errorCount += 1;
                    }
                    si = si.offset(1);
                }
            }
        }
        if (*into).num_leds == 0 as xkb_led_index_t {
            memcpy(
                &raw mut (*into).leds as *mut LedInfo as *mut ::core::ffi::c_void,
                &raw mut (*from).leds as *mut LedInfo as *const ::core::ffi::c_void,
                (::core::mem::size_of::<LedInfo>() as usize)
                    .wrapping_mul((*from).num_leds as usize),
            );
            (*into).num_leds = (*from).num_leds;
            (*from).num_leds = 0 as xkb_led_index_t;
        } else {
            let mut i: xkb_led_index_t = 0 as xkb_led_index_t;
            while i < (*from).num_leds {
                let mut ledi: *mut LedInfo =
                    (&raw mut (*from).leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
                (*ledi).merge = merge;
                if !AddLedMap(into, ledi, false_0 != 0) {
                    (*into).errorCount += 1;
                }
                i = i.wrapping_add(1);
            }
        };
    }
}
unsafe extern "C" fn HandleIncludeCompatMap(
    mut info: *mut CompatInfo,
    mut include: *mut IncludeStmt,
) -> bool {
    unsafe {
        let mut included: CompatInfo = CompatInfo {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            errorCount: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            },
            interps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<SymInterpInfo>(),
            },
            default_led: LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            },
            leds: [LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            }; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
        };
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as ::core::ffi::c_int;
            return false_0 != 0;
        }
        InitCompatInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as ::core::ffi::c_uint),
            &raw mut (*info).mods,
        );
        included.name = _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: CompatInfo = CompatInfo {
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                errorCount: 0,
                include_depth: 0,
                default_interp: SymInterpInfo {
                    defined: 0 as si_field,
                    merge: MERGE_DEFAULT,
                    interp: xkb_sym_interpret {
                        sym: 0,
                        match_0: MATCH_NONE,
                        mods: 0,
                        virtual_mod: 0,
                        level_one_only: false,
                        repeat_required: [0; 1],
                        num_actions: 0,
                        a: C2Rust_Unnamed_1 {
                            action: xkb_action {
                                type_0: ACTION_TYPE_NONE,
                            },
                        },
                    },
                },
                interps: C2Rust_Unnamed_18 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<SymInterpInfo>(),
                },
                default_led: LedInfo {
                    defined: 0 as led_field,
                    merge: MERGE_DEFAULT,
                    led: xkb_led {
                        name: 0,
                        which_groups_pending_groups: [0; 4],
                        groups: 0,
                        which_mods: 0 as xkb_state_component,
                        mods: xkb_mods { mods: 0, mask: 0 },
                        ctrls: 0 as xkb_action_controls,
                    },
                },
                leds: [LedInfo {
                    defined: 0 as led_field,
                    merge: MERGE_DEFAULT,
                    led: xkb_led {
                        name: 0,
                        which_groups_pending_groups: [0; 4],
                        groups: 0,
                        which_mods: 0 as xkb_state_component,
                        mods: xkb_mods { mods: 0, mask: 0 },
                        ctrls: 0 as xkb_action_controls,
                    },
                }; 32],
                num_leds: 0,
                default_actions: ActionsInfo {
                    actions: [xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    }; 21],
                },
                mods: xkb_mod_set {
                    mods: [xkb_mod {
                        name: 0,
                        type_0: 0 as mod_type,
                        mapping: 0,
                    }; 32],
                    num_mods: 0,
                    explicit_vmods: 0,
                },
                keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
                ctx: ::core::ptr::null_mut::<xkb_context>(),
            };
            let mut file: *mut XkbFile = ::core::ptr::null_mut::<XkbFile>();
            let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_COMPAT,
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            );
            if file.is_null() {
                (*info).errorCount += 10 as ::core::ffi::c_int;
                ClearCompatInfo(&raw mut included);
                return false_0 != 0;
            }
            InitCompatInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as ::core::ffi::c_uint),
                &raw mut included.mods,
            );
            next_incl.default_interp = (*info).default_interp;
            next_incl.default_led = (*info).default_led;
            HandleCompatMapFile(&raw mut next_incl, file);
            MergeIncludedCompatMaps(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearCompatInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedCompatMaps(info, &raw mut included, (*include).merge);
        ClearCompatInfo(&raw mut included);
        return (*info).errorCount == 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn SetInterpField(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    mut field: *const ::core::ffi::c_char,
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        if istreq(field, b"action\0".as_ptr() as *const ::core::ffi::c_char) {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if (*value).common.type_0 as ::core::ffi::c_uint
                == STMT_EXPR_ACTION_LIST as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut num_actions: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                let mut act: *mut ExprDef = (*value).actions.actions as *mut ExprDef;
                while !act.is_null() {
                    num_actions = num_actions.wrapping_add(1);
                    act = (*act).common.next as *mut ExprDef;
                }
                if num_actions > MAX_ACTIONS_PER_LEVEL as ::core::ffi::c_uint {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Interpret %s has too many actions; expected max %u, got: %u\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        siText(si, info),
                        65535 as ::core::ffi::c_int,
                        num_actions,
                    );
                    return false_0 != 0;
                }
                (*si).interp.num_actions = 0 as xkb_action_count_t;
                (*si).interp.a.action.type_0 = ACTION_TYPE_NONE;
                let mut actions: C2Rust_Unnamed_20 = C2Rust_Unnamed_20 {
                    size: 0 as darray_size_t,
                    alloc: 0 as darray_size_t,
                    item: ::core::ptr::null_mut::<xkb_action>(),
                };
                let mut act_0: *mut ExprDef = (*value).actions.actions as *mut ExprDef;
                while !act_0.is_null() {
                    let mut toAct: xkb_action = xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    };
                    match HandleActionDef(
                        (*info).keymap_info,
                        &raw mut (*info).default_actions,
                        &raw mut (*info).mods,
                        act_0,
                        &raw mut toAct,
                    ) as ::core::ffi::c_uint
                    {
                        1 => {
                            toAct.type_0 = ACTION_TYPE_NONE;
                        }
                        2 => {
                            free(actions.item as *mut ::core::ffi::c_void);
                            actions.item = ::core::ptr::null_mut::<xkb_action>();
                            actions.size = 0 as darray_size_t;
                            actions.alloc = 0 as darray_size_t;
                            return false_0 != 0;
                        }
                        _ => {}
                    }
                    if !(toAct.type_0 as ::core::ffi::c_uint
                        == ACTION_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
                    {
                        if (num_actions == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*si).interp.num_actions = 1 as xkb_action_count_t;
                            (*si).interp.a.action = toAct;
                        } else {
                            actions.size = actions.size.wrapping_add(1 as darray_size_t);
                            let mut __need: darray_size_t = actions.size;
                            if __need > actions.alloc {
                                actions.alloc = darray_next_alloc(
                                    actions.alloc,
                                    __need,
                                    ::core::mem::size_of::<xkb_action>() as usize,
                                );
                                actions.item =
                                    realloc(
                                        actions.item as *mut ::core::ffi::c_void,
                                        (actions.alloc as usize).wrapping_mul(
                                            ::core::mem::size_of::<xkb_action>() as usize,
                                        ),
                                    ) as *mut xkb_action;
                            }
                            *actions
                                .item
                                .offset(actions.size.wrapping_sub(1 as darray_size_t) as isize) =
                                toAct;
                        }
                    }
                    act_0 = (*act_0).common.next as *mut ExprDef;
                }
                match actions.size {
                    0 => {
                        if (*si).interp.num_actions as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"si->interp.num_actions <= 1\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"../src/xkbcomp/compat.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                556 as ::core::ffi::c_uint,
                                b"_Bool SetInterpField(CompatInfo *, SymInterpInfo *, const char *, ExprDef *, ExprDef *)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        };
                    }
                    1 => {
                        (*si).interp.num_actions = 1 as xkb_action_count_t;
                        (*si).interp.a.action =
                            *actions.item.offset(1 as ::core::ffi::c_int as isize);
                        free(actions.item as *mut ::core::ffi::c_void);
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                    }
                    _ => {
                        if actions.size > 0 as darray_size_t {
                            actions.alloc = actions.size;
                            actions.item = realloc(
                                actions.item as *mut ::core::ffi::c_void,
                                (actions.alloc as usize).wrapping_mul(::core::mem::size_of::<
                                    xkb_action,
                                >(
                                )
                                    as usize),
                            ) as *mut xkb_action;
                        }
                        (*si).interp.num_actions = actions.size as xkb_action_count_t;
                        (*si).interp.a.actions = actions.item;
                        if !::core::ptr::null_mut::<::core::ffi::c_void>().is_null() {
                            *(::core::ptr::null_mut::<::core::ffi::c_void>()
                                as *mut darray_size_t) = actions.size;
                        }
                        actions.item = ::core::ptr::null_mut::<xkb_action>();
                        actions.size = 0 as darray_size_t;
                        actions.alloc = 0 as darray_size_t;
                    }
                }
            } else {
                match HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    value,
                    &raw mut (*si).interp.a.action,
                ) as ::core::ffi::c_uint
                {
                    1 => {
                        (*si).interp.a.action.type_0 = ACTION_TYPE_NONE;
                        (*si).interp.num_actions = 0 as xkb_action_count_t;
                    }
                    2 => return false_0 != 0,
                    _ => {
                        (*si).interp.num_actions = ((*si).interp.a.action.type_0
                            as ::core::ffi::c_uint
                            != ACTION_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as ::core::ffi::c_int
                            as xkb_action_count_t;
                    }
                }
            }
            (*si).defined = ((*si).defined as ::core::ffi::c_uint
                | SI_FIELD_ACTION as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        } else if istreq(
            field,
            b"virtualmodifier\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || istreq(
                field,
                b"virtualmod\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            let mut ndx: xkb_mod_index_t = 0 as xkb_mod_index_t;
            if !ExprResolveMod(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut ndx,
            ) {
                return ReportSIBadType(
                    info,
                    si,
                    field,
                    b"virtual modifier\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*si).interp.virtual_mod = ndx;
            (*si).defined = ((*si).defined as ::core::ffi::c_uint
                | SI_FIELD_VIRTUAL_MOD as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        } else if istreq(field, b"repeat\0".as_ptr() as *const ::core::ffi::c_char) {
            let mut set: bool = false_0 != 0;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                return ReportSIBadType(
                    info,
                    si,
                    field,
                    b"boolean\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*si).interp.set_repeat(set as bool);
            (*si).defined = ((*si).defined as ::core::ffi::c_uint
                | SI_FIELD_AUTO_REPEAT as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        } else if istreq(field, b"locking\0".as_ptr() as *const ::core::ffi::c_char) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"The \"locking\" field in symbol interpretation is unsupported; Ignored\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if istreq(field, b"usemodmap\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || istreq(
                field,
                b"usemodmapmods\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            let mut val: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveEnum(
                (*info).ctx,
                value,
                &raw mut val,
                &raw const useModMapValueNames as *const LookupEntry,
            ) {
                return ReportSIBadType(
                    info,
                    si,
                    field,
                    b"level specification\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*si).interp.level_one_only = val != 0;
            (*si).defined = ((*si).defined as ::core::ffi::c_uint
                | SI_FIELD_LEVEL_ONE_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint)
                as si_field;
        } else {
            ReportBadField(
                (*info).ctx,
                b"symbol interpretation\0".as_ptr() as *const ::core::ffi::c_char,
                field,
                siText(si, info),
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_INTERPRET_FIELDS as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn SetLedMapField(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    mut field: *const ::core::ffi::c_char,
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"modifiers\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || istreq(field, b"mods\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
                != 0
        {
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveModMask(
                (*info).ctx,
                value,
                MOD_BOTH,
                &raw mut (*info).mods,
                &raw mut (*ledi).led.mods.mods,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"modifier mask\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*ledi).defined = ((*ledi).defined as ::core::ffi::c_uint
                | LED_FIELD_MODS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        } else if istreq(field, b"groups\0".as_ptr() as *const ::core::ffi::c_char) {
            let mut mask: xkb_layout_mask_t = 0 as xkb_layout_mask_t;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let mut pending: bool = false_0 != 0;
            if !ExprResolveGroupMask((*info).keymap_info, value, &raw mut mask, &raw mut pending) {
                if pending {
                    (*ledi).led.set_pending_groups((true_0 != 0) as bool);
                    let pending_index: darray_size_t =
                        (*(*(*info).keymap_info).pending_computations).size;
                    (*(*(*info).keymap_info).pending_computations).size = (*(*(*info).keymap_info)
                        .pending_computations)
                        .size
                        .wrapping_add(1 as darray_size_t);
                    let mut __need: darray_size_t =
                        (*(*(*info).keymap_info).pending_computations).size;
                    if __need > (*(*(*info).keymap_info).pending_computations).alloc {
                        (*(*(*info).keymap_info).pending_computations).alloc = darray_next_alloc(
                            (*(*(*info).keymap_info).pending_computations).alloc,
                            __need,
                            ::core::mem::size_of::<pending_computation>() as usize,
                        );
                        (*(*(*info).keymap_info).pending_computations).item = realloc(
                            (*(*(*info).keymap_info).pending_computations).item
                                as *mut ::core::ffi::c_void,
                            ((*(*(*info).keymap_info).pending_computations).alloc as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<pending_computation>() as usize
                                ),
                        )
                            as *mut pending_computation;
                    }
                    *(*(*(*info).keymap_info).pending_computations).item.offset(
                        (*(*(*info).keymap_info).pending_computations)
                            .size
                            .wrapping_sub(1 as darray_size_t) as isize,
                    ) = pending_computation {
                        expr: *value_ptr,
                        computed: false,
                        value: 0 as u32,
                    };
                    *value_ptr = ::core::ptr::null_mut::<ExprDef>();
                    mask = pending_index as xkb_layout_mask_t;
                } else {
                    return ReportLedBadType(
                        info,
                        ledi,
                        field,
                        b"group mask\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            } else {
                (*ledi).led.set_pending_groups((false_0 != 0) as bool);
            }
            (*ledi).led.groups = mask;
            (*ledi).defined = ((*ledi).defined as ::core::ffi::c_uint
                | LED_FIELD_GROUPS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        } else if istreq(field, b"controls\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || istreq(field, b"ctrls\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            let mut mask_0: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let offset: uint8_t = (*(*info).keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_0,
                (&raw const ctrlMaskNames as *const LookupEntry)
                    .offset(offset as ::core::ffi::c_int as isize),
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"controls mask\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*ledi).led.ctrls = mask_0 as xkb_action_controls;
            (*ledi).defined = ((*ledi).defined as ::core::ffi::c_uint
                | LED_FIELD_CTRLS as ::core::ffi::c_int as ::core::ffi::c_uint)
                as led_field;
        } else if istreq(
            field,
            b"allowexplicit\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if istreq(
            field,
            b"whichmodstate\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || istreq(
                field,
                b"whichmodifierstate\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            let mut mask_1: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_1,
                &raw const modComponentMaskNames as *const LookupEntry,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"mask of modifier state components\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*ledi).led.which_mods = mask_1 as xkb_state_component;
        } else if istreq(
            field,
            b"whichgroupstate\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            let mut mask_2: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_2,
                &raw const groupComponentMaskNames as *const LookupEntry,
            ) {
                return ReportLedBadType(
                    info,
                    ledi,
                    field,
                    b"mask of group state components\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*ledi)
                .led
                .set_which_groups(mask_2 as xkb_state_component as xkb_state_component);
        } else if istreq(field, b"driveskbd\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || istreq(
                field,
                b"driveskeyboard\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || istreq(
                field,
                b"leddriveskbd\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || istreq(
                field,
                b"leddriveskeyboard\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || istreq(
                field,
                b"indicatordriveskbd\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || istreq(
                field,
                b"indicatordriveskeyboard\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"The \"%s\" field in indicator statements is unsupported; Ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                field,
            );
        } else if istreq(field, b"index\0".as_ptr() as *const ::core::ffi::c_char) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"The \"index\" field in indicator statements is unsupported; Ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Unknown field \"%s\" in map for %s indicator; Definition ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                field,
                LEDText(info, ledi),
            );
            return (*(*info).keymap_info).strict as ::core::ffi::c_uint
                & PARSER_NO_UNKNOWN_LED_FIELDS as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn HandleGlobalVar(mut info: *mut CompatInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut field: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut ndx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut ndx,
        ) {
            ret = false_0 != 0;
        } else if !elem.is_null()
            && istreq(elem, b"interpret\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            let mut temp: SymInterpInfo = SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            };
            InitInterp(&raw mut temp);
            temp.merge = (if temp.merge as ::core::ffi::c_uint
                == MERGE_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                MERGE_OVERRIDE as ::core::ffi::c_int as ::core::ffi::c_uint
            } else {
                (*stmt).merge as ::core::ffi::c_uint
            }) as merge_mode;
            ret = SetInterpField(
                info,
                &raw mut temp,
                field,
                ndx,
                (*stmt).value as *mut ExprDef,
            );
            if ret {
                MergeInterp(
                    info,
                    &raw mut (*info).default_interp,
                    &raw mut temp,
                    true_0 != 0,
                );
            }
        } else if !elem.is_null()
            && istreq(elem, b"indicator\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            };
            InitLED(&raw mut temp_0);
            temp_0.merge = (if temp_0.merge as ::core::ffi::c_uint
                == MERGE_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                MERGE_OVERRIDE as ::core::ffi::c_int as ::core::ffi::c_uint
            } else {
                (*stmt).merge as ::core::ffi::c_uint
            }) as merge_mode;
            ret = SetLedMapField(info, &raw mut temp_0, field, ndx, &raw mut (*stmt).value);
            if ret {
                MergeLedMap(
                    info,
                    &raw mut (*info).default_led,
                    &raw mut temp_0,
                    true_0 != 0,
                );
            }
        } else if !elem.is_null() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                ndx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as ::core::ffi::c_uint
                != PARSER_FATAL_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint;
        } else {
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
                & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0;
        }
        return ret;
    }
}
unsafe extern "C" fn HandleInterpBody(
    mut info: *mut CompatInfo,
    mut def: *mut VarDef,
    mut si: *mut SymInterpInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true_0 != 0;
        let mut elem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut field: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
        while !def.is_null() {
            if !ExprResolveLhs(
                (*info).ctx,
                (*def).name,
                &raw mut elem,
                &raw mut field,
                &raw mut arrayNdx,
            ) {
                ok = false_0 != 0;
            } else if !elem.is_null() {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Cannot set a global default value for \"%s\" element from within an interpret statement; Move assignment to \"%s.%s\" to the global file scope\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    elem,
                    elem,
                    field,
                );
                ok = false_0 != 0;
            } else if !SetInterpField(info, si, field, arrayNdx, (*def).value as *mut ExprDef) {
                ok = false_0 != 0;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return ok;
    }
}
unsafe extern "C" fn HandleInterpDef(mut info: *mut CompatInfo, mut def: *mut InterpDef) -> bool {
    unsafe {
        let mut pred: xkb_match_operation = MATCH_NONE;
        let mut mods: xkb_mod_mask_t = 0;
        let mut si: SymInterpInfo = SymInterpInfo {
            defined: 0 as si_field,
            merge: MERGE_DEFAULT,
            interp: xkb_sym_interpret {
                sym: 0,
                match_0: MATCH_NONE,
                mods: 0,
                virtual_mod: 0,
                level_one_only: false,
                repeat_required: [0; 1],
                num_actions: 0,
                a: C2Rust_Unnamed_1 {
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                },
            },
        };
        if !ResolveStateAndPredicate(
            (*def).match_0 as *mut ExprDef,
            &raw mut pred,
            &raw mut mods,
            info,
        ) {
            xkb_log(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Couldn't determine matching modifiers; Symbol interpretation ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        si = (*info).default_interp;
        si.merge = (*def).merge;
        si.interp.sym = (*def).sym;
        si.interp.match_0 = pred;
        si.interp.mods = mods;
        if !HandleInterpBody(info, (*def).def, &raw mut si) {
            (*info).errorCount += 1;
            return false_0 != 0;
        }
        if !AddInterp(info, &raw mut si, true_0 != 0) {
            (*info).errorCount += 1;
            return false_0 != 0;
        }
        return true_0 != 0;
    }
}
unsafe extern "C" fn HandleLedMapDef(mut info: *mut CompatInfo, mut def: *mut LedMapDef) -> bool {
    unsafe {
        let mut ledi: LedInfo = (*info).default_led;
        ledi.merge = (*def).merge;
        ledi.led.name = (*def).name;
        let mut ok: bool = true_0 != 0;
        let mut var: *mut VarDef = (*def).body;
        while !var.is_null() {
            let mut elem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut field: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut arrayNdx: *mut ExprDef = ::core::ptr::null_mut::<ExprDef>();
            if !ExprResolveLhs(
                (*info).ctx,
                (*var).name,
                &raw mut elem,
                &raw mut field,
                &raw mut arrayNdx,
            ) {
                ok = false_0 != 0;
            } else if !elem.is_null() {
                xkb_log(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"[XKB-%03d] Cannot set defaults for \"%s\" element in indicator map; Assignment to %s.%s ignored\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as ::core::ffi::c_int,
                    elem,
                    elem,
                    field,
                );
                ok = false_0 != 0;
            } else if !SetLedMapField(info, &raw mut ledi, field, arrayNdx, &raw mut (*var).value) {
                ok = false_0 != 0;
            }
            var = (*var).common.next as *mut VarDef;
        }
        return ok as ::core::ffi::c_int != 0
            && AddLedMap(info, &raw mut ledi, true_0 != 0) as ::core::ffi::c_int != 0;
    }
}
unsafe extern "C" fn HandleCompatMapFile(mut info: *mut CompatInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        free((*info).name as *mut ::core::ffi::c_void);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as ::core::ffi::c_uint {
                1 => {
                    ok = HandleIncludeCompatMap(info, stmt as *mut IncludeStmt);
                }
                28 => {
                    ok = HandleInterpDef(info, stmt as *mut InterpDef);
                }
                32 => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_DEBUG,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"The \"group\" statement in compat is unsupported; Ignored\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    ok = true_0 != 0;
                }
                33 => {
                    ok = HandleLedMapDef(info, stmt as *mut LedMapDef);
                }
                26 => {
                    ok = HandleGlobalVar(info, stmt as *mut VarDef);
                }
                29 => {
                    ok = HandleVModDef((*info).ctx, &raw mut (*info).mods, stmt as *mut VModDef);
                }
                35 | 36 => {
                    xkb_log(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"[XKB-%03d] Unsupported compatibility %s statement \"%s\"; Ignoring\n\0"
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
                        b"Compat files may not include other types; Ignoring %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
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
                    b"Abandoning compatibility map \"%s\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    safe_map_name(file),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe extern "C" fn CopyInterps(
    mut info: *mut CompatInfo,
    mut needSymbol: bool,
    mut pred: xkb_match_operation,
    mut collect: *mut collect,
) {
    unsafe {
        let mut si: *mut SymInterpInfo = ::core::ptr::null_mut::<SymInterpInfo>();
        if !(*info).interps.item.is_null() {
            si = (*info)
                .interps
                .item
                .offset(0 as ::core::ffi::c_int as isize) as *mut SymInterpInfo;
            while si
                < (*info).interps.item.offset((*info).interps.size as isize) as *mut SymInterpInfo
            {
                if (*si).interp.match_0 as ::core::ffi::c_uint == pred as ::core::ffi::c_uint
                    && ((*si).interp.sym != XKB_KEY_NoSymbol as xkb_keysym_t) as ::core::ffi::c_int
                        == needSymbol as ::core::ffi::c_int
                {
                    (*collect).sym_interprets.size = (*collect)
                        .sym_interprets
                        .size
                        .wrapping_add(1 as darray_size_t);
                    let mut __need: darray_size_t = (*collect).sym_interprets.size;
                    if __need > (*collect).sym_interprets.alloc {
                        (*collect).sym_interprets.alloc = darray_next_alloc(
                            (*collect).sym_interprets.alloc,
                            __need,
                            ::core::mem::size_of::<xkb_sym_interpret>() as usize,
                        );
                        (*collect).sym_interprets.item = realloc(
                            (*collect).sym_interprets.item as *mut ::core::ffi::c_void,
                            ((*collect).sym_interprets.alloc as usize)
                                .wrapping_mul(::core::mem::size_of::<xkb_sym_interpret>() as usize),
                        )
                            as *mut xkb_sym_interpret;
                    }
                    *(*collect).sym_interprets.item.offset(
                        (*collect)
                            .sym_interprets
                            .size
                            .wrapping_sub(1 as darray_size_t) as isize,
                    ) = (*si).interp;
                }
                si = si.offset(1);
            }
        }
    }
}
unsafe extern "C" fn CopyLedMapDefsToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut CompatInfo,
) {
    unsafe {
        let mut c2rust_current_block_11: u64;
        let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
        while idx < (*info).num_leds {
            let mut ledi: *mut LedInfo =
                (&raw mut (*info).leds as *mut LedInfo).offset(idx as isize) as *mut LedInfo;
            let mut i: xkb_led_index_t = 0;
            let mut led: *mut xkb_led = ::core::ptr::null_mut::<xkb_led>();
            i = 0 as xkb_led_index_t;
            led = &raw mut (*keymap).leds as *mut xkb_led;
            while i < (*keymap).num_leds {
                if (*led).name == (*ledi).led.name {
                    break;
                }
                i = i.wrapping_add(1);
                led = led.offset(1);
            }
            if i >= (*keymap).num_leds {
                xkb_log(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Indicator name \"%s\" was not declared in the keycodes section; Adding new indicator\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LEDText(info, ledi),
                );
                i = 0 as xkb_led_index_t;
                led = &raw mut (*keymap).leds as *mut xkb_led;
                while i < (*keymap).num_leds {
                    if (*led).name == XKB_ATOM_NONE as xkb_atom_t {
                        break;
                    }
                    i = i.wrapping_add(1);
                    led = led.offset(1);
                }
                if i >= (*keymap).num_leds {
                    if i >= XKB_MAX_LEDS {
                        xkb_log(
                            (*keymap).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                            b"Too many indicators (maximum is %u); Indicator name \"%s\" ignored\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (::core::mem::size_of::<xkb_led_mask_t>() as usize)
                                .wrapping_mul(8 as usize) as xkb_led_index_t,
                            LEDText(info, ledi),
                        );
                        c2rust_current_block_11 = 792017965103506125;
                    } else {
                        let c2rust_fresh0 = (*keymap).num_leds;
                        (*keymap).num_leds = (*keymap).num_leds.wrapping_add(1);
                        led = (&raw mut (*keymap).leds as *mut xkb_led)
                            .offset(c2rust_fresh0 as isize)
                            as *mut xkb_led;
                        c2rust_current_block_11 = 17860125682698302841;
                    }
                } else {
                    c2rust_current_block_11 = 17860125682698302841;
                }
            } else {
                c2rust_current_block_11 = 17860125682698302841;
            }
            match c2rust_current_block_11 {
                17860125682698302841 => {
                    *led = (*ledi).led;
                    if (*led).which_groups() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && ((*led).groups != 0 as xkb_layout_mask_t
                            || (*led).pending_groups() as ::core::ffi::c_int != 0)
                    {
                        (*led).set_which_groups(XKB_STATE_LAYOUT_EFFECTIVE as xkb_state_component);
                    }
                    if (*led).which_mods as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                        && (*led).mods.mods != 0 as xkb_mod_mask_t
                    {
                        (*led).which_mods = XKB_STATE_MODS_EFFECTIVE;
                    }
                }
                _ => {}
            }
            idx = idx.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn CopyCompatToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut CompatInfo,
) -> bool {
    unsafe {
        (*keymap).compat_section_name = strdup_safe((*info).name);
        XkbEscapeMapName((*keymap).compat_section_name);
        (*keymap).mods = (*info).mods;
        if !((*info).interps.size == 0 as darray_size_t) {
            let mut collect: collect = collect {
                sym_interprets: C2Rust_Unnamed_19 {
                    size: 0,
                    alloc: 0,
                    item: ::core::ptr::null_mut::<xkb_sym_interpret>(),
                },
            };
            collect.sym_interprets.item = ::core::ptr::null_mut::<xkb_sym_interpret>();
            collect.sym_interprets.size = 0 as darray_size_t;
            collect.sym_interprets.alloc = 0 as darray_size_t;
            CopyInterps(info, true_0 != 0, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, true_0 != 0, MATCH_ALL, &raw mut collect);
            CopyInterps(info, true_0 != 0, MATCH_NONE, &raw mut collect);
            CopyInterps(info, true_0 != 0, MATCH_ANY, &raw mut collect);
            CopyInterps(info, true_0 != 0, MATCH_ANY_OR_NONE, &raw mut collect);
            CopyInterps(info, false_0 != 0, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, false_0 != 0, MATCH_ALL, &raw mut collect);
            CopyInterps(info, false_0 != 0, MATCH_NONE, &raw mut collect);
            CopyInterps(info, false_0 != 0, MATCH_ANY, &raw mut collect);
            CopyInterps(info, false_0 != 0, MATCH_ANY_OR_NONE, &raw mut collect);
            (*keymap).sym_interprets = collect.sym_interprets.item;
            if !(&raw mut (*keymap).num_sym_interprets).is_null() {
                *&raw mut (*keymap).num_sym_interprets = collect.sym_interprets.size;
            }
            collect.sym_interprets.item = ::core::ptr::null_mut::<xkb_sym_interpret>();
            collect.sym_interprets.size = 0 as darray_size_t;
            collect.sym_interprets.alloc = 0 as darray_size_t;
        }
        CopyLedMapDefsToKeymap(keymap, info);
        return true_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompileCompatMap(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: CompatInfo = CompatInfo {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            errorCount: 0,
            include_depth: 0,
            default_interp: SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat_required: [0; 1],
                    num_actions: 0,
                    a: C2Rust_Unnamed_1 {
                        action: xkb_action {
                            type_0: ACTION_TYPE_NONE,
                        },
                    },
                },
            },
            interps: C2Rust_Unnamed_18 {
                size: 0,
                alloc: 0,
                item: ::core::ptr::null_mut::<SymInterpInfo>(),
            },
            default_led: LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            },
            leds: [LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups_pending_groups: [0; 4],
                    groups: 0,
                    which_mods: 0 as xkb_state_component,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            }; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as mod_type,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            keymap_info: ::core::ptr::null::<xkb_keymap_info>(),
            ctx: ::core::ptr::null_mut::<xkb_context>(),
        };
        InitCompatInfo(
            &raw mut info,
            keymap_info,
            0 as ::core::ffi::c_uint,
            &raw mut (*keymap_info).keymap.mods,
        );
        if !file.is_null() {
            HandleCompatMapFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as ::core::ffi::c_int) {
            if CopyCompatToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearCompatInfo(&raw mut info);
                return true_0 != 0;
            }
        }
        ClearCompatInfo(&raw mut info);
        return false_0 != 0;
    }
}
