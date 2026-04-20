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
}
impl<'a> CompatInfo<'a> {
    #[inline]
    pub fn ctx(&self) -> &xkb_context {
        self.keymap_info.ctx()
    }
    #[inline]
    pub fn ctx_mut(&mut self) -> &mut xkb_context {
        self.keymap_info.ctx_mut()
    }
    pub fn new(keymap_info: &'a mut xkb_keymap_info) -> Self {
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
                    action: xkb_action::None,
                    actions: Vec::new(),
                },
            },
            interps: Vec::new(),
            default_led: zeroed_led,
            leds: [zeroed_led; 32],
            num_leds: 0,
            default_actions: ActionsInfo {
                actions: [xkb_action::None; 21],
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
        }
    }
}
#[derive(Copy, Clone)]
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
fn siText(si: &SymInterpInfo, info: &mut CompatInfo<'_>) -> &'static str {
    unsafe {
        if std::ptr::eq(si, &info.default_interp) {
            return "default";
        }
        let buf: *mut i8 = xkb_context_get_buffer(&mut (*info.ctx()).clone(), 128_usize);
        let (written, _) = crate::xkb::utils::snprintf_args(
            buf,
            128_usize,
            format_args!(
                "{}+{}({})",
                KeysymText(si.interp.sym),
                SIMatchText(si.interp.match_0),
                ModMaskText(info.ctx(), MOD_BOTH, &info.mods, si.interp.mods),
            ),
        );
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(buf as *const u8, written))
    }
}
#[inline]
fn ReportSINotArray(info: &mut CompatInfo<'_>, si: &SymInterpInfo, field: &str) -> bool {
    ReportNotArray("symbol interpretation", field, siText(si, info))
}
#[inline]
fn ReportSIBadType(
    info: &mut CompatInfo<'_>,
    si: &SymInterpInfo,
    field: &str,
    wanted: &str,
) -> bool {
    ReportBadType(
        XKB_ERROR_WRONG_FIELD_TYPE,
        "symbol interpretation",
        field,
        siText(si, info),
        wanted,
    )
}
fn LEDText(info: &mut CompatInfo<'_>, ledi: &LedInfo) -> &'static str {
    if std::ptr::eq(ledi, &info.default_led) {
        "default"
    } else {
        // SAFETY: atom_table strings live for the lifetime of the context.
        // We need 'static because the return is used after info is mutably borrowed.
        let atom_table = &info.ctx().atom_table as *const _;
        unsafe { xkb_atom_text(&*atom_table, ledi.led.name) }
    }
}
#[inline]
fn ReportLedBadType(info: &mut CompatInfo<'_>, ledi: &LedInfo, field: &str, wanted: &str) -> bool {
    ReportBadType(
        XKB_ERROR_WRONG_FIELD_TYPE,
        "indicator map",
        field,
        LEDText(info, ledi),
        wanted,
    )
}
#[inline]
fn ReportLedNotArray(info: &mut CompatInfo<'_>, ledi: &LedInfo, field: &str) -> bool {
    ReportNotArray("indicator map", field, LEDText(info, ledi))
}
#[inline]
fn InitInterp(info: &mut SymInterpInfo) {
    info.merge = MERGE_DEFAULT;
    info.interp.virtual_mod = XKB_MOD_INVALID;
}
#[inline]
fn InitLED(info: &mut LedInfo) {
    info.merge = MERGE_DEFAULT;
}
fn InitCompatInfo(info: &mut CompatInfo<'_>, include_depth: u32, mods: &xkb_mod_set) {
    info.include_depth = include_depth;
    InitActionsInfo(info.keymap_info.keymap_ref(), &mut info.default_actions);
    InitVMods(&mut info.mods, mods, include_depth > 0_u32);
    InitInterp(&mut info.default_interp);
    InitLED(&mut info.default_led);
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
    collide: &mut si_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            *collide = (*collide | field) as si_field;
        }
        return clobber;
    }
    false
}
fn MergeInterp(
    info: &mut CompatInfo<'_>,
    old: &mut SymInterpInfo,
    new: &mut SymInterpInfo,
    same_file: bool,
) -> bool {
    let clobber: bool = new.merge != MERGE_AUGMENT;
    let verbosity: i32 = xkb_context_get_log_verbosity(info.ctx());
    let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
    let mut collide: si_field = 0 as si_field;
    if new.merge == MERGE_REPLACE {
        if report {
            log::warn!(
                "Multiple definitions for \"{}\"; Earlier interpretation ignored\n",
                siText(new, info)
            );
        }
        *old = new.clone();
        return true;
    }
    if UseNewInterpField(
        SI_FIELD_VIRTUAL_MOD,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.virtual_mod = new.interp.virtual_mod;
        old.defined = (old.defined | SI_FIELD_VIRTUAL_MOD) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_ACTION,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        if old.interp.num_actions as i32 > 1_i32 {
            old.interp.actions.clear();
        }
        old.interp.num_actions = new.interp.num_actions;
        if new.interp.num_actions as i32 > 1_i32 {
            old.interp.actions = std::mem::take(&mut new.interp.actions);
            new.interp.action = xkb_action::None;
            new.interp.num_actions = 0_u16;
        } else {
            old.interp.action = new.interp.action;
        }
        old.defined = (old.defined | SI_FIELD_ACTION) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_AUTO_REPEAT,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.repeat = new.interp.repeat;
        old.defined = (old.defined | SI_FIELD_AUTO_REPEAT) as si_field;
    }
    if UseNewInterpField(
        SI_FIELD_LEVEL_ONE_ONLY,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.interp.level_one_only = new.interp.level_one_only;
        old.defined = (old.defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
    }
    if collide as u64 != 0 {
        log::warn!(
            "Multiple interpretations of \"{}\"; Using {} definition for duplicate fields\n",
            siText(old, info),
            if clobber { "last" } else { "first" }
        );
    }
    true
}
fn AddInterp(info: &mut CompatInfo<'_>, new: &mut SymInterpInfo, same_file: bool) -> bool {
    // FindMatchingInterp inlined
    let mut old_idx: Option<usize> = None;
    for i in 0..info.interps.len() {
        if info.interps[i].interp.sym == new.interp.sym
            && info.interps[i].interp.mods == new.interp.mods
            && info.interps[i].interp.match_0 == new.interp.match_0
        {
            old_idx = Some(i);
            break;
        }
    }
    if let Some(idx) = old_idx {
        // Safe: new is a separate local, not from info.interps
        let old = &mut info.interps[idx] as *mut SymInterpInfo;
        return MergeInterp(info, unsafe { &mut *old }, new, same_file);
    }
    info.interps.push(new.clone());
    true
}
fn ResolveStateAndPredicate(
    expr: Option<&ExprDef>,
    pred_rtrn: &mut u32,
    mods_rtrn: &mut u32,
    info: &mut CompatInfo<'_>,
) -> bool {
    let expr = match expr {
        None => {
            *pred_rtrn = MATCH_ANY_OR_NONE;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        Some(e) => e,
    };
    *pred_rtrn = MATCH_EXACTLY;
    let resolve_expr: &ExprDef;
    if expr.common.type_0 == STMT_EXPR_ACTION_DECL {
        let ExprKind::Action { name, args } = &expr.kind else {
            unreachable!()
        };
        let pred_txt: &str = xkb_atom_text(&info.ctx().atom_table, *name);
        let mut pred: u32 = 0_u32;
        if !LookupString(&symInterpretMatchMaskNames, pred_txt, &mut pred)
            || args.is_empty()
            || args.len() != 1
        {
            log::error!("Illegal modifier predicate \"{}\"; Ignored\n", pred_txt);
            return false;
        }
        *pred_rtrn = pred;
        resolve_expr = &args[0];
    } else if expr.common.type_0 == STMT_EXPR_IDENT {
        let ExprKind::Ident(ident_val) = &expr.kind else {
            unreachable!()
        };
        let pred_txt_0: &str = xkb_atom_text(&info.ctx().atom_table, *ident_val);
        if !pred_txt_0.is_empty() && pred_txt_0.eq_ignore_ascii_case("any") {
            *pred_rtrn = MATCH_ANY;
            *mods_rtrn = MOD_REAL_MASK_ALL;
            return true;
        }
        resolve_expr = expr;
    } else {
        resolve_expr = expr;
    }
    ExprResolveModMask(info.ctx(), resolve_expr, MOD_REAL, &info.mods, mods_rtrn)
}
fn UseNewLEDField(
    field: led_field,
    old: led_field,
    new: led_field,
    clobber: bool,
    report: bool,
    collide: &mut led_field,
) -> bool {
    if old & field == 0 {
        return new & field != 0;
    }
    if new & field != 0 {
        if report {
            *collide = (*collide | field) as led_field;
        }
        return clobber;
    }
    false
}
fn MergeLedMap(
    info: &mut CompatInfo<'_>,
    old: &mut LedInfo,
    new: &mut LedInfo,
    same_file: bool,
) -> bool {
    let mut collide: led_field;
    let clobber: bool = new.merge != MERGE_AUGMENT;
    let verbosity: i32 = xkb_context_get_log_verbosity(info.ctx());
    let report: bool = same_file as i32 != 0 && verbosity > 0_i32 || verbosity > 9_i32;
    if old.led.mods.mods == new.led.mods.mods
        && old.led.pending_groups as i32 == new.led.pending_groups as i32
        && old.led.groups == new.led.groups
        && old.led.ctrls == new.led.ctrls
        && old.led.which_mods == new.led.which_mods
        && old.led.which_groups as i32 == new.led.which_groups as i32
    {
        old.defined = (old.defined | new.defined) as led_field;
        return true;
    }
    if new.merge == MERGE_REPLACE {
        if report {
            log::warn!(
                "Map for indicator {} redefined; Earlier definition ignored\n",
                LEDText(info, old)
            );
        }
        *old = *new;
        return true;
    }
    collide = 0 as led_field;
    if UseNewLEDField(
        LED_FIELD_MODS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.which_mods = new.led.which_mods;
        old.led.mods = new.led.mods;
        old.defined = (old.defined | LED_FIELD_MODS) as led_field;
    }
    if UseNewLEDField(
        LED_FIELD_GROUPS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.which_groups = new.led.which_groups;
        old.led.groups = new.led.groups;
        old.led.pending_groups = new.led.pending_groups;
        old.defined = (old.defined | LED_FIELD_GROUPS) as led_field;
    }
    if UseNewLEDField(
        LED_FIELD_CTRLS,
        old.defined,
        new.defined,
        clobber,
        report,
        &mut collide,
    ) {
        old.led.ctrls = new.led.ctrls;
        old.defined = (old.defined | LED_FIELD_CTRLS) as led_field;
    }
    if collide as u64 != 0 {
        log::warn!(
            "Map for indicator {} redefined; Using {} definition for duplicate fields\n",
            LEDText(info, old),
            if clobber { "last" } else { "first" }
        );
    }
    true
}
fn AddLedMap(info: &mut CompatInfo<'_>, new: &mut LedInfo, same_file: bool) -> bool {
    let mut i: u32 = 0_u32;
    while i < info.num_leds {
        if info.leds[i as usize].led.name != new.led.name {
            i = i.wrapping_add(1);
        } else {
            let old = &mut info.leds[i as usize] as *mut LedInfo;
            return MergeLedMap(info, unsafe { &mut *old }, new, same_file);
        }
    }
    if info.num_leds >= XKB_MAX_LEDS {
        log::error!(
            "Too many LEDs defined (maximum {})\n",
            (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32
        );
        return false;
    }
    let c2rust_fresh1 = info.num_leds;
    info.num_leds = info.num_leds.wrapping_add(1);
    info.leds[c2rust_fresh1 as usize] = *new;
    true
}
fn MergeIncludedCompatMaps(
    into: &mut CompatInfo<'_>,
    from: &mut CompatInfo<'_>,
    merge: merge_mode,
) {
    if from.errorCount > 0_i32 {
        into.errorCount += from.errorCount;
        return;
    }
    MergeModSets(
        into.keymap_info.ctx_mut(),
        &mut into.mods,
        &from.mods,
        merge,
    );
    if into.name.is_none() {
        into.name = from.name.take();
    }
    if into.interps.is_empty() {
        into.interps = std::mem::take(&mut from.interps);
    } else {
        for i in 0..from.interps.len() {
            (&mut from.interps)[i].merge = merge;
            let si = &mut from.interps[i];
            if !AddInterp(into, si, false) {
                into.errorCount += 1;
            }
        }
    }
    if into.num_leds == 0_u32 {
        let n = from.num_leds as usize;
        into.leds[..n].copy_from_slice(&from.leds[..n]);
        into.num_leds = from.num_leds;
        from.num_leds = 0_u32;
    } else {
        for i in 0..from.num_leds as usize {
            from.leds[i].merge = merge;
            let ledi = &mut from.leds[i];
            if !AddLedMap(into, ledi, false) {
                into.errorCount += 1;
            }
        }
    };
}
fn HandleIncludeCompatMap(info: &mut CompatInfo<'_>, include: &mut IncludeStmt) -> bool {
    let ki_ptr = &raw mut *info.keymap_info;
    let ctx_ptr = unsafe { &raw mut (*(*ki_ptr).keymap).ctx };
    let mut included = CompatInfo::new(unsafe { &mut *ki_ptr });
    if ExceedsIncludeMaxDepth(info.include_depth) {
        info.errorCount += 10_i32;
        return false;
    }
    InitCompatInfo(
        &mut included,
        info.include_depth.wrapping_add(1_u32),
        &info.mods,
    );
    included.name = if include.stmt.is_empty() {
        None
    } else {
        Some(include.stmt.clone())
    };
    let mut current: Option<&IncludeStmt> = Some(include);
    while let Some(stmt) = current {
        let mut next_incl = CompatInfo::new(unsafe { &mut *ki_ptr });

        let mut path: [i8; 4096] = [0; 4096];
        let file: *mut XkbFile = ProcessIncludeFile(
            ctx_ptr,
            stmt,
            FILE_TYPE_COMPAT,
            path.as_mut_ptr(),
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
            &included.mods,
        );
        next_incl.default_interp = info.default_interp.clone();
        next_incl.default_led = info.default_led;
        HandleCompatMapFile(&mut next_incl, unsafe { &mut *file });
        MergeIncludedCompatMaps(&mut included, &mut next_incl, stmt.merge);
        ClearCompatInfo(&mut next_incl);
        FreeXkbFile(file);
        current = stmt.next_incl.as_deref();
    }
    MergeIncludedCompatMaps(info, &mut included, include.merge);
    ClearCompatInfo(&mut included);
    info.errorCount == 0_i32
}
fn SetInterpField(
    info: &mut CompatInfo<'_>,
    si: &mut SymInterpInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value: &mut ExprDef,
) -> bool {
    if field.eq_ignore_ascii_case("action") {
        if arrayNdx.is_some() {
            return ReportSINotArray(info, si, field);
        }
        if value.common.type_0 == STMT_EXPR_ACTION_LIST {
            let ExprKind::ActionList {
                actions: action_vec,
            } = &mut value.kind
            else {
                unreachable!()
            };
            let num_actions: u32 = action_vec.len() as u32;
            if num_actions > MAX_ACTIONS_PER_LEVEL as u32 {
                log::error!(
                    "Interpret {} has too many actions; expected max {}, got: {}\n",
                    siText(si, info),
                    65535_i32,
                    num_actions
                );
                return false;
            }
            si.interp.num_actions = 0_u16;
            si.interp.action.set_none();
            let mut actions: Vec<xkb_action> = Vec::new();
            for act_expr in action_vec.iter_mut() {
                let mut toAct: xkb_action = xkb_action::None;
                match HandleActionDef(
                    info.keymap_info,
                    &mut info.default_actions,
                    &info.mods,
                    act_expr,
                    &mut toAct,
                ) as u32
                {
                    1 => {
                        toAct.set_none();
                    }
                    2 => {
                        drop(actions);
                        return false;
                    }
                    _ => {}
                }
                if toAct.action_type() != ACTION_TYPE_NONE {
                    if (num_actions == 1_u32) as i64 != 0 {
                        si.interp.num_actions = 1_u16;
                        si.interp.action = toAct;
                    } else {
                        actions.push(toAct);
                    }
                }
            }
            match actions.len() as u32 {
                0 => {}
                1 => {
                    si.interp.num_actions = 1_u16;
                    si.interp.action = actions[0];
                }
                _ => {
                    si.interp.num_actions = actions.len() as u16;
                    si.interp.actions = actions;
                }
            }
        } else {
            match HandleActionDef(
                info.keymap_info,
                &mut info.default_actions,
                &info.mods,
                value,
                &mut si.interp.action,
            ) as u32
            {
                1 => {
                    si.interp.action.set_none();
                    si.interp.num_actions = 0_u16;
                }
                2 => return false,
                _ => {
                    si.interp.num_actions =
                        (si.interp.action.action_type() != ACTION_TYPE_NONE) as i32 as u16;
                }
            }
        }
        si.defined = (si.defined | SI_FIELD_ACTION) as si_field;
    } else if field.eq_ignore_ascii_case("virtualmodifier")
        || field.eq_ignore_ascii_case("virtualmod")
    {
        if arrayNdx.is_some() {
            return ReportSINotArray(info, si, field);
        }
        let mut ndx: u32 = 0_u32;
        if !ExprResolveMod(info.ctx(), value, MOD_VIRT, &info.mods, &mut ndx) {
            return ReportSIBadType(info, si, field, "virtual modifier");
        }
        si.interp.virtual_mod = ndx;
        si.defined = (si.defined | SI_FIELD_VIRTUAL_MOD) as si_field;
    } else if field.eq_ignore_ascii_case("repeat") {
        let mut set: bool = false;
        if arrayNdx.is_some() {
            return ReportSINotArray(info, si, field);
        }
        if !ExprResolveBoolean(info.ctx(), value, &mut set) {
            return ReportSIBadType(info, si, field, "boolean");
        }
        si.interp.repeat = set;
        si.defined = (si.defined | SI_FIELD_AUTO_REPEAT) as si_field;
    } else if field.eq_ignore_ascii_case("locking") {
        log::debug!("The \"locking\" field in symbol interpretation is unsupported; Ignored\n");
    } else if field.eq_ignore_ascii_case("usemodmap") || field.eq_ignore_ascii_case("usemodmapmods")
    {
        let mut val: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportSINotArray(info, si, field);
        }
        if !ExprResolveEnum(info.ctx(), value, &mut val, &useModMapValueNames) {
            return ReportSIBadType(info, si, field, "level specification");
        }
        si.interp.level_one_only = val != 0;
        si.defined = (si.defined | SI_FIELD_LEVEL_ONE_ONLY) as si_field;
    } else {
        ReportBadField("symbol interpretation", field, siText(si, info));
        return info.keymap_info.strict & PARSER_NO_UNKNOWN_INTERPRET_FIELDS == 0;
    }
    true
}
fn SetLedMapField(
    info: &mut CompatInfo<'_>,
    ledi: &mut LedInfo,
    field: &str,
    arrayNdx: Option<&ExprDef>,
    value_opt: &mut Option<Box<ExprDef>>,
) -> bool {
    let value: &ExprDef = value_opt.as_deref().unwrap();
    if field.eq_ignore_ascii_case("modifiers") || field.eq_ignore_ascii_case("mods") {
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ledi, field);
        }
        if !ExprResolveModMask(
            info.ctx(),
            value,
            MOD_BOTH,
            &info.mods,
            &mut ledi.led.mods.mods,
        ) {
            return ReportLedBadType(info, ledi, field, "modifier mask");
        }
        ledi.defined = (ledi.defined | LED_FIELD_MODS) as led_field;
    } else if field.eq_ignore_ascii_case("groups") {
        let mut mask: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ledi, field);
        }
        let mut pending: bool = false;
        if !ExprResolveGroupMask(info.keymap_info, value, &mut mask, &mut pending) {
            if pending {
                ledi.led.pending_groups = true;
                let pending_index: u32 = info.keymap_info.pending_computations.len() as u32;
                info.keymap_info
                    .pending_computations
                    .push(pending_computation {
                        expr: value_opt.take(),
                        computed: false,
                        value: 0_u32,
                    });
                mask = pending_index;
            } else {
                return ReportLedBadType(info, ledi, field, "group mask");
            }
        } else {
            ledi.led.pending_groups = false;
        }
        ledi.led.groups = mask;
        ledi.defined = (ledi.defined | LED_FIELD_GROUPS) as led_field;
    } else if field.eq_ignore_ascii_case("controls") || field.eq_ignore_ascii_case("ctrls") {
        let mut mask_0: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ledi, field);
        }
        let offset: u8 = info.keymap_info.features.controls_name_offset;
        if !ExprResolveMask(
            info.ctx(),
            value,
            &mut mask_0,
            &ctrlMaskNames[offset as usize..],
        ) {
            return ReportLedBadType(info, ledi, field, "controls mask");
        }
        ledi.led.ctrls = mask_0 as xkb_action_controls;
        ledi.defined = (ledi.defined | LED_FIELD_CTRLS) as led_field;
    } else if field.eq_ignore_ascii_case("allowexplicit") {
        log::debug!(
            "The \"allowExplicit\" field in indicator statements is unsupported; Ignored\n"
        );
    } else if field.eq_ignore_ascii_case("whichmodstate")
        || field.eq_ignore_ascii_case("whichmodifierstate")
    {
        let mut mask_1: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ledi, field);
        }
        if !ExprResolveMask(info.ctx(), value, &mut mask_1, &modComponentMaskNames) {
            return ReportLedBadType(info, ledi, field, "mask of modifier state components");
        }
        ledi.led.which_mods = mask_1;
    } else if field.eq_ignore_ascii_case("whichgroupstate") {
        let mut mask_2: u32 = 0_u32;
        if arrayNdx.is_some() {
            return ReportLedNotArray(info, ledi, field);
        }
        if !ExprResolveMask(info.ctx(), value, &mut mask_2, &groupComponentMaskNames) {
            return ReportLedBadType(info, ledi, field, "mask of group state components");
        }
        ledi.led.which_groups = mask_2;
    } else if field.eq_ignore_ascii_case("driveskbd")
        || field.eq_ignore_ascii_case("driveskeyboard")
        || field.eq_ignore_ascii_case("leddriveskbd")
        || field.eq_ignore_ascii_case("leddriveskeyboard")
        || field.eq_ignore_ascii_case("indicatordriveskbd")
        || field.eq_ignore_ascii_case("indicatordriveskeyboard")
    {
        log::debug!(
            "The \"{}\" field in indicator statements is unsupported; Ignored\n",
            field
        );
    } else if field.eq_ignore_ascii_case("index") {
        log::error!("The \"index\" field in indicator statements is unsupported; Ignored\n");
    } else {
        log::error!(
            "Unknown field \"{}\" in map for {} indicator; Definition ignored\n",
            field,
            LEDText(info, ledi)
        );
        return info.keymap_info.strict & PARSER_NO_UNKNOWN_LED_FIELDS == 0;
    }
    true
}
fn HandleGlobalVar(info: &mut CompatInfo<'_>, stmt: &mut VarDef) -> bool {
    let mut elem: &str = "";
    let mut field: &str = "";
    let mut ndx: Option<&ExprDef> = None;
    let ret: bool;
    if !ExprResolveLhs(
        info.ctx(),
        stmt.name.as_deref().unwrap(),
        &mut elem,
        &mut field,
        &mut ndx,
    ) {
        ret = false;
    } else {
        if !elem.is_empty() && elem.eq_ignore_ascii_case("interpret") {
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
                    action: xkb_action::None,
                    actions: Vec::new(),
                },
            };
            InitInterp(&mut temp);
            temp.merge = (if temp.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as merge_mode;
            // Break borrow from ExprResolveLhs
            let field_ptr: *const str = field;
            let ndx_ref = ndx.map(|r| r as *const ExprDef).map(|p| unsafe { &*p });
            let value_ref = stmt.value.as_deref_mut().unwrap();
            ret = SetInterpField(info, &mut temp, unsafe { &*field_ptr }, ndx_ref, value_ref);
            if ret {
                let default_ptr = &raw mut info.default_interp;
                MergeInterp(info, unsafe { &mut *default_ptr }, &mut temp, true);
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
            InitLED(&mut temp_0);
            temp_0.merge = (if temp_0.merge == MERGE_REPLACE {
                MERGE_OVERRIDE
            } else {
                stmt.merge
            }) as merge_mode;
            // Break borrow: ndx borrows from info.ctx via ExprResolveLhs lifetime
            let ndx_ref = ndx.map(|r| r as *const ExprDef).map(|p| unsafe { &*p });
            ret = SetLedMapField(info, &mut temp_0, field, ndx_ref, &mut stmt.value);
            if ret {
                let default_ptr = &raw mut info.default_led;
                MergeLedMap(info, unsafe { &mut *default_ptr }, &mut temp_0, true);
            }
        } else if !elem.is_empty() {
            ret = {
                // Break borrows from ExprResolveLhs for this branch
                let elem_ptr: *const str = elem;
                let field_ptr2: *const str = field;
                let ndx_ref2 = ndx.map(|r| r as *const ExprDef).map(|p| unsafe { &*p });
                let mut value_raw = stmt
                    .value
                    .take()
                    .map_or(std::ptr::null_mut(), |b| Box::into_raw(b));
                let r = SetDefaultActionField(
                    info.keymap_info,
                    &mut info.default_actions,
                    &mut info.mods,
                    unsafe { &*elem_ptr },
                    unsafe { &*field_ptr2 },
                    ndx_ref2,
                    &raw mut value_raw,
                    stmt.merge,
                ) as u32
                    != PARSER_FATAL_ERROR;
                stmt.value = unsafe { box_from_raw(value_raw) };
                r
            };
        } else {
            log::error!(
                "[XKB-{:03}] Default defined for unknown field \"{}\"; Ignored\n",
                XKB_ERROR_UNKNOWN_DEFAULT_FIELD as i32,
                field
            );
            return info.keymap_info.strict & PARSER_NO_UNKNOWN_COMPAT_GLOBAL_FIELDS == 0;
        }
    } // close else from ExprResolveLhs
    ret
}
fn HandleInterpBody(
    info: &mut CompatInfo<'_>,
    defs: &mut [VarDef],
    si: &mut SymInterpInfo,
) -> bool {
    let mut ok: bool = true;
    for def in defs {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: Option<&ExprDef> = None;
        if !ExprResolveLhs(
            info.ctx(),
            def.name.as_deref().unwrap(),
            &mut elem,
            &mut field,
            &mut arrayNdx,
        ) {
            ok = false;
        } else if !elem.is_empty() {
            log::error!("Cannot set a global default value for \"{}\" element from within an interpret statement; Move assignment to \"{}.{}\" to the global file scope\n",
                elem,
                elem,
                field);
            ok = false;
        } else {
            // Break the borrow from ExprResolveLhs by going through raw pointers
            let field_ptr: *const str = field;
            let arrayNdx_ptr = arrayNdx.map(|r| r as *const ExprDef);
            let arrayNdx_ref = arrayNdx_ptr.map(|p| unsafe { &*p });
            let value_ref = def.value.as_deref_mut().unwrap();
            if !SetInterpField(info, si, unsafe { &*field_ptr }, arrayNdx_ref, value_ref) {
                ok = false;
            }
        }
    }
    ok
}
fn HandleInterpDef(info: &mut CompatInfo<'_>, def: &mut InterpDef) -> bool {
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
            action: xkb_action::None,
            actions: Vec::new(),
        },
    };
    if !ResolveStateAndPredicate(def.match_0.as_deref(), &mut pred, &mut mods, info) {
        log::error!("Couldn't determine matching modifiers; Symbol interpretation ignored\n");
        return false;
    }
    si = info.default_interp.clone();
    si.merge = def.merge;
    si.interp.sym = def.sym;
    si.interp.match_0 = pred;
    si.interp.mods = mods;
    if !HandleInterpBody(info, &mut def.def, &mut si) {
        info.errorCount += 1;
        return false;
    }
    if !AddInterp(info, &mut si, true) {
        info.errorCount += 1;
        return false;
    }
    true
}
fn HandleLedMapDef(info: &mut CompatInfo<'_>, def: &mut LedMapDef) -> bool {
    let mut ledi: LedInfo = info.default_led;
    ledi.merge = def.merge;
    ledi.led.name = def.name;
    let mut ok: bool = true;
    for var in def.body.iter_mut() {
        let mut elem: &str = "";
        let mut field: &str = "";
        let mut arrayNdx: Option<&ExprDef> = None;
        if !ExprResolveLhs(
            info.ctx(),
            var.name.as_deref().unwrap(),
            &mut elem,
            &mut field,
            &mut arrayNdx,
        ) {
            ok = false;
        } else if !elem.is_empty() {
            log::error!("[XKB-{:03}] Cannot set defaults for \"{}\" element in indicator map; Assignment to {}.{} ignored\n",
                XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE as i32,
                elem,
                elem,
                field);
            ok = false;
        } else if !SetLedMapField(
            info,
            &mut ledi,
            field,
            arrayNdx
                .map(|r| r as *const ExprDef)
                .map(|p| unsafe { &*p }),
            &mut var.value,
        ) {
            ok = false;
        }
    }
    ok && AddLedMap(info, &mut ledi, true)
}
fn HandleCompatMapFile(info: &mut CompatInfo<'_>, file: &mut XkbFile) {
    {
        let mut ok: bool;
        info.name = if file.name.is_empty() {
            None
        } else {
            Some(file.name.clone())
        };
        for stmt in file.defs.iter_mut() {
            match stmt {
                Statement::Include(incl) => {
                    ok = HandleIncludeCompatMap(info, &mut **incl);
                }
                Statement::Interp(ip) => {
                    ok = HandleInterpDef(info, &mut **ip);
                }
                Statement::GroupCompat(_) => {
                    log::debug!("The \"group\" statement in compat is unsupported; Ignored\n");
                    ok = true;
                }
                Statement::LedMap(lm) => {
                    ok = HandleLedMapDef(info, &mut **lm);
                }
                Statement::Var(var) => {
                    ok = HandleGlobalVar(info, &mut **var);
                }
                Statement::VMod(vmod) => {
                    ok = HandleVModDef(info.keymap_info.ctx_mut(), &mut info.mods, &**vmod);
                }
                Statement::Unknown(unk) => {
                    log::error!(
                        "[XKB-{:03}] Unsupported compatibility {} statement \"{}\"; Ignoring\n",
                        XKB_ERROR_UNKNOWN_STATEMENT as i32,
                        if unk.common.type_0 == STMT_UNKNOWN_COMPOUND {
                            "compound"
                        } else {
                            "declaration"
                        },
                        &unk.name
                    );
                    ok = (*info.keymap_info).strict & PARSER_NO_UNKNOWN_STATEMENTS == 0;
                }
                _ => {
                    log::error!(
                        "Compat files may not include other types; Ignoring {}\n",
                        stmt_type_to_string(stmt.stmt_type())
                    );
                    ok = false;
                }
            }
            if !ok {
                info.errorCount += 1;
            }
            if info.errorCount > 10_i32 {
                log::error!("Abandoning compatibility map \"{}\"\n", safe_map_name(file));
                break;
            }
        }
    }
}
fn CopyInterps(info: &CompatInfo<'_>, needSymbol: bool, pred: u32, collect: &mut collect) {
    for i in 0..info.interps.len() {
        let si = &info.interps[i];
        if si.interp.match_0 == pred
            && (si.interp.sym != XKB_KEY_NoSymbol as u32) as i32 == needSymbol as i32
        {
            collect.sym_interprets.push(si.interp.clone());
        }
    }
}
fn CopyLedMapDefsToKeymap(info: &mut CompatInfo<'_>) {
    let keymap = info.keymap_info.keymap_mut();
    let mut c2rust_current_block_11: u64;
    let mut idx: u32 = 0_u32;
    while idx < info.num_leds {
        let ledi_led = info.leds[idx as usize].led;
        let is_default = std::ptr::eq(
            &info.leds[idx as usize] as *const LedInfo,
            &info.default_led as *const LedInfo,
        );
        let led_name_text = if is_default {
            "default"
        } else {
            xkb_atom_text(&keymap.ctx.atom_table, info.leds[idx as usize].led.name)
        };
        let mut i: u32;
        i = 0_u32;
        while i < keymap.num_leds {
            if keymap.leds[i as usize].name == ledi_led.name {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i >= keymap.num_leds {
            log::debug!("Indicator name \"{}\" was not declared in the keycodes section; Adding new indicator\n",
                led_name_text);
            i = 0_u32;
            while i < keymap.num_leds {
                if keymap.leds[i as usize].name == XKB_ATOM_NONE {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i >= keymap.num_leds {
                if i >= XKB_MAX_LEDS {
                    log::error!(
                        "Too many indicators (maximum is {}); Indicator name \"{}\" ignored\n",
                        (std::mem::size_of::<xkb_led_mask_t>()).wrapping_mul(8_usize) as u32,
                        led_name_text
                    );
                    c2rust_current_block_11 = 792017965103506125;
                } else {
                    i = keymap.num_leds;
                    keymap.num_leds = keymap.num_leds.wrapping_add(1);
                    c2rust_current_block_11 = 17860125682698302841;
                }
            } else {
                c2rust_current_block_11 = 17860125682698302841;
            }
        } else {
            c2rust_current_block_11 = 17860125682698302841;
        }
        if c2rust_current_block_11 == 17860125682698302841 {
            keymap.leds[i as usize] = ledi_led;
            let led = &mut keymap.leds[i as usize];
            if led.which_groups as i32 == 0_i32
                && (led.groups != 0_u32 || led.pending_groups as i32 != 0)
            {
                led.which_groups = XKB_STATE_LAYOUT_EFFECTIVE;
            }
            if led.which_mods == 0_u32 && led.mods.mods != 0_u32 {
                led.which_mods = XKB_STATE_MODS_EFFECTIVE;
            }
        }
        idx = idx.wrapping_add(1);
    }
}
fn CopyCompatToKeymap(info: &mut CompatInfo<'_>) -> bool {
    // Collect sym_interprets first (doesn't need keymap)
    let sym_interprets = if !info.interps.is_empty() {
        let mut collect: collect = collect {
            sym_interprets: Vec::new(),
        };
        CopyInterps(info, true, MATCH_EXACTLY, &mut collect);
        CopyInterps(info, true, MATCH_ALL, &mut collect);
        CopyInterps(info, true, MATCH_NONE, &mut collect);
        CopyInterps(info, true, MATCH_ANY, &mut collect);
        CopyInterps(info, true, MATCH_ANY_OR_NONE, &mut collect);
        CopyInterps(info, false, MATCH_EXACTLY, &mut collect);
        CopyInterps(info, false, MATCH_ALL, &mut collect);
        CopyInterps(info, false, MATCH_NONE, &mut collect);
        CopyInterps(info, false, MATCH_ANY, &mut collect);
        CopyInterps(info, false, MATCH_ANY_OR_NONE, &mut collect);
        Some(collect.sym_interprets)
    } else {
        None
    };
    // Now get keymap and assign everything
    let keymap = info.keymap_info.keymap_mut();
    keymap.compat_section_name = match &info.name {
        Some(s) => s.clone(),
        None => String::new(),
    };
    xkb_escape_map_name(&mut keymap.compat_section_name);
    keymap.mods = info.mods;
    if let Some(interps) = sym_interprets {
        keymap.sym_interprets = interps;
    }
    // CopyLedMapDefsToKeymap needs keymap dropped first since it gets its own
    drop(keymap);
    CopyLedMapDefsToKeymap(info);
    true
}
pub fn CompileCompatMap(file: Option<&mut XkbFile>, keymap_info: &mut xkb_keymap_info) -> bool {
    let mods = keymap_info.keymap_ref().mods;
    let mut info = CompatInfo::new(keymap_info);
    InitCompatInfo(&mut info, 0_u32, &mods);
    if let Some(file) = file {
        HandleCompatMapFile(&mut info, file);
    }
    if (info.errorCount == 0_i32) && CopyCompatToKeymap(&mut info) {
        ClearCompatInfo(&mut info);
        return true;
    }
    ClearCompatInfo(&mut info);
    false
}
use crate::xkb::context::xkb_context_get_log_verbosity;
use crate::xkb::shared_types::*;
