use super::prelude::*;
pub use crate::xkb::shared_ast_types::{KeyAliasDef, KeycodeDef, LedNameDef, ReportNotArray};
pub use crate::xkb::shared_types::{XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS};
use crate::xkb::utils::{cstr_as_bytes, cstr_free};
use crate::xkb::xkbcomp::expr::ExprResolveInteger;
#[derive(Clone)]
pub struct KeyNamesInfo {
    pub name: *mut i8,
    pub errorCount: i32,
    pub include_depth: u32,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: xkb_led_index_t,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
impl KeyNamesInfo {
    pub fn new_zeroed() -> Self {
        Self {
            name: std::ptr::null_mut(),
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: XKB_KEYCODE_INVALID as xkb_keycode_t,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
            led_names: [LedNameInfo {
                merge: MERGE_DEFAULT,
                name: 0,
            }; 32],
            num_led_names: 0,
            ctx: std::ptr::null_mut(),
            keymap_info: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedNameInfo {
    pub merge: merge_mode,
    pub name: xkb_atom_t,
}
#[derive(Clone)]
pub struct KeycodeStore {
    pub min: xkb_keycode_t,
    pub low: Vec<xkb_atom_t>,
    pub high: Vec<HighKeycodeEntry>,
    pub names: Vec<KeycodeMatch>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighKeycodeEntry {
    pub keycode: xkb_keycode_t,
    pub name: xkb_atom_t,
}
unsafe fn vec_resize_zero<T>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.reserve(new_len - v.len());
        let old_len = v.len();
        let ptr = v.as_mut_ptr().add(old_len);
        std::ptr::write_bytes(ptr, 0, new_len - old_len);
        v.set_len(new_len);
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

#[inline]
unsafe fn keycode_store_update_key(
    mut store: *mut KeycodeStore,
    mut match_0: KeycodeMatch,
    mut name: xkb_atom_t,
) {
    unsafe {
        if (!match_0.c2rust_unnamed.found || match_0.c2rust_unnamed.is_alias as i32 != 0) as i64
            != 0
        {
            return;
        } else if match_0.key.low {
            (&mut (*store).low)[match_0.key.index as usize] = name;
        } else {
            (&mut (*store).high)[match_0.key.index as usize].name = name;
        }
        if name >= (&(*store).names).len() as xkb_atom_t {
            vec_resize_zero(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        (&mut (*store).names)[name as usize] = match_0;
    }
}
unsafe fn keycode_store_insert_key(
    mut store: *mut KeycodeStore,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
) -> bool {
    unsafe {
        if name >= (&(*store).names).len() as xkb_atom_t {
            vec_resize_zero(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            if kc >= (&(*store).low).len() as xkb_keycode_t {
                vec_resize_zero(&mut (*store).low, (kc as usize).wrapping_add(1));
            }
            (&mut (*store).low)[kc as usize] = name;
            if kc < (*store).min {
                (*store).min = kc;
            }
            (&mut (*store).names)[name as usize] = KeycodeMatch {
                key: {
                    C2Rust_Unnamed_7 {
                        found: true,
                        low: true,
                        is_alias: false,
                        index: kc as u32,
                    }
                },
            };
        } else {
            let idx: u32 = (&(*store).high).len() as u32;
            if idx != 0 && (&(*store).high)[(idx.wrapping_sub(1 as u32)) as usize].keycode > kc {
                let mut lower: u32 = 0 as u32;
                let mut upper: u32 = idx;
                while lower < upper {
                    let mid: u32 = lower.wrapping_add(
                        upper
                            .wrapping_sub(1 as u32)
                            .wrapping_sub(lower)
                            .wrapping_div(2 as u32),
                    );
                    let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
                    if entry.keycode < kc {
                        lower = mid.wrapping_add(1 as u32);
                    } else if entry.keycode > kc {
                        upper = mid;
                    } else {
                    }
                }
                {
                    let high_ptr = (*store).high.as_mut_ptr();
                    let high_len = (&(*store).high).len() as u32;
                    let mut entry_0: *mut HighKeycodeEntry = high_ptr.offset(lower as isize);
                    while entry_0 < high_ptr.offset(high_len as isize) {
                        let ref mut c2rust_fresh4 =
                            (&mut (*store).names)[(*entry_0).name as usize].key;
                        (*c2rust_fresh4).index = (*c2rust_fresh4).index + 1 as u32;
                        entry_0 = entry_0.offset(1);
                    }
                }
                let __index: u32 = lower;
                (&mut (*store).high).insert(
                    __index as usize,
                    HighKeycodeEntry {
                        keycode: kc,
                        name: name,
                    },
                );
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    key: {
                        C2Rust_Unnamed_7 {
                            found: true,
                            low: false,
                            is_alias: false,
                            index: lower,
                        }
                    },
                };
            } else {
                (&mut (*store).high).push(HighKeycodeEntry {
                    keycode: kc,
                    name: name,
                });
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    key: {
                        C2Rust_Unnamed_7 {
                            found: true,
                            low: false,
                            is_alias: false,
                            index: idx,
                        }
                    },
                };
            }
            if (&(*store).low).len() == 0 {
                (*store).min = (&(*store).high)[0].keycode;
            }
        }
        return true;
    }
}
#[inline]
unsafe fn keycode_store_insert_alias(
    mut store: *mut KeycodeStore,
    mut alias: xkb_atom_t,
    mut real: xkb_atom_t,
) -> bool {
    unsafe {
        if alias >= (&(*store).names).len() as xkb_atom_t {
            vec_resize_zero(&mut (*store).names, (alias as usize).wrapping_add(1));
        }
        (&mut (*store).names)[alias as usize] = KeycodeMatch {
            alias: {
                C2Rust_Unnamed_6 {
                    found: true,
                    c2rust_unnamed: true,
                    is_alias: real != 0,
                    real: real,
                }
            },
        };
        return true;
    }
}
#[inline]
unsafe fn keycode_store_delete_name(mut store: *const KeycodeStore, mut name: xkb_atom_t) {
    unsafe {
        if (name as usize) < (&(*store).names).len() {
            let ref mut c2rust_fresh5 =
                (&mut (*(store as *mut KeycodeStore)).names)[name as usize].c2rust_unnamed;
            (*c2rust_fresh5).found = false;
        }
    }
}
unsafe fn keycode_store_delete_key(mut store: *mut KeycodeStore, match_0: KeycodeMatch) {
    unsafe {
        if (!match_0.c2rust_unnamed.found || match_0.c2rust_unnamed.is_alias as i32 != 0) as i64
            != 0
        {
            return;
        } else if match_0.key.low {
            let low_name = (&(*store).low)[match_0.key.index as usize];
            let ref mut c2rust_fresh1 = (&mut (*store).names)[low_name as usize].c2rust_unnamed;
            (*c2rust_fresh1).found = false;
            if match_0.key.index.wrapping_add(1 as u32) == (&(*store).low).len() as u32 {
                if (*store).min == match_0.key.index as xkb_keycode_t {
                    (&mut (*store).low).clear();
                } else {
                    let mut idx: u32 = match_0.key.index;
                    while idx > 0 as u32 {
                        if (&(*store).low)[(idx.wrapping_sub(1 as u32)) as usize]
                            != XKB_ATOM_NONE as xkb_atom_t
                        {
                            (&mut (*store).low).truncate(idx as usize);
                            break;
                        } else {
                            idx = idx.wrapping_sub(1);
                        }
                    }
                }
            } else {
                (&mut (*store).low)[match_0.key.index as usize] = XKB_ATOM_NONE as xkb_atom_t;
            }
        } else {
            let high_name = (&(*store).high)[match_0.key.index as usize].name;
            let ref mut c2rust_fresh2 = (&mut (*store).names)[high_name as usize].c2rust_unnamed;
            (*c2rust_fresh2).found = false;
            let __index: u32 = match_0.key.index;
            (&mut (*store).high).remove(__index as usize);
            {
                let names_ptr = (*store).names.as_mut_ptr();
                let names_len = (&(*store).names).len();
                let mut entry: *mut KeycodeMatch = names_ptr;
                while entry < names_ptr.add(names_len) {
                    if (*entry).c2rust_unnamed.found as i32 != 0
                        && !(*entry).c2rust_unnamed.is_alias
                        && !(*entry).key.low
                        && (*entry).key.index as i32 > match_0.key.index as i32
                    {
                        (*entry).key.index = (*entry).key.index - 1 as u32;
                    }
                    entry = entry.offset(1);
                }
            }
        }
        if (&(*store).low).len() == 0 {
            (*store).min = if (&(*store).high).len() == 0 {
                XKB_KEYCODE_INVALID as xkb_keycode_t
            } else {
                (&(*store).high)[0].keycode
            };
        } else {
            let mut kc: xkb_keycode_t = (*store).min;
            while kc < (&(*store).low).len() as xkb_keycode_t {
                if (&(*store).low)[kc as usize] != XKB_ATOM_NONE as xkb_atom_t {
                    (*store).min = kc;
                    break;
                } else {
                    kc = kc.wrapping_add(1);
                }
            }
        };
    }
}
unsafe fn keycode_store_lookup_keycode(
    mut store: *const KeycodeStore,
    mut kc: xkb_keycode_t,
) -> KeycodeMatch {
    unsafe {
        if kc < (&(*store).low).len() as xkb_keycode_t {
            return KeycodeMatch {
                key: {
                    C2Rust_Unnamed_7 {
                        found: true,
                        low: true,
                        is_alias: false,
                        index: kc as u32,
                    }
                },
            };
        } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as xkb_keycode_t {
            return KeycodeMatch {
                c2rust_unnamed: {
                    C2Rust_Unnamed_8 {
                        found: false,
                        c2rust_unnamed: false,
                        is_alias: false,
                        c2rust_unnamed_0: 0,
                    }
                },
            };
        }
        let mut lower: u32 = 0 as u32;
        let mut upper: u32 = (&(*store).high).len() as u32;
        while lower < upper {
            let mid: u32 = lower.wrapping_add(
                upper
                    .wrapping_sub(1 as u32)
                    .wrapping_sub(lower)
                    .wrapping_div(2 as u32),
            );
            let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
            if entry.keycode < kc {
                lower = mid.wrapping_add(1 as u32);
            } else if entry.keycode > kc {
                upper = mid;
            } else {
                return KeycodeMatch {
                    key: {
                        C2Rust_Unnamed_7 {
                            found: true,
                            low: false,
                            is_alias: false,
                            index: mid,
                        }
                    },
                };
            }
        }
        return KeycodeMatch {
            c2rust_unnamed: {
                C2Rust_Unnamed_8 {
                    found: false,
                    c2rust_unnamed: false,
                    is_alias: false,
                    c2rust_unnamed_0: 0,
                }
            },
        };
    }
}
unsafe fn keycode_store_lookup_name(
    mut store: *const KeycodeStore,
    mut name: xkb_atom_t,
) -> KeycodeMatch {
    unsafe {
        if name >= (&(*store).names).len() as xkb_atom_t {
            return KeycodeMatch {
                c2rust_unnamed: {
                    C2Rust_Unnamed_8 {
                        found: false,
                        c2rust_unnamed: false,
                        is_alias: false,
                        c2rust_unnamed_0: 0,
                    }
                },
            };
        } else {
            return (&(*store).names)[name as usize];
        };
    }
}
unsafe fn AddLedName(
    mut info: *mut KeyNamesInfo,
    mut same_file: bool,
    mut new: *mut LedNameInfo,
    mut new_idx: xkb_led_index_t,
    mut report: bool,
) -> bool {
    unsafe {
        let mut old_idx: xkb_led_index_t = 0;
        let mut old: *mut LedNameInfo = std::ptr::null_mut();
        let replace: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        // FindLedByName inlined
        {
            let mut idx: xkb_led_index_t = 0 as xkb_led_index_t;
            while idx < (*info).num_led_names {
                let mut ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                    .offset(idx as isize)
                    as *mut LedNameInfo;
                if (*ledi).name == (*new).name {
                    old_idx = idx;
                    old = ledi;
                    break;
                }
                idx = idx.wrapping_add(1);
            }
        }
        if !old.is_null() {
            if old_idx == new_idx {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Multiple indicators named \"{}\"; Identical definitions ignored\n",
                        crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    );
                }
                return true;
            }
            if report {
                let mut use_0: xkb_led_index_t = if replace as i32 != 0 {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                };
                let mut ignore: xkb_led_index_t = if replace as i32 != 0 {
                    old_idx.wrapping_add(1 as xkb_led_index_t)
                } else {
                    new_idx.wrapping_add(1 as xkb_led_index_t)
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple indicators named {}; Using {}, ignoring {}\n",
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, (*new).name)),
                    use_0,
                    ignore,
                );
            }
            if replace {
                (*old).name = XKB_ATOM_NONE as xkb_atom_t;
            } else {
                return true;
            }
        }
        if new_idx >= (*info).num_led_names {
            (*info).num_led_names = new_idx.wrapping_add(1 as xkb_led_index_t);
        }
        old = (&raw mut (*info).led_names as *mut LedNameInfo).offset(new_idx as isize)
            as *mut LedNameInfo;
        if (*old).name != XKB_ATOM_NONE as xkb_atom_t {
            if report {
                let use_1: xkb_atom_t = if replace as i32 != 0 {
                    (*new).name
                } else {
                    (*old).name
                };
                let ignore_0: xkb_atom_t = if replace as i32 != 0 {
                    (*old).name
                } else {
                    (*new).name
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple names for indicator {}; Using {}, ignoring {}\n",
                    new_idx.wrapping_add(1 as xkb_led_index_t),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, use_1)),
                    crate::xkb::utils::CStrDisplay(xkb_atom_text((*info).ctx, ignore_0)),
                );
            }
            if replace {
                *old = *new;
            }
            return true;
        }
        *old = *new;
        return true;
    }
}
unsafe fn ClearKeyNamesInfo(mut info: *mut KeyNamesInfo) {
    unsafe {
        cstr_free((*info).name);
        let store = &raw mut (*info).keycodes;
        (&mut (*store).low).clear();
        (&mut (*store).high).clear();
        (&mut (*store).names).clear();
    }
}
unsafe fn InitKeyNamesInfo(
    mut info: *mut KeyNamesInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
) {
    unsafe {
        std::ptr::write(info, KeyNamesInfo::new_zeroed());
        (*info).ctx = (*keymap_info).keymap.ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
    }
}
unsafe fn AddKeyName(
    mut info: *mut KeyNamesInfo,
    mut kc: xkb_keycode_t,
    mut name: xkb_atom_t,
    mut merge: merge_mode,
    mut report: bool,
) -> bool {
    unsafe {
        let mut match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, name);
        if match_name.c2rust_unnamed.found {
            let clobber: bool = merge as u32 != MERGE_AUGMENT as u32;
            if match_name.c2rust_unnamed.is_alias {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Key name {} already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_name(&raw mut (*info).keycodes, name);
                    match_name.c2rust_unnamed.found = false;
                } else {
                    return true;
                }
            } else {
                let old_kc: xkb_keycode_t = {
                    let store = &raw mut (*info).keycodes;
                    if !match_name.c2rust_unnamed.found
                        || match_name.c2rust_unnamed.is_alias as i32 != 0
                    {
                        XKB_KEYCODE_INVALID as xkb_keycode_t
                    } else if match_name.key.low {
                        match_name.key.index as xkb_keycode_t
                    } else {
                        (&(*store).high)[match_name.key.index as usize].keycode
                    }
                };
                if old_kc != kc {
                    if report {
                        let use_0: xkb_keycode_t = if clobber as i32 != 0 { kc } else { old_kc };
                        let ignore: xkb_keycode_t = if clobber as i32 != 0 { old_kc } else { kc };
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Key name {} assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                            use_0,
                            ignore,
                        );
                    }
                    if clobber {
                        keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                    } else {
                        return true;
                    }
                }
            }
        }
        let match_kc: KeycodeMatch =
            keycode_store_lookup_keycode(&raw mut (*info).keycodes, kc) as KeycodeMatch;
        let old_name: xkb_atom_t = {
            let store = &raw mut (*info).keycodes;
            if !match_kc.c2rust_unnamed.found || match_kc.c2rust_unnamed.is_alias as i32 != 0 {
                XKB_ATOM_NONE as xkb_atom_t
            } else if match_kc.key.low {
                (&(*store).low)[match_kc.key.index as usize]
            } else {
                (&(*store).high)[match_kc.key.index as usize].name
            }
        };
        if old_name != XKB_ATOM_NONE as xkb_atom_t {
            if old_name == name {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Multiple identical key name definitions; Later occurrences of \"{} = {}\" ignored\n",
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, old_name)),
                        kc,
                    );
                }
                return true;
            }
            let clobber_0: bool = merge as u32 != MERGE_AUGMENT as u32;
            if report {
                let kname: *const i8 = KeyNameText((*info).ctx, name) as *const i8;
                let old_kname: *const i8 = KeyNameText((*info).ctx, old_name) as *const i8;
                let use_1: *const i8 = if clobber_0 as i32 != 0 {
                    kname
                } else {
                    old_kname
                };
                let ignore_0: *const i8 = if clobber_0 as i32 != 0 {
                    old_kname
                } else {
                    kname
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple names for keycode {}; Using {}, ignoring {}\n",
                    kc,
                    crate::xkb::utils::CStrDisplay(use_1),
                    crate::xkb::utils::CStrDisplay(ignore_0),
                );
            }
            if clobber_0 {
                keycode_store_delete_name(&raw mut (*info).keycodes, old_name);
                keycode_store_update_key(&raw mut (*info).keycodes, match_kc, name);
            }
        } else if !keycode_store_insert_key(&raw mut (*info).keycodes, kc, name) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot add keycode\n",
                XKB_ERROR_ALLOCATION_ERROR as i32,
            );
            return false;
        }
        return true;
    }
}
unsafe fn MergeKeycodeStores(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (&(*into).keycodes.low).len() == 0
            && (&(*into).keycodes.high).len() == 0
            && (&(*into).keycodes.names).len() == 0
        {
            (*into).keycodes = std::mem::replace(
                &mut (*from).keycodes,
                KeycodeStore {
                    min: XKB_KEYCODE_INVALID as xkb_keycode_t,
                    low: Vec::new(),
                    high: Vec::new(),
                    names: Vec::new(),
                },
            );
        } else {
            let mut kc: xkb_keycode_t = (*from).keycodes.min;
            while kc < (&(*from).keycodes.low).len() as xkb_keycode_t {
                let name: xkb_atom_t = (&(*from).keycodes.low)[kc as usize];
                if !(name == XKB_ATOM_NONE as xkb_atom_t) {
                    if !AddKeyName(into, kc, name, merge, report) {
                        (*into).errorCount += 1;
                    }
                }
                kc = kc.wrapping_add(1);
            }
            {
                let high_ptr = (*from).keycodes.high.as_ptr();
                let high_len = (&(*from).keycodes.high).len();
                let mut new: *const HighKeycodeEntry = high_ptr;
                while new < high_ptr.add(high_len) {
                    if !AddKeyName(into, (*new).keycode, (*new).name, merge, report) {
                        (*into).errorCount += 1;
                    }
                    new = new.offset(1);
                }
            }
            {
                let names_ptr = (*from).keycodes.names.as_ptr();
                let names_len = (&(*from).keycodes.names).len();
                let mut match_0: *const KeycodeMatch = std::ptr::null();
                let mut alias: xkb_atom_t = 0;
                if names_len > 0 {
                    alias = 0 as xkb_atom_t;
                    match_0 = names_ptr;
                    while (alias as usize) < names_len {
                        if !(!(*match_0).c2rust_unnamed.found
                            || !(*match_0).c2rust_unnamed.is_alias)
                        {
                            let def: KeyAliasDef = KeyAliasDef {
                                common: _ParseCommon {
                                    next: std::ptr::null_mut(),
                                    type_0: STMT_UNKNOWN,
                                },
                                merge: merge,
                                alias: alias,
                                real: (*match_0).alias.real,
                            };
                            if !HandleAliasDef(into, &raw const def, report) {
                                (*into).errorCount += 1;
                            }
                        }
                        alias = alias.wrapping_add(1);
                        match_0 = match_0.offset(1);
                    }
                }
            }
        };
    }
}
unsafe fn MergeIncludedKeycodes(
    mut into: *mut KeyNamesInfo,
    mut from: *mut KeyNamesInfo,
    mut merge: merge_mode,
    mut report: bool,
) {
    unsafe {
        if (*from).errorCount > 0 as i32 {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        if (*into).name.is_null() {
            (*into).name =
                _steal(&raw mut (*from).name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        }
        MergeKeycodeStores(into, from, merge, report);
        if (*into).num_led_names == 0 as xkb_led_index_t {
            std::ptr::copy_nonoverlapping::<LedNameInfo>(
                &raw mut (*from).led_names as *mut LedNameInfo,
                &raw mut (*into).led_names as *mut LedNameInfo,
                (*from).num_led_names as usize,
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
                    if !AddLedName(into, false, ledi, idx, report) {
                        (*into).errorCount += 1;
                    }
                }
                idx = idx.wrapping_add(1);
            }
        };
    }
}
unsafe fn HandleIncludeKeycodes(
    mut info: *mut KeyNamesInfo,
    mut include: *mut IncludeStmt,
    mut report: bool,
) -> bool {
    unsafe {
        let mut included: KeyNamesInfo = KeyNamesInfo::new_zeroed();
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return false;
        }
        InitKeyNamesInfo(&raw mut included, (*info).keymap_info, 0 as u32);
        included.name =
            _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyNamesInfo = KeyNamesInfo::new_zeroed();
            let mut file: *mut XkbFile = std::ptr::null_mut();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_KEYCODES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearKeyNamesInfo(&raw mut included);
                return false;
            }
            InitKeyNamesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
            );
            HandleKeycodesFile(&raw mut next_incl, file);
            MergeIncludedKeycodes(&raw mut included, &raw mut next_incl, (*stmt).merge, report);
            ClearKeyNamesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeycodes(info, &raw mut included, (*include).merge, report);
        ClearKeyNamesInfo(&raw mut included);
        return (*info).errorCount == 0 as i32;
    }
}
unsafe fn HandleKeycodeDef(
    mut info: *mut KeyNamesInfo,
    mut stmt: *mut KeycodeDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*stmt).value < 0 as i64 || (*stmt).value > XKB_KEYCODE_MAX as i64 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal keycode {}: must be between 0..{}; Key ignored\n",
                (*stmt).value,
                (0xffffffff as u32).wrapping_sub(1 as u32),
            );
            return false;
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
unsafe fn HandleAliasDef(
    mut info: *mut KeyNamesInfo,
    mut def: *const KeyAliasDef,
    mut report: bool,
) -> bool {
    unsafe {
        let match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, (*def).alias) as KeycodeMatch;
        if match_name.c2rust_unnamed.found {
            let clobber: bool = (*def).merge as u32 != MERGE_AUGMENT as u32;
            if match_name.c2rust_unnamed.is_alias {
                if (*def).real == match_name.alias.real {
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Alias of {} for {} declared more than once; First definition ignored\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).real)),
                        );
                    }
                } else {
                    let use_0: xkb_atom_t = if clobber as i32 != 0 {
                        (*def).real
                    } else {
                        match_name.alias.real
                    };
                    let ignore: xkb_atom_t = if clobber as i32 != 0 {
                        match_name.alias.real
                    } else {
                        (*def).real
                    };
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Multiple definitions for alias {}; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, use_0)),
                            crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, ignore)),
                        );
                    }
                    {
                        let store = &raw mut (*info).keycodes;
                        let ref mut c2rust_fresh3 =
                            (&mut (*store).names)[(*def).alias as usize].alias;
                        (*c2rust_fresh3).real = use_0 as xkb_atom_t;
                    }
                }
                return true;
            } else {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Alias name {} already assigned to a real key; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, (*def).alias)),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"alias\0".as_ptr() as *const i8
                        } else {
                            b"key\0".as_ptr() as *const i8
                        }),
                        crate::xkb::utils::CStrDisplay(if clobber as i32 != 0 {
                            b"key\0".as_ptr() as *const i8
                        } else {
                            b"alias\0".as_ptr() as *const i8
                        }),
                    );
                }
                if clobber {
                    keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                } else {
                    return true;
                }
            }
        }
        return keycode_store_insert_alias(&raw mut (*info).keycodes, (*def).alias, (*def).real);
    }
}
unsafe fn HandleKeyNameVar(mut info: *mut KeyNamesInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: *const i8 = std::ptr::null();
        let mut field: *const i8 = std::ptr::null();
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &raw mut elem,
            &raw mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        }
        if !elem.is_null() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(elem),
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as u32
                == 0;
        }
        if !istreq(cstr_as_bytes(field), b"minimum") && !istreq(cstr_as_bytes(field), b"maximum") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::CStrDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS as u32
                == 0;
        }
        if !arrayNdx.is_null() {
            ReportNotArray(
                (*info).ctx,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                == 0;
        }
        let mut val: i64 = 0 as i64;
        if !ExprResolveInteger((*info).ctx, (*stmt).value, &raw mut val)
            || val < 0 as i64
            || val > u32::MAX as i64
        {
            ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"keycodes\0".as_ptr() as *const i8,
                field,
                b"defaults\0".as_ptr() as *const i8,
                b"integer 0..0xfffffffe\0".as_ptr() as *const i8,
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_FIELD_TYPE_MISMATCH as u32
                == 0;
        }
        return true;
    }
}
unsafe fn HandleLedNameDef(
    mut info: *mut KeyNamesInfo,
    mut def: *mut LedNameDef,
    mut report: bool,
) -> bool {
    unsafe {
        if (*def).ndx < 1 as i64 || (*def).ndx > XKB_MAX_LEDS as i64 {
            (*info).errorCount += 1;
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
                (*def).ndx,
                (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8 as usize) as xkb_led_index_t,
            );
            return false;
        }
        let mut name: xkb_atom_t = XKB_ATOM_NONE as xkb_atom_t;
        if !ExprResolveString((*info).ctx, (*def).name, &raw mut name) {
            let mut buf: [i8; 20] = [0; 20];
            crate::xkb::utils::snprintf_args(
                &raw mut buf as *mut i8,
                std::mem::size_of::<[i8; 20]>(),
                format_args!("{}", (*def).ndx),
            );
            (*info).errorCount += 1;
            return ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                b"indicator\0".as_ptr() as *const i8,
                b"name\0".as_ptr() as *const i8,
                &raw mut buf as *mut i8,
                b"string\0".as_ptr() as *const i8,
            );
        }
        let mut ledi: LedNameInfo = LedNameInfo {
            merge: (*def).merge,
            name: name,
        };
        return AddLedName(
            info,
            true,
            &raw mut ledi,
            ((*def).ndx as xkb_led_index_t).wrapping_sub(1 as xkb_led_index_t),
            report,
        );
    }
}
unsafe fn HandleKeycodesFile(mut info: *mut KeyNamesInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report_same_file: bool = verbosity > 0 as i32;
        let report_include: bool = verbosity > 7 as i32;
        cstr_free((*info).name);
        (*info).name = strdup_safe((*file).name);
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
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
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported keycodes {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        crate::xkb::utils::CStrDisplay(
                            if (*stmt).type_0 as u32 == STMT_UNKNOWN_COMPOUND as u32 {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_STATEMENTS as u32
                        == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
                        crate::xkb::utils::CStrDisplay(stmt_type_to_string((*stmt).type_0)),
                    );
                    ok = false;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10 as i32 {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Abandoning keycodes file \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn CopyKeyNamesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        if (&(*info).keycodes.low).len() == 0 && (&(*info).keycodes.high).len() == 0 {
            (*keymap).min_key_code = 8 as xkb_keycode_t;
            (*keymap).max_key_code = 255 as xkb_keycode_t;
            (*keymap).num_keys_low = (*keymap).max_key_code.wrapping_add(1 as xkb_keycode_t);
            (*keymap).num_keys = (*keymap).num_keys_low;
        } else {
            (*keymap).min_key_code = (*info).keycodes.min;
            (*keymap).max_key_code = if (&(*info).keycodes.high).len() == 0 {
                ((&(*info).keycodes.low).len() as xkb_keycode_t).wrapping_sub(1 as xkb_keycode_t)
            } else {
                (&(*info).keycodes.high)[(&(*info).keycodes.high).len().wrapping_sub(1)].keycode
            };
            (*keymap).num_keys_low = (&(*info).keycodes.low).len() as xkb_keycode_t;
            (*keymap).num_keys = (*keymap)
                .num_keys_low
                .wrapping_add((&(*info).keycodes.high).len() as xkb_keycode_t);
        }
        let keys: *mut xkb_key =
            calloc((*keymap).num_keys as usize, std::mem::size_of::<xkb_key>()) as *mut xkb_key;
        if keys.is_null() {
            (*keymap).num_keys = 0 as xkb_keycode_t;
            (*keymap).max_key_code = XKB_KEYCODE_INVALID as xkb_keycode_t;
            (*keymap).min_key_code = (*keymap).max_key_code;
            return false;
        }
        let mut kc: xkb_keycode_t = (*keymap).min_key_code;
        while kc < (*keymap).num_keys_low {
            (*keys.offset(kc as isize)).keycode = kc;
            kc = kc.wrapping_add(1);
        }
        let mut kc_0: xkb_keycode_t = (*info).keycodes.min;
        while kc_0 < (&(*info).keycodes.low).len() as xkb_keycode_t {
            (*keys.offset(kc_0 as isize)).name = (&(*info).keycodes.low)[kc_0 as usize];
            kc_0 = kc_0.wrapping_add(1);
        }
        let mut idx: xkb_keycode_t = (*keymap).num_keys_low;
        {
            let high_ptr = (*info).keycodes.high.as_ptr();
            let high_len = (&(*info).keycodes.high).len();
            let mut entry: *const HighKeycodeEntry = high_ptr;
            while entry < high_ptr.add(high_len) {
                (*keys.offset(idx as isize)).keycode = (*entry).keycode;
                (*keys.offset(idx as isize)).name = (*entry).name;
                idx = idx.wrapping_add(1);
                entry = entry.offset(1);
            }
        }
        (*keymap).keys = keys;
        return true;
    }
}
unsafe fn CopyKeycodeNameLUT(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
    unsafe {
        let names_len = (&(*info).keycodes.names).len();
        let names_ptr = (*info).keycodes.names.as_mut_ptr();
        {
            let mut match_0: *mut KeycodeMatch = std::ptr::null_mut();
            let mut name: xkb_atom_t = 0;
            if names_len > 0 {
                name = 0 as xkb_atom_t;
                match_0 = names_ptr;
                while (name as usize) < names_len {
                    if (*match_0).c2rust_unnamed.found {
                        if (*match_0).c2rust_unnamed.is_alias {
                            let match_real: KeycodeMatch = keycode_store_lookup_name(
                                &raw mut (*info).keycodes,
                                (*match_0).alias.real,
                            )
                                as KeycodeMatch;
                            if !match_real.c2rust_unnamed.found {
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_DETAILED as i32,
                                    "[XKB-{:03}] Attempt to alias {} to non-existent key {}; Ignored\n",
                                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                    crate::xkb::utils::CStrDisplay(KeyNameText((*info).ctx, name)),
                                    crate::xkb::utils::CStrDisplay(KeyNameText(
                                        (*info).ctx,
                                        (*match_0).alias.real
                                    )),
                                );
                                (*match_0).c2rust_unnamed.found = false;
                            } else {
                            }
                        } else if !(*match_0).key.low {
                            (*match_0).key.index =
                                (*match_0).key.index + (*keymap).num_keys_low as u32;
                        }
                    }
                    name = name.wrapping_add(1);
                    match_0 = match_0.offset(1);
                }
            }
        }
        if names_len > 0 {
            // Shrink the Vec to exact size, then steal the allocation
            (&mut (*info).keycodes.names).shrink_to_fit();
            let stolen_ptr = (*info).keycodes.names.as_mut_ptr();
            let stolen_len = (&(*info).keycodes.names).len();
            // Prevent Vec from freeing the allocation
            std::mem::forget(std::mem::replace(&mut (*info).keycodes.names, Vec::new()));
            (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = stolen_len as u32;
            (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = stolen_ptr;
        } else {
            (*keymap).c2rust_unnamed.c2rust_unnamed.num_key_names = 0 as u32;
            (*keymap).c2rust_unnamed.c2rust_unnamed.key_names = std::ptr::null_mut();
        }
        return true;
    }
}
unsafe fn CopyLedNamesToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut KeyNamesInfo) -> bool {
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
        return true;
    }
}
unsafe fn CopyKeyNamesInfoToKeymap(
    mut keymap: *mut xkb_keymap,
    mut info: *mut KeyNamesInfo,
) -> bool {
    unsafe {
        if !CopyKeyNamesToKeymap(keymap, info)
            || !CopyKeycodeNameLUT(keymap, info)
            || !CopyLedNamesToKeymap(keymap, info)
        {
            return false;
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
        return true;
    }
}
pub unsafe fn CompileKeycodes(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: KeyNamesInfo = KeyNamesInfo::new_zeroed();
        InitKeyNamesInfo(&raw mut info, keymap_info, 0 as u32);
        if !file.is_null() {
            HandleKeycodesFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopyKeyNamesInfoToKeymap(&raw mut (*keymap_info).keymap, &raw mut info) {
                ClearKeyNamesInfo(&raw mut info);
                return true;
            }
        }
        ClearKeyNamesInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
