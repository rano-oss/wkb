use super::prelude::*;
use crate::xkb::context::xkb_context_get_buffer;
pub use crate::xkb::shared_ast_types::{InterpDef, LedMapDef, ReportBadField, ReportNotArray};
pub use crate::xkb::shared_types::{
    C2Rust_Unnamed_13, MAX_ACTIONS_PER_LEVEL, MOD_REAL_MASK_ALL, XKB_MAX_LEDS,
};
use crate::xkb::text::{
    ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
    useModMapValueNames, LookupString, ModMaskText, SIMatchText,
};
pub use crate::xkb::xkbcomp::action::{
    ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField,
};
use crate::xkb::xkbcomp::expr::{ExprResolveGroupMask, ExprResolveMask, ExprResolveMod};
#[derive(Clone)]
pub struct CompatInfo {
    pub name: Option<String>,
    pub errorCount: i32,
    pub include_depth: u32,
    pub default_interp: SymInterpInfo,
    pub interps: Vec<SymInterpInfo>,
    pub default_led: LedInfo,
    pub leds: [LedInfo; 32],
    pub num_leds: u32,
    pub default_actions: ActionsInfo,
    pub mods: xkb_mod_set,
    pub keymap_info: *const xkb_keymap_info,
    pub ctx: *mut xkb_context,
}
impl CompatInfo {
    pub fn new_zeroed() -> Self {
        let zeroed_led = LedInfo {
            defined: 0 as led_field,
            merge: MERGE_DEFAULT,
            led: xkb_led {
                name: 0,
                which_groups: 0,
                pending_groups: false,
                groups: 0,
                which_mods: 0 as u32,
                mods: xkb_mods { mods: 0, mask: 0 },
                ctrls: 0 as xkb_action_controls,
            },
        };
        Self {
            name: None,
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
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                    actions: Vec::new(),
                },
            },
            interps: Vec::new(),
            default_led: zeroed_led,
            leds: [zeroed_led; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action {
                    type_0: ACTION_TYPE_NONE,
                }; 21],
            },
            mods: xkb_mod_set {
                mods: [xkb_mod {
                    name: 0,
                    type_0: 0 as u32,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            keymap_info: std::ptr::null(),
            ctx: std::ptr::null_mut(),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LedInfo {
    pub defined: led_field,
    pub merge: merge_mode,
    pub led: xkb_led,
}
pub type led_field = u32;
pub const LED_FIELD_CTRLS: led_field = 4;
pub const LED_FIELD_GROUPS: led_field = 2;
pub const LED_FIELD_MODS: led_field = 1;
// C2Rust_Unnamed_18 removed: replaced by Vec<SymInterpInfo>
#[derive(Clone)]
#[repr(C)]
pub struct SymInterpInfo {
    pub defined: si_field,
    pub merge: merge_mode,
    pub interp: xkb_sym_interpret,
}
pub type si_field = u32;
pub const SI_FIELD_LEVEL_ONE_ONLY: si_field = 8;
pub const SI_FIELD_AUTO_REPEAT: si_field = 4;
pub const SI_FIELD_ACTION: si_field = 2;
pub const SI_FIELD_VIRTUAL_MOD: si_field = 1;
// C2Rust_Unnamed_19 removed: replaced by Vec<xkb_sym_interpret>
pub struct collect {
    pub sym_interprets: Vec<xkb_sym_interpret>,
}
// C2Rust_Unnamed_20 removed: replaced by Vec<xkb_action>
unsafe fn siText(mut si: *mut SymInterpInfo, mut info: *mut CompatInfo) -> &'static [u8] {
    unsafe {
        if si == &raw mut (*info).default_interp {
            return b"default";
        }
        let mut buf: *mut i8 = xkb_context_get_buffer((*info).ctx, 128 as usize);
        let (written, _) = crate::xkb::utils::snprintf_args(
            buf,
            128 as usize,
            format_args!(
                "{}+{}({})",
                crate::xkb::utils::ByteSliceDisplay(KeysymText((*info).ctx, (*si).interp.sym)),
                crate::xkb::utils::ByteSliceDisplay(SIMatchText((*si).interp.match_0)),
                crate::xkb::utils::ByteSliceDisplay(ModMaskText(
                    (*info).ctx,
                    MOD_BOTH,
                    &raw mut (*info).mods,
                    (*si).interp.mods,
                )),
            ),
        );
        return std::slice::from_raw_parts(buf as *const u8, written);
    }
}
#[inline]
unsafe fn ReportSINotArray(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    field: &[u8],
) -> bool {
    unsafe {
        return ReportNotArray(
            (*info).ctx,
            b"symbol interpretation",
            field,
            siText(si, info),
        );
    }
}
#[inline]
unsafe fn ReportSIBadType(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    field: &[u8],
    wanted: &[u8],
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"symbol interpretation",
            field,
            siText(si, info),
            wanted,
        );
    }
}
unsafe fn LEDText(mut info: *mut CompatInfo, mut ledi: *mut LedInfo) -> &'static [u8] {
    unsafe {
        if ledi == &raw mut (*info).default_led {
            return b"default";
        } else {
            return xkb_atom_text_bytes((*info).ctx, (*ledi).led.name);
        };
    }
}
#[inline]
unsafe fn ReportLedBadType(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    field: &[u8],
    wanted: &[u8],
) -> bool {
    unsafe {
        return ReportBadType(
            (*info).ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            b"indicator map",
            field,
            LEDText(info, ledi),
            wanted,
        );
    }
}
#[inline]
unsafe fn ReportLedNotArray(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    field: &[u8],
) -> bool {
    unsafe {
        return ReportNotArray((*info).ctx, b"indicator map", field, LEDText(info, ledi));
    }
}
#[inline]
unsafe fn InitInterp(mut info: *mut SymInterpInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
        (*info).interp.virtual_mod = XKB_MOD_INVALID as u32;
    }
}
#[inline]
unsafe fn InitLED(mut info: *mut LedInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
    }
}
unsafe fn InitCompatInfo(
    mut info: *mut CompatInfo,
    mut keymap_info: *const xkb_keymap_info,
    mut include_depth: u32,
    mut mods: *const xkb_mod_set,
) {
    unsafe {
        std::ptr::write_bytes::<CompatInfo>(info as *mut CompatInfo, 0u8, 1);
        (*info).ctx = &raw mut (*(*keymap_info).keymap).ctx;
        (*info).keymap_info = keymap_info;
        (*info).include_depth = include_depth;
        InitActionsInfo(
            (*keymap_info).keymap,
            &raw mut (*info).default_actions,
        );
        InitVMods(&raw mut (*info).mods, mods, include_depth > 0 as u32);
        InitInterp(&raw mut (*info).default_interp);
        InitLED(&raw mut (*info).default_led);
    }
}
unsafe fn ClearCompatInfo(mut info: *mut CompatInfo) {
    unsafe {
        (*info).name = None;
        (*info).interps.clear();
    }
}
unsafe fn UseNewInterpField(
    mut field: si_field,
    mut old: si_field,
    mut new: si_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut si_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as si_field;
            }
            return clobber;
        }
        return false;
    }
}
unsafe fn MergeInterp(
    mut info: *mut CompatInfo,
    mut old: *mut SymInterpInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        let mut collide: si_field = 0 as si_field;
        if (*new).merge as u32 == MERGE_REPLACE as u32 {
            if report {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple definitions for \"{}\"; Earlier interpretation ignored\n",
                    crate::xkb::utils::ByteSliceDisplay(siText(new, info)),
                );
            }
            *old = (*new).clone();
            return true;
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
            (*old).defined = ((*old).defined as u32 | SI_FIELD_VIRTUAL_MOD as u32) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_ACTION,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            if (*old).interp.num_actions as i32 > 1 as i32 {
                (*old).interp.actions.clear();
            }
            (*old).interp.num_actions = (*new).interp.num_actions;
            if (*new).interp.num_actions as i32 > 1 as i32 {
                (*old).interp.actions = std::mem::take(&mut (*new).interp.actions);
                (*new).interp.action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                (*new).interp.num_actions = 0 as u16;
            } else {
                (*old).interp.action = (*new).interp.action;
            }
            (*old).defined = ((*old).defined as u32 | SI_FIELD_ACTION as u32) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_AUTO_REPEAT,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).interp.repeat = (*new).interp.repeat;
            (*old).defined = ((*old).defined as u32 | SI_FIELD_AUTO_REPEAT as u32) as si_field;
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
            (*old).defined = ((*old).defined as u32 | SI_FIELD_LEVEL_ONE_ONLY as u32) as si_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple interpretations of \"{}\"; Using {} definition for duplicate fields\n",
                crate::xkb::utils::ByteSliceDisplay(siText(old, info)),
                crate::xkb::utils::ByteSliceDisplay(if clobber {
                    b"last" as &[u8]
                } else {
                    b"first"
                }),
            );
        }
        return true;
    }
}
unsafe fn AddInterp(
    mut info: *mut CompatInfo,
    mut new: *mut SymInterpInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        // FindMatchingInterp inlined
        let mut old: *mut SymInterpInfo = std::ptr::null_mut();
        for i in 0..(*info).interps.len() {
            let candidate = (&mut (*info).interps)[i..].as_mut_ptr();
            if (*candidate).interp.sym == (*new).interp.sym
                && (*candidate).interp.mods == (*new).interp.mods
                && (*candidate).interp.match_0 as u32 == (*new).interp.match_0 as u32
            {
                old = candidate;
                break;
            }
        }
        if !old.is_null() {
            return MergeInterp(info, old, new, same_file);
        }
        (*info).interps.push((*new).clone());
        return true;
    }
}
unsafe fn ResolveStateAndPredicate(
    mut expr: *mut ExprDef,
    mut pred_rtrn: *mut u32,
    mut mods_rtrn: *mut u32,
    mut info: *mut CompatInfo,
) -> bool {
    unsafe {
        if expr.is_null() {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        *pred_rtrn = MATCH_EXACTLY;
        if (*expr).common.type_0 as u32 == STMT_EXPR_ACTION_DECL as u32 {
            let pred_txt: &[u8] = xkb_atom_text_bytes((*info).ctx, (*expr).action.name);
            let mut pred: u32 = 0 as u32;
            if !LookupString(
                &raw const symInterpretMatchMaskNames as *const LookupEntry,
                pred_txt,
                &raw mut pred,
            ) || (*expr).action.args.is_null()
                || !(*(*expr).action.args).common.next.is_null()
            {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Illegal modifier predicate \"{}\"; Ignored\n",
                    crate::xkb::utils::ByteSliceDisplay(pred_txt),
                );
                return false;
            }
            *pred_rtrn = pred as u32;
            expr = (*expr).action.args as *mut ExprDef;
        } else if (*expr).common.type_0 as u32 == STMT_EXPR_IDENT as u32 {
            let pred_txt_0: &[u8] = xkb_atom_text_bytes((*info).ctx, (*expr).ident.ident);
            if !pred_txt_0.is_empty() && istreq(pred_txt_0, b"any") as i32 != 0 {
                *pred_rtrn = MATCH_ANY;
                *mods_rtrn = MOD_REAL_MASK_ALL;
                return true;
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
unsafe fn UseNewLEDField(
    mut field: led_field,
    mut old: led_field,
    mut new: led_field,
    mut clobber: bool,
    mut report: bool,
    mut collide: *mut led_field,
) -> bool {
    unsafe {
        if old as u32 & field as u32 == 0 {
            return new as u32 & field as u32 != 0;
        }
        if new as u32 & field as u32 != 0 {
            if report {
                *collide = (*collide as u32 | field as u32) as led_field;
            }
            return clobber;
        }
        return false;
    }
}
unsafe fn MergeLedMap(
    mut info: *mut CompatInfo,
    mut old: *mut LedInfo,
    mut new: *mut LedInfo,
    mut same_file: bool,
) -> bool {
    unsafe {
        let mut collide: led_field = 0 as led_field;
        let clobber: bool = (*new).merge as u32 != MERGE_AUGMENT as u32;
        let verbosity: i32 = xkb_context_get_log_verbosity((*info).ctx) as i32;
        let report: bool = same_file as i32 != 0 && verbosity > 0 as i32 || verbosity > 9 as i32;
        if (*old).led.mods.mods == (*new).led.mods.mods
            && (*old).led.pending_groups as i32 == (*new).led.pending_groups as i32
            && (*old).led.groups == (*new).led.groups
            && (*old).led.ctrls as u32 == (*new).led.ctrls as u32
            && (*old).led.which_mods as u32 == (*new).led.which_mods as u32
            && (*old).led.which_groups as i32 == (*new).led.which_groups as i32
        {
            (*old).defined = ((*old).defined as u32 | (*new).defined as u32) as led_field;
            return true;
        }
        if (*new).merge as u32 == MERGE_REPLACE as u32 {
            if report {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Map for indicator {} redefined; Earlier definition ignored\n",
                    crate::xkb::utils::ByteSliceDisplay(LEDText(info, old)),
                );
            }
            *old = *new;
            return true;
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
            (*old).defined = ((*old).defined as u32 | LED_FIELD_MODS as u32) as led_field;
        }
        if UseNewLEDField(
            LED_FIELD_GROUPS,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            (*old).led.which_groups = (*new).led.which_groups;
            (*old).led.groups = (*new).led.groups;
            (*old).led.pending_groups = (*new).led.pending_groups as bool;
            (*old).defined = ((*old).defined as u32 | LED_FIELD_GROUPS as u32) as led_field;
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
            (*old).defined = ((*old).defined as u32 | LED_FIELD_CTRLS as u32) as led_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Map for indicator {} redefined; Using {} definition for duplicate fields\n",
                crate::xkb::utils::ByteSliceDisplay(LEDText(info, old)),
                crate::xkb::utils::ByteSliceDisplay(if clobber {
                    b"last" as &[u8]
                } else {
                    b"first"
                }),
            );
        }
        return true;
    }
}
unsafe fn AddLedMap(mut info: *mut CompatInfo, mut new: *mut LedInfo, mut same_file: bool) -> bool {
    unsafe {
        let mut i: u32 = 0 as u32;
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
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Too many LEDs defined (maximum {})\n",
                (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8 as usize) as u32,
            );
            return false;
        }
        let c2rust_fresh1 = (*info).num_leds;
        (*info).num_leds = (*info).num_leds.wrapping_add(1);
        (*info).leds[c2rust_fresh1 as usize] = *new;
        return true;
    }
}
unsafe fn MergeIncludedCompatMaps(
    mut into: *mut CompatInfo,
    mut from: *mut CompatInfo,
    mut merge: merge_mode,
) {
    unsafe {
        if (*from).errorCount > 0 as i32 {
            (*into).errorCount += (*from).errorCount;
            return;
        }
        MergeModSets(
            (*into).ctx,
            &raw mut (*into).mods,
            &raw mut (*from).mods,
            merge,
        );
        if (*into).name.is_none() {
            (*into).name = (*from).name.take();
        }
        if (*into).interps.is_empty() {
            (*into).interps = std::mem::take(&mut (*from).interps);
        } else {
            for i in 0..(*from).interps.len() {
                (&mut (*from).interps)[i].merge = merge;
                let si = (&mut (*from).interps)[i..].as_mut_ptr();
                if !AddInterp(into, si, false) {
                    (*into).errorCount += 1;
                }
            }
        }
        if (*into).num_leds == 0 as u32 {
            std::ptr::copy_nonoverlapping::<LedInfo>(
                &raw mut (*from).leds as *mut LedInfo,
                &raw mut (*into).leds as *mut LedInfo,
                (*from).num_leds as usize,
            );
            (*into).num_leds = (*from).num_leds;
            (*from).num_leds = 0 as u32;
        } else {
            let mut i: u32 = 0 as u32;
            while i < (*from).num_leds {
                let mut ledi: *mut LedInfo =
                    (&raw mut (*from).leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
                (*ledi).merge = merge;
                if !AddLedMap(into, ledi, false) {
                    (*into).errorCount += 1;
                }
                i = i.wrapping_add(1);
            }
        };
    }
}
unsafe fn HandleIncludeCompatMap(mut info: *mut CompatInfo, mut include: *mut IncludeStmt) -> bool {
    unsafe {
        let mut included: CompatInfo = CompatInfo::new_zeroed();
        if ExceedsIncludeMaxDepth((*info).ctx, (*info).include_depth) {
            (*info).errorCount += 10 as i32;
            return false;
        }
        InitCompatInfo(
            &raw mut included,
            (*info).keymap_info,
            (*info).include_depth.wrapping_add(1 as u32),
            &raw mut (*info).mods,
        );
        included.name = if (*include).stmt.is_null() {
            None
        } else {
            Some(String::from(
                std::ffi::CStr::from_ptr((*include).stmt)
                    .to_str()
                    .unwrap_or(""),
            ))
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl: CompatInfo = CompatInfo::new_zeroed();
            let mut file: *mut XkbFile = std::ptr::null_mut();
            let mut path: [i8; 4096] = [0; 4096];
            file = ProcessIncludeFile(
                (*info).ctx,
                stmt,
                FILE_TYPE_COMPAT,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                (*info).errorCount += 10 as i32;
                ClearCompatInfo(&raw mut included);
                return false;
            }
            InitCompatInfo(
                &raw mut next_incl,
                (*info).keymap_info,
                (*info).include_depth.wrapping_add(1 as u32),
                &raw mut included.mods,
            );
            next_incl.default_interp = (*info).default_interp.clone();
            next_incl.default_led = (*info).default_led;
            HandleCompatMapFile(&raw mut next_incl, file);
            MergeIncludedCompatMaps(&raw mut included, &raw mut next_incl, (*stmt).merge);
            ClearCompatInfo(&raw mut next_incl);
            FreeXkbFile(file);
            stmt = (*stmt).next_incl as *mut IncludeStmt;
        }
        MergeIncludedCompatMaps(info, &raw mut included, (*include).merge);
        ClearCompatInfo(&raw mut included);
        return (*info).errorCount == 0 as i32;
    }
}
unsafe fn SetInterpField(
    mut info: *mut CompatInfo,
    mut si: *mut SymInterpInfo,
    field: &[u8],
    mut arrayNdx: *mut ExprDef,
    mut value: *mut ExprDef,
) -> bool {
    unsafe {
        if istreq(field, b"action") {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if (*value).common.type_0 as u32 == STMT_EXPR_ACTION_LIST as u32 {
                let mut num_actions: u32 = 0 as u32;
                let mut act: *mut ExprDef = (*value).actions.actions as *mut ExprDef;
                while !act.is_null() {
                    num_actions = num_actions.wrapping_add(1);
                    act = (*act).common.next as *mut ExprDef;
                }
                if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Interpret {} has too many actions; expected max {}, got: {}\n",
                        crate::xkb::utils::ByteSliceDisplay(siText(si, info)),
                        65535 as i32,
                        num_actions,
                    );
                    return false;
                }
                (*si).interp.num_actions = 0 as u16;
                (*si).interp.action.type_0 = ACTION_TYPE_NONE;
                let mut actions: Vec<xkb_action> = Vec::new();
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
                    ) as u32
                    {
                        1 => {
                            toAct.type_0 = ACTION_TYPE_NONE;
                        }
                        2 => {
                            drop(actions);
                            return false;
                        }
                        _ => {}
                    }
                    if !(toAct.type_0 as u32 == ACTION_TYPE_NONE as u32) {
                        if (num_actions == 1 as u32) as i64 != 0 {
                            (*si).interp.num_actions = 1 as u16;
                            (*si).interp.action = toAct;
                        } else {
                            actions.push(toAct);
                        }
                    }
                    act_0 = (*act_0).common.next as *mut ExprDef;
                }
                match actions.len() as u32 {
                    0 => {}
                    1 => {
                        (*si).interp.num_actions = 1 as u16;
                        (*si).interp.action = actions[1];
                    }
                    _ => {
                        (*si).interp.num_actions = actions.len() as u16;
                        (*si).interp.actions = actions;
                    }
                }
            } else {
                match HandleActionDef(
                    (*info).keymap_info,
                    &raw mut (*info).default_actions,
                    &raw mut (*info).mods,
                    value,
                    &raw mut (*si).interp.action,
                ) as u32
                {
                    1 => {
                        (*si).interp.action.type_0 = ACTION_TYPE_NONE;
                        (*si).interp.num_actions = 0 as u16;
                    }
                    2 => return false,
                    _ => {
                        (*si).interp.num_actions = ((*si).interp.action.type_0 as u32
                            != ACTION_TYPE_NONE as u32)
                            as i32 as u16;
                    }
                }
            }
            (*si).defined = ((*si).defined as u32 | SI_FIELD_ACTION as u32) as si_field;
        } else if istreq(field, b"virtualmodifier") as i32 != 0
            || istreq(field, b"virtualmod") as i32 != 0
        {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            let mut ndx: u32 = 0 as u32;
            if !ExprResolveMod(
                (*info).ctx,
                value,
                MOD_VIRT,
                &raw mut (*info).mods,
                &raw mut ndx,
            ) {
                return ReportSIBadType(info, si, field, b"virtual modifier");
            }
            (*si).interp.virtual_mod = ndx;
            (*si).defined = ((*si).defined as u32 | SI_FIELD_VIRTUAL_MOD as u32) as si_field;
        } else if istreq(field, b"repeat") {
            let mut set: bool = false;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveBoolean((*info).ctx, value, &raw mut set) {
                return ReportSIBadType(info, si, field, b"boolean");
            }
            (*si).interp.repeat = set;
            (*si).defined = ((*si).defined as u32 | SI_FIELD_AUTO_REPEAT as u32) as si_field;
        } else if istreq(field, b"locking") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"locking\" field in symbol interpretation is unsupported; Ignored\n",
            );
        } else if istreq(field, b"usemodmap") as i32 != 0
            || istreq(field, b"usemodmapmods") as i32 != 0
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
                return ReportSIBadType(info, si, field, b"level specification");
            }
            (*si).interp.level_one_only = val != 0;
            (*si).defined = ((*si).defined as u32 | SI_FIELD_LEVEL_ONE_ONLY as u32) as si_field;
        } else {
            ReportBadField(
                (*info).ctx,
                b"symbol interpretation",
                field,
                siText(si, info),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_INTERPRET_FIELDS as u32
                == 0;
        }
        return true;
    }
}
unsafe fn SetLedMapField(
    mut info: *mut CompatInfo,
    mut ledi: *mut LedInfo,
    field: &[u8],
    mut arrayNdx: *mut ExprDef,
    mut value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if istreq(field, b"modifiers") as i32 != 0 || istreq(field, b"mods") as i32 != 0 {
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
                return ReportLedBadType(info, ledi, field, b"modifier mask");
            }
            (*ledi).defined = ((*ledi).defined as u32 | LED_FIELD_MODS as u32) as led_field;
        } else if istreq(field, b"groups") {
            let mut mask: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let mut pending: bool = false;
            if !ExprResolveGroupMask((*info).keymap_info, value, &raw mut mask, &raw mut pending) {
                if pending {
                    (*ledi).led.pending_groups = (true) as bool;
                    let pending_index: u32 =
                        (&*(*(*info).keymap_info).pending_computations).len() as u32;
                    (&mut *(*(*info).keymap_info).pending_computations).push(pending_computation {
                        expr: *value_ptr,
                        computed: false,
                        value: 0 as u32,
                    });
                    *value_ptr = std::ptr::null_mut();
                    mask = pending_index as u32;
                } else {
                    return ReportLedBadType(info, ledi, field, b"group mask");
                }
            } else {
                (*ledi).led.pending_groups = (false) as bool;
            }
            (*ledi).led.groups = mask;
            (*ledi).defined = ((*ledi).defined as u32 | LED_FIELD_GROUPS as u32) as led_field;
        } else if istreq(field, b"controls") as i32 != 0 || istreq(field, b"ctrls") as i32 != 0 {
            let mut mask_0: u32 = 0 as u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let offset: u8 = (*(*info).keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                (*info).ctx,
                value,
                &raw mut mask_0,
                (&raw const ctrlMaskNames as *const LookupEntry).offset(offset as i32 as isize),
            ) {
                return ReportLedBadType(info, ledi, field, b"controls mask");
            }
            (*ledi).led.ctrls = mask_0 as xkb_action_controls;
            (*ledi).defined = ((*ledi).defined as u32 | LED_FIELD_CTRLS as u32) as led_field;
        } else if istreq(field, b"allowexplicit") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n",
            );
        } else if istreq(field, b"whichmodstate") as i32 != 0
            || istreq(field, b"whichmodifierstate") as i32 != 0
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
                return ReportLedBadType(info, ledi, field, b"mask of modifier state components");
            }
            (*ledi).led.which_mods = mask_1 as u32;
        } else if istreq(field, b"whichgroupstate") {
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
                return ReportLedBadType(info, ledi, field, b"mask of group state components");
            }
            (*ledi).led.which_groups = mask_2 as u32;
        } else if istreq(field, b"driveskbd") as i32 != 0
            || istreq(field, b"driveskeyboard") as i32 != 0
            || istreq(field, b"leddriveskbd") as i32 != 0
            || istreq(field, b"leddriveskeyboard") as i32 != 0
            || istreq(field, b"indicatordriveskbd") as i32 != 0
            || istreq(field, b"indicatordriveskeyboard") as i32 != 0
        {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"{}\" field in indicator statements is unsupported; Ignored\n",
                crate::xkb::utils::ByteSliceDisplay(field),
            );
        } else if istreq(field, b"index") {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"index\" field in indicator statements is unsupported; Ignored\n",
            );
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Unknown field \"{}\" in map for {} indicator; Definition ignored\n",
                crate::xkb::utils::ByteSliceDisplay(field),
                crate::xkb::utils::ByteSliceDisplay(LEDText(info, ledi)),
            );
            return (*(*info).keymap_info).strict as u32 & PARSER_NO_UNKNOWN_LED_FIELDS as u32 == 0;
        }
        return true;
    }
}
unsafe fn HandleGlobalVar(mut info: *mut CompatInfo, mut stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &[u8] = b"";
        let mut field: &[u8] = b"";
        let mut ndx: *mut ExprDef = std::ptr::null_mut();
        let mut ret: bool = false;
        if !ExprResolveLhs(
            (*info).ctx,
            (*stmt).name,
            &mut elem,
            &mut field,
            &raw mut ndx,
        ) {
            ret = false;
        } else if !elem.is_empty() && istreq(elem, b"interpret") as i32 != 0 {
            let mut temp: SymInterpInfo = SymInterpInfo {
                defined: 0 as si_field,
                merge: MERGE_DEFAULT,
                interp: xkb_sym_interpret {
                    sym: 0,
                    match_0: MATCH_NONE,
                    mods: 0,
                    virtual_mod: 0,
                    level_one_only: false,
                    repeat: false,
                    required: false,
                    num_actions: 0,
                    action: xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    },
                    actions: Vec::new(),
                },
            };
            InitInterp(&raw mut temp);
            temp.merge = (if temp.merge as u32 == MERGE_REPLACE as u32 {
                MERGE_OVERRIDE as u32
            } else {
                (*stmt).merge as u32
            }) as merge_mode;
            ret = SetInterpField(
                info,
                &raw mut temp,
                field,
                ndx,
                (*stmt).value as *mut ExprDef,
            );
            if ret {
                MergeInterp(info, &raw mut (*info).default_interp, &raw mut temp, true);
            }
        } else if !elem.is_empty() && istreq(elem, b"indicator") as i32 != 0 {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups: 0,
                    pending_groups: false,
                    groups: 0,
                    which_mods: 0 as u32,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            };
            InitLED(&raw mut temp_0);
            temp_0.merge = (if temp_0.merge as u32 == MERGE_REPLACE as u32 {
                MERGE_OVERRIDE as u32
            } else {
                (*stmt).merge as u32
            }) as merge_mode;
            ret = SetLedMapField(info, &raw mut temp_0, field, ndx, &raw mut (*stmt).value);
            if ret {
                MergeLedMap(info, &raw mut (*info).default_led, &raw mut temp_0, true);
            }
        } else if !elem.is_empty() {
            ret = SetDefaultActionField(
                (*info).keymap_info,
                &raw mut (*info).default_actions,
                &raw mut (*info).mods,
                elem,
                field,
                ndx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as u32
                != PARSER_FATAL_ERROR as u32;
        } else {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                crate::xkb::utils::ByteSliceDisplay(field),
            );
            return (*(*info).keymap_info).strict as u32
                & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS as u32
                == 0;
        }
        return ret;
    }
}
unsafe fn HandleInterpBody(
    mut info: *mut CompatInfo,
    mut def: *mut VarDef,
    mut si: *mut SymInterpInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true;
        let mut elem: &[u8] = b"";
        let mut field: &[u8] = b"";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        while !def.is_null() {
            if !ExprResolveLhs(
                (*info).ctx,
                (*def).name,
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set a global default value for \"{}\" element from within an interpret statement; Move assignment to \"{}.{}\" to the global file scope\n",
                    crate::xkb::utils::ByteSliceDisplay(elem),
                    crate::xkb::utils::ByteSliceDisplay(elem),
                    crate::xkb::utils::ByteSliceDisplay(field),
                );
                ok = false;
            } else if !SetInterpField(info, si, field, arrayNdx, (*def).value as *mut ExprDef) {
                ok = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        return ok;
    }
}
unsafe fn HandleInterpDef(mut info: *mut CompatInfo, mut def: *mut InterpDef) -> bool {
    unsafe {
        let mut pred: u32 = MATCH_NONE;
        let mut mods: u32 = 0;
        let mut si: SymInterpInfo = SymInterpInfo {
            defined: 0 as si_field,
            merge: MERGE_DEFAULT,
            interp: xkb_sym_interpret {
                sym: 0,
                match_0: MATCH_NONE,
                mods: 0,
                virtual_mod: 0,
                level_one_only: false,
                repeat: false,
                required: false,
                num_actions: 0,
                action: xkb_action {
                    type_0: ACTION_TYPE_NONE,
                },
                actions: Vec::new(),
            },
        };
        if !ResolveStateAndPredicate(
            (*def).match_0 as *mut ExprDef,
            &raw mut pred,
            &raw mut mods,
            info,
        ) {
            xkb_logf!(
                (*info).ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Couldn't determine matching modifiers; Symbol interpretation ignored\n",
            );
            return false;
        }
        si = (*info).default_interp.clone();
        si.merge = (*def).merge;
        si.interp.sym = (*def).sym;
        si.interp.match_0 = pred;
        si.interp.mods = mods;
        if !HandleInterpBody(info, (*def).def, &raw mut si) {
            (*info).errorCount += 1;
            return false;
        }
        if !AddInterp(info, &raw mut si, true) {
            (*info).errorCount += 1;
            return false;
        }
        return true;
    }
}
unsafe fn HandleLedMapDef(mut info: *mut CompatInfo, mut def: *mut LedMapDef) -> bool {
    unsafe {
        let mut ledi: LedInfo = (*info).default_led;
        ledi.merge = (*def).merge;
        ledi.led.name = (*def).name;
        let mut ok: bool = true;
        let mut var: *mut VarDef = (*def).body;
        while !var.is_null() {
            let mut elem: &[u8] = b"";
            let mut field: &[u8] = b"";
            let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
            if !ExprResolveLhs(
                (*info).ctx,
                (*var).name,
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                xkb_logf!(
                    (*info).ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot set defaults for \"{}\" element in indicator map; Assignment to {}.{} ignored\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    crate::xkb::utils::ByteSliceDisplay(elem),
                    crate::xkb::utils::ByteSliceDisplay(elem),
                    crate::xkb::utils::ByteSliceDisplay(field),
                );
                ok = false;
            } else if !SetLedMapField(info, &raw mut ledi, field, arrayNdx, &raw mut (*var).value) {
                ok = false;
            }
            var = (*var).common.next as *mut VarDef;
        }
        return ok as i32 != 0 && AddLedMap(info, &raw mut ledi, true) as i32 != 0;
    }
}
unsafe fn HandleCompatMapFile(mut info: *mut CompatInfo, mut file: *mut XkbFile) {
    unsafe {
        let mut ok: bool = false;
        (*info).name = if (*file).name.is_null() {
            None
        } else {
            Some(String::from(
                std::ffi::CStr::from_ptr((*file).name)
                    .to_str()
                    .unwrap_or(""),
            ))
        };
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 as u32 {
                1 => {
                    ok = HandleIncludeCompatMap(info, stmt as *mut IncludeStmt);
                }
                28 => {
                    ok = HandleInterpDef(info, stmt as *mut InterpDef);
                }
                32 => {
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_DEBUG,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "The \"group\" statement in compat is unsupported; Ignored\n",
                    );
                    ok = true;
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
                    xkb_logf!(
                        (*info).ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported compatibility {} statement \"{}\"; Ignoring\n",
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
                        "Compat files may not include other types; Ignoring {}\n",
                        stmt_type_to_string((*stmt).type_0),
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
                    "Abandoning compatibility map \"{}\"\n",
                    crate::xkb::utils::CStrDisplay(safe_map_name(file)),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
unsafe fn CopyInterps(
    mut info: *mut CompatInfo,
    mut needSymbol: bool,
    mut pred: u32,
    mut collect: *mut collect,
) {
    unsafe {
        for i in 0..(*info).interps.len() {
            let si = &(&(*info).interps)[i];
            if si.interp.match_0 as u32 == pred as u32
                && (si.interp.sym != XKB_KEY_NoSymbol as u32) as i32 == needSymbol as i32
            {
                (*collect).sym_interprets.push(si.interp.clone());
            }
        }
    }
}
unsafe fn CopyLedMapDefsToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut CompatInfo) {
    unsafe {
        let mut c2rust_current_block_11: u64;
        let mut idx: u32 = 0 as u32;
        while idx < (*info).num_leds {
            let mut ledi: *mut LedInfo =
                (&raw mut (*info).leds as *mut LedInfo).offset(idx as isize) as *mut LedInfo;
            let mut i: u32 = 0;
            let mut led: *mut xkb_led = std::ptr::null_mut();
            i = 0 as u32;
            led = &raw mut (*keymap).leds as *mut xkb_led;
            while i < (*keymap).num_leds {
                if (*led).name == (*ledi).led.name {
                    break;
                }
                i = i.wrapping_add(1);
                led = led.offset(1);
            }
            if i >= (*keymap).num_leds {
                xkb_logf!(
                    (*keymap).ctx,
                    XKB_LOG_LEVEL_DEBUG,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Indicator name \"{}\" was not declared in the keycodes section; Adding new indicator\n",
                    crate::xkb::utils::ByteSliceDisplay(LEDText(info, ledi)),
                );
                i = 0 as u32;
                led = &raw mut (*keymap).leds as *mut xkb_led;
                while i < (*keymap).num_leds {
                    if (*led).name == XKB_ATOM_NONE as u32 {
                        break;
                    }
                    i = i.wrapping_add(1);
                    led = led.offset(1);
                }
                if i >= (*keymap).num_leds {
                    if i >= XKB_MAX_LEDS {
                        xkb_logf!(
                            (*keymap).ctx,
                            XKB_LOG_LEVEL_ERROR,
                            XKB_LOG_VERBOSITY_MINIMAL as i32,
                            "Too many indicators (maximum is {}); Indicator name \"{}\" ignored\n",
                            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8 as usize) as u32,
                            crate::xkb::utils::ByteSliceDisplay(LEDText(info, ledi)),
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
                    if (*led).which_groups as i32 == 0 as i32
                        && ((*led).groups != 0 as u32 || (*led).pending_groups as i32 != 0)
                    {
                        (*led).which_groups = XKB_STATE_LAYOUT_EFFECTIVE as u32;
                    }
                    if (*led).which_mods as u32 == 0 as u32 && (*led).mods.mods != 0 as u32 {
                        (*led).which_mods = XKB_STATE_MODS_EFFECTIVE;
                    }
                }
                _ => {}
            }
            idx = idx.wrapping_add(1);
        }
    }
}
unsafe fn CopyCompatToKeymap(mut keymap: *mut xkb_keymap, mut info: *mut CompatInfo) -> bool {
    unsafe {
        (*keymap).compat_section_name = match &(*info).name {
            Some(s) => {
                let cs = std::ffi::CString::new(s.as_str()).unwrap();
                strdup_safe(cs.as_ptr())
            }
            None => std::ptr::null_mut(),
        };
        XkbEscapeMapName((*keymap).compat_section_name);
        (*keymap).mods = (*info).mods;
        if !(*info).interps.is_empty() {
            let mut collect: collect = collect {
                sym_interprets: Vec::new(),
            };
            CopyInterps(info, true, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, true, MATCH_ALL, &raw mut collect);
            CopyInterps(info, true, MATCH_NONE, &raw mut collect);
            CopyInterps(info, true, MATCH_ANY, &raw mut collect);
            CopyInterps(info, true, MATCH_ANY_OR_NONE, &raw mut collect);
            CopyInterps(info, false, MATCH_EXACTLY, &raw mut collect);
            CopyInterps(info, false, MATCH_ALL, &raw mut collect);
            CopyInterps(info, false, MATCH_NONE, &raw mut collect);
            CopyInterps(info, false, MATCH_ANY, &raw mut collect);
            CopyInterps(info, false, MATCH_ANY_OR_NONE, &raw mut collect);
            if collect.sym_interprets.is_empty() {
                (*keymap).sym_interprets = std::ptr::null_mut();
                *&raw mut (*keymap).num_sym_interprets = 0 as u32;
            } else {
                collect.sym_interprets.shrink_to_fit();
                *&raw mut (*keymap).num_sym_interprets = collect.sym_interprets.len() as u32;
                (*keymap).sym_interprets = collect.sym_interprets.as_mut_ptr();
                std::mem::forget(collect.sym_interprets);
            }
        }
        CopyLedMapDefsToKeymap(keymap, info);
        return true;
    }
}
pub unsafe fn CompileCompatMap(
    mut file: *mut XkbFile,
    mut keymap_info: *mut xkb_keymap_info,
) -> bool {
    unsafe {
        let mut info: CompatInfo = CompatInfo::new_zeroed();
        InitCompatInfo(
            &raw mut info,
            keymap_info,
            0 as u32,
            &raw mut (*(*keymap_info).keymap).mods,
        );
        if !file.is_null() {
            HandleCompatMapFile(&raw mut info, file);
        }
        if !(info.errorCount != 0 as i32) {
            if CopyCompatToKeymap((*keymap_info).keymap, &raw mut info) {
                ClearCompatInfo(&raw mut info);
                return true;
            }
        }
        ClearCompatInfo(&raw mut info);
        return false;
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
