pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}

pub mod types_h {
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __dev_t = u64;
    pub type __uid_t = u32;
    pub type __gid_t = u32;
    pub type __ino_t = u64;
    pub type __mode_t = u32;
    pub type __nlink_t = u64;
    pub type __off_t = i64;
    pub type __off64_t = i64;
    pub type __time_t = i64;
    pub type __blksize_t = i64;
    pub type __blkcnt_t = i64;
    pub type __syscall_slong_t = i64;
}
pub mod stdint_intn_h {
    pub type i8 = __int8_t;
    pub type i16 = __int16_t;
    pub type i32 = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: i32,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
        pub _cur_column: u16,
        pub _vtable_offset: i8,
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
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    pub const _IONBF: i32 = 2 as i32;
    pub const BUFSIZ: i32 = 8192 as i32;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    use super::FILE_h::FILE;

    extern "C" {
        pub static mut stdout: *mut FILE;
        pub static mut stderr: *mut FILE;
        pub fn fclose(__stream: *mut FILE) -> i32;
        pub fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
        pub fn setvbuf(__stream: *mut FILE, __buf: *mut i8, __modes: i32, __n: usize) -> i32;
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: usize,
            __n: usize,
            __stream: *mut FILE,
        ) -> u64;
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: usize,
            __n: usize,
            __s: *mut FILE,
        ) -> u64;
        pub fn feof(__stream: *mut FILE) -> i32;
        pub fn ferror(__stream: *mut FILE) -> i32;
        pub fn fileno(__stream: *mut FILE) -> i32;
    }
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
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
pub mod darray_h {
    pub use crate::xkb::shared_types::*;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_string {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut i8,
    }
    pub unsafe fn darray_next_alloc(
        mut alloc: darray_size_t,
        need: darray_size_t,
        itemSize: usize,
    ) -> darray_size_t {
        let max_alloc = (u32::MAX as usize) / itemSize / 2;
        assert!(
            (need as usize) < max_alloc,
            "need < darray_max_alloc(itemSize) / 2"
        );
        if alloc == 0 {
            alloc = 4;
        }
        while alloc < need {
            alloc = alloc.wrapping_mul(2);
        }
        alloc
    }
}
pub mod xkbcommon_h {
    use super::rmlvo_h::xkb_rmlvo_builder;
    use super::xkbcommon_errors_h::xkb_error_code;
    use super::FILE_h::FILE;
    pub use crate::xkb::context::{
        xkb_context_include_path_append, xkb_context_new, xkb_context_unref,
    };
    pub use crate::xkb::shared_types::*;

