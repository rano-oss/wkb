use super::prelude::*;
use crate::xkb::context::xkb_context_get_buffer;
pub use crate::xkb::shared_ast_types::{InterpDef, LedMapDef, ReportBadField, ReportNotArray};
pub use crate::xkb::shared_types::{MAX_ACTIONS_PER_LEVEL, MOD_REAL_MASK_ALL, XKB_MAX_LEDS};
use crate::xkb::text::{
    ctrlMaskNames, groupComponentMaskNames, modComponentMaskNames, symInterpretMatchMaskNames,
    useModMapValueNames, LookupString, ModMaskText, SIMatchText,
};
pub use crate::xkb::xkbcomp::action::{
    ActionsInfo, HandleActionDef, InitActionsInfo, SetDefaultActionField,
};
use crate::xkb::xkbcomp::expr::{ExprResolveGroupMask, ExprResolveMask, ExprResolveMod};
pub struct CompatInfo<'a> {
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
    pub keymap_info: &'a mut xkb_keymap_info,
    pub ctx: &'a mut xkb_context,
}
impl<'a> CompatInfo<'a> {
    #[inline]
    pub fn ctx(&self) -> &xkb_context {
        &*self.ctx
    }
    pub fn new(ctx: &'a mut xkb_context, keymap_info: &'a mut xkb_keymap_info) -> Self {
        let zeroed_led = LedInfo {
            defined: 0 as led_field,
            merge: MERGE_DEFAULT,
            led: xkb_led {
                name: 0,
                which_groups: 0,
                pending_groups: false,
                groups: 0,
                which_mods: 0_u32,
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
                    type_0: 0_u32,
                    mapping: 0,
                }; 32],
                num_mods: 0,
                explicit_vmods: 0,
            },
            keymap_info,
            ctx,
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
fn siText(si: *mut SymInterpInfo, info: &mut CompatInfo<'_>) -> &'static str {
    unsafe {
        if si == &raw mut info.default_interp {
            return "default";
        }
        let buf: *mut i8 = xkb_context_get_buffer(&mut (*info.ctx).clone(), 128_usize);
        let (written, _) = crate::xkb::utils::snprintf_args(
            buf,
            128_usize,
            format_args!(
                "{}+{}({})",
                KeysymText((*si).interp.sym),
                SIMatchText((*si).interp.match_0),
                ModMaskText(info.ctx(), MOD_BOTH, &info.mods, (*si).interp.mods),
            ),
        );
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(buf as *const u8, written))
    }
}
#[inline]
fn ReportSINotArray(info: &mut CompatInfo<'_>, si: *mut SymInterpInfo, field: &str) -> bool {
    unsafe { ReportNotArray(info.ctx, "symbol interpretation", field, siText(si, info)) }
}
#[inline]
fn ReportSIBadType(
    info: &mut CompatInfo<'_>,
    si: *mut SymInterpInfo,
    field: &str,
    wanted: &str,
) -> bool {
    unsafe {
        ReportBadType(
            info.ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            "symbol interpretation",
            field,
            siText(si, info),
            wanted,
        )
    }
}
fn LEDText(info: &mut CompatInfo<'_>, ledi: *mut LedInfo) -> &'static str {
    unsafe {
        if ledi == &raw mut info.default_led {
            "default"
        } else {
            let ctx_ptr = info.ctx as *const xkb_context;
            xkb_atom_text(&(*ctx_ptr).atom_table, (*ledi).led.name)
        }
    }
}
#[inline]
fn ReportLedBadType(
    info: &mut CompatInfo<'_>,
    ledi: *mut LedInfo,
    field: &str,
    wanted: &str,
) -> bool {
    unsafe {
        ReportBadType(
            info.ctx,
            XKB_ERROR_WRONG_FIELD_TYPE,
            "indicator map",
            field,
            LEDText(info, ledi),
            wanted,
        )
    }
}
#[inline]
fn ReportLedNotArray(info: &mut CompatInfo<'_>, ledi: *mut LedInfo, field: &str) -> bool {
    unsafe { ReportNotArray(info.ctx, "indicator map", field, LEDText(info, ledi)) }
}
#[inline]
fn InitInterp(info: *mut SymInterpInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
        (*info).interp.virtual_mod = XKB_MOD_INVALID;
    }
}
#[inline]
fn InitLED(info: *mut LedInfo) {
    unsafe {
        (*info).merge = MERGE_DEFAULT;
    }
}
fn InitCompatInfo(info: &mut CompatInfo<'_>, include_depth: u32, mods: *const xkb_mod_set) {
    unsafe {
        info.include_depth = include_depth;
        InitActionsInfo((*info.keymap_info).keymap, &raw mut info.default_actions);
        InitVMods(&mut info.mods, &*mods, include_depth > 0_u32);
        InitInterp(&raw mut info.default_interp);
        InitLED(&raw mut info.default_led);
    }
}
fn ClearCompatInfo(info: &mut CompatInfo<'_>) {
    info.name = None;
    info.interps.clear();
}
fn UseNewInterpField(
    field: si_field,
    old: si_field,
    new: si_field,
    clobber: bool,
    report: bool,
    collide: *mut si_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            unsafe { *collide = (*collide | field) as si_field };
        }
        return clobber;
    }
    false
}
fn MergeInterp(
    info: &mut CompatInfo<'_>,
    old: *mut SymInterpInfo,
    new: *mut SymInterpInfo,
    same_file: bool,
) -> bool {
    unsafe {
        let clobber: bool = (*new).merge != MERGE_AUGMENT;
        let verbosity: i32 = xkb_context_get_log_verbosity(info.ctx());
        let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
        let mut collide: si_field = 0 as si_field;
        if (*new).merge == MERGE_REPLACE {
            if report {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Multiple definitions for \"{}\"; Earlier interpretation ignored\n",
                    siText(new, info),
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
            (*old).defined = ((*old).defined | SI_FIELD_VIRTUAL_MOD) as si_field;
        }
        if UseNewInterpField(
            SI_FIELD_ACTION,
            (*old).defined,
            (*new).defined,
            clobber,
            report,
            &raw mut collide,
        ) {
            if (*old).interp.num_actions as i32 > 1_i32 {
                (*old).interp.actions.clear();
            }
            (*old).interp.num_actions = (*new).interp.num_actions;
            if (*new).interp.num_actions as i32 > 1_i32 {
                (*old).interp.actions = std::mem::take(&mut (*new).interp.actions);
                (*new).interp.action = xkb_action {
                    type_0: ACTION_TYPE_NONE,
                };
                (*new).interp.num_actions = 0_u16;
            } else {
                (*old).interp.action = (*new).interp.action;
            }
            (*old).defined = ((*old).defined | SI_FIELD_ACTION) as si_field;
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
            (*old).defined = ((*old).defined | SI_FIELD_AUTO_REPEAT) as si_field;
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
            (*old).defined = ((*old).defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Multiple interpretations of \"{}\"; Using {} definition for duplicate fields\n",
                siText(old, info),
                if clobber { "last" } else { "first" },
            );
        }
        true
    }
}
fn AddInterp(info: &mut CompatInfo<'_>, new: *mut SymInterpInfo, same_file: bool) -> bool {
    unsafe {
        // FindMatchingInterp inlined
        let mut old: *mut SymInterpInfo = std::ptr::null_mut();
        for i in 0..info.interps.len() {
            let candidate = (&mut info.interps)[i..].as_mut_ptr();
            if (*candidate).interp.sym == (*new).interp.sym
                && (*candidate).interp.mods == (*new).interp.mods
                && (*candidate).interp.match_0 == (*new).interp.match_0
            {
                old = candidate;
                break;
            }
        }
        if !old.is_null() {
            return MergeInterp(info, old, new, same_file);
        }
        info.interps.push((*new).clone());
        true
    }
}
fn ResolveStateAndPredicate(
    mut expr: *mut ExprDef,
    pred_rtrn: *mut u32,
    mods_rtrn: *mut u32,
    info: &mut CompatInfo<'_>,
) -> bool {
    unsafe {
        if expr.is_null() {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        *pred_rtrn = MATCH_EXACTLY;
        if (*expr).common.type_0 == STMT_EXPR_ACTION_DECL {
            let ExprKind::Action { name, args } = &(*expr).kind else {
                unreachable!()
            };
            let pred_txt: &str = xkb_atom_text(&info.ctx().atom_table, *name);
            let mut pred: u32 = 0_u32;
            if !LookupString(&symInterpretMatchMaskNames, pred_txt, &mut pred)
                || (*args).is_null()
                || !(**args).common.next.is_null()
            {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Illegal modifier predicate \"{}\"; Ignored\n",
                    pred_txt,
                );
                return false;
            }
            *pred_rtrn = pred;
            expr = *args;
        } else if (*expr).common.type_0 == STMT_EXPR_IDENT {
            let ExprKind::Ident(ident_val) = &(*expr).kind else {
                unreachable!()
            };
            let pred_txt_0: &str = xkb_atom_text(&info.ctx().atom_table, *ident_val);
            if !pred_txt_0.is_empty() && pred_txt_0.eq_ignore_ascii_case("any") {
                *pred_rtrn = MATCH_ANY;
                *mods_rtrn = MOD_REAL_MASK_ALL;
                return true;
            }
        }
        ExprResolveModMask(info.ctx, expr, MOD_REAL, &info.mods, mods_rtrn)
    }
}
fn UseNewLEDField(
    field: led_field,
    old: led_field,
    new: led_field,
    clobber: bool,
    report: bool,
    collide: *mut led_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            unsafe { *collide = (*collide | field) as led_field };
        }
        return clobber;
    }
    false
}
fn MergeLedMap(
    info: &mut CompatInfo<'_>,
    old: *mut LedInfo,
    new: *mut LedInfo,
    same_file: bool,
) -> bool {
    unsafe {
        let mut collide: led_field;
        let clobber: bool = (*new).merge != MERGE_AUGMENT;
        let verbosity: i32 = xkb_context_get_log_verbosity(info.ctx());
        let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
        if (*old).led.mods.mods == (*new).led.mods.mods
            && (*old).led.pending_groups as i32 == (*new).led.pending_groups as i32
            && (*old).led.groups == (*new).led.groups
            && (*old).led.ctrls == (*new).led.ctrls
            && (*old).led.which_mods == (*new).led.which_mods
            && (*old).led.which_groups as i32 == (*new).led.which_groups as i32
        {
            (*old).defined = ((*old).defined | (*new).defined) as led_field;
            return true;
        }
        if (*new).merge == MERGE_REPLACE {
            if report {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_WARNING,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Map for indicator {} redefined; Earlier definition ignored\n",
                    LEDText(info, old),
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
            (*old).defined = ((*old).defined | LED_FIELD_MODS) as led_field;
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
            (*old).led.pending_groups = (*new).led.pending_groups;
            (*old).defined = ((*old).defined | LED_FIELD_GROUPS) as led_field;
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
            (*old).defined = ((*old).defined | LED_FIELD_CTRLS) as led_field;
        }
        if collide as u64 != 0 {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_WARNING,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Map for indicator {} redefined; Using {} definition for duplicate fields\n",
                LEDText(info, old),
                if clobber { "last" } else { "first" },
            );
        }
        true
    }
}
fn AddLedMap(info: &mut CompatInfo<'_>, new: *mut LedInfo, same_file: bool) -> bool {
    unsafe {
        let mut i: u32 = 0_u32;
        while i < info.num_leds {
            let old: *mut LedInfo =
                (&raw mut info.leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
            if (*old).led.name != (*new).led.name {
                i = i.wrapping_add(1);
            } else {
                return MergeLedMap(info, old, new, same_file);
            }
        }
        if info.num_leds >= XKB_MAX_LEDS {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Too many LEDs defined (maximum {})\n",
                (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32,
            );
            return false;
        }
        let c2rust_fresh1 = info.num_leds;
        info.num_leds = info.num_leds.wrapping_add(1);
        info.leds[c2rust_fresh1 as usize] = *new;
        true
    }
}
fn MergeIncludedCompatMaps(
    into: &mut CompatInfo<'_>,
    from: &mut CompatInfo<'_>,
    merge: merge_mode,
) {
    unsafe {
        if from.errorCount > 0_i32 {
            into.errorCount += from.errorCount;
            return;
        }
        MergeModSets(&mut *into.ctx, &mut into.mods, &from.mods, merge);
        if into.name.is_none() {
            into.name = from.name.take();
        }
        if into.interps.is_empty() {
            into.interps = std::mem::take(&mut from.interps);
        } else {
            for i in 0..from.interps.len() {
                (&mut from.interps)[i].merge = merge;
                let si = (&mut from.interps)[i..].as_mut_ptr();
                if !AddInterp(into, si, false) {
                    into.errorCount += 1;
                }
            }
        }
        if into.num_leds == 0_u32 {
            std::ptr::copy_nonoverlapping::<LedInfo>(
                &raw mut from.leds as *mut LedInfo,
                &raw mut into.leds as *mut LedInfo,
                from.num_leds as usize,
            );
            into.num_leds = from.num_leds;
            from.num_leds = 0_u32;
        } else {
            let mut i: u32 = 0_u32;
            while i < from.num_leds {
                let ledi: *mut LedInfo =
                    (&raw mut from.leds as *mut LedInfo).offset(i as isize) as *mut LedInfo;
                (*ledi).merge = merge;
                if !AddLedMap(into, ledi, false) {
                    into.errorCount += 1;
                }
                i = i.wrapping_add(1);
            }
        };
    }
}
fn HandleIncludeCompatMap(info: &mut CompatInfo<'_>, include: *mut IncludeStmt) -> bool {
    unsafe {
        let ctx_ptr = &raw mut *info.ctx;
        let ki_ptr = &raw mut *info.keymap_info;
        let mut included = CompatInfo::new(&mut *ctx_ptr, &mut *ki_ptr);
        if ExceedsIncludeMaxDepth(&mut *ctx_ptr, info.include_depth) {
            info.errorCount += 10_i32;
            return false;
        }
        InitCompatInfo(
            &mut included,
            info.include_depth.wrapping_add(1_u32),
            &info.mods,
        );
        included.name = {
            let inc = &*include;
            if inc.stmt.is_empty() {
                None
            } else {
                Some(inc.stmt.clone())
            }
        };
        let mut stmt: *mut IncludeStmt = include;
        while !stmt.is_null() {
            let mut next_incl = CompatInfo::new(&mut *ctx_ptr, &mut *ki_ptr);

            let mut path: [i8; 4096] = [0; 4096];
            let file: *mut XkbFile = ProcessIncludeFile(
                &mut *ctx_ptr,
                stmt,
                FILE_TYPE_COMPAT,
                &raw mut path as *mut i8,
                std::mem::size_of::<[i8; 4096]>(),
            );
            if file.is_null() {
                info.errorCount += 10_i32;
                ClearCompatInfo(&mut included);
                return false;
            }
            InitCompatInfo(
                &mut next_incl,
                info.include_depth.wrapping_add(1_u32),
                &raw mut included.mods,
            );
            next_incl.default_interp = info.default_interp.clone();
            next_incl.default_led = info.default_led;
            HandleCompatMapFile(&mut next_incl, file);
            MergeIncludedCompatMaps(&mut included, &mut next_incl, (*stmt).merge);
            ClearCompatInfo(&mut next_incl);
            FreeXkbFile(file);
            stmt = match (*stmt).next_incl {
                Some(ref mut b) => b.as_mut() as *mut IncludeStmt,
                None => std::ptr::null_mut(),
            };
        }
        MergeIncludedCompatMaps(info, &mut included, (*include).merge);
        ClearCompatInfo(&mut included);
        info.errorCount == 0_i32
    }
}
fn SetInterpField(
    info: &mut CompatInfo<'_>,
    si: *mut SymInterpInfo,
    field: &str,
    arrayNdx: *mut ExprDef,
    value: *mut ExprDef,
) -> bool {
    unsafe {
        if field.eq_ignore_ascii_case("action") {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if (*value).common.type_0 == STMT_EXPR_ACTION_LIST {
                let ExprKind::ActionList {
                    actions: action_head,
                } = &(*value).kind
                else {
                    unreachable!()
                };
                let mut num_actions: u32 = 0_u32;
                let mut act: *mut ExprDef = *action_head;
                while !act.is_null() {
                    num_actions = num_actions.wrapping_add(1);
                    act = (*act).common.next as *mut ExprDef;
                }
                if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                    xkb_logf!(
                        info.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Interpret {} has too many actions; expected max {}, got: {}\n",
                        siText(si, info),
                        65535_i32,
                        num_actions,
                    );
                    return false;
                }
                (*si).interp.num_actions = 0_u16;
                (*si).interp.action.type_0 = ACTION_TYPE_NONE;
                let mut actions: Vec<xkb_action> = Vec::new();
                let mut act_0: *mut ExprDef = *action_head;
                while !act_0.is_null() {
                    let mut toAct: xkb_action = xkb_action {
                        type_0: ACTION_TYPE_NONE,
                    };
                    match HandleActionDef(
                        info.keymap_info,
                        &raw mut info.default_actions,
                        &raw mut info.mods,
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
                    if toAct.type_0 != ACTION_TYPE_NONE {
                        if (num_actions == 1_u32) as i64 != 0 {
                            (*si).interp.num_actions = 1_u16;
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
                        (*si).interp.num_actions = 1_u16;
                        (*si).interp.action = actions[1];
                    }
                    _ => {
                        (*si).interp.num_actions = actions.len() as u16;
                        (*si).interp.actions = actions;
                    }
                }
            } else {
                match HandleActionDef(
                    info.keymap_info,
                    &raw mut info.default_actions,
                    &info.mods,
                    value,
                    &raw mut (*si).interp.action,
                ) as u32
                {
                    1 => {
                        (*si).interp.action.type_0 = ACTION_TYPE_NONE;
                        (*si).interp.num_actions = 0_u16;
                    }
                    2 => return false,
                    _ => {
                        (*si).interp.num_actions =
                            ((*si).interp.action.type_0 != ACTION_TYPE_NONE) as i32 as u16;
                    }
                }
            }
            (*si).defined = ((*si).defined | SI_FIELD_ACTION) as si_field;
        } else if field.eq_ignore_ascii_case("virtualmodifier")
            || field.eq_ignore_ascii_case("virtualmod")
        {
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            let mut ndx: u32 = 0_u32;
            if !ExprResolveMod(info.ctx, value, MOD_VIRT, &info.mods, &raw mut ndx) {
                return ReportSIBadType(info, si, field, "virtual modifier");
            }
            (*si).interp.virtual_mod = ndx;
            (*si).defined = ((*si).defined | SI_FIELD_VIRTUAL_MOD) as si_field;
        } else if field.eq_ignore_ascii_case("repeat") {
            let mut set: bool = false;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveBoolean(info.ctx, value, &raw mut set) {
                return ReportSIBadType(info, si, field, "boolean");
            }
            (*si).interp.repeat = set;
            (*si).defined = ((*si).defined | SI_FIELD_AUTO_REPEAT) as si_field;
        } else if field.eq_ignore_ascii_case("locking") {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"locking\" field in symbol interpretation is unsupported; Ignored\n",
            );
        } else if field.eq_ignore_ascii_case("usemodmap")
            || field.eq_ignore_ascii_case("usemodmapmods")
        {
            let mut val: u32 = 0_u32;
            if !arrayNdx.is_null() {
                return ReportSINotArray(info, si, field);
            }
            if !ExprResolveEnum(info.ctx, value, &raw mut val, &useModMapValueNames) {
                return ReportSIBadType(info, si, field, "level specification");
            }
            (*si).interp.level_one_only = val != 0;
            (*si).defined = ((*si).defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
        } else {
            ReportBadField(info.ctx, "symbol interpretation", field, siText(si, info));
            return (*info.keymap_info).strict & PARSER_NO_UNKNOWN_INTERPRET_FIELDS == 0;
        }
        true
    }
}
fn SetLedMapField(
    info: &mut CompatInfo<'_>,
    ledi: *mut LedInfo,
    field: &str,
    arrayNdx: *mut ExprDef,
    value_ptr: *mut *mut ExprDef,
) -> bool {
    unsafe {
        let value: *mut ExprDef = *value_ptr;
        if field.eq_ignore_ascii_case("modifiers") || field.eq_ignore_ascii_case("mods") {
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveModMask(
                info.ctx,
                value,
                MOD_BOTH,
                &info.mods,
                &raw mut (*ledi).led.mods.mods,
            ) {
                return ReportLedBadType(info, ledi, field, "modifier mask");
            }
            (*ledi).defined = ((*ledi).defined | LED_FIELD_MODS) as led_field;
        } else if field.eq_ignore_ascii_case("groups") {
            let mut mask: u32 = 0_u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let mut pending: bool = false;
            if !ExprResolveGroupMask(info.keymap_info, value, &raw mut mask, &raw mut pending) {
                if pending {
                    (*ledi).led.pending_groups = true;
                    let pending_index: u32 = (*info.keymap_info).pending_computations.len() as u32;
                    (*info.keymap_info)
                        .pending_computations
                        .push(pending_computation {
                            expr: *value_ptr,
                            computed: false,
                            value: 0_u32,
                        });
                    *value_ptr = std::ptr::null_mut();
                    mask = pending_index;
                } else {
                    return ReportLedBadType(info, ledi, field, "group mask");
                }
            } else {
                (*ledi).led.pending_groups = false;
            }
            (*ledi).led.groups = mask;
            (*ledi).defined = ((*ledi).defined | LED_FIELD_GROUPS) as led_field;
        } else if field.eq_ignore_ascii_case("controls") || field.eq_ignore_ascii_case("ctrls") {
            let mut mask_0: u32 = 0_u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            let offset: u8 = (*info.keymap_info).features.controls_name_offset;
            if !ExprResolveMask(
                info.ctx,
                value,
                &raw mut mask_0,
                &ctrlMaskNames[offset as usize..],
            ) {
                return ReportLedBadType(info, ledi, field, "controls mask");
            }
            (*ledi).led.ctrls = mask_0 as xkb_action_controls;
            (*ledi).defined = ((*ledi).defined | LED_FIELD_CTRLS) as led_field;
        } else if field.eq_ignore_ascii_case("allowexplicit") {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n",
            );
        } else if field.eq_ignore_ascii_case("whichmodstate")
            || field.eq_ignore_ascii_case("whichmodifierstate")
        {
            let mut mask_1: u32 = 0_u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(info.ctx, value, &raw mut mask_1, &modComponentMaskNames) {
                return ReportLedBadType(info, ledi, field, "mask of modifier state components");
            }
            (*ledi).led.which_mods = mask_1;
        } else if field.eq_ignore_ascii_case("whichgroupstate") {
            let mut mask_2: u32 = 0_u32;
            if !arrayNdx.is_null() {
                return ReportLedNotArray(info, ledi, field);
            }
            if !ExprResolveMask(info.ctx, value, &raw mut mask_2, &groupComponentMaskNames) {
                return ReportLedBadType(info, ledi, field, "mask of group state components");
            }
            (*ledi).led.which_groups = mask_2;
        } else if field.eq_ignore_ascii_case("driveskbd")
            || field.eq_ignore_ascii_case("driveskeyboard")
            || field.eq_ignore_ascii_case("leddriveskbd")
            || field.eq_ignore_ascii_case("leddriveskeyboard")
            || field.eq_ignore_ascii_case("indicatordriveskbd")
            || field.eq_ignore_ascii_case("indicatordriveskeyboard")
        {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_DEBUG,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"{}\" field in indicator statements is unsupported; Ignored\n",
                field,
            );
        } else if field.eq_ignore_ascii_case("index") {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "The \"index\" field in indicator statements is unsupported; Ignored\n",
            );
        } else {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Unknown field \"{}\" in map for {} indicator; Definition ignored\n",
                field,
                LEDText(info, ledi),
            );
            return (*info.keymap_info).strict & PARSER_NO_UNKNOWN_LED_FIELDS == 0;
        }
        true
    }
}
fn HandleGlobalVar(info: &mut CompatInfo<'_>, stmt: *mut VarDef) -> bool {
    unsafe {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut ndx: *mut ExprDef = std::ptr::null_mut();
        let ret: bool;
        if !ExprResolveLhs(info.ctx, (*stmt).name, &mut elem, &mut field, &raw mut ndx) {
            ret = false;
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("interpret") {
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
            temp.merge = (if temp.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                (*stmt).merge
            }) as merge_mode;
            ret = SetInterpField(
                info,
                &raw mut temp,
                field,
                ndx,
                (*stmt).value as *mut ExprDef,
            );
            if ret {
                let default_ptr = &raw mut info.default_interp;
                MergeInterp(info, default_ptr, &raw mut temp, true);
            }
        } else if !elem.is_empty() && elem.eq_ignore_ascii_case("indicator") {
            let mut temp_0: LedInfo = LedInfo {
                defined: 0 as led_field,
                merge: MERGE_DEFAULT,
                led: xkb_led {
                    name: 0,
                    which_groups: 0,
                    pending_groups: false,
                    groups: 0,
                    which_mods: 0_u32,
                    mods: xkb_mods { mods: 0, mask: 0 },
                    ctrls: 0 as xkb_action_controls,
                },
            };
            InitLED(&raw mut temp_0);
            temp_0.merge = (if temp_0.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                (*stmt).merge
            }) as merge_mode;
            ret = SetLedMapField(info, &raw mut temp_0, field, ndx, &raw mut (*stmt).value);
            if ret {
                let default_ptr = &raw mut info.default_led;
                MergeLedMap(info, default_ptr, &raw mut temp_0, true);
            }
        } else if !elem.is_empty() {
            ret = SetDefaultActionField(
                info.keymap_info,
                &raw mut info.default_actions,
                &raw mut info.mods,
                elem,
                field,
                ndx,
                &raw mut (*stmt).value,
                (*stmt).merge,
            ) as u32
                != PARSER_FATAL_ERROR;
        } else {
            xkb_logf!(
                info.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field,
            );
            return (*info.keymap_info).strict & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS == 0;
        }
        ret
    }
}
fn HandleInterpBody(
    info: &mut CompatInfo<'_>,
    mut def: *mut VarDef,
    si: *mut SymInterpInfo,
) -> bool {
    unsafe {
        let mut ok: bool = true;
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
        while !def.is_null() {
            if !ExprResolveLhs(
                info.ctx,
                (*def).name,
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Cannot set a global default value for \"{}\" element from within an interpret statement; Move assignment to \"{}.{}\" to the global file scope\n",
                    elem,
                    elem,
                    field,
                );
                ok = false;
            } else if !SetInterpField(info, si, field, arrayNdx, (*def).value as *mut ExprDef) {
                ok = false;
            }
            def = (*def).common.next as *mut VarDef;
        }
        ok
    }
}
fn HandleInterpDef(info: &mut CompatInfo<'_>, def: *mut InterpDef) -> bool {
    unsafe {
        let mut pred: u32 = MATCH_NONE;
        let mut mods: u32 = 0;
        #[allow(unused_assignments)]
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
                info.ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as i32,
                "Couldn't determine matching modifiers; Symbol interpretation ignored\n",
            );
            return false;
        }
        si = info.default_interp.clone();
        si.merge = (*def).merge;
        si.interp.sym = (*def).sym;
        si.interp.match_0 = pred;
        si.interp.mods = mods;
        if !HandleInterpBody(info, (*def).def, &raw mut si) {
            info.errorCount += 1;
            return false;
        }
        if !AddInterp(info, &raw mut si, true) {
            info.errorCount += 1;
            return false;
        }
        true
    }
}
fn HandleLedMapDef(info: &mut CompatInfo<'_>, def: *mut LedMapDef) -> bool {
    unsafe {
        let mut ledi: LedInfo = info.default_led;
        ledi.merge = (*def).merge;
        ledi.led.name = (*def).name;
        let mut ok: bool = true;
        let mut var: *mut VarDef = (*def).body;
        while !var.is_null() {
            let mut elem: &str = "";
            let mut field: &str = "";
            let mut arrayNdx: *mut ExprDef = std::ptr::null_mut();
            if !ExprResolveLhs(
                info.ctx,
                (*var).name,
                &mut elem,
                &mut field,
                &raw mut arrayNdx,
            ) {
                ok = false;
            } else if !elem.is_empty() {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "[XKB-{:03}] Cannot set defaults for \"{}\" element in indicator map; Assignment to {}.{} ignored\n",
                    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                    elem,
                    elem,
                    field,
                );
                ok = false;
            } else if !SetLedMapField(info, &raw mut ledi, field, arrayNdx, &raw mut (*var).value) {
                ok = false;
            }
            var = (*var).common.next as *mut VarDef;
        }
        ok as i32 != 0 && AddLedMap(info, &raw mut ledi, true) as i32 != 0
    }
}
fn HandleCompatMapFile(info: &mut CompatInfo<'_>, file: *mut XkbFile) {
    unsafe {
        let mut ok: bool;
        info.name = {
            let file_ref = &*file;
            if file_ref.name.is_empty() {
                None
            } else {
                Some(file_ref.name.clone())
            }
        };
        let mut stmt: *mut ParseCommon = (*file).defs;
        while !stmt.is_null() {
            match (*stmt).type_0 {
                1 => {
                    ok = HandleIncludeCompatMap(info, stmt as *mut IncludeStmt);
                }
                28 => {
                    ok = HandleInterpDef(info, stmt as *mut InterpDef);
                }
                32 => {
                    xkb_logf!(
                        info.ctx,
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
                    ok = HandleVModDef(info.ctx, &raw mut info.mods, stmt as *mut VModDef);
                }
                35 | 36 => {
                    xkb_logf!(
                        info.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "[XKB-{:03}] Unsupported compatibility {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if (*stmt).type_0 == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &(*(stmt as *mut UnknownStatement)).name,
                    );
                    ok = (*info.keymap_info).strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    xkb_logf!(
                        info.ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as i32,
                        "Compat files may not include other types; Ignoring {}\n",
                        stmt_type_to_string((*stmt).type_0),
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                xkb_logf!(
                    info.ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as i32,
                    "Abandoning compatibility map \"{}\"\n",
                    safe_map_name(&*file),
                );
                break;
            } else {
                stmt = (*stmt).next as *mut ParseCommon;
            }
        }
    }
}
fn CopyInterps(info: &CompatInfo<'_>, needSymbol: bool, pred: u32, collect: *mut collect) {
    unsafe {
        for i in 0..info.interps.len() {
            let si = &info.interps[i];
            if si.interp.match_0 == pred
                && (si.interp.sym != XKB_KEY_NoSymbol as u32) as i32 == needSymbol as i32
            {
                (*collect).sym_interprets.push(si.interp.clone());
            }
        }
    }
}
fn CopyLedMapDefsToKeymap(keymap: *mut xkb_keymap, info: &mut CompatInfo<'_>) {
    unsafe {
        let mut c2rust_current_block_11: u64;
        let mut idx: u32 = 0_u32;
        while idx < info.num_leds {
            let ledi: *mut LedInfo =
                (&raw mut info.leds as *mut LedInfo).offset(idx as isize) as *mut LedInfo;
            let mut i: u32;
            let mut led: *mut xkb_led;
            i = 0_u32;
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
                    LEDText(info, ledi),
                );
                i = 0_u32;
                led = &raw mut (*keymap).leds as *mut xkb_led;
                while i < (*keymap).num_leds {
                    if (*led).name == XKB_ATOM_NONE {
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
                            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32,
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
            if c2rust_current_block_11 == 17860125682698302841 {
                *led = (*ledi).led;
                if (*led).which_groups as i32 == 0_i32
                    && ((*led).groups != 0_u32 || (*led).pending_groups as i32 != 0)
                {
                    (*led).which_groups = XKB_STATE_LAYOUT_EFFECTIVE;
                }
                if (*led).which_mods == 0_u32 && (*led).mods.mods != 0_u32 {
                    (*led).which_mods = XKB_STATE_MODS_EFFECTIVE;
                }
            }
            idx = idx.wrapping_add(1);
        }
    }
}
fn CopyCompatToKeymap(keymap: *mut xkb_keymap, info: &mut CompatInfo<'_>) -> bool {
    unsafe {
        (*keymap).compat_section_name = match &info.name {
            Some(s) => s.clone(),
            None => String::new(),
        };
        xkb_escape_map_name(&mut (*keymap).compat_section_name);
        (*keymap).mods = info.mods;
        if !info.interps.is_empty() {
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
                (*keymap).sym_interprets = Vec::new();
            } else {
                (*keymap).sym_interprets = collect.sym_interprets;
            }
        }
        CopyLedMapDefsToKeymap(keymap, info);
        true
    }
}
pub fn CompileCompatMap(file: *mut XkbFile, keymap_info: *mut xkb_keymap_info) -> bool {
    unsafe {
        let ctx = &mut (*(*keymap_info).keymap).ctx;
        let mut info = CompatInfo::new(ctx, &mut *keymap_info);
        InitCompatInfo(&mut info, 0_u32, &raw mut (*(*keymap_info).keymap).mods);
        if !file.is_null() {
            HandleCompatMapFile(&mut info, file);
        }
        if (info.errorCount == 0_i32) && CopyCompatToKeymap((*keymap_info).keymap, &mut info) {
            ClearCompatInfo(&mut info);
            return true;
        }
        ClearCompatInfo(&mut info);
        false
    }
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
