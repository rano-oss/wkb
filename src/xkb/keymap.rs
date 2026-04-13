use crate::xkb::context_priv::{xkb_atom_lookup, xkb_atom_text, xkb_context_sanitize_rule_names};
use crate::xkb_logf;
#[c2rust::header_src = "/usr/include/bits/types.h:17"]
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:17"]
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:17"]
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:19"]
#[c2rust::src_loc = "18:1"]
pub type size_t = usize;
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/keymap.h:22"]

// text_v1_keymap_format_ops is defined in xkbcomp::xkbcomp with a local type.
// Both types are #[repr(C)] with identical layout, so pointer cast is safe.
unsafe fn text_v1_keymap_format_ops_ptr() -> *const xkb_keymap_format_ops {
    &raw const crate::xkb::xkbcomp::xkbcomp::text_v1_keymap_format_ops as *const _
        as *const xkb_keymap_format_ops
}
pub use crate::xkb::keymap_priv::{xkb_keymap_new, XkbModNameToIndex, XkbWrapGroupIntoRange};
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
pub use crate::xkb::rmlvo::{
    xkb_rmlvo_builder, xkb_rmlvo_builder_layout, xkb_rmlvo_builder_option,
};
pub use crate::xkb::shared_types::darray_size_t;
pub use crate::xkb::shared_types::{
    entry_is_active, mod_type, xkb_action, xkb_action_controls, xkb_action_count_t,
    xkb_action_flags, xkb_action_type, xkb_controls_action, xkb_explicit_components, xkb_group,
    xkb_group_action, xkb_internal_action, xkb_internal_action_flags, xkb_key, xkb_key_alias,
    xkb_key_type, xkb_key_type_entry, xkb_keymap, xkb_keymap_format_ops, xkb_keysym_count_t,
    xkb_led, xkb_level, xkb_match_operation, xkb_mod, xkb_mod_action, xkb_mod_set, xkb_mods,
    xkb_overlay_mask_t, xkb_pointer_action, xkb_pointer_button_action, xkb_pointer_default_action,
    xkb_private_action, xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret,
    C2Rust_Unnamed_1, C2Rust_Unnamed_10, C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7,
    C2Rust_Unnamed_8, C2Rust_Unnamed_9, KeycodeMatch, XkbKey, XkbKeyNumLevels,
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
    EXPLICIT_INTERP, EXPLICIT_OVERLAY, EXPLICIT_REPEAT, EXPLICIT_SYMBOLS, EXPLICIT_TYPES,
    EXPLICIT_VMODMAP, INTERNAL_BREAKS_GROUP_LATCH, INTERNAL_BREAKS_MOD_LATCH, MATCH_ALL, MATCH_ANY,
    MATCH_ANY_OR_NONE, MATCH_EXACTLY, MATCH_NONE, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_GROUPS,
};
pub use crate::xkb::shared_types::{
    RMLVO, RMLVO_LAYOUT, RMLVO_MODEL, RMLVO_OPTIONS, RMLVO_RULES, RMLVO_VARIANT,
};
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/rmlvo.h:22"]
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/messages-codes.h:21"]
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/features/enums.h:21"]
#[c2rust::header_src = "/usr/include/stdlib.h:20"]
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:17"]
#[c2rust::header_src = "/usr/include/stdint.h:17"]
#[c2rust::header_src = "/usr/lib/clang/21/include/stdbool.h:22"]
pub use crate::xkb::shared_types::{
    XKB_A11Y_FLAGS_VALUES, XKB_COMPOSE_COMPILE_FLAGS_VALUES, XKB_COMPOSE_FEED_RESULT_VALUES,
    XKB_COMPOSE_FORMAT_VALUES, XKB_COMPOSE_STATE_FLAGS_VALUES, XKB_COMPOSE_STATUS_VALUES,
    XKB_CONSUMED_MODE_VALUES, XKB_CONTEXT_FLAGS_VALUES, XKB_EVENTS_FLAGS_VALUES,
    XKB_EVENT_TYPE_VALUES, XKB_KEYBOARD_CONTROL_FLAGS_VALUES, XKB_KEYMAP_COMPILE_FLAGS_VALUES,
    XKB_KEYMAP_FORMAT_VALUES, XKB_KEYMAP_KEY_ITERATOR_FLAGS_VALUES,
    XKB_KEYMAP_SERIALIZE_FLAGS_VALUES, XKB_KEYSYM_FLAGS_VALUES, XKB_KEY_DIRECTION_VALUES,
    XKB_LAYOUT_OUT_OF_RANGE_POLICY_VALUES, XKB_RMLVO_BUILDER_FLAGS_VALUES,
    XKB_STATE_COMPONENT_VALUES, XKB_STATE_MATCH_VALUES,
};
use crate::xkb::utils::{cstr_free, cstr_len};
use libc::{free, FILE};
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
#[c2rust::src_loc = "26:1"]
pub unsafe fn xkb_keymap_ref(mut keymap: *mut xkb_keymap) -> *mut xkb_keymap {
    unsafe {
        (*keymap).refcnt += 1;
        return keymap;
    }
}
#[c2rust::src_loc = "34:1"]
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
#[c2rust::src_loc = "50:1"]
pub unsafe fn xkb_keymap_unref(mut keymap: *mut xkb_keymap) {
    unsafe {
        if keymap.is_null() || {
            (*keymap).refcnt -= 1;
            (*keymap).refcnt > 0 as i32
        } {
            return;
        }
        if !(*keymap).keys.is_null() {
            let mut key: *mut xkb_key = std::ptr::null_mut();
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
        cstr_free((*keymap).keycodes_section_name);
        cstr_free((*keymap).symbols_section_name);
        cstr_free((*keymap).types_section_name);
        cstr_free((*keymap).compat_section_name);
        xkb_context_unref((*keymap).ctx);
        drop(Box::from_raw(keymap));
    }
}
#[c2rust::src_loc = "99:1"]
unsafe fn get_keymap_format_ops(mut format: xkb_keymap_format) -> *const xkb_keymap_format_ops {
    unsafe {
        static mut keymap_format_ops: [*const xkb_keymap_format_ops; 3] =
            [std::ptr::null(), std::ptr::null(), std::ptr::null()];
        // Initialize lazily to bridge the type difference with xkbcomp's local type
        if keymap_format_ops[1].is_null() {
            let ptr = text_v1_keymap_format_ops_ptr();
            keymap_format_ops[1] = ptr;
            keymap_format_ops[2] = ptr;
        }
        if (format as i32) < 0 as i32
            || format as i32
                >= (std::mem::size_of::<[*const xkb_keymap_format_ops; 3]>())
                    .wrapping_div(std::mem::size_of::<*const xkb_keymap_format_ops>())
                    as i32
        {
            return std::ptr::null();
        }
        return keymap_format_ops[format as usize];
    }
}
#[c2rust::src_loc = "113:1"]
pub unsafe fn xkb_keymap_new_from_rmlvo(
    mut rmlvo: *const xkb_rmlvo_builder,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_rmlvo.is_none() {
            xkb_logf!(
                (*rmlvo).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unsupported keymap format: {}\n",
                crate::xkb::utils::CStrDisplay(b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const i8),
                format as u32,
            );
            return std::ptr::null_mut();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            (*rmlvo).ctx,
            b"xkb_keymap_new_from_rmlvo\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return std::ptr::null_mut();
        }
        if !(*ops)
            .keymap_new_from_rmlvo
            .expect("non-null function pointer")(keymap, rmlvo)
        {
            xkb_keymap_unref(keymap);
            return std::ptr::null_mut();
        }
        return keymap;
    }
}
#[c2rust::src_loc = "138:1"]
pub unsafe fn xkb_keymap_new_from_names2(
    mut ctx: *mut xkb_context,
    mut rmlvo_in: *const xkb_rule_names,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_names.is_none() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unsupported keymap format: {}\n",
                crate::xkb::utils::CStrDisplay(
                    b"xkb_keymap_new_from_names2\0".as_ptr() as *const i8
                ),
                format as u32,
            );
            return std::ptr::null_mut();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_names2\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return std::ptr::null_mut();
        }
        let mut rmlvo: xkb_rule_names = xkb_rule_names {
            rules: std::ptr::null(),
            model: std::ptr::null(),
            layout: std::ptr::null(),
            variant: std::ptr::null(),
            options: std::ptr::null(),
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
            return std::ptr::null_mut();
        }
        return keymap;
    }
}
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
#[c2rust::src_loc = "178:1"]
pub unsafe fn xkb_keymap_new_from_string(
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        return xkb_keymap_new_from_buffer(ctx, string, cstr_len(string), format, flags);
    }
}
#[c2rust::src_loc = "188:1"]
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
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unsupported keymap format: {}\n",
                crate::xkb::utils::CStrDisplay(
                    b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8
                ),
                format as u32,
            );
            return std::ptr::null_mut();
        }
        if buffer.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: no buffer specified\n",
                crate::xkb::utils::CStrDisplay(
                    b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8
                ),
            );
            return std::ptr::null_mut();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_buffer\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return std::ptr::null_mut();
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
            return std::ptr::null_mut();
        }
        return keymap;
    }
}
#[c2rust::src_loc = "223:1"]
pub unsafe fn xkb_keymap_new_from_file(
    mut ctx: *mut xkb_context,
    mut file: *mut FILE,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_compile_flags,
) -> *mut xkb_keymap {
    unsafe {
        let mut ops: *const xkb_keymap_format_ops = get_keymap_format_ops(format);
        if ops.is_null() || (*ops).keymap_new_from_file.is_none() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unsupported keymap format: {}\n",
                crate::xkb::utils::CStrDisplay(b"xkb_keymap_new_from_file\0".as_ptr() as *const i8),
                format as u32,
            );
            return std::ptr::null_mut();
        }
        if file.is_null() {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: no file specified\n",
                crate::xkb::utils::CStrDisplay(b"xkb_keymap_new_from_file\0".as_ptr() as *const i8),
            );
            return std::ptr::null_mut();
        }
        let mut keymap: *mut xkb_keymap = xkb_keymap_new(
            ctx,
            b"xkb_keymap_new_from_file\0".as_ptr() as *const i8,
            format,
            flags,
        );
        if keymap.is_null() {
            return std::ptr::null_mut();
        }
        if !(*ops)
            .keymap_new_from_file
            .expect("non-null function pointer")(keymap, file)
        {
            xkb_keymap_unref(keymap);
            return std::ptr::null_mut();
        }
        return keymap;
    }
}
#[c2rust::src_loc = "254:1"]
pub unsafe fn xkb_keymap_get_as_string2(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
    mut flags: xkb_keymap_serialize_flags,
) -> *mut i8 {
    unsafe {
        static mut XKB_KEYMAP_SERIALIZE_FLAGS: xkb_keymap_serialize_flags =
            XKB_KEYMAP_SERIALIZE_FLAGS_VALUES as i32 as xkb_keymap_serialize_flags;
        if flags as u32 & !(XKB_KEYMAP_SERIALIZE_FLAGS as u32) != 0 {
            xkb_logf!(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unrecognized serialization flags: {:#x}\n",
                crate::xkb::utils::CStrDisplay(b"xkb_keymap_get_as_string2\0".as_ptr() as *const i8),
                flags as u32 & !(XKB_KEYMAP_SERIALIZE_FLAGS as u32),
            );
            return std::ptr::null_mut();
        }
        if format as u32 == XKB_KEYMAP_USE_ORIGINAL_FORMAT as u32 {
            format = (*keymap).format;
        }
        let ops: *const xkb_keymap_format_ops =
            get_keymap_format_ops(format) as *const xkb_keymap_format_ops;
        if ops.is_null() || (*ops).keymap_get_as_string.is_none() {
            xkb_logf!(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unsupported keymap format: {}\n",
                crate::xkb::utils::CStrDisplay(b"xkb_keymap_get_as_string2\0".as_ptr() as *const i8),
                format as u32,
            );
            return std::ptr::null_mut();
        }
        return (*ops)
            .keymap_get_as_string
            .expect("non-null function pointer")(keymap, format, flags);
    }
}
#[c2rust::src_loc = "283:1"]
pub unsafe fn xkb_keymap_get_as_string(
    mut keymap: *mut xkb_keymap,
    mut format: xkb_keymap_format,
) -> *mut i8 {
    unsafe {
        return xkb_keymap_get_as_string2(keymap, format, XKB_KEYMAP_SERIALIZE_NO_FLAGS);
    }
}
#[c2rust::src_loc = "294:1"]
pub unsafe fn xkb_keymap_num_mods(mut keymap: *mut xkb_keymap) -> xkb_mod_index_t {
    unsafe {
        return (*keymap).mods.num_mods;
    }
}
#[c2rust::src_loc = "303:1"]
pub unsafe fn xkb_keymap_mod_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_mod_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).mods.num_mods {
            return std::ptr::null();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).mods.mods[idx as usize].name);
    }
}
#[c2rust::src_loc = "315:1"]
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
#[c2rust::src_loc = "350:1"]
pub unsafe fn xkb_keymap_num_layouts(mut keymap: *mut xkb_keymap) -> xkb_layout_index_t {
    unsafe {
        return (*keymap).num_groups;
    }
}
#[c2rust::src_loc = "359:1"]
pub unsafe fn xkb_keymap_layout_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_layout_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_group_names {
            return std::ptr::null();
        }
        return xkb_atom_text((*keymap).ctx, *(*keymap).group_names.offset(idx as isize));
    }
}
#[c2rust::src_loc = "371:1"]
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
#[c2rust::src_loc = "390:1"]
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
#[c2rust::src_loc = "426:1"]
pub unsafe fn xkb_keymap_num_leds(mut keymap: *mut xkb_keymap) -> xkb_led_index_t {
    unsafe {
        return (*keymap).num_leds;
    }
}
#[c2rust::src_loc = "435:1"]
pub unsafe fn xkb_keymap_led_get_name(
    mut keymap: *mut xkb_keymap,
    mut idx: xkb_led_index_t,
) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_leds {
            return std::ptr::null();
        }
        return xkb_atom_text((*keymap).ctx, (*keymap).leds[idx as usize].name);
    }
}
#[c2rust::src_loc = "447:1"]
pub unsafe fn xkb_keymap_led_get_index(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_led_index_t {
    unsafe {
        let mut atom: xkb_atom_t = xkb_atom_lookup((*keymap).ctx, name);
        let mut i: xkb_led_index_t = 0;
        let mut led: *const xkb_led = std::ptr::null();
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
            let mut empty_mapped: bool = false;
            let mut i: darray_size_t = 0 as darray_size_t;
            while i < (*type_0).num_entries && count < masks_size {
                if entry_is_active((*type_0).entries.offset(i as isize) as *mut xkb_key_type_entry)
                    as i32
                    != 0
                    && (*(*type_0).entries.offset(i as isize)).mods.mask == 0 as xkb_mod_mask_t
                {
                    empty_mapped = true;
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
#[c2rust::src_loc = "525:1"]
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
            return std::ptr::null_mut();
        }
        if level >= XkbKeyNumLevels(key, layout) {
            return std::ptr::null_mut();
        }
        return (*(*key).groups.offset(layout as isize))
            .levels
            .offset(level as isize) as *mut xkb_level;
    }
}
#[c2rust::src_loc = "545:1"]
pub unsafe fn xkb_keymap_key_get_syms_by_level(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
    mut layout: xkb_layout_index_t,
    mut level: xkb_level_index_t,
    mut syms_out: *mut *const xkb_keysym_t,
) -> i32 {
    unsafe {
        let mut leveli: *const xkb_level = std::ptr::null();
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
        *syms_out = std::ptr::null();
        return 0 as i32;
    }
}
#[c2rust::src_loc = "579:1"]
pub unsafe fn xkb_keymap_min_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).min_key_code;
    }
}
#[c2rust::src_loc = "585:1"]
pub unsafe fn xkb_keymap_max_keycode(mut keymap: *mut xkb_keymap) -> xkb_keycode_t {
    unsafe {
        return (*keymap).max_key_code;
    }
}
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
            xkb_logf!(
                (*keymap).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "unrecognized keymap iterator flags: {:#x}\n",
                flags as u32 & !(XKB_KEYMAP_KEY_ITERATOR_FLAGS as u32),
            );
            return std::ptr::null_mut();
        }
        let iter: *mut xkb_keymap_key_iterator =
            Box::into_raw(Box::new(std::mem::zeroed::<xkb_keymap_key_iterator>()));
        (*iter).keymap = xkb_keymap_ref(keymap);
        if (*keymap).num_keys == 0 as xkb_keycode_t {
            (*iter).next = std::ptr::null();
            (*iter).min = std::ptr::null();
            (*iter).max = std::ptr::null();
            return iter;
        }
        (*iter).skip_unbound = flags as u32 & XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND as u32 != 0;
        (*iter).increment = (if flags as u32 & XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER as u32 != 0
        {
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
#[c2rust::src_loc = "649:1"]
pub unsafe fn xkb_keymap_key_iterator_destroy(mut iter: *mut xkb_keymap_key_iterator) {
    unsafe {
        if iter.is_null() {
            return;
        }
        xkb_keymap_unref((*iter).keymap);
        drop(Box::from_raw(iter));
    }
}
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
                (*iter).next = std::ptr::null();
                return XKB_KEYCODE_INVALID as xkb_keycode_t;
            }
        }
        let ret: xkb_keycode_t = (*next).keycode;
        next = next.offset((*iter).increment as i32 as isize);
        (*iter).next = if next < (*iter).min || next > (*iter).max {
            std::ptr::null()
        } else {
            next
        };
        return ret;
    }
}
#[c2rust::src_loc = "686:1"]
pub unsafe fn xkb_keymap_key_for_each(
    mut keymap: *mut xkb_keymap,
    mut iter: xkb_keymap_key_iter_t,
    mut data: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
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
#[c2rust::src_loc = "696:1"]
pub unsafe fn xkb_keymap_key_get_name(
    mut keymap: *mut xkb_keymap,
    mut kc: xkb_keycode_t,
) -> *const i8 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return std::ptr::null();
        }
        return xkb_atom_text((*keymap).ctx, (*key).name);
    }
}
#[c2rust::src_loc = "707:1"]
pub unsafe fn xkb_keymap_key_by_name(
    mut keymap: *mut xkb_keymap,
    mut name: *const i8,
) -> xkb_keycode_t {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
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
use crate::xkb::context::xkb_context_unref;
use crate::xkb::shared_types::*;