    pub type xkb_rmlvo_builder_flags = u32;
    pub const XKB_RMLVO_BUILDER_NO_FLAGS: xkb_rmlvo_builder_flags = 0;
    pub type xkb_context_flags = u32;
    pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
    pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
    pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
    pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
    pub type xkb_event_type = u32;
    pub const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 4;
    pub const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 3;
    pub const XKB_EVENT_TYPE_KEY_REPEATED: xkb_event_type = 2;
    pub const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
    pub type xkb_key_direction = u32;
    pub const XKB_KEY_REPEATED: xkb_key_direction = 2;
    pub const XKB_KEY_DOWN: xkb_key_direction = 1;
    pub const XKB_KEY_UP: xkb_key_direction = 0;
    pub type xkb_consumed_mode = u32;
    pub const XKB_CONSUMED_MODE_GTK: xkb_consumed_mode = 1;
    pub const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;
    pub const XKB_KEYMAP_USE_ORIGINAL_FORMAT: xkb_keymap_format = 4294967295 as xkb_keymap_format;
    extern "C" {
        pub type xkb_machine;
        pub type xkb_state;
        pub type xkb_event;
        pub type xkb_events;
        pub fn xkb_rmlvo_builder_new(
            context: *mut xkb_context,
            rules: *const i8,
            model: *const i8,
            flags: xkb_rmlvo_builder_flags,
        ) -> *mut xkb_rmlvo_builder;
        pub fn xkb_rmlvo_builder_append_layout(
            rmlvo: *mut xkb_rmlvo_builder,
            layout: *const i8,
            variant: *const i8,
            options: *const *const i8,
            options_len: usize,
        ) -> bool;
        pub fn xkb_rmlvo_builder_append_option(
            rmlvo: *mut xkb_rmlvo_builder,
            option: *const i8,
        ) -> bool;
        pub fn xkb_rmlvo_builder_unref(rmlvo: *mut xkb_rmlvo_builder);
        pub fn xkb_keysym_get_name(keysym: xkb_keysym_t, buffer: *mut i8, size: usize) -> i32;
        pub fn xkb_keymap_new_from_rmlvo(
            rmlvo: *const xkb_rmlvo_builder,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_names2(
            context: *mut xkb_context,
            names: *const xkb_rule_names,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_string(
            context: *mut xkb_context,
            string: *const i8,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_new_from_buffer(
            context: *mut xkb_context,
            buffer: *const i8,
            length: usize,
            format: xkb_keymap_format,
            flags: xkb_keymap_compile_flags,
        ) -> *mut xkb_keymap;
        pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
        pub fn xkb_keymap_get_as_string2(
            keymap: *mut xkb_keymap,
            format: xkb_keymap_format,
            flags: xkb_keymap_serialize_flags,
        ) -> *mut i8;
        pub fn xkb_keymap_num_leds(keymap: *mut xkb_keymap) -> xkb_led_index_t;
        pub fn xkb_event_get_type(event: *const xkb_event) -> xkb_event_type;
        pub fn xkb_event_get_keycode(event: *const xkb_event) -> xkb_keycode_t;
        pub fn xkb_events_next(events: *mut xkb_events) -> *const xkb_event;
        pub fn xkb_machine_process_key(
            machine: *mut xkb_machine,
            key: xkb_keycode_t,
            direction: xkb_key_direction,
            events: *mut xkb_events,
        ) -> xkb_error_code;
        pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
        pub fn xkb_state_unref(state: *mut xkb_state);
        pub fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;
        pub fn xkb_state_update_key(
            state: *mut xkb_state,
            key: xkb_keycode_t,
            direction: xkb_key_direction,
        ) -> xkb_state_component;
        pub fn xkb_state_update_event(
            state: *mut xkb_state,
            event: *const xkb_event,
        ) -> xkb_state_component;
        pub fn xkb_state_key_get_syms(
            state: *mut xkb_state,
            key: xkb_keycode_t,
            syms_out: *mut *const xkb_keysym_t,
        ) -> i32;
        pub fn xkb_state_serialize_mods(
            state: *mut xkb_state,
            components: xkb_state_component,
        ) -> xkb_mod_mask_t;
        pub fn xkb_state_serialize_layout(
            state: *mut xkb_state,
            components: xkb_state_component,
        ) -> xkb_layout_index_t;
        pub fn xkb_state_led_index_is_active(state: *mut xkb_state, idx: xkb_led_index_t) -> i32;
    }
}
pub mod keymap_h {
    pub use crate::xkb::shared_types::*;

    pub type C2Rust_Unnamed_13 = u32;
    pub const _LAST_XKB_EVENT_TYPE: C2Rust_Unnamed_13 = 4;
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
    use super::darray_h::darray_size_t;
    use super::xkbcommon_h::xkb_layout_index_t;
}
pub mod xkbcommon_compose_h {
    extern "C" {
        pub type xkb_compose_state;
    }
}
pub mod test_h {
    pub type key_seq_state = u32;
    pub const FINISH: key_seq_state = 5;
    pub const NEXT: key_seq_state = 4;
    pub const BOTH: key_seq_state = 3;
    pub const UP: key_seq_state = 2;
    pub const REPEAT: key_seq_state = 1;
    pub const DOWN: key_seq_state = 0;
    pub type test_context_flags = u32;
    pub const CONTEXT_ALLOW_ENVIRONMENT_NAMES: test_context_flags = 1;
    pub const CONTEXT_NO_FLAG: test_context_flags = 0;
    pub type test_compile_buffer_t = Option<
        unsafe fn(
            *mut xkb_context,
            xkb_keymap_format,
            *const i8,
            usize,
            *mut ::core::ffi::c_void,
        ) -> *mut xkb_keymap,
    >;
    pub type test_third_party_compile_buffer_t = Option<
        unsafe fn(*const i8, usize, *mut ::core::ffi::c_void, *mut *mut i8, *mut usize) -> i32,
    >;
    pub const EVDEV_OFFSET: i32 = 8 as i32;

    use super::context_h::xkb_context;
    use super::keymap_h::xkb_keymap;
    use super::xkbcommon_h::xkb_keymap_format;
}
pub mod tools_common_h {
    pub type print_state_options = u32;
    pub const DEFAULT_PRINT_OPTIONS: print_state_options = 15;
    pub const PRINT_UNILINE: print_state_options = 8;
    pub const PRINT_VERBOSE: print_state_options = 4;
    pub const PRINT_VERBOSE_ONE_LINE_FIELDS: print_state_options = 3;
    pub const PRINT_ALL_FIELDS: print_state_options = 3;
    pub const PRINT_UNICODE: print_state_options = 2;
    pub const PRINT_LAYOUT: print_state_options = 1;
    use super::xkbcommon_compose_h::xkb_compose_state;
    use super::xkbcommon_h::{xkb_consumed_mode, xkb_key_direction, xkb_keycode_t, xkb_state};
    extern "C" {
        pub fn tools_print_keycode_state(
            prefix: *const i8,
            state: *mut xkb_state,
            compose_state: *mut xkb_compose_state,
            keycode: xkb_keycode_t,
            direction: xkb_key_direction,
            consumed_mode: xkb_consumed_mode,
            options: print_state_options,
        );
    }
}
pub mod stdlib_h {
    pub const EXIT_SUCCESS: i32 = 0 as i32;

    extern "C" {
        pub fn malloc(__size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn getenv(__name: *const i8) -> *mut i8;
        pub fn unsetenv(__name: *const i8) -> i32;
        pub fn mkdtemp(__template: *mut i8) -> *mut i8;
    }
}
pub mod stat_h {
    use super::struct_stat_h::stat;
    use super::types_h::__mode_t;
    extern "C" {
        pub fn fstat(__fd: i32, __buf: *mut stat) -> i32;
        pub fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe fn streq(s1: *const i8, s2: *const i8) -> bool {
        assert!(!s1.is_null() && !s2.is_null(), "s1 && s2");
        unsafe { std::ffi::CStr::from_ptr(s1) == std::ffi::CStr::from_ptr(s2) }
    }
    #[inline]
    pub unsafe fn isempty(s: *const i8) -> bool {
        s.is_null() || unsafe { *s == 0 }
    }
}
pub mod include_locale_h {
    pub const LC_ALL: i32 = __LC_ALL;
    use super::locale_h::__LC_ALL;
    extern "C" {
        pub fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    }
}
pub mod utils_numbers_h {
    #[inline]
    pub unsafe fn parse_dec_to_uint32_t(s: *const i8, len: usize, out: *mut u32) -> i32 {
        unsafe {
            let bytes = std::slice::from_raw_parts(s as *const u8, len);
            let mut result: u32 = 0;
            let mut i: usize = 0;
            while i < bytes.len() {
                let digit = bytes[i].wrapping_sub(b'0');
                if digit >= 10 {
                    break;
                }
                if result > u32::MAX / 10 {
                    break;
                }
                let next = result.wrapping_mul(10);
                if next > u32::MAX - digit as u32 {
                    break;
                }
                result = next + digit as u32;
                i += 1;
            }
            *out = result;
            if i >= len || bytes.get(i).map_or(true, |&b| b.wrapping_sub(b'0') >= 10) {
                i as i32
            } else {
                -1
            }
        }
    }

    use super::stdint_uintn_h::u32;
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: i32 = 0 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
    pub const NULL_0: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod stdint_h {
    pub const SIZE_MAX: u64 = 18446744073709551615 as u64;
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const i8,
            __file: *const i8,
            __line: u32,
            __function: *const i8,
        ) -> !;
    }
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut i32;
    }
}
pub mod utils_paths_h {
    pub use crate::xkb::utils_paths::is_absolute_path;
}
pub mod rules_h {
    pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
}
pub mod locale_h {
    pub const __LC_ALL: i32 = 6 as i32;
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::{NULL, NULL_0};

use self::assert_h::__assert_fail;
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_next_alloc, darray_size_t, darray_string};
use self::errno_h::__errno_location;
pub use self::include_locale_h::{setlocale, LC_ALL};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::keymap_h::{
    mod_type, xkb_action, xkb_action_controls, xkb_action_count_t, xkb_action_flags,
    xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action,
    xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type,
    xkb_key_type_entry, xkb_keymap, xkb_keysym_count_t, xkb_led, xkb_level, xkb_match_operation,
    xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t, xkb_pointer_action,
    xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_1,
    C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_13, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7,
    C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, _ACTION_TYPE_NUM_ENTRIES,
    _LAST_XKB_EVENT_TYPE, ACTION_ABSOLUTE_SWITCH, ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y,
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
};
pub use self::locale_h::__LC_ALL;
pub use self::rmlvo_h::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_layouts,
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options,
};
pub use self::rules_h::OPTIONS_GROUP_SPECIFIER_PREFIX;
use self::stat_h::{fstat, mkdir};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::SIZE_MAX;
pub use self::stdint_intn_h::{i16, i32, i8};
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::stdio_h::{
    fclose, feof, ferror, fileno, fopen, fread, fwrite, setvbuf, stderr, stdout, va_list, _IONBF,
    BUFSIZ,
};
pub use self::stdlib_h::{free, getenv, malloc, mkdtemp, realloc, unsetenv, EXIT_SUCCESS};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::test_h::{
    key_seq_state, test_compile_buffer_t, test_context_flags, test_third_party_compile_buffer_t,
    BOTH, CONTEXT_ALLOW_ENVIRONMENT_NAMES, CONTEXT_NO_FLAG, DOWN, EVDEV_OFFSET, FINISH, NEXT,
    REPEAT, UP,
};
pub use self::tools_common_h::{
    print_state_options, tools_print_keycode_state, DEFAULT_PRINT_OPTIONS, PRINT_ALL_FIELDS,
    PRINT_LAYOUT, PRINT_UNICODE, PRINT_UNILINE, PRINT_VERBOSE, PRINT_VERBOSE_ONE_LINE_FIELDS,
};
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __int16_t, __int32_t, __int8_t, __mode_t,
    __nlink_t, __off64_t, __off_t, __syscall_slong_t, __time_t, __uid_t, __uint16_t, __uint32_t,
    __uint64_t, __uint8_t,
};
pub use self::utils_h::{isempty, streq};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
use self::utils_paths_h::is_absolute_path;
use self::xkbcommon_compose_h::xkb_compose_state;
pub use self::xkbcommon_errors_h::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_h::{
    xkb_consumed_mode, xkb_context_flags, xkb_context_include_path_append, xkb_context_new,
    xkb_context_unref, xkb_event, xkb_event_get_keycode, xkb_event_get_type, xkb_event_type,
    xkb_events, xkb_events_next, xkb_key_direction, xkb_keycode_t, xkb_keymap_compile_flags,
    xkb_keymap_format, xkb_keymap_get_as_string2, xkb_keymap_new_from_buffer,
    xkb_keymap_new_from_file, xkb_keymap_new_from_names2, xkb_keymap_new_from_rmlvo,
    xkb_keymap_new_from_string, xkb_keymap_num_leds, xkb_keymap_serialize_flags, xkb_keymap_unref,
    xkb_keysym_get_name, xkb_keysym_t, xkb_layout_index_t, xkb_layout_mask_t,
    xkb_layout_out_of_range_policy, xkb_led_index_t, xkb_led_mask_t, xkb_level_index_t,
    xkb_log_level, xkb_machine, xkb_machine_process_key, xkb_mod_index_t, xkb_mod_mask_t,
    xkb_rmlvo_builder_append_layout, xkb_rmlvo_builder_append_option, xkb_rmlvo_builder_flags,
    xkb_rmlvo_builder_new, xkb_rmlvo_builder_unref, xkb_rule_names, xkb_state, xkb_state_component,
    xkb_state_get_keymap, xkb_state_key_get_syms, xkb_state_led_index_is_active, xkb_state_new,
    xkb_state_serialize_layout, xkb_state_serialize_mods, xkb_state_unref, xkb_state_update_event,
    xkb_state_update_key, XKB_CONSUMED_MODE_GTK, XKB_CONSUMED_MODE_XKB,
    XKB_CONTEXT_NO_DEFAULT_INCLUDES, XKB_CONTEXT_NO_ENVIRONMENT_NAMES, XKB_CONTEXT_NO_FLAGS,
    XKB_CONTEXT_NO_SECURE_GETENV, XKB_EVENT_TYPE_COMPONENTS_CHANGE, XKB_EVENT_TYPE_KEY_DOWN,
    XKB_EVENT_TYPE_KEY_REPEATED, XKB_EVENT_TYPE_KEY_UP, XKB_KEYMAP_COMPILE_NO_FLAGS,
    XKB_KEYMAP_COMPILE_STRICT_MODE, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
    XKB_KEYMAP_SERIALIZE_EXPLICIT, XKB_KEYMAP_SERIALIZE_KEEP_UNUSED, XKB_KEYMAP_SERIALIZE_NO_FLAGS,
    XKB_KEYMAP_SERIALIZE_PRETTY, XKB_KEYMAP_USE_ORIGINAL_FORMAT, XKB_KEY_DOWN, XKB_KEY_REPEATED,
    XKB_KEY_UP, XKB_LAYOUT_INVALID, XKB_LAYOUT_OUT_OF_RANGE_CLAMP,
    XKB_LAYOUT_OUT_OF_RANGE_REDIRECT, XKB_LAYOUT_OUT_OF_RANGE_WRAP, XKB_LOG_LEVEL_CRITICAL,
    XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
    XKB_RMLVO_BUILDER_NO_FLAGS, XKB_STATE_CONTROLS, XKB_STATE_LAYOUT_DEPRESSED,
    XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_LAYOUT_LATCHED, XKB_STATE_LAYOUT_LOCKED, XKB_STATE_LEDS,
    XKB_STATE_MODS_DEPRESSED, XKB_STATE_MODS_EFFECTIVE, XKB_STATE_MODS_LATCHED,
    XKB_STATE_MODS_LOCKED,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use self::FILE_h::FILE;
use crate::xkb::utils::{cstr_dup, darray_growalloc};
use crate::xkb::utils::cstr_len;
pub type events_consume_flags = u32;
pub const UNTIL_KEY_EVENT: events_consume_flags = 1;
pub const ALL_EVENTS: events_consume_flags = 0;
pub const MAX_FILE_SIZE: C2Rust_Unnamed_14 = 1048576;
pub type C2Rust_Unnamed_14 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut darray_string,
}
pub unsafe fn test_init() {
    unsafe {
        setvbuf(
            stdout,
            ::core::ptr::null_mut::<i8>(),
            _IONBF,
            BUFSIZ as usize,
        );
        setlocale(LC_ALL, b"\0".as_ptr() as *const i8);
    }
}
pub unsafe fn print_detailed_state(mut state: *mut xkb_state) {
    unsafe {
        eprintln!(
            "  Layout: base: {}, latched: {}, locked: {}, effective: {}",
            xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_DEPRESSED),
            xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LATCHED),
            xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_LOCKED),
            xkb_state_serialize_layout(state, XKB_STATE_LAYOUT_EFFECTIVE),
        );
        eprintln!(
            "  Modifiers: base: {:#x}, latched: {:#x}, locked: {:#x}, effective: {:#x}",
            xkb_state_serialize_mods(state, XKB_STATE_MODS_DEPRESSED),
            xkb_state_serialize_mods(state, XKB_STATE_MODS_LATCHED),
            xkb_state_serialize_mods(state, XKB_STATE_MODS_LOCKED),
            xkb_state_serialize_mods(state, XKB_STATE_MODS_EFFECTIVE),
        );
        let mut keymap: *mut xkb_keymap = xkb_state_get_keymap(state);
        let mut leds: xkb_led_mask_t = 0 as xkb_led_mask_t;
        let mut led: xkb_led_index_t = 0 as xkb_led_index_t;
        while led < xkb_keymap_num_leds(keymap) {
            if xkb_state_led_index_is_active(state, led) > 0 as i32 {
                leds = (leds as u32 | (1 as u32) << led) as xkb_led_mask_t;
            }
            led = led.wrapping_add(1);
        }
        eprintln!("  LEDs: 0x{:x}", leds);
    }
}
unsafe fn consume_events(
    mut sm: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut state: *mut xkb_state,
    mut flags: events_consume_flags,
    mut kc: *mut xkb_keycode_t,
) -> bool {
    unsafe {
        let mut event: *const xkb_event = ::core::ptr::null::<xkb_event>();
        loop {
            event = xkb_events_next(events);
            if event.is_null() {
                break;
            }
            match xkb_event_get_type(event) as u32 {
                1 | 2 | 3 => {
                    *kc = xkb_event_get_keycode(event);
                    if flags as u32 & UNTIL_KEY_EVENT as i32 as u32 != 0 {
                        return true_0 != 0;
                    }
                }
                4 => {
                    xkb_state_update_event(state, event);
                }
                _ => {}
            }
        }
        return true_0 != 0;
    }
}
pub unsafe fn test_key_seq_va(
    mut keymap: *mut xkb_keymap,
    mut sm: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut ap: ::core::ffi::VaList,
) -> i32 {
    unsafe {
        let mut c2rust_current_block: u64;
        if sm.is_null() as i32 ^ events.is_null() as i32 == 0 {
        } else {
            __assert_fail(
                b"!(!sm ^ !events)\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                137 as u32,
                b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                    .as_ptr() as *const i8,
            );
        };
        eprintln!("----");
        let state: *mut xkb_state = xkb_state_new(keymap) as *mut xkb_state;
        if !state.is_null() {
        } else {
            __assert_fail(
                b"state\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                142 as u32,
                b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                    .as_ptr() as *const i8,
            );
        };
        let mut count: u32 = 0 as u32;
        let mut ksbuf: [i8; 31] = [0; 31];
        's_21: loop {
            let kc: xkb_keycode_t = (ap.arg::<i32>() + EVDEV_OFFSET) as xkb_keycode_t;
            let op: i32 = ap.arg::<i32>();
            let mut opstr: *const i8 = ::core::ptr::null::<i8>();
            match op {
                0 => {
                    opstr = b"DOWN\0".as_ptr() as *const i8;
                }
                1 => {
                    opstr = b"REPEAT\0".as_ptr() as *const i8;
                }
                2 => {
                    opstr = b"UP\0".as_ptr() as *const i8;
                }
                3 => {
                    opstr = b"BOTH\0".as_ptr() as *const i8;
                }
                4 => {
                    opstr = b"NEXT\0".as_ptr() as *const i8;
                }
                5 => {
                    opstr = b"FINISH\0".as_ptr() as *const i8;
                }
                _ => {
                    eprintln!("ERROR: Unsupported operation: {}", op);
                    c2rust_current_block = 10334003491957544446;
                    break;
                }
            }
            let mut kc_new: xkb_keycode_t = kc;
            if !events.is_null() {
                if op == DOWN as i32 || op == REPEAT as i32 || op == BOTH as i32 {
                    if xkb_machine_process_key(
                        sm,
                        kc,
                        (if op == REPEAT as i32 {
                            XKB_KEY_REPEATED as i32
                        } else {
                            XKB_KEY_DOWN as i32
                        }) as xkb_key_direction,
                        events,
                    ) as u64
                        == 0
                    {
                    } else {
                        __assert_fail(
                            b"!xkb_machine_process_key( sm, kc, (op == REPEAT ? XKB_KEY_REPEATED : XKB_KEY_DOWN), events )\0"
                                .as_ptr() as *const i8,
                            b"../test/common.c\0".as_ptr() as *const i8,
                            174 as u32,
                            b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                                .as_ptr() as *const i8,
                        );
                    };
                    if consume_events(sm, events, state, UNTIL_KEY_EVENT, &raw mut kc_new) as i32
                        != 0
                    {
                    } else {
                        __assert_fail(
                            b"consume_events(sm, events, state, UNTIL_KEY_EVENT, &kc_new)\0"
                                .as_ptr() as *const i8,
                            b"../test/common.c\0".as_ptr() as *const i8,
                            175 as u32,
                            b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                                .as_ptr() as *const i8,
                        );
                    };
                }
                if op == UP as i32 || op == BOTH as i32 {
                    if op == BOTH as i32 {
                        if consume_events(sm, events, state, ALL_EVENTS, &raw mut kc_new) as i32
                            != 0
                        {
                        } else {
                            __assert_fail(
                                b"consume_events(sm, events, state, ALL_EVENTS, &kc_new)\0"
                                    .as_ptr() as *const i8,
                                b"../test/common.c\0".as_ptr()
                                    as *const i8,
                                180 as u32,
                                b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                                    .as_ptr() as *const i8,
                            );
                        };
                    }
                    if xkb_machine_process_key(sm, kc, XKB_KEY_UP, events) as i32 == 0 as i32 {
                    } else {
                        __assert_fail(
                            b"xkb_machine_process_key(sm, kc, XKB_KEY_UP, events) == 0\0"
                                .as_ptr() as *const i8,
                            b"../test/common.c\0".as_ptr() as *const i8,
                            183 as u32,
                            b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                                .as_ptr() as *const i8,
                        );
                    };
                    if consume_events(sm, events, state, UNTIL_KEY_EVENT, &raw mut kc_new) as i32
                        != 0
                    {
                    } else {
                        __assert_fail(
                            b"consume_events(sm, events, state, UNTIL_KEY_EVENT, &kc_new)\0"
                                .as_ptr() as *const i8,
                            b"../test/common.c\0".as_ptr() as *const i8,
                            184 as u32,
                            b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                                .as_ptr() as *const i8,
                        );
                    };
                }
            }
            let mut syms: *const xkb_keysym_t = ::core::ptr::null::<xkb_keysym_t>();
            let nsyms: i32 = xkb_state_key_get_syms(state, kc_new, &raw mut syms) as i32;
            if !events.is_null() {
                if consume_events(sm, events, state, ALL_EVENTS, &raw mut kc_new) as i32 != 0 {
                } else {
                    __assert_fail(
                        b"consume_events(sm, events, state, ALL_EVENTS, &kc_new)\0"
                            .as_ptr() as *const i8,
                        b"../test/common.c\0".as_ptr() as *const i8,
                        193 as u32,
                        b"int test_key_seq_va(struct xkb_keymap *, struct xkb_machine *, struct xkb_events *, struct __va_list_tag *)\0"
                            .as_ptr() as *const i8,
                    );
                };
            } else {
                if op == DOWN as i32 || op == REPEAT as i32 || op == BOTH as i32 {
                    xkb_state_update_key(
                        state,
                        kc,
                        (if op == REPEAT as i32 {
                            XKB_KEY_REPEATED as i32
                        } else {
                            XKB_KEY_DOWN as i32
                        }) as xkb_key_direction,
                    );
                }
                if op == UP as i32 || op == BOTH as i32 {
                    xkb_state_update_key(state, kc, XKB_KEY_UP);
                }
            }
            let direction: xkb_key_direction = (if op == DOWN as i32 {
                XKB_KEY_DOWN as i32
            } else if op == REPEAT as i32 {
                XKB_KEY_REPEATED as i32
            } else {
                XKB_KEY_UP as i32
            }) as xkb_key_direction;
            tools_print_keycode_state(
                b"\0".as_ptr() as *const i8,
                state,
                ::core::ptr::null_mut::<xkb_compose_state>(),
                kc,
                direction,
                XKB_CONSUMED_MODE_XKB,
                (PRINT_ALL_FIELDS as i32 | PRINT_UNILINE as i32) as print_state_options,
            );
            count = count.wrapping_add(1);
            eprint!(
                "#{:02} op {:<6} got {} syms for keycode {:3}",
                count,
                crate::xkb::utils::CStrDisplay(opstr),
                nsyms,
                kc,
            );
            if kc_new != kc {
                eprint!(" (redirected to {:3})", kc_new);
            }
            eprint!(": [");
            let mut keysym: xkb_keysym_t = 0;
            let mut i: i32 = 0 as i32;
            while i < nsyms {
                keysym = ap.arg::<i32>() as xkb_keysym_t;
                xkb_keysym_get_name(
                    *syms.offset(i as isize),
                    &raw mut ksbuf as *mut i8,
                    ::core::mem::size_of::<[i8; 31]>() as usize,
                );
                eprint!(
                    "{}{}",
                    crate::xkb::utils::CStrDisplay(if i != 0 as i32 {
                        b", \0".as_ptr() as *const i8
                    } else {
                        b"\0".as_ptr() as *const i8
                    }),
                    crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8),
                );
                if keysym == FINISH as i32 as xkb_keysym_t || keysym == NEXT as i32 as xkb_keysym_t
                {
                    xkb_keysym_get_name(
                        *syms.offset(i as isize),
                        &raw mut ksbuf as *mut i8,
                        ::core::mem::size_of::<[i8; 31]>() as usize,
                    );
                    eprintln!(
                        "\nERROR: Did not expect keysym: {}.",
                        crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8)
                    );
                    c2rust_current_block = 10334003491957544446;
                    break 's_21;
                } else if keysym != *syms.offset(i as isize) {
                    xkb_keysym_get_name(
                        keysym,
                        &raw mut ksbuf as *mut i8,
                        ::core::mem::size_of::<[i8; 31]>() as usize,
                    );
                    eprint!(
                        "\nERROR: Expected keysym: {}. ",
                        crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8)
                    );
                    xkb_keysym_get_name(
                        *syms.offset(i as isize),
                        &raw mut ksbuf as *mut i8,
                        ::core::mem::size_of::<[i8; 31]>() as usize,
                    );
                    eprintln!(
                        " Got keysym: {}.",
                        crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8)
                    );
                    c2rust_current_block = 10334003491957544446;
                    break 's_21;
                } else {
                    i += 1;
                }
            }
            if nsyms == 0 as i32 {
                keysym = ap.arg::<i32>() as xkb_keysym_t;
                if keysym != XKB_KEY_NoSymbol as xkb_keysym_t {
                    xkb_keysym_get_name(
                        keysym,
                        &raw mut ksbuf as *mut i8,
                        ::core::mem::size_of::<[i8; 31]>() as usize,
                    );
                    eprintln!(
                        "\nERROR: Expected {}, but got no keysyms.",
                        crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8)
                    );
                    c2rust_current_block = 10334003491957544446;
                    break;
                }
            }
            eprintln!("]");
            keysym = ap.arg::<i32>() as xkb_keysym_t;
            if keysym == NEXT as i32 as xkb_keysym_t {
                continue;
            }
            if keysym == FINISH as i32 as xkb_keysym_t {
                c2rust_current_block = 17836213544692497527;
                break;
            }
            xkb_keysym_get_name(
                keysym,
                &raw mut ksbuf as *mut i8,
                ::core::mem::size_of::<[i8; 31]>() as usize,
            );
            eprintln!(
                "\nERROR: Expected keysym: {}. Didn't get it.",
                crate::xkb::utils::CStrDisplay(&raw mut ksbuf as *mut i8)
            );
            c2rust_current_block = 10334003491957544446;
            break;
        }
        match c2rust_current_block {
            10334003491957544446 => {
                eprintln!("Current state:");
                print_detailed_state(state);
                xkb_state_unref(state);
                return 0 as i32;
            }
            _ => {
                xkb_state_unref(state);
                return 1 as i32;
            }
        };
    }
}
pub unsafe extern "C" fn test_key_seq(mut keymap: *mut xkb_keymap, mut c2rust_args: ...) -> i32 {
    unsafe {
        let mut ap: ::core::ffi::VaList;
        let mut ret: i32 = 0;
        ap = c2rust_args.clone();
        ret = test_key_seq_va(
            keymap,
            ::core::ptr::null_mut::<xkb_machine>(),
            ::core::ptr::null_mut::<xkb_events>(),
            ap,
        );
        return ret;
    }
}
pub unsafe extern "C" fn test_key_seq2(
    mut keymap: *mut xkb_keymap,
    mut sm: *mut xkb_machine,
    mut events: *mut xkb_events,
    mut c2rust_args: ...
) -> i32 {
    unsafe {
        let mut ap: ::core::ffi::VaList;
        let mut ret: i32 = 0;
        ap = c2rust_args.clone();
        ret = test_key_seq_va(keymap, sm, events, ap);
        return ret;
    }
}
pub unsafe fn test_makedir(mut parent: *const i8, mut path: *const i8) -> *mut i8 {
    unsafe {
        let mut err: i32 = 0;
        let dirname_s = format!(
            "{}/{}",
            crate::xkb::utils::CStrDisplay(parent),
            crate::xkb::utils::CStrDisplay(path)
        );
        let dirname: *mut i8 = std::ffi::CString::new(dirname_s).unwrap().into_raw();
        err = mkdir(dirname, 0o777 as __mode_t);
        if err == 0 as i32 {
        } else {
            __assert_fail(
                b"err == 0\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                317 as u32,
                b"char *test_makedir(const char *, const char *)\0".as_ptr() as *const i8,
            );
        };
        return dirname;
    }
}
pub unsafe fn test_maketempdir(mut template: *const i8) -> *mut i8 {
    unsafe {
        let tmpdir_s = format!("/tmp/{}", crate::xkb::utils::CStrDisplay(template));
        let mut tmpdir: *mut i8 = std::ffi::CString::new(tmpdir_s).unwrap().into_raw();
        let mut tmp: *mut i8 = mkdtemp(tmpdir);
        if tmp == tmpdir {
        } else {
            __assert_fail(
                b"tmp == tmpdir\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                345 as u32,
                b"char *test_maketempdir(const char *)\0".as_ptr() as *const i8,
            );
        };
        return tmpdir;
    }
}
pub unsafe fn test_get_path(mut path_rel: *const i8) -> *mut i8 {
    unsafe {
        let mut srcdir: *const i8 = ::core::ptr::null::<i8>();
        srcdir = getenv(b"top_srcdir\0".as_ptr() as *const i8);
        if srcdir.is_null() {
            srcdir = b".\0".as_ptr() as *const i8;
        }
        if is_absolute_path(path_rel) {
            return cstr_dup(path_rel);
        }
        let sep = if *path_rel.offset(0 as i32 as isize) as i32 != 0 {
            "/"
        } else {
            ""
        };
        let path_s = format!(
            "{}/test/data{}{}",
            crate::xkb::utils::CStrDisplay(srcdir),
            sep,
            crate::xkb::utils::CStrDisplay(path_rel)
        );
        std::ffi::CString::new(path_s).unwrap().into_raw()
    }
}
pub unsafe fn read_file(mut path: *const i8, mut file: *mut FILE) -> *mut i8 {
    unsafe {
        if file.is_null() {
            return ::core::ptr::null_mut::<i8>();
        }
        *__errno_location() = 0 as i32;
        let fd: i32 = fileno(file) as i32;
        if fd < 0 as i32 {
            eprintln!(
                "Error getting file descriptor for {}: {}",
                crate::xkb::utils::CStrDisplay(path),
                crate::xkb::utils::StrerrorDisplay(*__errno_location()),
            );
            return ::core::ptr::null_mut::<i8>();
        }
        *__errno_location() = 0 as i32;
        let mut info: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if fstat(fd, &raw mut info) != 0 as i32 {
            eprintln!(
                "Error getting file stats for {}: {}",
                crate::xkb::utils::CStrDisplay(path),
                crate::xkb::utils::StrerrorDisplay(*__errno_location()),
            );
            return ::core::ptr::null_mut::<i8>();
        }
        let size: usize = info.st_size as usize;
        if size > MAX_FILE_SIZE as i32 as usize {
            eprintln!(
                "Error: file {} exceeds maximum size",
                crate::xkb::utils::CStrDisplay(path)
            );
            return ::core::ptr::null_mut::<i8>();
        }
        let mut ret: *mut i8 = malloc(size.wrapping_add(1 as usize)) as *mut i8;
        if ret.is_null() {
            return ::core::ptr::null_mut::<i8>();
        }
        *ret.offset(size as isize) = '\0' as i32 as i8;
        *__errno_location() = 0 as i32;
        let count: usize = fread(
            ret as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<i8>() as usize,
            size,
            file,
        ) as usize;
        if count != size {
            if feof(file) == 0 {
                println!(
                    "Error reading file {}: unexpected end of file",
                    crate::xkb::utils::CStrDisplay(path)
                );
            } else if ferror(file) != 0 {
                eprintln!(
                    "Error reading file {}: {}",
                    crate::xkb::utils::CStrDisplay(path),
                    crate::xkb::utils::StrerrorDisplay(*__errno_location()),
                );
            }
            free(ret as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<i8>();
        }
        return ret;
    }
}
pub unsafe fn test_read_file(mut path_rel: *const i8) -> *mut i8 {
    unsafe {
        let mut path: *mut i8 = test_get_path(path_rel);
        if path.is_null() {
            return ::core::ptr::null_mut::<i8>();
        }
        let mut file: *mut FILE = fopen(path, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        let mut ret: *mut i8 = ::core::ptr::null_mut::<i8>();
        if !file.is_null() {
            ret = read_file(path, file);
        }
        if !file.is_null() {
            fclose(file);
        }
        free(path as *mut ::core::ffi::c_void);
        return ret;
    }
}
pub unsafe fn test_get_context(mut test_flags: test_context_flags) -> *mut xkb_context {
    unsafe {
        let mut ctx_flags: xkb_context_flags = XKB_CONTEXT_NO_DEFAULT_INCLUDES;
        if test_flags as u32 & CONTEXT_ALLOW_ENVIRONMENT_NAMES as i32 as u32 != 0 {
            unsetenv(b"XKB_DEFAULT_RULES\0".as_ptr() as *const i8);
            unsetenv(b"XKB_DEFAULT_MODEL\0".as_ptr() as *const i8);
            unsetenv(b"XKB_DEFAULT_LAYOUT\0".as_ptr() as *const i8);
            unsetenv(b"XKB_DEFAULT_VARIANT\0".as_ptr() as *const i8);
            unsetenv(b"XKB_DEFAULT_OPTIONS\0".as_ptr() as *const i8);
        } else {
            ctx_flags = (ctx_flags as u32 | XKB_CONTEXT_NO_ENVIRONMENT_NAMES as i32 as u32)
                as xkb_context_flags;
        }
        let ctx: *mut xkb_context = xkb_context_new(ctx_flags) as *mut xkb_context;
        if ctx.is_null() {
            return ::core::ptr::null_mut::<xkb_context>();
        }
        let path: *mut i8 = test_get_path(b"\0".as_ptr() as *const i8) as *mut i8;
        if path.is_null() {
            xkb_context_unref(ctx);
            return ::core::ptr::null_mut::<xkb_context>();
        }
        xkb_context_include_path_append(ctx, path);
        free(path as *mut ::core::ffi::c_void);
        return ctx;
    }
}
pub unsafe fn test_compile_file(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut path_rel: *const i8,
) -> *mut xkb_keymap {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut path: *mut i8 = ::core::ptr::null_mut::<i8>();
        path = test_get_path(path_rel);
        if path.is_null() {
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        file = fopen(path, b"rb\0".as_ptr() as *const i8) as *mut FILE;
        if file.is_null() {
            eprintln!(
                "Failed to open path: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            free(path as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if !file.is_null() {
        } else {
            __assert_fail(
                b"file != NULL\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                496 as u32,
                b"struct xkb_keymap *test_compile_file(struct xkb_context *, enum xkb_keymap_format, const char *)\0"
                    .as_ptr() as *const i8,
            );
        };
        keymap = xkb_keymap_new_from_file(context, file, format, XKB_KEYMAP_COMPILE_STRICT_MODE);
        fclose(file);
        if keymap.is_null() {
            eprintln!(
                "Failed to compile path: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            free(path as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        eprintln!(
            "Successfully compiled path: {}",
            crate::xkb::utils::CStrDisplay(path)
        );
        free(path as *mut ::core::ffi::c_void);
        return keymap;
    }
}
pub unsafe fn test_compile_string(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut string: *const i8,
) -> *mut xkb_keymap {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        keymap =
            xkb_keymap_new_from_string(context, string, format, XKB_KEYMAP_COMPILE_STRICT_MODE);
        if keymap.is_null() {
            eprintln!("Failed to compile string");
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
pub unsafe fn test_compile_buffer(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut buf: *const i8,
    mut len: usize,
) -> *mut xkb_keymap {
    unsafe {
        return test_compile_buffer2(context, format, XKB_KEYMAP_COMPILE_STRICT_MODE, buf, len);
    }
}
pub unsafe fn test_compile_buffer2(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
    mut buf: *const i8,
    mut len: usize,
) -> *mut xkb_keymap {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        keymap = xkb_keymap_new_from_buffer(context, buf, len, format, flags);
        if keymap.is_null() {
            eprintln!("Failed to compile keymap from memory buffer");
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
pub unsafe fn test_compile_rules(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut rules: *const i8,
    mut model: *const i8,
    mut layout: *const i8,
    mut variant: *const i8,
    mut options: *const i8,
) -> *mut xkb_keymap {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: if isempty(rules) as i32 != 0 {
                ::core::ptr::null::<i8>()
            } else {
                rules
            },
            model: if isempty(model) as i32 != 0 {
                ::core::ptr::null::<i8>()
            } else {
                model
            },
            layout: if isempty(layout) as i32 != 0 {
                ::core::ptr::null::<i8>()
            } else {
                layout
            },
            variant: if isempty(variant) as i32 != 0 {
                ::core::ptr::null::<i8>()
            } else {
                variant
            },
            options: if isempty(options) as i32 != 0 {
                ::core::ptr::null::<i8>()
            } else {
                options
            },
        };
        if rules.is_null()
            && model.is_null()
            && layout.is_null()
            && variant.is_null()
            && options.is_null()
        {
            keymap = xkb_keymap_new_from_names2(
                context,
                ::core::ptr::null::<xkb_rule_names>(),
                format,
                XKB_KEYMAP_COMPILE_STRICT_MODE,
            );
        } else {
            keymap = xkb_keymap_new_from_names2(
                context,
                &raw mut rmlvo,
                format,
                XKB_KEYMAP_COMPILE_STRICT_MODE,
            );
        }
        if keymap.is_null() {
            eprintln!(
                "Failed to compile RMLVO: '{}', '{}', '{}', '{}', '{}'",
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(model),
                crate::xkb::utils::CStrDisplay(layout),
                crate::xkb::utils::CStrDisplay(variant),
                crate::xkb::utils::CStrDisplay(options),
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        return keymap;
    }
}
unsafe fn xkb_rules_names_to_rmlvo_builder(
    mut context: *mut xkb_context,
    mut names: *const xkb_rule_names,
) -> *mut xkb_rmlvo_builder {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rmlvo: *mut xkb_rmlvo_builder = xkb_rmlvo_builder_new(
            context,
            (*names).rules,
            (*names).model,
            XKB_RMLVO_BUILDER_NO_FLAGS,
        );
        if rmlvo.is_null() {
            eprintln!("ERROR: xkb_rmlvo_builder_new() failed");
            return ::core::ptr::null_mut::<xkb_rmlvo_builder>();
        }
        let mut buf: [i8; 1024] = [
            0 as i32 as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut loptions: C2Rust_Unnamed_15 = C2Rust_Unnamed_15 {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<darray_string>(),
        };
        if !isempty((*names).options) {
            let mut o: *const i8 = (*names).options;
            loop {
                if !(*o as i32 != '\0' as i32) {
                    c2rust_current_block = 11777552016271000781;
                    break;
                }
                let mut option: *const i8 = o;
                while *o as i32 != '\0' as i32
                    && *o as i32 != ',' as i32
                    && *o as i32 != OPTIONS_GROUP_SPECIFIER_PREFIX
                {
                    o = o.offset(1);
                }
                let len: usize = o.offset_from(option) as i64 as usize;
                if len >= ::core::mem::size_of::<[i8; 1024]>() as usize {
                    c2rust_current_block = 4427821232739340156;
                    break;
                }
                std::ptr::copy_nonoverlapping(option as *const u8, &raw mut buf as *mut u8, len);
                buf[len as usize] = '\0' as i32 as i8;
                let mut layout: xkb_layout_index_t = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                if *o as i32 == OPTIONS_GROUP_SPECIFIER_PREFIX {
                    o = o.offset(1);
                    let mut count: i32 =
                        parse_dec_to_uint32_t(o, SIZE_MAX as usize, &raw mut layout);
                    if count > 0 as i32
                        && layout > 0 as xkb_layout_index_t
                        && layout <= XKB_MAX_GROUPS as xkb_layout_index_t
                    {
                        o = o.offset(count as isize);
                        layout = layout.wrapping_sub(1);
                    } else {
                        layout = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                    }
                    let layout_index_end: *const i8 = o;
                    while *o as i32 != '\0' as i32 && *o as i32 != ',' as i32 {
                        o = o.offset(1);
                    }
                    if count <= 0 as i32 || layout_index_end != o {
                        layout = XKB_LAYOUT_INVALID as xkb_layout_index_t;
                    }
                }
                if layout != XKB_LAYOUT_INVALID as xkb_layout_index_t {
                    let mut opt: *mut i8 = cstr_dup(&raw mut buf as *mut i8);
                    if opt.is_null() {
                        c2rust_current_block = 4427821232739340156;
                        break;
                    }
                    let mut __oldSize: darray_size_t = loptions.size;
                    let mut __newSize: darray_size_t =
                        (layout as darray_size_t).wrapping_add(1 as darray_size_t);
                    loptions.size = __newSize;
                    if __newSize > __oldSize {
                        darray_growalloc(&mut loptions.item, &mut loptions.alloc, __newSize);
                        std::ptr::write_bytes(
                            loptions.item.offset(__oldSize as isize) as *mut darray_string
                                as *mut u8,
                            0u8,
                            (__newSize.wrapping_sub(__oldSize) as usize)
                                .wrapping_mul(::core::mem::size_of::<darray_string>() as usize),
                        );
                    }
                    let ref mut c2rust_fresh0 = (*loptions.item.offset(layout as isize)).size;
                    *c2rust_fresh0 = (*loptions.item.offset(layout as isize))
                        .size
                        .wrapping_add(1 as darray_size_t);
                    let mut __need_0: darray_size_t = *c2rust_fresh0;
                    if __need_0 > (*loptions.item.offset(layout as isize)).alloc {
                        let ref mut c2rust_fresh1 = (*loptions.item.offset(layout as isize)).alloc;
                        *c2rust_fresh1 = darray_next_alloc(
                            (*loptions.item.offset(layout as isize)).alloc,
                            __need_0,
                            ::core::mem::size_of::<*mut i8>() as usize,
                        );
                        let ref mut c2rust_fresh2 = (*loptions.item.offset(layout as isize)).item;
                        *c2rust_fresh2 = realloc(
                            (*loptions.item.offset(layout as isize)).item
                                as *mut ::core::ffi::c_void,
                            (*c2rust_fresh1 as usize)
                                .wrapping_mul(::core::mem::size_of::<*mut i8>() as usize),
                        ) as *mut *mut i8;
                    }
                    let ref mut c2rust_fresh3 =
                        *(*loptions.item.offset(layout as isize)).item.offset(
                            (*loptions.item.offset(layout as isize))
                                .size
                                .wrapping_sub(1 as darray_size_t)
                                as isize,
                        );
                    *c2rust_fresh3 = opt;
                } else if !xkb_rmlvo_builder_append_option(rmlvo, &raw mut buf as *mut i8) {
                    c2rust_current_block = 4427821232739340156;
                    break;
                }
                if *o as i32 == ',' as i32 {
                    o = o.offset(1);
                }
            }
        } else {
            c2rust_current_block = 11777552016271000781;
        }
        match c2rust_current_block {
            11777552016271000781 => {
                if !isempty((*names).layout) {
                    let mut l: *const i8 = (*names).layout;
                    let mut v: *const i8 = (*names).variant;
                    if (*names).variant.is_null() {
                        v = b"\0".as_ptr() as *const i8;
                    }
                    let mut layout_count: xkb_layout_index_t = 0 as xkb_layout_index_t;
                    loop {
                        if !(*l as i32 != '\0' as i32) {
                            c2rust_current_block = 14498840325876589332;
                            break;
                        }
                        let mut layout_0: *const i8 = l;
                        let mut variant: *const i8 = v;
                        let mut start: *mut i8 = &raw mut buf as *mut i8;
                        let mut buf_size: usize = ::core::mem::size_of::<[i8; 1024]>() as usize;
                        while *l as i32 != '\0' as i32 && *l as i32 != ',' as i32 {
                            l = l.offset(1);
                        }
                        while *v as i32 != '\0' as i32 && *v as i32 != ',' as i32 {
                            v = v.offset(1);
                        }
                        let mut len_0: usize = l.offset_from(layout_0) as i64 as usize;
                        if len_0 >= buf_size {
                            c2rust_current_block = 4427821232739340156;
                            break;
                        }
                        std::ptr::copy_nonoverlapping(
                            layout_0 as *const u8,
                            start as *mut u8,
                            len_0,
                        );
                        *start.offset(len_0 as isize) = '\0' as i32 as i8;
                        start = start.offset(len_0 as isize);
                        buf_size = buf_size.wrapping_sub(len_0.wrapping_add(1 as usize));
                        len_0 = v.offset_from(variant) as i64 as usize;
                        if len_0 >= buf_size {
                            c2rust_current_block = 4427821232739340156;
                            break;
                        }
                        start = start.offset(1);
                        std::ptr::copy_nonoverlapping(
                            variant as *const u8,
                            start as *mut u8,
                            len_0,
                        );
                        *start.offset(len_0 as isize) = '\0' as i32 as i8;
                        let mut opts: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
                        let mut opts_count: usize = 0 as usize;
                        if layout_count < loptions.size as xkb_layout_index_t {
                            opts = (*loptions.item.offset(layout_count as isize)).item;
                            opts_count =
                                (*loptions.item.offset(layout_count as isize)).size as usize;
                        }
                        if !xkb_rmlvo_builder_append_layout(
                            rmlvo,
                            &raw mut buf as *mut i8,
                            start,
                            opts as *mut *const i8,
                            opts_count,
                        ) {
                            c2rust_current_block = 4427821232739340156;
                            break;
                        }
                        if *l as i32 == ',' as i32 {
                            l = l.offset(1);
                        }
                        if *v as i32 == ',' as i32 {
                            v = v.offset(1);
                        }
                        layout_count = layout_count.wrapping_add(1 as xkb_layout_index_t);
                    }
                } else {
                    c2rust_current_block = 14498840325876589332;
                }
            }
            _ => {}
        }
        match c2rust_current_block {
            4427821232739340156 => {
                eprintln!(
                    "ERROR: {}",
                    crate::xkb::utils::CStrDisplay(
                        b"xkb_rules_names_to_rmlvo_builder\0".as_ptr() as *const i8
                    ),
                );
                xkb_rmlvo_builder_unref(rmlvo);
                rmlvo = ::core::ptr::null_mut::<xkb_rmlvo_builder>();
            }
            _ => {}
        }
        let mut opts_0: *mut darray_string = ::core::ptr::null_mut::<darray_string>();
        if !loptions.item.is_null() {
            opts_0 = loptions.item.offset(0 as i32 as isize) as *mut darray_string;
            while opts_0 < loptions.item.offset(loptions.size as isize) as *mut darray_string {
                let mut opt_0: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
                if !(*opts_0).item.is_null() {
                    opt_0 = (*opts_0).item.offset(0 as i32 as isize) as *mut *mut i8;
                    while opt_0 < (*opts_0).item.offset((*opts_0).size as isize) as *mut *mut i8 {
                        free(*opt_0 as *mut ::core::ffi::c_void);
                        opt_0 = opt_0.offset(1);
                    }
                }
                free((*opts_0).item as *mut ::core::ffi::c_void);
                (*opts_0).item = ::core::ptr::null_mut::<*mut i8>();
                (*opts_0).size = 0 as darray_size_t;
                (*opts_0).alloc = 0 as darray_size_t;
                opts_0 = opts_0.offset(1);
            }
        }
        free(loptions.item as *mut ::core::ffi::c_void);
        loptions.item = ::core::ptr::null_mut::<darray_string>();
        loptions.size = 0 as darray_size_t;
        loptions.alloc = 0 as darray_size_t;
        return rmlvo;
    }
}
pub unsafe fn test_compile_rmlvo(
    mut context: *mut xkb_context,
    mut format: xkb_keymap_format,
    mut rules: *const i8,
    mut model: *const i8,
    mut layout: *const i8,
    mut variant: *const i8,
    mut options: *const i8,
) -> *mut xkb_keymap {
    unsafe {
        let mut keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
        let names: xkb_rule_names = xkb_rule_names {
            rules: rules,
            model: model,
            layout: layout,
            variant: variant,
            options: options,
        };
        let mut rmlvo: *mut xkb_rmlvo_builder =
            xkb_rules_names_to_rmlvo_builder(context, &raw const names);
        if rmlvo.is_null() {
            eprintln!(
                "Failed to create RMLVO builder: '{}', '{}', '{}', '{}', '{}'",
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(model),
                crate::xkb::utils::CStrDisplay(layout),
                crate::xkb::utils::CStrDisplay(variant),
                crate::xkb::utils::CStrDisplay(options),
            );
            return ::core::ptr::null_mut::<xkb_keymap>();
        }
        if xkb_keymap_new_from_rmlvo(rmlvo, format, 4294967295 as xkb_keymap_compile_flags)
            .is_null()
        {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_rmlvo(rmlvo, format, -1)\0".as_ptr()
                    as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                752 as u32,
                b"struct xkb_keymap *test_compile_rmlvo(struct xkb_context *, enum xkb_keymap_format, const char *, const char *, const char *, const char *, const char *)\0"
                    .as_ptr() as *const i8,
            );
        };
        if xkb_keymap_new_from_rmlvo(rmlvo, format, 65535 as xkb_keymap_compile_flags).is_null() {
        } else {
            __assert_fail(
                b"!xkb_keymap_new_from_rmlvo(rmlvo, format, 0xffff)\0".as_ptr()
                    as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                753 as u32,
                b"struct xkb_keymap *test_compile_rmlvo(struct xkb_context *, enum xkb_keymap_format, const char *, const char *, const char *, const char *, const char *)\0"
                    .as_ptr() as *const i8,
            );
        };
        keymap = xkb_keymap_new_from_rmlvo(rmlvo, format, XKB_KEYMAP_COMPILE_STRICT_MODE);
        xkb_rmlvo_builder_unref(rmlvo);
        if keymap.is_null() {
            eprintln!(
                "Failed to compile RMLVO from builder: '{}', '{}', '{}', '{}', '{}'",
                crate::xkb::utils::CStrDisplay(rules),
                crate::xkb::utils::CStrDisplay(model),
                crate::xkb::utils::CStrDisplay(layout),
                crate::xkb::utils::CStrDisplay(variant),
                crate::xkb::utils::CStrDisplay(options),
            );
        }
        return keymap;
    }
}
pub unsafe fn test_compile_output(
    mut ctx: *mut xkb_context,
    mut input_format: xkb_keymap_format,
    mut output_format: xkb_keymap_format,
    mut compile_buffer: test_compile_buffer_t,
    mut compile_buffer_private: *mut ::core::ffi::c_void,
    mut test_title: *const i8,
    mut keymap_str: *const i8,
    mut keymap_len: usize,
    mut rel_path: *const i8,
    mut update_output_files: bool,
) -> bool {
    unsafe {
        return test_compile_output2(
            ctx,
            input_format,
            output_format,
            (XKB_KEYMAP_SERIALIZE_PRETTY as i32 | XKB_KEYMAP_SERIALIZE_KEEP_UNUSED as i32)
                as xkb_keymap_serialize_flags,
            compile_buffer,
            compile_buffer_private,
            test_title,
            keymap_str,
            keymap_len,
            rel_path,
            update_output_files,
        );
    }
}
pub unsafe fn test_compile_output2(
    mut ctx: *mut xkb_context,
    mut input_format: xkb_keymap_format,
    mut output_format: xkb_keymap_format,
    mut serialize_flags: xkb_keymap_serialize_flags,
    mut compile_buffer: test_compile_buffer_t,
    mut compile_buffer_private: *mut ::core::ffi::c_void,
    mut test_title: *const i8,
    mut keymap_str: *const i8,
    mut keymap_len: usize,
    mut rel_path: *const i8,
    mut update_output_files: bool,
) -> bool {
    unsafe {
        let mut success: i32 = true_0;
        if !test_title.is_null() {
            eprintln!("*** {} ***", crate::xkb::utils::CStrDisplay(test_title));
        }
        let mut keymap: *mut xkb_keymap = compile_buffer.expect("non-null function pointer")(
            ctx,
            input_format,
            keymap_str,
            keymap_len,
            compile_buffer_private,
        );
        if rel_path.is_null() {
            if !keymap.is_null() {
                let mut got: *mut i8 =
                    xkb_keymap_get_as_string2(keymap, output_format, serialize_flags);
                xkb_keymap_unref(keymap);
                if !got.is_null() {
                } else {
                    __assert_fail(
                        b"got\0".as_ptr() as *const i8,
                        b"../test/common.c\0".as_ptr() as *const i8,
                        810 as u32,
                        b"_Bool test_compile_output2(struct xkb_context *, enum xkb_keymap_format, enum xkb_keymap_format, enum xkb_keymap_serialize_flags, test_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                            .as_ptr() as *const i8,
                    );
                };
                eprintln!(
                    "Unexpected keymap compilation success:\n{}",
                    crate::xkb::utils::CStrDisplay(got)
                );
                free(got as *mut ::core::ffi::c_void);
            }
            return keymap.is_null();
        }
        if keymap.is_null() {
            eprintln!("Unexpected keymap compilation failure");
            return false_0 != 0;
        }
        let mut got_0: *mut i8 = xkb_keymap_get_as_string2(keymap, output_format, serialize_flags);
        if got_0.is_null() {
            eprintln!("Unexpected keymap serialization failure");
            return false_0 != 0;
        }
        xkb_keymap_unref(keymap);
        let path: *mut i8 = test_get_path(rel_path) as *mut i8;
        if !path.is_null() {
        } else {
            __assert_fail(
                b"path\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                833 as u32,
                b"_Bool test_compile_output2(struct xkb_context *, enum xkb_keymap_format, enum xkb_keymap_format, enum xkb_keymap_serialize_flags, test_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                    .as_ptr() as *const i8,
            );
        };
        if update_output_files {
            eprintln!(
                "Writing golden test output to: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            let mut file: *mut FILE = fopen(path, b"wb\0".as_ptr() as *const i8) as *mut FILE;
            if !file.is_null() {
            } else {
                __assert_fail(
                    b"file\0".as_ptr() as *const i8,
                    b"../test/common.c\0".as_ptr() as *const i8,
                    838 as u32,
                    b"_Bool test_compile_output2(struct xkb_context *, enum xkb_keymap_format, enum xkb_keymap_format, enum xkb_keymap_serialize_flags, test_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                        .as_ptr() as *const i8,
                );
            };
            fwrite(
                got_0 as *const ::core::ffi::c_void,
                1 as usize,
                cstr_len(got_0),
                file,
            );
            fclose(file);
        } else {
            eprintln!(
                "Reading golden test output: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            let expected: *mut i8 = test_read_file(rel_path) as *mut i8;
            if !expected.is_null() {
            } else {
                __assert_fail(
                    b"expected\0".as_ptr() as *const i8,
                    b"../test/common.c\0".as_ptr() as *const i8,
                    844 as u32,
                    b"_Bool test_compile_output2(struct xkb_context *, enum xkb_keymap_format, enum xkb_keymap_format, enum xkb_keymap_serialize_flags, test_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                        .as_ptr() as *const i8,
                );
            };
            static mut label: [*const i8; 2] = [
                b"Golden test\0".as_ptr() as *const i8,
                b"Roundtrip\0".as_ptr() as *const i8,
            ];
            let mut test_round_trip: bool = output_format as u32 == input_format as u32
                || output_format as u32 == XKB_KEYMAP_USE_ORIGINAL_FORMAT as u32;
            let mut k: u32 = 0 as u32;
            while (k as usize)
                < (::core::mem::size_of::<[*const i8; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
                && success != 0
            {
                if streq(expected, got_0) {
                    eprintln!(
                        "{} succeeded.",
                        crate::xkb::utils::CStrDisplay(label[k as usize])
                    );
                    if !test_round_trip {
                        break;
                    }
                    keymap = compile_buffer.expect("non-null function pointer")(
                        ctx,
                        input_format,
                        expected,
                        cstr_len(expected),
                        compile_buffer_private,
                    );
                    if keymap.is_null() {
                        eprintln!("Unexpected keymap roundtrip compilation failure");
                        success = false_0;
                        break;
                    } else {
                        free(got_0 as *mut ::core::ffi::c_void);
                        got_0 = xkb_keymap_get_as_string2(keymap, output_format, serialize_flags);
                        if got_0.is_null() {
                            eprintln!("Unexpected keymap roundtrip serialization failure");
                            success = false_0;
                        }
                        xkb_keymap_unref(keymap);
                        test_round_trip = false_0 != 0;
                    }
                } else {
                    eprintln!(
                        "{} failed: dumped map differs from expected.",
                        crate::xkb::utils::CStrDisplay(label[k as usize])
                    );
                    eprintln!(
                        "Path to expected file: {}",
                        crate::xkb::utils::CStrDisplay(path)
                    );
                    eprintln!(
                        "Length: expected {}, got: {}",
                        cstr_len(expected),
                        cstr_len(got_0)
                    );
                    eprintln!("Dumped map:");
                    eprintln!("{}", crate::xkb::utils::CStrDisplay(got_0));
                    success = false_0;
                }
                k = k.wrapping_add(1);
            }
            free(expected as *mut ::core::ffi::c_void);
        }
        free(got_0 as *mut ::core::ffi::c_void);
        free(path as *mut ::core::ffi::c_void);
        return success != 0;
    }
}
pub unsafe fn test_third_pary_compile_output(
    mut compile_buffer: test_third_party_compile_buffer_t,
    mut compile_buffer_private: *mut ::core::ffi::c_void,
    mut test_title: *const i8,
    mut keymap_in: *const i8,
    mut keymap_in_size: usize,
    mut rel_path: *const i8,
    mut update_output_files: bool,
) -> bool {
    unsafe {
        let mut success: i32 = true_0;
        eprintln!("*** {} ***", crate::xkb::utils::CStrDisplay(test_title));
        let mut got: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut got_size: usize = 0 as usize;
        let mut ret: i32 = compile_buffer.expect("non-null function pointer")(
            keymap_in,
            keymap_in_size,
            compile_buffer_private,
            &raw mut got,
            &raw mut got_size,
        );
        if rel_path.is_null() {
            if ret == EXIT_SUCCESS {
                eprintln!(
                    "Unexpected keymap compilation success:\nstdout:\n{}",
                    crate::xkb::utils::CStrDisplay(got)
                );
            }
            free(got as *mut ::core::ffi::c_void);
            return ret != EXIT_SUCCESS;
        }
        if ret != EXIT_SUCCESS || isempty(got) as i32 != 0 {
            eprintln!(
                "Unexpected keymap compilation failure.\nstdout:\n{}",
                crate::xkb::utils::CStrDisplay(got)
            );
            free(got as *mut ::core::ffi::c_void);
            return false_0 != 0;
        }
        let mut path: *mut i8 = test_get_path(rel_path);
        if !path.is_null() {
        } else {
            __assert_fail(
                b"path\0".as_ptr() as *const i8,
                b"../test/common.c\0".as_ptr() as *const i8,
                927 as u32,
                b"_Bool test_third_pary_compile_output(test_third_party_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                    .as_ptr() as *const i8,
            );
        };
        if update_output_files {
            eprintln!(
                "Writing golden test output to: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            let mut file: *mut FILE = fopen(path, b"wb\0".as_ptr() as *const i8) as *mut FILE;
            if !file.is_null() {
            } else {
                __assert_fail(
                    b"file\0".as_ptr() as *const i8,
                    b"../test/common.c\0".as_ptr() as *const i8,
                    932 as u32,
                    b"_Bool test_third_pary_compile_output(test_third_party_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                        .as_ptr() as *const i8,
                );
            };
            fwrite(
                got as *const ::core::ffi::c_void,
                1 as usize,
                got_size,
                file,
            );
            fclose(file);
        } else {
            eprintln!(
                "Reading golden test output: {}",
                crate::xkb::utils::CStrDisplay(path)
            );
            let mut expected: *mut i8 = test_read_file(rel_path);
            if !expected.is_null() {
            } else {
                __assert_fail(
                    b"expected\0".as_ptr() as *const i8,
                    b"../test/common.c\0".as_ptr() as *const i8,
                    938 as u32,
                    b"_Bool test_third_pary_compile_output(test_third_party_compile_buffer_t, void *, const char *, const char *, usize, const char *, _Bool)\0"
                        .as_ptr() as *const i8,
                );
            };
            let mut label: [*const i8; 2] = [
                b"Golden test\0".as_ptr() as *const i8,
                b"Roundtrip\0".as_ptr() as *const i8,
            ];
            let mut k: u32 = 0 as u32;
            while (k as usize)
                < (::core::mem::size_of::<[*const i8; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<*const i8>() as usize)
                && success != 0
            {
                if streq(expected, got) {
                    eprintln!(
                        "{} succeeded.",
                        crate::xkb::utils::CStrDisplay(label[k as usize])
                    );
                    if !(k > 0 as u32) {
                        break;
                    }
                } else {
                    eprintln!(
                        "{} failed: dumped map differs from expected.",
                        crate::xkb::utils::CStrDisplay(label[k as usize])
                    );
                    eprintln!(
                        "Path to expected file: {}",
                        crate::xkb::utils::CStrDisplay(path)
                    );
                    eprintln!(
                        "Length: expected {}, got: {}",
                        cstr_len(expected),
                        cstr_len(got)
                    );
                    eprintln!("Dumped map:");
                    eprintln!("{}", crate::xkb::utils::CStrDisplay(got));
                    success = false_0;
                }
                k = k.wrapping_add(1);
            }
            free(expected as *mut ::core::ffi::c_void);
        }
        free(got as *mut ::core::ffi::c_void);
        free(path as *mut ::core::ffi::c_void);
        return success != 0;
    }
}
