use crate::xkb::context::{
    xkb_atom_intern, xkb_atom_lookup, xkb_atom_text, xkb_context_sanitize_rule_names,
};
#[c2rust::header_src = "/usr/include/bits/types.h:17"]
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:17"]
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:17"]
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:19"]
#[c2rust::header_src = "/home/rano/Public/libxkbcommon/src/keymap.h:22"]
// text_v1_keymap_format_ops is defined in xkbcomp::xkbcomp with a local type.
// Both types are #[repr(C)] with identical layout, so pointer cast is safe.
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
pub use crate::xkb::shared_types::{
    entry_is_active, xkb_action, xkb_action_controls, xkb_action_flags, xkb_action_type,
    xkb_controls_action, xkb_explicit_components, xkb_group, xkb_group_action, xkb_internal_action,
    xkb_internal_action_flags, xkb_key, xkb_key_alias, xkb_key_type, xkb_key_type_entry,
    xkb_keymap, xkb_keymap_format_ops, xkb_keysym_count_t, xkb_led, xkb_level, xkb_mod,
    xkb_mod_action, xkb_mod_set, xkb_mods, xkb_overlay_mask_t, xkb_pointer_action,
    xkb_pointer_button_action, xkb_pointer_default_action, xkb_private_action,
    xkb_redirect_key_action, xkb_switch_screen_action, xkb_sym_interpret, C2Rust_Unnamed_10,
    C2Rust_Unnamed_11, C2Rust_Unnamed_12, C2Rust_Unnamed_2, C2Rust_Unnamed_3, C2Rust_Unnamed_4,
    C2Rust_Unnamed_5, C2Rust_Unnamed_6, C2Rust_Unnamed_7, C2Rust_Unnamed_8, C2Rust_Unnamed_9,
    KeycodeMatch, XkbKey, XkbKeyNumLevels, _ACTION_TYPE_NUM_ENTRIES, ACTION_ABSOLUTE_SWITCH,
    ACTION_ABSOLUTE_X, ACTION_ABSOLUTE_Y, ACTION_ACCEL, ACTION_LATCH_ON_PRESS,
    ACTION_LATCH_TO_LOCK, ACTION_LOCK_CLEAR, ACTION_LOCK_NO_LOCK, ACTION_LOCK_NO_UNLOCK,
    ACTION_LOCK_ON_RELEASE, ACTION_MODS_LOOKUP_MODMAP, ACTION_PENDING_COMPUTATION,
    ACTION_SAME_SCREEN, ACTION_TYPE_CTRL_LOCK, ACTION_TYPE_CTRL_SET, ACTION_TYPE_GROUP_LATCH,
    ACTION_TYPE_GROUP_LOCK, ACTION_TYPE_GROUP_SET, ACTION_TYPE_INTERNAL, ACTION_TYPE_MOD_LATCH,
    ACTION_TYPE_MOD_LOCK, ACTION_TYPE_MOD_SET, ACTION_TYPE_NONE, ACTION_TYPE_PRIVATE,
    ACTION_TYPE_PTR_BUTTON, ACTION_TYPE_PTR_DEFAULT, ACTION_TYPE_PTR_LOCK, ACTION_TYPE_PTR_MOVE,
    ACTION_TYPE_REDIRECT_KEY, ACTION_TYPE_SWITCH_VT, ACTION_TYPE_TERMINATE, ACTION_TYPE_UNKNOWN,
    ACTION_TYPE_UNSUPPORTED_LEGACY, ACTION_TYPE_VOID, ACTION_UNLOCK_ON_PRESS, CONTROL_ALL,
    CONTROL_ALL_BOOLEAN, CONTROL_ALL_BOOLEAN_V1, CONTROL_ALL_V1, CONTROL_AX, CONTROL_AX_FEEDBACK,
    CONTROL_AX_TIMEOUT, CONTROL_BELL, CONTROL_DEBOUNCE, CONTROL_GROUPS_WRAP,
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
use crate::xkb_logf;
use libc::{free, FILE};
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
                (if (*keymap).num_keys_low == 0 as u32 {
                    0 as u32
                } else {
                    (*keymap).min_key_code
                }) as isize,
            );
            while key < (*keymap).keys.offset((*keymap).num_keys as isize) {
                if !(*key).groups.is_null() {
                    let mut i: u32 = 0 as u32;
                    while i < (*key).num_groups {
                        if !(*(*key).groups.offset(i as isize)).levels.is_null() {
                            let mut j: u32 = 0 as u32;
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
                if !(*key).overlays_inline && !(*key).c2rust_unnamed.overlays_keys.is_null() {
                    free((*key).c2rust_unnamed.overlays_keys as *mut ::core::ffi::c_void);
                }
                key = key.offset(1);
            }
            free((*keymap).keys as *mut ::core::ffi::c_void);
        }
        if !(*keymap).types.is_null() {
            let mut i_0: u32 = 0 as u32;
            while i_0 < (*keymap).num_types {
                free((*(*keymap).types.offset(i_0 as isize)).entries as *mut ::core::ffi::c_void);
                free(
                    (*(*keymap).types.offset(i_0 as isize)).level_names as *mut ::core::ffi::c_void,
                );
                i_0 = i_0.wrapping_add(1);
            }
            free((*keymap).types as *mut ::core::ffi::c_void);
        }
        let mut k: u32 = 0 as u32;
        while k < (*keymap).num_sym_interprets {
            // clear_interpret inlined
            let interp = (*keymap).sym_interprets.offset(k as isize) as *mut xkb_sym_interpret;
            (*interp).actions.clear();
            k = k.wrapping_add(1);
        }
        free((*keymap).sym_interprets as *mut ::core::ffi::c_void);
        free((*keymap).c2rust_unnamed.c2rust_unnamed_0.key_aliases as *mut ::core::ffi::c_void);
        free((*keymap).group_names as *mut ::core::ffi::c_void);
        cstr_free((*keymap).keycodes_section_name);
        cstr_free((*keymap).symbols_section_name);
        cstr_free((*keymap).types_section_name);
        cstr_free((*keymap).compat_section_name);
        // Drop the owned ctx field (contains Vecs, HashMap — needs proper cleanup)
        std::ptr::drop_in_place(&raw mut (*keymap).ctx);
        // Free the keymap allocation (was allocated with alloc_zeroed)
        let layout = std::alloc::Layout::new::<xkb_keymap>();
        std::alloc::dealloc(keymap as *mut u8, layout);
    }
}
#[c2rust::src_loc = "99:1"]
unsafe fn get_keymap_format_ops(mut format: u32) -> *const xkb_keymap_format_ops {
    unsafe {
        static mut keymap_format_ops: [*const xkb_keymap_format_ops; 3] =
            [std::ptr::null(), std::ptr::null(), std::ptr::null()];
        // Initialize lazily to bridge the type difference with xkbcomp's local type
        if keymap_format_ops[1].is_null() {
            let ptr = &raw const crate::xkb::xkbcomp::xkbcomp::text_v1_keymap_format_ops as *const _
                as *const xkb_keymap_format_ops;
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
    mut format: u32,
    mut flags: u32,
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
    mut format: u32,
    mut flags: u32,
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
    mut flags: u32,
) -> *mut xkb_keymap {
    unsafe {
        return xkb_keymap_new_from_names2(ctx, rmlvo_in, XKB_KEYMAP_FORMAT_TEXT_V2, flags);
    }
}
#[c2rust::src_loc = "178:1"]
pub unsafe fn xkb_keymap_new_from_string(
    mut ctx: *mut xkb_context,
    mut string: *const i8,
    mut format: u32,
    mut flags: u32,
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
    mut format: u32,
    mut flags: u32,
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
    mut format: u32,
    mut flags: u32,
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
    mut format: u32,
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
pub unsafe fn xkb_keymap_get_as_string(mut keymap: *mut xkb_keymap, mut format: u32) -> *mut i8 {
    unsafe {
        return xkb_keymap_get_as_string2(keymap, format, XKB_KEYMAP_SERIALIZE_NO_FLAGS);
    }
}
#[c2rust::src_loc = "294:1"]
pub unsafe fn xkb_keymap_num_mods(mut keymap: *mut xkb_keymap) -> u32 {
    unsafe {
        return (*keymap).mods.num_mods;
    }
}
#[c2rust::src_loc = "303:1"]
pub unsafe fn xkb_keymap_mod_get_name(mut keymap: *mut xkb_keymap, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= (*keymap).mods.num_mods {
            return std::ptr::null();
        }
        return xkb_atom_text(
            &raw mut (*keymap).ctx,
            (*keymap).mods.mods[idx as usize].name,
        );
    }
}
#[c2rust::src_loc = "315:1"]
pub unsafe fn xkb_keymap_mod_get_index(mut keymap: *mut xkb_keymap, mut name: *const i8) -> u32 {
    unsafe {
        let atom: u32 = xkb_atom_lookup(&raw mut (*keymap).ctx, name) as u32;
        return if atom == XKB_ATOM_NONE as u32 {
            XKB_MOD_INVALID as u32
        } else {
            XkbModNameToIndex(&raw mut (*keymap).mods, atom, MOD_BOTH)
        };
    }
}
#[c2rust::src_loc = "327:1"]
pub unsafe fn xkb_keymap_mod_get_mask(mut keymap: *mut xkb_keymap, mut name: *const i8) -> u32 {
    unsafe {
        let idx: u32 = xkb_keymap_mod_get_index(keymap, name) as u32;
        return if idx >= (*keymap).mods.num_mods {
            0 as u32
        } else {
            (*keymap).mods.mods[idx as usize].mapping
        };
    }
}
#[c2rust::src_loc = "350:1"]
pub unsafe fn xkb_keymap_num_layouts(mut keymap: *mut xkb_keymap) -> u32 {
    unsafe {
        return (*keymap).num_groups;
    }
}
#[c2rust::src_loc = "359:1"]
pub unsafe fn xkb_keymap_layout_get_name(mut keymap: *mut xkb_keymap, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_group_names {
            return std::ptr::null();
        }
        return xkb_atom_text(
            &raw mut (*keymap).ctx,
            *(*keymap).group_names.offset(idx as isize),
        );
    }
}
#[c2rust::src_loc = "371:1"]
pub unsafe fn xkb_keymap_layout_get_index(mut keymap: *mut xkb_keymap, mut name: *const i8) -> u32 {
    unsafe {
        let mut atom: u32 = xkb_atom_lookup(&raw mut (*keymap).ctx, name);
        let mut i: u32 = 0;
        if atom == XKB_ATOM_NONE as u32 {
            return XKB_LAYOUT_INVALID as u32;
        }
        i = 0 as u32;
        while i < (*keymap).num_group_names {
            if *(*keymap).group_names.offset(i as isize) == atom {
                return i;
            }
            i = i.wrapping_add(1);
        }
        return XKB_LAYOUT_INVALID as u32;
    }
}
#[c2rust::src_loc = "390:1"]
pub unsafe fn xkb_keymap_num_layouts_for_key(mut keymap: *mut xkb_keymap, mut kc: u32) -> u32 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as u32;
        }
        return (*key).num_groups;
    }
}
#[c2rust::src_loc = "404:1"]
pub unsafe fn xkb_keymap_num_levels_for_key(
    mut keymap: *mut xkb_keymap,
    mut kc: u32,
    mut layout: u32,
) -> u32 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as u32;
        }
        layout = XkbWrapGroupIntoRange(
            layout as i32,
            (*key).num_groups,
            (*key).out_of_range_group_policy,
            (*key).out_of_range_group_number,
        );
        if layout == XKB_LAYOUT_INVALID as u32 {
            return 0 as u32;
        }
        return XkbKeyNumLevels(key, layout);
    }
}
#[c2rust::src_loc = "426:1"]
pub unsafe fn xkb_keymap_num_leds(mut keymap: *mut xkb_keymap) -> u32 {
    unsafe {
        return (*keymap).num_leds;
    }
}
#[c2rust::src_loc = "435:1"]
pub unsafe fn xkb_keymap_led_get_name(mut keymap: *mut xkb_keymap, mut idx: u32) -> *const i8 {
    unsafe {
        if idx >= (*keymap).num_leds {
            return std::ptr::null();
        }
        return xkb_atom_text(&raw mut (*keymap).ctx, (*keymap).leds[idx as usize].name);
    }
}
#[c2rust::src_loc = "447:1"]
pub unsafe fn xkb_keymap_led_get_index(mut keymap: *mut xkb_keymap, mut name: *const i8) -> u32 {
    unsafe {
        let mut atom: u32 = xkb_atom_lookup(&raw mut (*keymap).ctx, name);
        let mut i: u32 = 0;
        let mut led: *const xkb_led = std::ptr::null();
        if atom == XKB_ATOM_NONE as u32 {
            return XKB_LED_INVALID as u32;
        }
        i = 0 as u32;
        led = &raw mut (*keymap).leds as *mut xkb_led;
        while i < (*keymap).num_leds {
            if (*led).name == atom {
                return i;
            }
            i = i.wrapping_add(1);
            led = led.offset(1);
        }
        return XKB_LED_INVALID as u32;
    }
}
#[c2rust::src_loc = "525:1"]
pub unsafe fn xkb_keymap_key_get_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: u32,
    mut level: u32,
) -> *mut xkb_level {
    unsafe {
        layout = XkbWrapGroupIntoRange(
            layout as i32,
            (*key).num_groups,
            (*key).out_of_range_group_policy,
            (*key).out_of_range_group_number,
        );
        if layout == XKB_LAYOUT_INVALID as u32 {
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
    mut kc: u32,
    mut layout: u32,
    mut level: u32,
    mut syms_out: *mut *const u32,
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
pub unsafe fn xkb_keymap_min_keycode(mut keymap: *mut xkb_keymap) -> u32 {
    unsafe {
        return (*keymap).min_key_code;
    }
}
#[c2rust::src_loc = "585:1"]
pub unsafe fn xkb_keymap_max_keycode(mut keymap: *mut xkb_keymap) -> u32 {
    unsafe {
        return (*keymap).max_key_code;
    }
}
#[c2rust::src_loc = "696:1"]
pub unsafe fn xkb_keymap_key_get_name(mut keymap: *mut xkb_keymap, mut kc: u32) -> *const i8 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return std::ptr::null();
        }
        return xkb_atom_text(&raw mut (*keymap).ctx, (*key).name);
    }
}
#[c2rust::src_loc = "707:1"]
pub unsafe fn xkb_keymap_key_by_name(mut keymap: *mut xkb_keymap, mut name: *const i8) -> u32 {
    unsafe {
        let mut key: *mut xkb_key = std::ptr::null_mut();
        let mut atom: u32 = 0;
        atom = xkb_atom_lookup(&raw mut (*keymap).ctx, name);
        if atom != 0 {
            let mut i: u32 = 0 as u32;
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
            return XKB_KEYCODE_INVALID as u32;
        }
        key = (*keymap).keys.offset(
            (if (*keymap).num_keys_low == 0 as u32 {
                0 as u32
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
        return XKB_KEYCODE_INVALID as u32;
    }
}
#[c2rust::src_loc = "733:1"]
pub unsafe fn xkb_keymap_key_repeats(mut keymap: *mut xkb_keymap, mut kc: u32) -> i32 {
    unsafe {
        let mut key: *const xkb_key = XkbKey(keymap, kc);
        if key.is_null() {
            return 0 as i32;
        }
        return (*key).repeats as i32;
    }
}
use crate::xkb::context::{xkb_context_ref, xkb_context_unref};
use crate::xkb::shared_types::*;

// --- Merged from keymap_priv.rs ---

pub const XKB_MOD_NAME_SHIFT: [i8; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [i8; 6]>(*b"Shift\0") };
pub const XKB_MOD_NAME_CAPS: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Lock\0") };
pub const XKB_MOD_NAME_CTRL: [i8; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [i8; 8]>(*b"Control\0") };
pub const XKB_MOD_NAME_MOD1: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod1\0") };
pub const XKB_MOD_NAME_MOD2: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod2\0") };
pub const XKB_MOD_NAME_MOD3: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod3\0") };
pub const XKB_MOD_NAME_MOD4: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod4\0") };
pub const XKB_MOD_NAME_MOD5: [i8; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [i8; 5]>(*b"Mod5\0") };

unsafe fn update_builtin_keymap_fields(mut keymap: *mut xkb_keymap) {
    unsafe {
        static mut builtin_mods: [*const i8; 8] = [
            XKB_MOD_NAME_SHIFT.as_ptr(),
            XKB_MOD_NAME_CAPS.as_ptr(),
            XKB_MOD_NAME_CTRL.as_ptr(),
            XKB_MOD_NAME_MOD1.as_ptr(),
            XKB_MOD_NAME_MOD2.as_ptr(),
            XKB_MOD_NAME_MOD3.as_ptr(),
            XKB_MOD_NAME_MOD4.as_ptr(),
            XKB_MOD_NAME_MOD5.as_ptr(),
        ];
        let mut i: u32 = 0 as u32;
        while (i as usize)
            < (std::mem::size_of::<[*const i8; 8]>()).wrapping_div(std::mem::size_of::<*const i8>())
        {
            (*keymap).mods.mods[i as usize].name = xkb_atom_intern(
                &raw mut (*keymap).ctx,
                builtin_mods[i as usize],
                cstr_len(builtin_mods[i as usize]),
            );
            (*keymap).mods.mods[i as usize].type_0 = MOD_REAL;
            (*keymap).mods.mods[i as usize].mapping = ((1 as u32) << i) as u32;
            i = i.wrapping_add(1);
        }
        (*keymap).mods.num_mods = (std::mem::size_of::<[*const i8; 8]>())
            .wrapping_div(std::mem::size_of::<*const i8>())
            as u32;
        (*keymap).canonical_state_mask = MOD_REAL_MASK_ALL;
    }
}
pub unsafe fn xkb_keymap_new(
    mut ctx: *mut xkb_context,
    mut func: *const i8,
    mut format: u32,
    mut flags: u32,
) -> *mut xkb_keymap {
    unsafe {
        static mut XKB_KEYMAP_COMPILE_FLAGS: u32 = XKB_KEYMAP_COMPILE_FLAGS_VALUES as i32 as u32;
        if flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32) != 0 {
            xkb_logf!(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "{}: unrecognized keymap compilation flags: 0x{:x}\n",
                crate::xkb::utils::CStrDisplay(func),
                flags as u32 & !(XKB_KEYMAP_COMPILE_FLAGS as u32),
            );
            return std::ptr::null_mut();
        }
        let layout = std::alloc::Layout::new::<xkb_keymap>();
        let keymap: *mut xkb_keymap = std::alloc::alloc_zeroed(layout) as *mut xkb_keymap;
        if keymap.is_null() {
            return std::ptr::null_mut();
        }
        // ctx field must be properly initialized (contains Vec/HashMap, not zero-safe)
        std::ptr::write(&raw mut (*keymap).ctx, (*ctx).clone());
        (*keymap).refcnt = 1 as i32;
        (*keymap).format = format;
        (*keymap).flags = flags;
        update_builtin_keymap_fields(keymap);
        return keymap;
    }
}
pub unsafe fn XkbEscapeMapName(mut name: *mut i8) {
    unsafe {
        static mut legal: [u8; 32] = [
            0, 0, 0, 0, 0, 0xa7, 0xff, 0x83, 0xfe, 0xff, 0xff, 0x87, 0xfe, 0xff, 0xff, 0x7, 0, 0,
            0, 0, 0, 0, 0, 0, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, 0x7f, 0xff,
        ];
        if name.is_null() {
            return;
        }
        while *name != 0 {
            let mut c: ::core::ffi::c_uchar = *name as ::core::ffi::c_uchar;
            if legal[(c as i32 / 8 as i32) as usize] as u32 & (1 as u32) << c as i32 % 8 as i32 == 0
            {
                *name = '_' as i32 as i8;
            }
            name = name.offset(1);
        }
    }
}
pub unsafe fn XkbModNameToIndex(
    mut mods: *const xkb_mod_set,
    mut name: u32,
    mut type_0: u32,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let mut mod_0: *const xkb_mod = std::ptr::null();
        i = 0 as u32;
        mod_0 = &raw const (*mods).mods as *const xkb_mod;
        while i < (*mods).num_mods {
            if (*mod_0).type_0 as u32 & type_0 as u32 != 0 && name == (*mod_0).name {
                return i;
            }
            i = i.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        return XKB_MOD_INVALID as u32;
    }
}
pub unsafe fn XkbLevelsSameSyms(mut a: *const xkb_level, mut b: *const xkb_level) -> bool {
    unsafe {
        if (*a).num_syms as i32 != (*b).num_syms as i32 {
            return false;
        }
        if (*a).num_syms as i32 <= 1 as i32 {
            return (*a).s.sym == (*b).s.sym;
        }
        {
            let n = (std::mem::size_of::<u32>()).wrapping_mul((*a).num_syms as usize);
            return std::slice::from_raw_parts((*a).s.syms as *const u8, n)
                == std::slice::from_raw_parts((*b).s.syms as *const u8, n);
        }
    }
}
pub unsafe fn action_equal(mut a: *const xkb_action, mut b: *const xkb_action) -> bool {
    unsafe {
        if (*a).type_0 as u32 != (*b).type_0 as u32 {
            return false;
        }
        match (*a).type_0 as u32 {
            0 | 1 => return true,
            2 | 3 | 4 => {
                return (*a).mods.flags as u32 == (*b).mods.flags as u32
                    && (*a).mods.mods.mask == (*b).mods.mods.mask
                    && (*a).mods.mods.mods == (*b).mods.mods.mods;
            }
            5 | 6 | 7 => {
                return (*a).group.flags as u32 == (*b).group.flags as u32
                    && (*a).group.group == (*b).group.group;
            }
            8 => {
                return (*a).ptr.flags as u32 == (*b).ptr.flags as u32
                    && (*a).ptr.x as i32 == (*b).ptr.x as i32
                    && (*a).ptr.y as i32 == (*b).ptr.y as i32;
            }
            9 | 10 => {
                return (*a).btn.flags as u32 == (*b).btn.flags as u32
                    && (*a).btn.button as i32 == (*b).btn.button as i32
                    && (*a).btn.count as i32 == (*b).btn.count as i32;
            }
            11 => {
                return (*a).dflt.flags as u32 == (*b).dflt.flags as u32
                    && (*a).dflt.value as i32 == (*b).dflt.value as i32;
            }
            12 => return true,
            13 => {
                return (*a).screen.flags as u32 == (*b).screen.flags as u32
                    && (*a).screen.screen as i32 == (*b).screen.screen as i32;
            }
            14 | 15 => {
                return (*a).ctrls.flags as u32 == (*b).ctrls.flags as u32
                    && (*a).ctrls.ctrls as u32 == (*b).ctrls.ctrls as u32;
            }
            16 => {
                return (*a).redirect.keycode == (*b).redirect.keycode
                    && (*a).redirect.affect == (*b).redirect.affect
                    && (*a).redirect.mods == (*b).redirect.mods;
            }
            17 | 18 => return true,
            20 => {
                return (*a).internal.flags as u32 == (*b).internal.flags as u32
                    && (*a).internal.c2rust_unnamed.clear_latched_mods
                        == (*b).internal.c2rust_unnamed.clear_latched_mods;
            }
            _ => {
                return std::slice::from_raw_parts(
                    &raw const (*a).priv_0.data as *const u8,
                    std::mem::size_of::<[u8; 7]>(),
                ) == std::slice::from_raw_parts(
                    &raw const (*b).priv_0.data as *const u8,
                    std::mem::size_of::<[u8; 7]>(),
                );
            }
        };
    }
}
pub unsafe fn XkbLevelsSameActions(mut a: *const xkb_level, mut b: *const xkb_level) -> bool {
    unsafe {
        if (*a).num_actions as i32 != (*b).num_actions as i32 {
            return false;
        }
        if (*a).num_actions as i32 <= 1 as i32 {
            return action_equal(&raw const (*a).a.action, &raw const (*b).a.action);
        }
        let mut k: u16 = 0 as u16;
        while (k as i32) < (*a).num_actions as i32 {
            if !action_equal(
                (*a).a.actions.offset(k as isize) as *mut xkb_action,
                (*b).a.actions.offset(k as isize) as *mut xkb_action,
            ) {
                return false;
            }
            k = k.wrapping_add(1);
        }
        return true;
    }
}
pub unsafe fn XkbWrapGroupIntoRange(
    mut group: i32,
    mut num_groups: u32,
    mut out_of_range_group_policy: u32,
    mut out_of_range_group_number: u32,
) -> u32 {
    if num_groups == 0 as u32 {
        return XKB_LAYOUT_INVALID as u32;
    }
    if group >= 0 as i32 && (group as u32) < num_groups {
        return group as u32;
    }
    match out_of_range_group_policy as u32 {
        2 => {
            if out_of_range_group_number >= num_groups {
                return 0 as u32;
            }
            return out_of_range_group_number;
        }
        1 => {
            if group < 0 as i32 {
                return 0 as u32;
            } else {
                return num_groups.wrapping_sub(1 as u32);
            }
        }
        0 | _ => {
            let rem: i32 = group % num_groups as i32;
            return (if rem >= 0 as i32 {
                rem
            } else {
                rem + num_groups as i32
            }) as u32;
        }
    };
}
pub unsafe fn xkb_keymap_key_get_actions_by_level(
    mut keymap: *mut xkb_keymap,
    mut key: *const xkb_key,
    mut layout: u32,
    mut level: u32,
    mut actions: *mut *const xkb_action,
) -> u16 {
    unsafe {
        let mut count: u16 = 0;
        let mut c2rust_current_block: u64;
        if !key.is_null() {
            layout = XkbWrapGroupIntoRange(
                layout as i32,
                (*key).num_groups,
                (*key).out_of_range_group_policy,
                (*key).out_of_range_group_number,
            );
            if !(layout == XKB_LAYOUT_INVALID as u32) {
                if !(level >= XkbKeyNumLevels(key, layout)) {
                    count = (*(*(*key).groups.offset(layout as isize))
                        .levels
                        .offset(level as isize))
                    .num_actions;
                    match count as i32 {
                        0 => {}
                        1 => {
                            c2rust_current_block = 945040705843674513;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                        _ => {
                            c2rust_current_block = 15050610111240922756;
                            match c2rust_current_block {
                                15050610111240922756 => {
                                    *actions = (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .actions;
                                }
                                _ => {
                                    *actions = &raw mut (*(*(*key).groups.offset(layout as isize))
                                        .levels
                                        .offset(level as isize))
                                    .a
                                    .action;
                                }
                            }
                            return count;
                        }
                    }
                }
            }
        }
        *actions = std::ptr::null();
        return 0 as u16;
    }
}
