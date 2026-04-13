pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: u64,
        pub st_ino: u64,
        pub st_nlink: u64,
        pub st_mode: u32,
        pub st_uid: u32,
        pub st_gid: u32,
        pub __pad0: i32,
        pub st_rdev: u64,
        pub st_size: i64,
        pub st_blksize: i64,
        pub st_blocks: i64,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [i64; 3],
    }
    use super::struct_timespec_h::timespec;
}
pub mod rmlvo_h {
    pub use crate::xkb::rmlvo::rmlvo_h::*;
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

    use crate::xkb::shared_types::xkb_context;
    use crate::xkb::shared_types::xkb_keymap;
    use crate::xkb::shared_types::xkb_keymap_format;
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
    use crate::xkb::shared_types::{xkb_consumed_mode, xkb_key_direction, xkb_keycode_t};
    use crate::xkb::state::xkb_state;
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
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn fstat(__fd: i32, __buf: *mut stat) -> i32;
        pub fn mkdir(__path: *const i8, __mode: u32) -> i32;
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
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: i32 = 0 as i32;
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
pub mod utils_paths_h {
    pub use crate::xkb::utils_paths::is_absolute_path;
}
pub mod rules_h {
    pub const OPTIONS_GROUP_SPECIFIER_PREFIX: i32 = '!' as i32;
}
pub mod locale_h {
    pub const __LC_ALL: i32 = 6 as i32;
}

use self::assert_h::__assert_fail;
use crate::xkb::utils::__errno_location;
pub use self::include_locale_h::{setlocale, LC_ALL};
pub use self::locale_h::__LC_ALL;
pub use self::rmlvo_h::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_layouts,
    xkb_rmlvo_builder_option, xkb_rmlvo_builder_options,
};
pub use self::rules_h::OPTIONS_GROUP_SPECIFIER_PREFIX;
use self::stat_h::{fstat, mkdir};
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
pub use crate::xkb::utils::{isempty, streq};
pub use self::utils_numbers_h::parse_dec_to_uint32_t;
use self::utils_paths_h::is_absolute_path;
use self::xkbcommon_compose_h::xkb_compose_state;
pub use crate::xkb::shared_types::{
    xkb_error_code, XKB_ERROR_ABI_BACKWARD_COMPAT, XKB_ERROR_ABI_FORWARD_COMPAT,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE, XKB_ERROR_INVALID, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK, XKB_SUCCESS,
};
pub use self::xkbcommon_keysyms_h::XKB_KEY_NoSymbol;
pub use crate::xkb::shared_types::*;
pub use crate::xkb::shared_types::{darray_size_t, darray_string};
pub use crate::xkb::shared_types::{
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
// Functions from xkb modules (previously extern "C" in xkbcommon_h block)
use crate::xkb::context::{xkb_context_include_path_append, xkb_context_new, xkb_context_unref};
use crate::xkb::keymap::{
    xkb_keymap_get_as_string2, xkb_keymap_new_from_buffer, xkb_keymap_new_from_file,
    xkb_keymap_new_from_names2, xkb_keymap_new_from_rmlvo, xkb_keymap_new_from_string,
    xkb_keymap_num_leds, xkb_keymap_unref,
};
use crate::xkb::keysym::xkb_keysym_get_name;
use crate::xkb::rmlvo::{
    xkb_rmlvo_builder_append_layout, xkb_rmlvo_builder_append_option, xkb_rmlvo_builder_new,
    xkb_rmlvo_builder_unref,
};
use crate::xkb::state::state_priv_h::xkb_event;
use crate::xkb::state::{
    xkb_event_get_keycode, xkb_event_get_type, xkb_events, xkb_events_next, xkb_machine,
    xkb_machine_process_key, xkb_state, xkb_state_get_keymap, xkb_state_key_get_syms,
    xkb_state_led_index_is_active, xkb_state_new, xkb_state_serialize_layout,
    xkb_state_serialize_mods, xkb_state_unref, xkb_state_update_event, xkb_state_update_key,
};

use crate::xkb::utils::cstr_len;
use crate::xkb::utils::{cstr_dup, darray_append, darray_free, darray_resize_zero};
use libc::{
    fclose, feof, ferror, fileno, fopen, fread, free, fwrite, getenv, malloc, mkdtemp, setvbuf,
    unsetenv, _IONBF, BUFSIZ, EXIT_SUCCESS, FILE,
};
extern "C" {
    pub static stdout: *mut libc::FILE;
}
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
                        return 1 != 0;
                    }
                }
                4 => {
                    xkb_state_update_event(state, event);
                }
                _ => {}
            }
        }
        return 1 != 0;
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
        err = mkdir(dirname, 0o777 as u32);
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
                        parse_dec_to_uint32_t(o, usize::MAX as usize, &raw mut layout);
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
                    darray_resize_zero(
                        &mut loptions.item,
                        &mut loptions.size,
                        &mut loptions.alloc,
                        (layout as darray_size_t).wrapping_add(1 as darray_size_t),
                    );
                    darray_append(
                        &mut (*loptions.item.offset(layout as isize)).item,
                        &mut (*loptions.item.offset(layout as isize)).size,
                        &mut (*loptions.item.offset(layout as isize)).alloc,
                        opt,
                    );
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
                darray_free(
                    &mut (*opts_0).item,
                    &mut (*opts_0).size,
                    &mut (*opts_0).alloc,
                );
                opts_0 = opts_0.offset(1);
            }
        }
        darray_free(&mut loptions.item, &mut loptions.size, &mut loptions.alloc);
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
        let mut success: i32 = 1;
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
            return 0 != 0;
        }
        let mut got_0: *mut i8 = xkb_keymap_get_as_string2(keymap, output_format, serialize_flags);
        if got_0.is_null() {
            eprintln!("Unexpected keymap serialization failure");
            return 0 != 0;
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
                        success = 0;
                        break;
                    } else {
                        free(got_0 as *mut ::core::ffi::c_void);
                        got_0 = xkb_keymap_get_as_string2(keymap, output_format, serialize_flags);
                        if got_0.is_null() {
                            eprintln!("Unexpected keymap roundtrip serialization failure");
                            success = 0;
                        }
                        xkb_keymap_unref(keymap);
                        test_round_trip = 0 != 0;
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
                    success = 0;
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
        let mut success: i32 = 1;
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
            return 0 != 0;
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
                    success = 0;
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
