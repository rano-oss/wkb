use super::prelude::*;
pub use crate::xkb::shared_ast_types::{KeyAliasDef, KeycodeDef, LedNameDef, ReportNotArray};
pub use crate::xkb::shared_types::{XKB_KEYCODE_MAX_CONTIGUOUS, XKB_MAX_LEDS};
use crate::xkb::xkbcomp::expr::ExprResolveInteger;
pub struct KeyNamesInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub keycodes: KeycodeStore,
    pub led_names: [LedNameInfo; 32],
    pub num_led_names: u32,
}
impl KeyNamesInfo {
    pub fn new() -> Self {
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
        }
    }
}
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, Default)]
pub struct HighKeycodeEntry {
    pub keycode: u32,
    pub name: u32,
}
fn vec_resize_zero<T: Default>(v: &mut Vec<T>, new_len: usize) {
    if new_len > v.len() {
        v.resize_with(new_len, Default::default);
    } else if new_len < v.len() {
        v.truncate(new_len);
    }
}

#[inline]
fn keycode_store_update_key(store: &mut KeycodeStore, match_0: KeycodeMatch, name: u32) {
    if !match_0.found || match_0.is_alias {
        return;
    } else if match_0.low {
        store.low[match_0.index as usize] = name;
    } else {
        store.high[match_0.index as usize].name = name;
    }
    if name >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (name as usize).wrapping_add(1));
    }
    store.names[name as usize] = match_0;
}
fn keycode_store_insert_key(store: &mut KeycodeStore, kc: u32, name: u32) -> bool {
    if name >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (name as usize).wrapping_add(1));
    }
    if kc <= XKB_KEYCODE_MAX_CONTIGUOUS as u32 {
        if kc >= store.low.len() as u32 {
            vec_resize_zero(&mut store.low, (kc as usize).wrapping_add(1));
        }
        store.low[kc as usize] = name;
        if kc < store.min {
            store.min = kc;
        }
        store.names[name as usize] = KeycodeMatch {
            found: true,
            low: true,
            is_alias: false,
            index: kc,
        };
    } else {
        let idx: u32 = store.high.len() as u32;
        if idx != 0 && store.high[(idx.wrapping_sub(1_u32)) as usize].keycode > kc {
            let mut lower: u32 = 0_u32;
            let mut upper: u32 = idx;
            while lower < upper {
                let mid: u32 = lower.wrapping_add(
                    upper
                        .wrapping_sub(1_u32)
                        .wrapping_sub(lower)
                        .wrapping_div(2_u32),
                );
                let entry: &HighKeycodeEntry = &store.high[mid as usize];
                if entry.keycode < kc {
                    lower = mid.wrapping_add(1_u32);
                } else if entry.keycode > kc {
                    upper = mid;
                }
            }
            for i in lower as usize..store.high.len() {
                let name_idx = store.high[i].name;
                store.names[name_idx as usize].index += 1;
            }
            store
                .high
                .insert(lower as usize, HighKeycodeEntry { keycode: kc, name });
            store.names[name as usize] = KeycodeMatch {
                found: true,
                low: false,
                is_alias: false,
                index: lower,
            };
        } else {
            store.high.push(HighKeycodeEntry { keycode: kc, name });
            store.names[name as usize] = KeycodeMatch {
                found: true,
                low: false,
                is_alias: false,
                index: idx,
            };
        }
        if store.low.is_empty() {
            store.min = store.high[0].keycode;
        }
    }
    true
}
#[inline]
fn keycode_store_insert_alias(store: &mut KeycodeStore, alias: u32, real: u32) -> bool {
    if alias >= store.names.len() as u32 {
        vec_resize_zero(&mut store.names, (alias as usize).wrapping_add(1));
    }
    store.names[alias as usize] = KeycodeMatch {
        found: true,
        low: true,
        is_alias: real != 0,
        index: real,
    };
    true
}
#[inline]
fn keycode_store_delete_name(store: &mut KeycodeStore, name: u32) {
    if (name as usize) < store.names.len() {
        store.names[name as usize].found = false;
    }
}
fn keycode_store_delete_key(store: &mut KeycodeStore, match_0: KeycodeMatch) {
    if !match_0.found || match_0.is_alias {
        return;
    } else if match_0.low {
        let low_name = store.low[match_0.index as usize];
        store.names[low_name as usize].found = false;
        if match_0.index.wrapping_add(1_u32) == store.low.len() as u32 {
            if store.min == match_0.index {
                store.low.clear();
            } else {
                let mut idx: u32 = match_0.index;
                while idx > 0_u32 {
                    if store.low[(idx.wrapping_sub(1_u32)) as usize] != XKB_ATOM_NONE {
                        store.low.truncate(idx as usize);
                        break;
                    } else {
                        idx = idx.wrapping_sub(1);
                    }
                }
            }
        } else {
            store.low[match_0.index as usize] = XKB_ATOM_NONE;
        }
    } else {
        let high_name = store.high[match_0.index as usize].name;
        store.names[high_name as usize].found = false;
        store.high.remove(match_0.index as usize);
        for entry in store.names.iter_mut() {
            if entry.found
                && !entry.is_alias
                && !entry.low
                && entry.index as i32 > match_0.index as i32
            {
                entry.index -= 1_u32;
            }
        }
    }
    if store.low.is_empty() {
        store.min = if store.high.is_empty() {
            XKB_KEYCODE_INVALID
        } else {
            store.high[0].keycode
        };
    } else {
        let mut kc: u32 = store.min;
        while kc < store.low.len() as u32 {
            if store.low[kc as usize] != XKB_ATOM_NONE {
                store.min = kc;
                break;
            } else {
                kc = kc.wrapping_add(1);
            }
        }
    }
}
fn keycode_store_lookup_keycode(store: &KeycodeStore, kc: u32) -> KeycodeMatch {
    if kc < store.low.len() as u32 {
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
    let mut upper: u32 = store.high.len() as u32;
    while lower < upper {
        let mid: u32 = lower.wrapping_add(
            upper
                .wrapping_sub(1_u32)
                .wrapping_sub(lower)
                .wrapping_div(2_u32),
        );
        let entry: &HighKeycodeEntry = &store.high[mid as usize];
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
fn keycode_store_lookup_name(store: &KeycodeStore, name: u32) -> KeycodeMatch {
    if name >= store.names.len() as u32 {
        KeycodeMatch {
            found: false,
            low: false,
            is_alias: false,
            index: 0,
        }
    } else {
        store.names[name as usize]
    }
}
fn AddLedName(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    _same_file: bool,
    new: &LedNameInfo,
    new_idx: u32,
    report: bool,
) -> bool {
    let replace: bool = new.merge != MERGE_AUGMENT;
    // FindLedByName inlined
    let mut found_old: Option<u32> = None;
    {
        let mut idx: u32 = 0_u32;
        while idx < info.num_led_names {
            if info.led_names[idx as usize].name == new.name {
                found_old = Some(idx);
                break;
            }
            idx = idx.wrapping_add(1);
        }
    }
    if let Some(old_idx) = found_old {
        if old_idx == new_idx {
            if report {
                log::warn!(
                    "Multiple indicators named \"{}\"; Identical definitions ignored\n",
                    xkb_atom_text(&ki.ctx().atom_table, new.name)
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
            log::warn!(
                "Multiple indicators named {}; Using {}, ignoring {}\n",
                xkb_atom_text(&ki.ctx().atom_table, new.name),
                use_0,
                ignore
            );
        }
        if replace {
            info.led_names[old_idx as usize].name = XKB_ATOM_NONE;
        } else {
            return true;
        }
    }
    if new_idx >= info.num_led_names {
        info.num_led_names = new_idx.wrapping_add(1_u32);
    }
    if info.led_names[new_idx as usize].name != XKB_ATOM_NONE {
        if report {
            let use_1: u32 = if replace as i32 != 0 {
                new.name
            } else {
                info.led_names[new_idx as usize].name
            };
            let ignore_0: u32 = if replace as i32 != 0 {
                info.led_names[new_idx as usize].name
            } else {
                new.name
            };
            log::warn!(
                "Multiple names for indicator {}; Using {}, ignoring {}\n",
                new_idx.wrapping_add(1_u32),
                xkb_atom_text(&ki.ctx().atom_table, use_1),
                xkb_atom_text(&ki.ctx().atom_table, ignore_0)
            );
        }
        if replace {
            info.led_names[new_idx as usize] = *new;
        }
        return true;
    }
    info.led_names[new_idx as usize] = *new;
    true
}
fn ClearKeyNamesInfo(info: &mut KeyNamesInfo) {
    info.name = None;
    let store = &mut info.keycodes;
    store.high.clear();
    store.names.clear();
}
fn InitKeyNamesInfo(info: &mut KeyNamesInfo, include_depth: u32) {
    info.name = None;
    info.errorCount = 0;
    info.include_depth = include_depth;
    info.keycodes = KeycodeStore {
        min: XKB_KEYCODE_INVALID,
        low: Vec::new(),
        high: Vec::new(),
        names: Vec::new(),
    };
    info.led_names = [LedNameInfo {
        merge: MERGE_DEFAULT,
        name: 0,
    }; 32];
    info.num_led_names = 0;
}
fn AddKeyName(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    kc: u32,
    name: u32,
    merge: merge_mode,
    report: bool,
) -> bool {
    let match_name: KeycodeMatch = keycode_store_lookup_name(&info.keycodes, name);
    if match_name.found {
        let clobber: bool = merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if report {
                log::warn!("[XKB-{:03}] Key name <{}> already assigned to an alias; Using {}, ignoring {}\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        xkb_atom_text(&ki.ctx().atom_table, name),
                        if clobber { "key" } else { "alias" },
                        if clobber { "alias" } else { "key" });
            }
            if clobber {
                keycode_store_delete_name(&mut info.keycodes, name);
                // dead store removed: match_name.found = false;
            } else {
                return true;
            }
        } else {
            let old_kc: u32 = {
                if !match_name.found || match_name.is_alias as i32 != 0 {
                    XKB_KEYCODE_INVALID
                } else if match_name.low {
                    match_name.index
                } else {
                    info.keycodes.high[match_name.index as usize].keycode
                }
            };
            if old_kc != kc {
                if report {
                    let use_0: u32 = if clobber as i32 != 0 { kc } else { old_kc };
                    let ignore: u32 = if clobber as i32 != 0 { old_kc } else { kc };
                    log::warn!("[XKB-{:03}] Key name <{}> assigned to multiple keys; Using {}, ignoring {}\n",
                            XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                        xkb_atom_text(&ki.ctx().atom_table, name),
                            use_0,
                            ignore);
                }
                if clobber {
                    keycode_store_delete_key(&mut info.keycodes, match_name);
                } else {
                    return true;
                }
            }
        }
    }
    let match_kc: KeycodeMatch = keycode_store_lookup_keycode(&info.keycodes, kc);
    let old_name: u32 = {
        if !match_kc.found || match_kc.is_alias as i32 != 0 {
            XKB_ATOM_NONE
        } else if match_kc.low {
            info.keycodes.low[match_kc.index as usize]
        } else {
            info.keycodes.high[match_kc.index as usize].name
        }
    };
    if old_name != XKB_ATOM_NONE {
        if old_name == name {
            if report {
                log::warn!("Multiple identical key name definitions; Later occurrences of \"<{}> = {}\" ignored\n",
                        xkb_atom_text(&ki.ctx().atom_table, old_name),
                        kc);
            }
            return true;
        }
        let clobber_0: bool = merge != MERGE_AUGMENT;
        if report {
            let kname: &str = xkb_atom_text(&ki.ctx().atom_table, name);
            let old_kname: &str = xkb_atom_text(&ki.ctx().atom_table, old_name);
            let use_1: &str = if clobber_0 { kname } else { old_kname };
            let ignore_0: &str = if clobber_0 { old_kname } else { kname };
            log::warn!(
                "Multiple names for keycode {}; Using <{}>, ignoring <{}>\n",
                kc,
                use_1,
                ignore_0
            );
        }
        if clobber_0 {
            keycode_store_delete_name(&mut info.keycodes, old_name);
            keycode_store_update_key(&mut info.keycodes, match_kc, name);
        }
    } else if !keycode_store_insert_key(&mut info.keycodes, kc, name) {
        log::error!(
            "[XKB-{:03}] Cannot add keycode\n",
            XKB_ERROR_ALLOCATION_ERROR as i32
        );
        return false;
    }
    true
}
fn MergeKeycodeStores(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    merge: merge_mode,
    report: bool,
) {
    if into.keycodes.low.is_empty()
        && into.keycodes.high.is_empty()
        && into.keycodes.names.is_empty()
    {
        into.keycodes = std::mem::replace(
            &mut from.keycodes,
            KeycodeStore {
                min: XKB_KEYCODE_INVALID,
                low: Vec::new(),
                high: Vec::new(),
                names: Vec::new(),
            },
        );
    } else {
        let mut kc: u32 = from.keycodes.min;
        while kc < from.keycodes.low.len() as u32 {
            let name: u32 = (&from.keycodes.low)[kc as usize];
            if (name != XKB_ATOM_NONE) && !AddKeyName(into, ki, kc, name, merge, report) {
                into.errorCount += 1;
            }
            kc = kc.wrapping_add(1);
        }
        for entry in from.keycodes.high.iter() {
            if !AddKeyName(into, ki, entry.keycode, entry.name, merge, report) {
                into.errorCount += 1;
            }
        }
        {
            let names_len = from.keycodes.names.len();
            if names_len > 0 {
                for alias in 0..names_len as u32 {
                    let match_0 = from.keycodes.names[alias as usize];
                    if match_0.found && match_0.is_alias {
                        let def: KeyAliasDef = KeyAliasDef {
                            merge,
                            alias,
                            real: match_0.index,
                        };
                        if !HandleAliasDef(into, ki, &def, report) {
                            into.errorCount += 1;
                        }
                    }
                }
            }
        }
    };
}
fn MergeIncludedKeycodes(
    into: &mut KeyNamesInfo,
    from: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    merge: merge_mode,
    report: bool,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    if into.name.is_none() {
        into.name = from.name.take();
    }
    MergeKeycodeStores(into, from, ki, merge, report);
    if into.num_led_names == 0_u32 {
        into.led_names[..from.num_led_names as usize]
            .copy_from_slice(&from.led_names[..from.num_led_names as usize]);
        into.num_led_names = from.num_led_names;
        from.num_led_names = 0_u32;
    } else {
        let mut idx: u32 = 0_u32;
        while idx < from.num_led_names {
            let ledi = from.led_names[idx as usize];
            if ledi.name != XKB_ATOM_NONE {
                let mut ledi = ledi;
                ledi.merge = merge;
                if !AddLedName(into, ki, false, &ledi, idx, report) {
                    into.errorCount += 1;
                }
            }
            idx = idx.wrapping_add(1);
        }
    };
}
fn HandleIncludeKeycodes(
    info: &mut KeyNamesInfo,
    include: &mut IncludeStmt,
    ki: &mut xkb_keymap_info<'_>,
    report: bool,
) -> bool {
    let mut included = KeyNamesInfo::new();
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitKeyNamesInfo(&mut included, 0_u32);
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(std::mem::take(&mut include.stmt))
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = KeyNamesInfo::new();

        let file: Option<Box<XkbFile>> = ProcessIncludeFile(ki.ctx_mut(), stmt, FILE_TYPE_KEYCODES);
        let Some(mut file) = file else {
            info.errorCount += 10_i32;
            ClearKeyNamesInfo(&mut included);
            return false;
        };
        InitKeyNamesInfo(&mut next_incl, info.include_depth.wrapping_add(1_u32));
        HandleKeycodesFile(&mut next_incl, &mut *file, ki);
        MergeIncludedKeycodes(&mut included, &mut next_incl, ki, stmt.merge, report);
        ClearKeyNamesInfo(&mut next_incl);
        drop(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedKeycodes(info, &mut included, ki, include.merge, report);
    ClearKeyNamesInfo(&mut included);
    info.errorCount == 0_i32
}
fn HandleKeycodeDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    stmt: &KeycodeDef,
    report: bool,
) -> bool {
    if stmt.value < 0_i64 || stmt.value > XKB_KEYCODE_MAX as i64 {
        log::error!(
            "Illegal keycode {}: must be between 0..{}; Key ignored\n",
            stmt.value,
            0xffffffff_u32.wrapping_sub(1_u32)
        );
        return false;
    }
    AddKeyName(info, ki, stmt.value as u32, stmt.name, stmt.merge, report)
}
fn HandleAliasDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &KeyAliasDef,
    report: bool,
) -> bool {
    let match_name: KeycodeMatch =
        keycode_store_lookup_name(&info.keycodes, def.alias) as KeycodeMatch;
    if match_name.found {
        let clobber: bool = def.merge != MERGE_AUGMENT;
        if match_name.is_alias {
            if def.real == match_name.index {
                if report {
                    log::warn!("[XKB-{:03}] Alias of <{}> for <{}> declared more than once; First definition ignored\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                         xkb_atom_text(&ki.ctx().atom_table, def.alias),
                         xkb_atom_text(&ki.ctx().atom_table, def.real));
                }
            } else {
                let use_0: u32 = if clobber as i32 != 0 {
                    def.real
                } else {
                    match_name.index
                };
                let ignore: u32 = if clobber as i32 != 0 {
                    match_name.index
                } else {
                    def.real
                };
                if report {
                    log::warn!("[XKB-{:03}] Multiple definitions for alias <{}>; Using <{}>, ignoring <{}>\n",
                        XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                         xkb_atom_text(&ki.ctx().atom_table, def.alias),
                        xkb_atom_text(&ki.ctx().atom_table, use_0),
                        xkb_atom_text(&ki.ctx().atom_table, ignore));
                }
                {
                    info.keycodes.names[def.alias as usize].index = use_0;
                }
            }
            return true;
        } else {
            if report {
                log::warn!("[XKB-{:03}] Alias name <{}> already assigned to a real key; Using {}, ignoring {}\n",
                    XKB_WARNING_CONFLICTING_KEY_NAME as i32,
                    xkb_atom_text(&ki.ctx().atom_table, def.alias),
                    if clobber { "alias" } else { "key" },
                    if clobber { "key" } else { "alias" });
            }
            if clobber {
                keycode_store_delete_key(&mut info.keycodes, match_name);
            } else {
                return true;
            }
        }
    }
    keycode_store_insert_alias(&mut info.keycodes, def.alias, def.real)
}
fn HandleKeyNameVar(info: &mut KeyNamesInfo, ki: &mut xkb_keymap_info<'_>, stmt: &VarDef) -> bool {
    let mut elem_atom: u32 = 0;
    let mut field_atom: u32 = 0;
    let mut arrayNdx: Option<&ExprDef> = None;
    let name_ref = stmt.name.as_deref().unwrap();
    if !ExprResolveLhs(name_ref, &mut elem_atom, &mut field_atom, &mut arrayNdx) {
        return false;
    }
    let elem = xkb_atom_text(&ki.ctx().atom_table, elem_atom);
    let field = xkb_atom_text(&ki.ctx().atom_table, field_atom);
    if !elem.is_empty() {
        log::error!("[XKB-{:03}] Cannot set global defaults for \"{}\" element; Assignment to \"{}.{}\" ignored\n",
            XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
            elem,
            elem,
            field);
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if !field.eq_ignore_ascii_case("minimum") && !field.eq_ignore_ascii_case("maximum") {
        log::error!(
            "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
            XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
            field
        );
        return ki.strict & PARSER_NO_UNKNOWN_KEYCODES_GLOBAL_FIELDS == 0;
    }
    if arrayNdx.is_some() {
        ReportNotArray("keycodes", field, "defaults");
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    let mut val: i64 = 0_i64;
    let value_ref = stmt.value.as_deref().unwrap();
    if !ExprResolveInteger(ki.ctx(), value_ref, &mut val) || val < 0_i64 || val > u32::MAX as i64 {
        ReportBadType(
            XKB_ERROR_WRONG_FIELD_TYPE,
            "keycodes",
            field,
            "defaults",
            "integer 0..0xfffffffe",
        );
        return ki.strict & PARSER_NO_FIELD_TYPE_MISMATCH == 0;
    }
    true
}
fn HandleLedNameDef(
    info: &mut KeyNamesInfo,
    ki: &mut xkb_keymap_info<'_>,
    def: &LedNameDef,
    report: bool,
) -> bool {
    if def.ndx < 1_i64 || def.ndx > XKB_MAX_LEDS as i64 {
        info.errorCount += 1;
        log::error!(
            "Illegal indicator index ({}) specified; must be between 1 .. {}; Ignored\n",
            def.ndx,
            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    let mut name: u32 = XKB_ATOM_NONE;
    let name_expr = def.name.as_deref().unwrap();
    if !ExprResolveString(ki.ctx(), name_expr, &mut name) {
        let mut buf: [u8; 20] = [0; 20];
        let buf_len = {
            let mut w = crate::xkb::utils::LogBuf::new(&mut buf[..19]);
            let _ = core::fmt::Write::write_fmt(&mut w, format_args!("{}", def.ndx));
            w.pos
        };
        info.errorCount += 1;
        return ReportBadType(
            XKB_ERROR_WRONG_FIELD_TYPE,
            "indicator",
            "name",
            std::str::from_utf8(&buf[..buf_len]).unwrap_or("?"),
            "string",
        );
    }
    let ledi: LedNameInfo = LedNameInfo {
        merge: def.merge,
        name,
    };
    AddLedName(
        info,
        ki,
        true,
        &ledi,
        (def.ndx as u32).wrapping_sub(1_u32),
        report,
    )
}
fn HandleKeycodesFile(info: &mut KeyNamesInfo, file: &mut XkbFile, ki: &mut xkb_keymap_info<'_>) {
    {
        let mut ok: bool;
        let verbosity: i32 = xkb_context_get_log_verbosity(ki.ctx());
        let report_same_file: bool = verbosity > 0_i32;
        let report_include: bool = verbosity > 2_i32;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = HandleIncludeKeycodes(info, &mut **incl, ki, report_include);
                }
                Statement::Keycode(kc) => {
                    ok = HandleKeycodeDef(info, ki, &**kc, report_same_file);
                }
                Statement::KeyAlias(ka) => {
                    ok = HandleAliasDef(info, ki, &**ka, report_same_file);
                }
                Statement::Var(var) => {
                    ok = HandleKeyNameVar(info, ki, &**var);
                }
                Statement::LedName(ln) => {
                    ok = HandleLedNameDef(info, ki, &**ln, report_same_file);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported keycodes {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if unk.stmt_type == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &unk.name
                    );
                    ok = ki.strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    log::error!(
                        "Keycode files may define key and indicator names only; Ignoring {}\n",
                        stmt_type_to_string(stmt.stmt_type())
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!("Abandoning keycodes file \"{}\"\n", safe_map_name(file));
                break;
            }
        }
    }
}
fn CopyKeyNamesToKeymap(keymap: &mut xkb_keymap, keycodes: &KeycodeStore) -> bool {
    if keycodes.low.is_empty() && keycodes.high.is_empty() {
        keymap.min_key_code = 8_u32;
        keymap.max_key_code = 255_u32;
        keymap.num_keys_low = keymap.max_key_code.wrapping_add(1_u32);
        keymap.num_keys = keymap.num_keys_low;
    } else {
        keymap.min_key_code = keycodes.min;
        keymap.max_key_code = if keycodes.high.is_empty() {
            (keycodes.low.len() as u32).wrapping_sub(1_u32)
        } else {
            (&keycodes.high)[keycodes.high.len().wrapping_sub(1)].keycode
        };
        keymap.num_keys_low = keycodes.low.len() as u32;
        keymap.num_keys = keymap.num_keys_low.wrapping_add(keycodes.high.len() as u32);
    }
    let mut keys: Vec<xkb_key> = (0..keymap.num_keys as usize)
        .map(|_| xkb_key::default())
        .collect();
    let mut kc: u32 = keymap.min_key_code;
    while kc < keymap.num_keys_low {
        keys[kc as usize].keycode = kc;
        kc = kc.wrapping_add(1);
    }
    let mut kc_0: u32 = keycodes.min;
    while kc_0 < keycodes.low.len() as u32 {
        keys[kc_0 as usize].name = (&keycodes.low)[kc_0 as usize];
        kc_0 = kc_0.wrapping_add(1);
    }
    let mut idx: u32 = keymap.num_keys_low;
    for entry in keycodes.high.iter() {
        keys[idx as usize].keycode = entry.keycode;
        keys[idx as usize].name = entry.name;
        idx = idx.wrapping_add(1);
    }
    keymap.keys = keys;
    true
}
fn CopyKeycodeNameLUT(keymap: &mut xkb_keymap, keycodes: &mut KeycodeStore) -> bool {
    let names_len = keycodes.names.len();
    {
        if names_len > 0 {
            for name in 0..names_len as u32 {
                let entry = keycodes.names[name as usize];
                if entry.found {
                    if entry.is_alias {
                        let match_real: KeycodeMatch =
                            keycode_store_lookup_name(keycodes, entry.index);
                        if !match_real.found {
                            log::warn!("[XKB-{:03}] Attempt to alias <{}> to non-existent key <{}>; Ignored\n",
                                XKB_WARNING_UNDEFINED_KEYCODE as i32,
                                xkb_atom_text(&keymap.ctx.atom_table, name),
                                xkb_atom_text(
                                    &keymap.ctx.atom_table,
                                    entry.index
                                ));
                            keycodes.names[name as usize].found = false;
                        }
                    } else if !entry.low {
                        keycodes.names[name as usize].index += keymap.num_keys_low;
                    }
                }
            }
        }
    }
    if names_len > 0 {
        // Move the Vec directly to keymap
        keymap.key_names = std::mem::take(&mut keycodes.names);
    } else {
        keymap.key_names = Vec::new();
    }
    true
}
fn CopyLedNamesToKeymap(
    keymap: &mut xkb_keymap,
    led_names: &[LedNameInfo; 32],
    num_led_names: u32,
) -> bool {
    keymap.num_leds = num_led_names;
    let mut idx: u32 = 0_u32;
    while idx < num_led_names {
        let ledi = &led_names[idx as usize];
        if ledi.name != XKB_ATOM_NONE {
            keymap.leds[idx as usize].name = ledi.name;
        }
        idx = idx.wrapping_add(1);
    }
    true
}
fn CopyKeyNamesInfoToKeymap(info: &mut KeyNamesInfo, ki: &mut xkb_keymap_info<'_>) -> bool {
    let keymap = ki.keymap_mut();
    if !CopyKeyNamesToKeymap(keymap, &info.keycodes)
        || !CopyKeycodeNameLUT(keymap, &mut info.keycodes)
        || !CopyLedNamesToKeymap(keymap, &info.led_names, info.num_led_names)
    {
        return false;
    }
    if keymap.num_keys == 0 || keymap.min_key_code > 0_u32 {
        keymap.redirect_key_auto = 0_u32;
    } else {
        let mut keycode: u32 = XKB_KEYCODE_INVALID.wrapping_sub(1_u32);
        let mut k: u32 = keymap.num_keys;
        loop {
            let c2rust_fresh0 = k;
            k = k.wrapping_sub(1);
            if c2rust_fresh0 <= keymap.num_keys_low {
                break;
            }
            if keycode > (&keymap.keys)[k as usize].keycode {
                break;
            }
            keycode = (&keymap.keys)[k as usize].keycode.wrapping_sub(1_u32);
        }
        keymap.redirect_key_auto = keycode;
    }
    keymap.keycodes_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.keycodes_section_name);
    true
}
pub fn CompileKeycodes(file: Option<&mut XkbFile>, keymap_info: &mut xkb_keymap_info<'_>) -> bool {
    let mut info = KeyNamesInfo::new();
    InitKeyNamesInfo(&mut info, 0_u32);
    if let Some(file) = file {
        HandleKeycodesFile(&mut info, file, keymap_info);
    }
    if (info.errorCount == 0_i32) && CopyKeyNamesInfoToKeymap(&mut info, keymap_info) {
        ClearKeyNamesInfo(&mut info);
        return true;
    }
    ClearKeyNamesInfo(&mut info);
    false
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
