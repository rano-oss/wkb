use super::prelude::*;
pub use crate::xkb::shared_ast_types::{KeyAliasDef, KeycodeDef, LedNameDef, ReportNotArray};
pub use crate::xkb::shared_types::{XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS};
use crate::xkb::utils::cstr_free;
use crate::xkb::xkbcomp::expr::ExprResolveInteger;
#[derive(Clone)]
pub struct KeyNamesInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: u32,
    pub ctx: *mut xkb_context,
    pub keymap_info: *const xkb_keymap_info,
}
impl KeyNamesInfo {
    pub fn new_zeroed() -> Self {
        Self {
            name: None,
            errorCount: 0,
            include_depth: 0,
            keycodes: KeycodeStore {
                min: XKB_KEYCODE_INVALID,
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
    pub name: u32,
}
#[derive(Clone)]
pub struct KeycodeStore {
    pub min: u32,
    pub low: Vec<u32>,
    pub high: Vec<HighKeycodeEntry>,
    pub names: Vec<KeycodeMatch>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighKeycodeEntry {
    pub keycode: u32,
    pub name: u32,
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
unsafe fn keycode_store_update_key(store: *mut KeycodeStore, match_0: KeycodeMatch, name: u32) {
    unsafe {
        if (!match_0.found || match_0.is_alias as i32 != 0) as i64 != 0 {
            return;
        } else if match_0.low {
            (&mut (*store).low)[match_0.index as usize] = name;
        } else {
            (&mut (*store).high)[match_0.index as usize].name = name;
        }
        if name >= (*store).names.len() as u32 {
            vec_resize_zero(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        (&mut (*store).names)[name as usize] = match_0;
    }
}
unsafe fn keycode_store_insert_key(store: *mut KeycodeStore, kc: u32, name: u32) -> bool {
    unsafe {
        if name >= (*store).names.len() as u32 {
            vec_resize_zero(&mut (*store).names, (name as usize).wrapping_add(1));
        }
        if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
            if kc >= (*store).low.len() as u32 {
                vec_resize_zero(&mut (*store).low, (kc as usize).wrapping_add(1));
            }
            (&mut (*store).low)[kc as usize] = name;
            if kc < (*store).min {
                (*store).min = kc;
            }
            (&mut (*store).names)[name as usize] = KeycodeMatch {
                found: true,
                low: true,
                is_alias: false,
                index: kc,
            };
        } else {
            let idx: u32 = (*store).high.len() as u32;
            if idx != 0 && (&(*store).high)[(idx.wrapping_sub(1_u32)) as usize].keycode > kc {
                let mut lower: u32 = 0_u32;
                let mut upper: u32 = idx;
                while lower < upper {
                    let mid: u32 = lower.wrapping_add(
                        upper
                            .wrapping_sub(1_u32)
                            .wrapping_sub(lower)
                            .wrapping_div(2_u32),
                    );
                    let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
                    if entry.keycode < kc {
                        lower = mid.wrapping_add(1_u32);
                    } else if entry.keycode > kc {
                        upper = mid;
                    }
                }
                {
                    let high_ptr = (*store).high.as_mut_ptr();
                    let high_len = (*store).high.len() as u32;
                    let mut entry_0: *mut HighKeycodeEntry = high_ptr.offset(lower as isize);
                    while entry_0 < high_ptr.offset(high_len as isize) {
                        (&mut (*store).names)[(*entry_0).name as usize].index += 1;
                        entry_0 = entry_0.offset(1);
                    }
                }
                let __index: u32 = lower;
                (*store)
                    .high
                    .insert(__index as usize, HighKeycodeEntry { keycode: kc, name });
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    found: true,
                    low: false,
                    is_alias: false,
                    index: lower,
                };
            } else {
                (*store).high.push(HighKeycodeEntry { keycode: kc, name });
                (&mut (*store).names)[name as usize] = KeycodeMatch {
                    found: true,
                    low: false,
                    is_alias: false,
                    index: idx,
                };
            }
            if (*store).low.is_empty() {
                (*store).min = (&(*store).high)[0].keycode;
            }
        }
        true
    }
}
#[inline]
unsafe fn keycode_store_insert_alias(store: *mut KeycodeStore, alias: u32, real: u32) -> bool {
    unsafe {
        if alias >= (*store).names.len() as u32 {
            vec_resize_zero(&mut (*store).names, (alias as usize).wrapping_add(1));
        }
        (&mut (*store).names)[alias as usize] = KeycodeMatch {
            found: true,
            low: true,
            is_alias: real != 0,
            index: real,
        };
        true
    }
}
#[inline]
unsafe fn keycode_store_delete_name(store: *const KeycodeStore, name: u32) {
    unsafe {
        if (name as usize) < (*store).names.len() {
            (&mut (*(store as *mut KeycodeStore)).names)[name as usize].found = false;
        }
    }
}
unsafe fn keycode_store_delete_key(store: *mut KeycodeStore, match_0: KeycodeMatch) {
    unsafe {
        if (!match_0.found || match_0.is_alias as i32 != 0) as i64 != 0 {
            return;
        } else if match_0.low {
            let low_name = (&(*store).low)[match_0.index as usize];
            (&mut (*store).names)[low_name as usize].found = false;
            if match_0.index.wrapping_add(1_u32) == (*store).low.len() as u32 {
                if (*store).min == match_0.index {
                    (*store).low.clear();
                } else {
                    let mut idx: u32 = match_0.index;
                    while idx > 0_u32 {
                        if (&(*store).low)[(idx.wrapping_sub(1_u32)) as usize] != XKB_ATOM_NONE {
                            (*store).low.truncate(idx as usize);
                            break;
                        } else {
                            idx = idx.wrapping_sub(1);
                        }
                    }
                }
            } else {
                (&mut (*store).low)[match_0.index as usize] = XKB_ATOM_NONE;
            }
        } else {
            let high_name = (&(*store).high)[match_0.index as usize].name;
            (&mut (*store).names)[high_name as usize].found = false;
            let __index: u32 = match_0.index;
            (*store).high.remove(__index as usize);
            {
                let names_ptr = (*store).names.as_mut_ptr();
                let names_len = (*store).names.len();
                let mut entry: *mut KeycodeMatch = names_ptr;
                while entry < names_ptr.add(names_len) {
                    if (*entry).found as i32 != 0
                        && !(*entry).is_alias
                        && !(*entry).low
                        && (*entry).index as i32 > match_0.index as i32
                    {
                        (*entry).index -= 1_u32;
                    }
                    entry = entry.offset(1);
                }
            }
        }
        if (*store).low.is_empty() {
            (*store).min = if (*store).high.is_empty() {
                XKB_KEYCODE_INVALID
            } else {
                (&(*store).high)[0].keycode
            };
        } else {
            let mut kc: u32 = (*store).min;
            while kc < (*store).low.len() as u32 {
                if (&(*store).low)[kc as usize] != XKB_ATOM_NONE {
                    (*store).min = kc;
                    break;
                } else {
                    kc = kc.wrapping_add(1);
                }
            }
        };
    }
}
unsafe fn keycode_store_lookup_keycode(store: *const KeycodeStore, kc: u32) -> KeycodeMatch {
    unsafe {
        if kc < (*store).low.len() as u32 {
            return KeycodeMatch {
                found: true,
                low: true,
                is_alias: false,
                index: kc,
            };
        } else if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
            return KeycodeMatch {
                found: false,
                low: false,
                is_alias: false,
                index: 0,
            };
        }
        let mut lower: u32 = 0_u32;
        let mut upper: u32 = (*store).high.len() as u32;
        while lower < upper {
            let mid: u32 = lower.wrapping_add(
                upper
                    .wrapping_sub(1_u32)
                    .wrapping_sub(lower)
                    .wrapping_div(2_u32),
            );
            let entry: &HighKeycodeEntry = &(&(*store).high)[mid as usize];
            if entry.keycode < kc {
                lower = mid.wrapping_add(1_u32);
            } else if entry.keycode > kc {
                upper = mid;
            } else {
                return KeycodeMatch {
                    found: true,
                    low: false,
                    is_alias: false,
                    index: mid,
                };
            }
        }
        KeycodeMatch {
            found: false,
            low: false,
            is_alias: false,
            index: 0,
        }
    }
}
unsafe fn keycode_store_lookup_name(store: *const KeycodeStore, name: u32) -> KeycodeMatch {
    unsafe {
        if name >= (*store).names.len() as u32 {
            KeycodeMatch {
                found: false,
                low: false,
                is_alias: false,
                index: 0,
            }
        } else {
            (&(*store).names)[name as usize]
        }
    }
}
unsafe fn AddLedName(
    info: *mut KeyNamesInfo,
    _same_file: bool,
    new: *mut LedNameInfo,
    new_idx: u32,
    report: bool,
) -> bool {
    unsafe {
        let mut old_idx: u32 = 0;
        let mut old: *mut LedNameInfo = std::ptr::null_mut();
        let replace: bool = (*new).merge != MERGE_AUGMENT;
        // FindLedByName inlined
        {
            let mut idx: u32 = 0_u32;
            while idx < (*info).num_led_names {
                let ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
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
                        xkb_atom_text(&(*(*info).ctx).atom_table, (*new).name),
                    );
                }
                return true;
            }
            if report {
                let use_0: u32 = if replace as i32 != 0 {
                    new_idx.wrapping_add(1_u32)
                } else {
                    old_idx.wrapping_add(1_u32)
                };
                let ignore: u32 = if replace as i32 != 0 {
                    old_idx.wrapping_add(1_u32)
                } else {
                    new_idx.wrapping_add(1_u32)
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple indicators named {}; Using {}, ignoring {}\n",
                    xkb_atom_text(&(*(*info).ctx).atom_table, (*new).name),
                    use_0,
                    ignore,
                );
            }
            if replace {
                (*old).name = XKB_ATOM_NONE;
            } else {
                return true;
            }
        }
        if new_idx >= (*info).num_led_names {
            (*info).num_led_names = new_idx.wrapping_add(1_u32);
        }
        old = (&raw mut (*info).led_names as *mut LedNameInfo).offset(new_idx as isize)
            as *mut LedNameInfo;
        if (*old).name != XKB_ATOM_NONE {
            if report {
                let use_1: u32 = if replace as i32 != 0 {
                    (*new).name
                } else {
                    (*old).name
                };
                let ignore_0: u32 = if replace as i32 != 0 {
                    (*old).name
                } else {
                    (*new).name
                };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple names for indicator {}; Using {}, ignoring {}\n",
                    new_idx.wrapping_add(1_u32),
                    xkb_atom_text(&(*(*info).ctx).atom_table, use_1),
                    xkb_atom_text(&(*(*info).ctx).atom_table, ignore_0),
                );
            }
            if replace {
                *old = *new;
            }
            return true;
        }
        *old = *new;
        true
    }
}
unsafe fn ClearKeyNamesInfo(info: *mut KeyNamesInfo) {
    unsafe {
        (*info).name = None;
        let store = &raw mut (*info).keycodes;
        (*store).high.clear();
        (*store).names.clear();
    }
}
unsafe fn InitKeyNamesInfo(
    info: *mut KeyNamesInfo,
    keymap_info: *const xkb_keymap_info,
    include_depth: u32,
) {
    unsafe {
        std::ptr::write(info, KeyNamesInfo::new_zeroed());
        (*info).ctx = &raw mut (*(*keymap_info).keymap).ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
    }
}
unsafe fn AddKeyName(
    info: *mut KeyNamesInfo,
    kc: u32,
    name: u32,
    merge: merge_mode,
    report: bool,
) -> bool {
    unsafe {
        let match_name: KeycodeMatch = keycode_store_lookup_name(&raw mut (*info).keycodes, name);
        if match_name.found {
            let clobber: bool = merge != MERGE_AUGMENT;
            if match_name.is_alias {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Key name {} already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), name)),
                        crate::xkb::utils::ByteSliceDisplay(if clobber { b"key" as &[u8] } else { b"alias" }),
                        crate::xkb::utils::ByteSliceDisplay(if clobber { b"alias" as &[u8] } else { b"key" }),
                    );
                }
                if clobber {
                    keycode_store_delete_name(&raw mut (*info).keycodes, name);
                    // dead store removed: match_name.found = false;
                } else {
                    return true;
                }
            } else {
                let old_kc: u32 = {
                    let store = &raw mut (*info).keycodes;
                    if !match_name.found || match_name.is_alias as i32 != 0 {
                        XKB_KEYCODE_INVALID
                    } else if match_name.low {
                        match_name.index
                    } else {
                        (&(*store).high)[match_name.index as usize].keycode
                    }
                };
                if old_kc != kc {
                    if report {
                        let use_0: u32 = if clobber as i32 != 0 { kc } else { old_kc };
                        let ignore: u32 = if clobber as i32 != 0 { old_kc } else { kc };
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Key name {} assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), name)),
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
        let old_name: u32 = {
            let store = &raw mut (*info).keycodes;
            if !match_kc.found || match_kc.is_alias as i32 != 0 {
                XKB_ATOM_NONE
            } else if match_kc.low {
                (&(*store).low)[match_kc.index as usize]
            } else {
                (&(*store).high)[match_kc.index as usize].name
            }
        };
        if old_name != XKB_ATOM_NONE {
            if old_name == name {
                if report {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Multiple identical key name definitions; Later occurrences of \"{} = {}\" ignored\n",
                        crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), old_name)),
                        kc,
                    );
                }
                return true;
            }
            let clobber_0: bool = merge != MERGE_AUGMENT;
            if report {
                let kname: &[u8] = KeyNameText((*(*info).ctx).clone(), name);
                let old_kname: &[u8] = KeyNameText((*(*info).ctx).clone(), old_name);
                let use_1: &[u8] = if clobber_0 { kname } else { old_kname };
                let ignore_0: &[u8] = if clobber_0 { old_kname } else { kname };
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple names for keycode {}; Using {}, ignoring {}\n",
                    kc,
                    crate::xkb::utils::ByteSliceDisplay(use_1),
                    crate::xkb::utils::ByteSliceDisplay(ignore_0),
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
        true
    }
}
unsafe fn MergeKeycodeStores(
    into: *mut KeyNamesInfo,
    from: *mut KeyNamesInfo,
    merge: merge_mode,
    report: bool,
) {
    unsafe {
        if (*into).keycodes.low.is_empty()
            && (*into).keycodes.high.is_empty()
            && (*into).keycodes.names.is_empty()
        {
            (*into).keycodes = std::mem::replace(
                &mut (*from).keycodes,
                KeycodeStore {
                    min: XKB_KEYCODE_INVALID,
                    low: Vec::new(),
                    high: Vec::new(),
                    names: Vec::new(),
                },
            );
        } else {
            let mut kc: u32 = (*from).keycodes.min;
            while kc < (*from).keycodes.low.len() as u32 {
                let name: u32 = (&(*from).keycodes.low)[kc as usize];
                if (name != XKB_ATOM_NONE) && !AddKeyName(into, kc, name, merge, report) {
                    (*into).errorCount += 1;
                }
                kc = kc.wrapping_add(1);
            }
            {
                let high_ptr = (*from).keycodes.high.as_ptr();
                let high_len = (*from).keycodes.high.len();
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
                let names_len = (*from).keycodes.names.len();
                let mut match_0: *const KeycodeMatch;
                let mut alias: u32;
                if names_len > 0 {
                    alias = 0_u32;
                    match_0 = names_ptr;
                    while (alias as usize) < names_len {
                        if !(!(*match_0).found || !(*match_0).is_alias) {
                            let def: KeyAliasDef = KeyAliasDef {
                                common: _ParseCommon {
                                    next: std::ptr::null_mut(),
                                    type_0: STMT_UNKNOWN,
                                },
                                merge,
                                alias,
                                real: (*match_0).index,
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
    into: *mut KeyNamesInfo,
    from: *mut KeyNamesInfo,
    merge: merge_mode,
    report: bool,
) {
    unsafe {
        if (*from).errorCount > 0_i32 {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        if (*into).name.is_none() {
            (*into).name = (*from).name.take();
        }
        MergeKeycodeStores(into, from, merge, report);
        if (*into).num_led_names == 0_u32 {
            std::ptr::copy_nonoverlapping::<LedNameInfo>(
                &raw mut (*from).led_names as *mut LedNameInfo,
                &raw mut (*into).led_names as *mut LedNameInfo,
                (*from).num_led_names as usize,
            );
            (*into).num_led_names = (*from).num_led_names;
            (*from).num_led_names = 0_u32;
        } else {
            let mut idx: u32 = 0_u32;
            while idx < (*from).num_led_names {
                let ledi: *mut LedNameInfo = (&raw mut (*from).led_names as *mut LedNameInfo)
                    .offset(idx as isize)
                    as *mut LedNameInfo;
                if (*ledi).name != XKB_ATOM_NONE {
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
    info: *mut KeyNamesInfo,
    include: *mut IncludeStmt,
    report: bool,
) -> bool {
    unsafe {
        let mut included: KeyNamesInfo = KeyNamesInfo::new_zeroed();
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10_i32;
            return false;
        }
        InitKeyNamesInfo(&raw mut included, (*info).keymap_info, 0_u32);
        included.name = {
            let ptr = _steal(&raw mut (*include).stmt as *mut ::core::ffi::c_void) as *mut i8;
            if ptr.is_null() {
                None
            } else {
                let s = std::ffi::CStr::from_ptr(ptr)
                    .to_str()
                    .unwrap_or("")
                    .to_string();
                cstr_free(ptr);
                Some(s)
            }
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: KeyNamesInfo = KeyNamesInfo::new_zeroed();

            let mut path: [i8; 4096] = [0; 4096];
            let file: *mut XkbFile = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_KEYCODES,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10_i32;
                ClearKeyNamesInfo(&raw mut included);
                return false;
            }
            InitKeyNamesInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1_u32),
            );
            HandleKeycodesFile(&raw mut next_incl, file);
            MergeIncludedKeycodes(&raw mut included, &raw mut next_incl, (*stmt).merge, report);
            ClearKeyNamesInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedKeycodes(info, &raw mut included, (*include).merge, report);
        ClearKeyNamesInfo(&raw mut included);
        (*info).errorCount == 0_i32
    }
}
unsafe fn HandleKeycodeDef(info: *mut KeyNamesInfo, stmt: *mut KeycodeDef, report: bool) -> bool {
    unsafe {
        if (*stmt).value < 0_i64 || (*stmt).value > XKB_KEYCODE_MAX as i64 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal keycode {}: must be between 0..{}; Key ignored\n",
                (*stmt).value,
                0xffffffff_u32.wrapping_sub(1_u32),
            );
            return false;
        }
        AddKeyName(
            info,
            (*stmt).value as u32,
            (*stmt).name,
            (*stmt).merge,
            report,
        )
    }
}
unsafe fn HandleAliasDef(info: *mut KeyNamesInfo, def: *const KeyAliasDef, report: bool) -> bool {
    unsafe {
        let match_name: KeycodeMatch =
            keycode_store_lookup_name(&raw mut (*info).keycodes, (*def).alias) as KeycodeMatch;
        if match_name.found {
            let clobber: bool = (*def).merge != MERGE_AUGMENT;
            if match_name.is_alias {
                if (*def).real == match_name.index {
                    if report {
                        xkb_logf!(
                            (*info).ctx,
                            XKB_LOG_LEVEL_WARNING,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "[XKB-{:03}] Alias of {} for {} declared more than once; First definition ignored\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), (*def).alias)),
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), (*def).real)),
                        );
                    }
                } else {
                    let use_0: u32 = if clobber as i32 != 0 {
                        (*def).real
                    } else {
                        match_name.index
                    };
                    let ignore: u32 = if clobber as i32 != 0 {
                        match_name.index
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
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), (*def).alias)),
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), use_0)),
                            crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), ignore)),
                        );
                    }
                    {
                        let store = &raw mut (*info).keycodes;
                        (&mut (*store).names)[(*def).alias as usize].index = use_0;
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
                        crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), (*def).alias)),
                        crate::xkb::utils::ByteSliceDisplay(if clobber { b"alias" as &[u8] } else { b"key" }),
                        crate::xkb::utils::ByteSliceDisplay(if clobber { b"key" as &[u8] } else { b"alias" }),
                    );
                }
                if clobber {
                    keycode_store_delete_key(&raw mut (*info).keycodes, match_name);
                } else {
                    return true;
                }
            }
        }
        keycode_store_insert_alias(&raw mut (*info).keycodes, (*def).alias, (*def).real)
    }
}
unsafe fn HandleKeyNameVar(info: *mut KeyNamesInfo, stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &mut elem,
            &mut field,
            &raw mut arrayNdx,
        ) {
            return false;
        }
        if !elem.is_empty() {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                elem,
                elem,
                field,
            );
            return (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
        }
        if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field,
            );
            return (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
        }
        if !arrayNdx.is_null() {
            ReportNotArray((*info).ctx, "keycodes", field, "defaults");
            return (*(*info).keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
        }
        let mut val: i64 = 0_i64;
        if !ExprResolveInteger((*info).ctx, (*stmt).value, &raw mut val)
            || val < 0_i64
            || val > u32::MAX as i64
        {
            ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                "keycodes",
                field,
                "defaults",
                "integer 0..0xfffffffe",
            );
            return (*(*info).keymap_info).strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
        }
        true
    }
}
unsafe fn HandleLedNameDef(info: *mut KeyNamesInfo, def: *mut LedNameDef, report: bool) -> bool {
    unsafe {
        if (*def).ndx < 1_i64 || (*def).ndx > XKB_MAX_LEDS as i64 {
            (*info).errorCount += 1;
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
                (*def).ndx,
                (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32,
            );
            return false;
        }
        let mut name: u32 = XKB_ATOM_NONE;
        if !ExprResolveString((*info).ctx, (*def).name, &raw mut name) {
            let mut buf: [u8; 20] = [0; 20];
            let buf_len = {
                let mut w = crate::xkb::utils::LogBuf::new(&mut buf[..19]);
                let _ = core::fmt::Write::write_fmt(&mut w, format_args!("{}", (*def).ndx));
                w.pos
            };
            (*info).errorCount += 1;
            return ReportBadType(
                (*info).ctx,
                XKB_ERROR_WRONG_FIELD_TYPE,
                "indicator",
                "name",
                std::str::from_utf8_unchecked(&buf[..buf_len]),
                "string",
            );
        }
        let mut ledi: LedNameInfo = LedNameInfo {
            merge: (*def).merge,
            name,
        };
        AddLedName(
            info,
            true,
            &raw mut ledi,
            ((*def).ndx as u32).wrapping_sub(1_u32),
            report,
        )
    }
}
unsafe fn HandleKeycodesFile(info: *mut KeyNamesInfo, file: *mut XkbFile) {
    unsafe {
        let mut ok: bool;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx);
        let report_same_file: bool = verbosity > 0_i32;
        let report_include: bool = verbosity > 7_i32;
        (*info).name = {
            let ptr = (*file).name;
            if ptr.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr(ptr)
                        .to_str()
                        .unwrap_or("")
                        .to_string(),
                )
            }
        };
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 {
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
                            if (*stmt).type_0 == STMT_UNKNOWN_COMPOUND {
                                b"compound\0".as_ptr() as *const i8
                            } else {
                                b"declaration\0".as_ptr() as *const i8
                            }
                        ),
                        crate::xkb::utils::CStrDisplay((*(stmt as *mut UnknownStatement)).name),
                    );
                    ok = (*(*info).keymap_info).strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false;
                }
            }
            if !ok {
                (*info).errorCount += 1;
            }
            if (*info).errorCount > 10_i32 {
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
unsafe fn CopyKeyNamesToKeymap(keymap: *mut xkb_keymap, info: *mut KeyNamesInfo) -> bool {
    unsafe {
        if (*info).keycodes.low.is_empty() && (*info).keycodes.high.is_empty() {
            (*keymap).min_key_code = 8_u32;
            (*keymap).max_key_code = 255_u32;
            (*keymap).num_keys_low = (*keymap).max_key_code.wrapping_add(1_u32);
            (*keymap).num_keys = (*keymap).num_keys_low;
        } else {
            (*keymap).min_key_code = (*info).keycodes.min;
            (*keymap).max_key_code = if (*info).keycodes.high.is_empty() {
                ((*info).keycodes.low.len() as u32).wrapping_sub(1_u32)
            } else {
                (&(*info).keycodes.high)[(*info).keycodes.high.len().wrapping_sub(1)].keycode
            };
            (*keymap).num_keys_low = (*info).keycodes.low.len() as u32;
            (*keymap).num_keys = (*keymap)
                .num_keys_low
                .wrapping_add((*info).keycodes.high.len() as u32);
        }
        let mut keys: Vec<xkb_key> = (0..(*keymap).num_keys as usize)
            .map(|_| xkb_key::default())
            .collect();
        let mut kc: u32 = (*keymap).min_key_code;
        while kc < (*keymap).num_keys_low {
            keys[kc as usize].keycode = kc;
            kc = kc.wrapping_add(1);
        }
        let mut kc_0: u32 = (*info).keycodes.min;
        while kc_0 < (*info).keycodes.low.len() as u32 {
            keys[kc_0 as usize].name = (&(*info).keycodes.low)[kc_0 as usize];
            kc_0 = kc_0.wrapping_add(1);
        }
        let mut idx: u32 = (*keymap).num_keys_low;
        {
            let high_ptr = (*info).keycodes.high.as_ptr();
            let high_len = (*info).keycodes.high.len();
            let mut entry: *const HighKeycodeEntry = high_ptr;
            while entry < high_ptr.add(high_len) {
                keys[idx as usize].keycode = (*entry).keycode;
                keys[idx as usize].name = (*entry).name;
                idx = idx.wrapping_add(1);
                entry = entry.offset(1);
            }
        }
        (*keymap).keys = keys;
        true
    }
}
unsafe fn CopyKeycodeNameLUT(keymap: *mut xkb_keymap, info: *mut KeyNamesInfo) -> bool {
    unsafe {
        let names_len = (*info).keycodes.names.len();
        let names_ptr = (*info).keycodes.names.as_mut_ptr();
        {
            let mut match_0: *mut KeycodeMatch;
            let mut name: u32;
            if names_len > 0 {
                name = 0_u32;
                match_0 = names_ptr;
                while (name as usize) < names_len {
                    if (*match_0).found {
                        if (*match_0).is_alias {
                            let match_real: KeycodeMatch = keycode_store_lookup_name(
                                &raw mut (*info).keycodes,
                                (*match_0).index,
                            )
                                as KeycodeMatch;
                            if !match_real.found {
                                xkb_logf!(
                                    (*info).ctx,
                                    XKB_LOG_LEVEL_WARNING,
                                    XKB_LOG_VERBOSITY_DETAILED as i32,
                                    "[XKB-{:03}] Attempt to alias {} to non-existent key {}; Ignored\n",
                                    XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                    crate::xkb::utils::ByteSliceDisplay(KeyNameText((*(*info).ctx).clone(), name)),
                                    crate::xkb::utils::ByteSliceDisplay(KeyNameText(
                                        (*(*info).ctx).clone(),
                                        (*match_0).index
                                    )),
                                );
                                (*match_0).found = false;
                            }
                        } else if !(*match_0).low {
                            (*match_0).index += (*keymap).num_keys_low;
                        }
                    }
                    name = name.wrapping_add(1);
                    match_0 = match_0.offset(1);
                }
            }
        }
        if names_len > 0 {
            // Move the Vec directly to keymap
            (*keymap).key_names = std::mem::take(&mut (*info).keycodes.names);
        } else {
            (*keymap).key_names = Vec::new();
        }
        true
    }
}
unsafe fn CopyLedNamesToKeymap(keymap: *mut xkb_keymap, info: *mut KeyNamesInfo) -> bool {
    unsafe {
        (*keymap).num_leds = (*info).num_led_names;
        let mut idx: u32 = 0_u32;
        while idx < (*info).num_led_names {
            let ledi: *mut LedNameInfo = (&raw mut (*info).led_names as *mut LedNameInfo)
                .offset(idx as isize) as *mut LedNameInfo;
            if (*ledi).name != XKB_ATOM_NONE {
                (*keymap).leds[idx as usize].name = (*ledi).name;
            }
            idx = idx.wrapping_add(1);
        }
        true
    }
}
unsafe fn CopyKeyNamesInfoToKeymap(keymap: *mut xkb_keymap, info: *mut KeyNamesInfo) -> bool {
    unsafe {
        if !CopyKeyNamesToKeymap(keymap, info)
            || !CopyKeycodeNameLUT(keymap, info)
            || !CopyLedNamesToKeymap(keymap, info)
        {
            return false;
        }
        if (*keymap).num_keys == 0 || (*keymap).min_key_code > 0_u32 {
            (*keymap).redirect_key_auto = 0_u32;
        } else {
            let mut keycode: u32 = XKB_KEYCODE_INVALID.wrapping_sub(1_u32);
            let mut k: u32 = (*keymap).num_keys;
            loop {
                let c2rust_fresh0 = k;
                k = k.wrapping_sub(1);
                if c2rust_fresh0 <= (*keymap).num_keys_low {
                    break;
                }
                if keycode > (&(*keymap).keys)[k as usize].keycode {
                    break;
                }
                keycode = (&(*keymap).keys)[k as usize].keycode.wrapping_sub(1_u32);
            }
            (*keymap).redirect_key_auto = keycode;
        }
        (*keymap).keycodes_section_name = match &(*info).name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut (*keymap).keycodes_section_name);
        true
    }
}
pub unsafe fn CompileKeycodes(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let mut info: KeyNamesInfo = KeyNamesInfo::new_zeroed();
        InitKeyNamesInfo(&raw mut info, keymap_info, 0_u32);
        if !file.is_null() {
            HandleKeycodesFile(&raw mut info, file);
        }
        if (info.errorCount == 0_i32)
            && CopyKeyNamesInfoToKeymap((*keymap_info).keymap, &raw mut info)
        {
            ClearKeyNamesInfo(&raw mut info);
            return true;
        }
        ClearKeyNamesInfo(&raw mut info);
        false
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
