//! xkbcommon-compat — thin `extern "C"` wrapper over xkb-core.
//!
//! Provides a C-compatible API matching libxkbcommon's function signatures.
//! Opaque types are heap-allocated and managed via ref-counting or explicit free.

#![allow(non_camel_case_types)]

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use xkb_core::rust_types::{Context, Keymap, RuleNames, State};

// --- Opaque C types (boxed Rust types) ---

/// Opaque context handle.
pub struct xkb_context(Context);

/// Opaque keymap handle.
pub struct xkb_keymap(Keymap);

/// Opaque state handle.
pub struct xkb_state(State);

// --- Rule names ---

/// C-compatible rule names struct.
#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const c_char,
    pub model: *const c_char,
    pub layout: *const c_char,
    pub variant: *const c_char,
    pub options: *const c_char,
}

// --- Helper: convert nullable C string to Rust String ---

unsafe fn cstr_to_string(p: *const c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        CStr::from_ptr(p).to_string_lossy().into_owned()
    }
}

// --- Context API ---

/// Create a new XKB context. Returns null on failure.
/// Flags are currently ignored.
#[no_mangle]
pub extern "C" fn xkb_context_new(_flags: u32) -> *mut xkb_context {
    match Context::new() {
        Some(ctx) => Box::into_raw(Box::new(xkb_context(ctx))),
        None => ptr::null_mut(),
    }
}

/// Free an XKB context.
#[no_mangle]
pub unsafe extern "C" fn xkb_context_unref(ctx: *mut xkb_context) {
    if !ctx.is_null() {
        drop(Box::from_raw(ctx));
    }
}

// --- Keymap API ---

/// Create a keymap from RMLVO names. Returns null on failure.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_new_from_names(
    ctx: *mut xkb_context,
    names: *const xkb_rule_names,
    _flags: u32,
) -> *mut xkb_keymap {
    if ctx.is_null() {
        return ptr::null_mut();
    }
    let ctx = &(*ctx).0;

    let rules = if names.is_null() {
        RuleNames::default()
    } else {
        let n = &*names;
        RuleNames {
            rules: cstr_to_string(n.rules),
            model: cstr_to_string(n.model),
            layout: cstr_to_string(n.layout),
            variant: cstr_to_string(n.variant),
            options: cstr_to_string(n.options),
        }
    };

    match (*ctx).clone().keymap_from_names(&rules) {
        Some(km) => Box::into_raw(Box::new(xkb_keymap(km))),
        None => ptr::null_mut(),
    }
}

/// Create a keymap from an XKB keymap string. Returns null on failure.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_new_from_string(
    ctx: *mut xkb_context,
    string: *const c_char,
    _format: u32,
    _flags: u32,
) -> *mut xkb_keymap {
    if ctx.is_null() || string.is_null() {
        return ptr::null_mut();
    }
    let ctx = &(*ctx).0;
    let s = CStr::from_ptr(string).to_string_lossy();

    match (*ctx).clone().keymap_from_string(&s) {
        Some(km) => Box::into_raw(Box::new(xkb_keymap(km))),
        None => ptr::null_mut(),
    }
}

/// Free a keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_unref(keymap: *mut xkb_keymap) {
    if !keymap.is_null() {
        drop(Box::from_raw(keymap));
    }
}

/// Get the minimum keycode in the keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_min_keycode(keymap: *const xkb_keymap) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.min_keycode()
}

/// Get the maximum keycode in the keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_max_keycode(keymap: *const xkb_keymap) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.max_keycode()
}

/// Get the number of modifiers in the keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_num_mods(keymap: *const xkb_keymap) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.num_mods()
}

/// Get the number of layouts in the keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_num_layouts(keymap: *const xkb_keymap) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.num_layouts()
}

/// Get the number of LEDs in the keymap.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_num_leds(keymap: *const xkb_keymap) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.num_leds()
}

/// Check whether a key repeats.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_key_repeats(keymap: *const xkb_keymap, keycode: u32) -> i32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.key_repeats(keycode) as i32
}

/// Get the number of layouts for a specific key.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_num_layouts_for_key(
    keymap: *const xkb_keymap,
    keycode: u32,
) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.num_layouts_for_key(keycode)
}

/// Get the number of levels for a key in a given layout.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_num_levels_for_key(
    keymap: *const xkb_keymap,
    keycode: u32,
    layout: u32,
) -> u32 {
    if keymap.is_null() {
        return 0;
    }
    (*keymap).0.num_levels_for_key(keycode, layout)
}

/// Get keysyms for a key at a specific layout and level.
/// Returns the number of keysyms, and writes the pointer to `syms_out`.
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_key_get_syms_by_level(
    keymap: *const xkb_keymap,
    keycode: u32,
    layout: u32,
    level: u32,
    syms_out: *mut *const u32,
) -> i32 {
    if keymap.is_null() || syms_out.is_null() {
        return 0;
    }
    let syms = (*keymap).0.key_get_syms_by_level(keycode, layout, level);
    if syms.is_empty() {
        *syms_out = ptr::null();
        0
    } else {
        *syms_out = syms.as_ptr();
        syms.len() as i32
    }
}

// --- State API ---

/// Create a new keyboard state from a keymap. Returns null on failure.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_new(keymap: *const xkb_keymap) -> *mut xkb_state {
    if keymap.is_null() {
        return ptr::null_mut();
    }
    match (*keymap).0.new_state() {
        Some(st) => Box::into_raw(Box::new(xkb_state(st))),
        None => ptr::null_mut(),
    }
}

/// Free a keyboard state.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_unref(state: *mut xkb_state) {
    if !state.is_null() {
        drop(Box::from_raw(state));
    }
}

/// Update state for a key press/release. Returns a bitmask of state components that changed.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_update_key(state: *mut xkb_state, keycode: u32, direction: u32) {
    if !state.is_null() {
        (*state).0.update_key(keycode, direction);
    }
}

/// Get the single keysym for a key in the current state.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_key_get_one_sym(state: *const xkb_state, keycode: u32) -> u32 {
    if state.is_null() {
        return 0;
    }
    (*state).0.key_get_one_sym(keycode)
}

/// Get keysyms for a key in the current state.
/// Returns the number of keysyms, writes pointer to `syms_out`.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_key_get_syms(
    state: *const xkb_state,
    keycode: u32,
    syms_out: *mut *const u32,
) -> i32 {
    if state.is_null() || syms_out.is_null() {
        return 0;
    }
    let syms = (*state).0.key_get_syms(keycode);
    if syms.is_empty() {
        *syms_out = ptr::null();
        0
    } else {
        *syms_out = syms.as_ptr();
        syms.len() as i32
    }
}

/// Get UTF-8 string for a key in the current state.
/// Writes into `buffer` (up to `size` bytes). Returns the number of bytes needed
/// (including NUL), or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_key_get_utf8(
    state: *const xkb_state,
    keycode: u32,
    buffer: *mut c_char,
    size: usize,
) -> i32 {
    if state.is_null() {
        return 0;
    }
    let s = (*state).0.key_get_utf8(keycode);
    let needed = s.len() + 1; // include NUL

    if !buffer.is_null() && size > 0 {
        let copy_len = std::cmp::min(s.len(), size - 1);
        ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, buffer, copy_len);
        *buffer.add(copy_len) = 0; // NUL terminate
    }

    needed as i32
}

/// Check if a modifier is active by name.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_mod_name_is_active(
    state: *const xkb_state,
    name: *const c_char,
    state_type: u32,
) -> i32 {
    if state.is_null() || name.is_null() {
        return 0;
    }
    let name = CStr::from_ptr(name).to_string_lossy();
    (*state).0.mod_name_is_active(&name, state_type) as i32
}

/// Check if a modifier is active by index.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_mod_index_is_active(
    state: *const xkb_state,
    idx: u32,
    state_type: u32,
) -> i32 {
    if state.is_null() {
        return 0;
    }
    (*state).0.mod_index_is_active(idx, state_type) as i32
}

/// Serialize the modifier state as a mask.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_serialize_mods(state: *const xkb_state, component: u32) -> u32 {
    if state.is_null() {
        return 0;
    }
    (*state).0.serialize_mods(component)
}

/// Serialize the layout state as an index.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_serialize_layout(
    state: *const xkb_state,
    component: u32,
) -> u32 {
    if state.is_null() {
        return 0;
    }
    (*state).0.serialize_layout(component)
}

/// Update modifier/layout masks directly.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_update_mask(
    state: *mut xkb_state,
    depressed_mods: u32,
    latched_mods: u32,
    locked_mods: u32,
    depressed_layout: u32,
    latched_layout: u32,
    locked_layout: u32,
) {
    if !state.is_null() {
        (*state).0.update_mask(
            depressed_mods,
            latched_mods,
            locked_mods,
            depressed_layout,
            latched_layout,
            locked_layout,
        );
    }
}

/// Check if an LED is active by name.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_led_name_is_active(
    state: *const xkb_state,
    name: *const c_char,
) -> i32 {
    if state.is_null() || name.is_null() {
        return 0;
    }
    let name = CStr::from_ptr(name).to_string_lossy();
    (*state).0.led_name_is_active(&name) as i32
}

/// Check if an LED is active by index.
#[no_mangle]
pub unsafe extern "C" fn xkb_state_led_index_is_active(state: *const xkb_state, idx: u32) -> i32 {
    if state.is_null() {
        return 0;
    }
    (*state).0.led_index_is_active(idx) as i32
}

// --- Keysym utilities ---

/// Convert a keysym to a UTF-32 codepoint. Returns 0 on failure.
#[no_mangle]
pub extern "C" fn xkb_keysym_to_utf32(keysym: u32) -> u32 {
    xkb_core::keysym_utf::keysym_to_utf32(keysym)
}

/// Convert a UTF-32 codepoint to a keysym. Returns 0 on failure.
#[no_mangle]
pub extern "C" fn xkb_utf32_to_keysym(ucs: u32) -> u32 {
    xkb_core::keysym_utf::utf32_to_keysym(ucs)
}

// --- Compose API ---

/// Opaque compose table handle.
pub struct xkb_compose_table {
    inner: xkb_core::compose::ComposeTable,
}

/// Opaque compose state handle.
pub struct xkb_compose_state {
    inner: xkb_core::compose::ComposeState<'static>,
    // Safety: the ComposeTable is kept alive by the xkb_compose_table Box.
    // We use a raw pointer to extend the lifetime; the user must ensure
    // the table outlives the state (matching libxkbcommon's contract).
    _table: *const xkb_compose_table,
}

/// Create a compose table from a locale string. Returns null on failure.
/// The context parameter is currently unused but kept for API compatibility.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_table_new_from_locale(
    _ctx: *mut xkb_context,
    locale: *const c_char,
    _flags: u32,
) -> *mut xkb_compose_table {
    if locale.is_null() {
        return ptr::null_mut();
    }
    let locale_str = CStr::from_ptr(locale).to_string_lossy();
    match xkb_core::compose::ComposeTable::new_from_locale(&locale_str) {
        Some(table) => Box::into_raw(Box::new(xkb_compose_table { inner: table })),
        None => ptr::null_mut(),
    }
}

/// Free a compose table.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_table_unref(table: *mut xkb_compose_table) {
    if !table.is_null() {
        drop(Box::from_raw(table));
    }
}

/// Create a compose state from a table. Returns null on failure.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_new(
    table: *mut xkb_compose_table,
    _flags: u32,
) -> *mut xkb_compose_state {
    if table.is_null() {
        return ptr::null_mut();
    }
    // Extend the lifetime of the table reference to 'static.
    // This is safe as long as the table outlives the state (caller's responsibility).
    let table_ref: &'static xkb_core::compose::ComposeTable =
        &*(&(*table).inner as *const xkb_core::compose::ComposeTable);
    let state = table_ref.new_state();
    Box::into_raw(Box::new(xkb_compose_state {
        inner: state,
        _table: table,
    }))
}

/// Free a compose state.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_unref(state: *mut xkb_compose_state) {
    if !state.is_null() {
        drop(Box::from_raw(state));
    }
}

/// Feed a keysym to the compose state. Returns 0 (ignored) or 1 (accepted).
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_feed(state: *mut xkb_compose_state, keysym: u32) -> u32 {
    if state.is_null() {
        return 0;
    }
    match (*state).inner.feed(keysym) {
        xkb_core::compose::ComposeFeedResult::Ignored => 0,
        xkb_core::compose::ComposeFeedResult::Accepted => 1,
    }
}

/// Get the current compose status.
/// Returns: 0=Nothing, 1=Composing, 2=Composed, 3=Cancelled
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_get_status(state: *const xkb_compose_state) -> u32 {
    if state.is_null() {
        return 0;
    }
    (*state).inner.status() as u32
}

/// Get the composed keysym. Valid when status is Composed.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_get_one_sym(state: *const xkb_compose_state) -> u32 {
    if state.is_null() {
        return 0;
    }
    (*state).inner.keysym()
}

/// Get the composed UTF-8 string. Writes to buffer, returns bytes needed (excl NUL).
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_get_utf8(
    state: *const xkb_compose_state,
    buffer: *mut c_char,
    size: usize,
) -> i32 {
    if state.is_null() {
        return 0;
    }
    let s = (*state).inner.utf8();
    let needed = s.len();

    if !buffer.is_null() && size > 0 {
        let copy_len = std::cmp::min(needed, size - 1);
        ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, buffer, copy_len);
        *buffer.add(copy_len) = 0;
    }

    needed as i32
}

/// Reset the compose state.
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_state_reset(state: *mut xkb_compose_state) {
    if !state.is_null() {
        (*state).inner.reset();
    }
}
