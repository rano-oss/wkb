pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}

pub mod context_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct xkb_context {
        pub refcnt: ::core::ffi::c_int,
        pub log_fn: Option<
            unsafe extern "C" fn(
                *mut xkb_context,
                xkb_log_level,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub log_level: xkb_log_level,
        pub log_verbosity: ::core::ffi::c_int,
        pub user_data: *mut ::core::ffi::c_void,
        pub names_dflt: xkb_rule_names,
        pub includes: C2Rust_Unnamed_0,
        pub failed_includes: C2Rust_Unnamed,
        pub atom_table: *mut atom_table,
        pub x11_atom_cache: *mut ::core::ffi::c_void,
        pub text_buffer: [::core::ffi::c_char; 2048],
        pub text_next: usize,
        #[bitfield(name = "use_environment_names", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "use_secure_getenv", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "pending_default_includes", ty = "bool", bits = "2..=2")]
        pub use_environment_names_use_secure_getenv_pending_default_includes: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }

    use super::atom_h::{atom_table, xkb_atom_t};
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
    extern "C" {
        pub fn xkb_atom_text(ctx: *mut xkb_context, atom: xkb_atom_t)
            -> *const ::core::ffi::c_char;
        pub fn xkb_log(
            ctx: *mut xkb_context,
            level: xkb_log_level,
            verbosity: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    use super::darray_h::darray_size_t;
    extern "C" {
        pub type atom_table;
    }
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
}
pub mod xkbcommon_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_rule_names {
        pub rules: *const ::core::ffi::c_char,
        pub model: *const ::core::ffi::c_char,
        pub layout: *const ::core::ffi::c_char,
        pub variant: *const ::core::ffi::c_char,
        pub options: *const ::core::ffi::c_char,
    }
    pub type xkb_log_level = ::core::ffi::c_uint;
    pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
    pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
    pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
    pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
    pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
    pub type xkb_mod_mask_t = u32;
    pub type xkb_mod_index_t = u32;
    pub type xkb_keysym_t = u32;
}
pub mod keymap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod_set {
        pub mods: [xkb_mod; 32],
        pub num_mods: xkb_mod_index_t,
        pub explicit_vmods: xkb_mod_mask_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_mod {
        pub name: xkb_atom_t,
        pub type_0: mod_type,
        pub mapping: xkb_mod_mask_t,
    }
    pub type mod_type = ::core::ffi::c_uint;
    pub const MOD_BOTH: mod_type = 3;
    pub const MOD_VIRT: mod_type = 2;
    pub const MOD_REAL: mod_type = 1;
    pub const XKB_MAX_MODS: xkb_mod_index_t = (::core::mem::size_of::<xkb_mod_mask_t>() as usize)
        .wrapping_mul(CHAR_BIT as usize)
        as xkb_mod_index_t;
    use super::atom_h::xkb_atom_t;
    use super::limits_h::CHAR_BIT;
    use super::xkbcommon_h::{xkb_mod_index_t, xkb_mod_mask_t};
}
pub mod messages_codes_h {
    pub type xkb_log_verbosity = ::core::ffi::c_int;
    pub const XKB_LOG_VERBOSITY_DEFAULT: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_COMPREHENSIVE: xkb_log_verbosity = 11;
    pub const XKB_LOG_VERBOSITY_VERBOSE: xkb_log_verbosity = 10;
    pub const XKB_LOG_VERBOSITY_DETAILED: xkb_log_verbosity = 5;
    pub const XKB_LOG_VERBOSITY_BRIEF: xkb_log_verbosity = 1;
    pub const XKB_LOG_VERBOSITY_MINIMAL: xkb_log_verbosity = 0;
    pub const XKB_LOG_VERBOSITY_SILENT: xkb_log_verbosity = -1;
}
pub mod ast_h {
    pub type stmt_type = ::core::ffi::c_uint;
    pub const _STMT_NUM_VALUES: stmt_type = 37;
    pub const STMT_UNKNOWN_COMPOUND: stmt_type = 36;
    pub const STMT_UNKNOWN_DECLARATION: stmt_type = 35;
    pub const STMT_LED_NAME: stmt_type = 34;
    pub const STMT_LED_MAP: stmt_type = 33;
    pub const STMT_GROUP_COMPAT: stmt_type = 32;
    pub const STMT_MODMAP: stmt_type = 31;
    pub const STMT_SYMBOLS: stmt_type = 30;
    pub const STMT_VMOD: stmt_type = 29;
    pub const STMT_INTERP: stmt_type = 28;
    pub const STMT_TYPE: stmt_type = 27;
    pub const STMT_VAR: stmt_type = 26;
    pub const STMT_EXPR_UNARY_PLUS: stmt_type = 25;
    pub const STMT_EXPR_INVERT: stmt_type = 24;
    pub const STMT_EXPR_NEGATE: stmt_type = 23;
    pub const STMT_EXPR_NOT: stmt_type = 22;
    pub const STMT_EXPR_ASSIGN: stmt_type = 21;
    pub const STMT_EXPR_DIVIDE: stmt_type = 20;
    pub const STMT_EXPR_MULTIPLY: stmt_type = 19;
    pub const STMT_EXPR_SUBTRACT: stmt_type = 18;
    pub const STMT_EXPR_ADD: stmt_type = 17;
    pub const STMT_EXPR_ACTION_LIST: stmt_type = 16;
    pub const STMT_EXPR_KEYSYM_LIST: stmt_type = 15;
    pub const STMT_EXPR_EMPTY_LIST: stmt_type = 14;
    pub const STMT_EXPR_ARRAY_REF: stmt_type = 13;
    pub const STMT_EXPR_FIELD_REF: stmt_type = 12;
    pub const STMT_EXPR_ACTION_DECL: stmt_type = 11;
    pub const STMT_EXPR_IDENT: stmt_type = 10;
    pub const STMT_EXPR_KEYSYM_LITERAL: stmt_type = 9;
    pub const STMT_EXPR_KEYNAME_LITERAL: stmt_type = 8;
    pub const STMT_EXPR_BOOLEAN_LITERAL: stmt_type = 7;
    pub const STMT_EXPR_FLOAT_LITERAL: stmt_type = 6;
    pub const STMT_EXPR_INTEGER_LITERAL: stmt_type = 5;
    pub const STMT_EXPR_STRING_LITERAL: stmt_type = 4;
    pub const STMT_ALIAS: stmt_type = 3;
    pub const STMT_KEYCODE: stmt_type = 2;
    pub const STMT_INCLUDE: stmt_type = 1;
    pub const STMT_UNKNOWN: stmt_type = 0;
    pub type merge_mode = ::core::ffi::c_uint;
    pub const _MERGE_MODE_NUM_ENTRIES: merge_mode = 4;
    pub const MERGE_REPLACE: merge_mode = 3;
    pub const MERGE_OVERRIDE: merge_mode = 2;
    pub const MERGE_AUGMENT: merge_mode = 1;
    pub const MERGE_DEFAULT: merge_mode = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _ParseCommon {
        pub next: *mut _ParseCommon,
        pub type_0: stmt_type,
    }
    pub type ParseCommon = _ParseCommon;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ExprDef {
        pub common: ParseCommon,
        pub ident: ExprIdent,
        pub string: ExprString,
        pub boolean: ExprBoolean,
        pub integer: ExprInteger,
        pub key_name: ExprKeyName,
        pub keysym: ExprKeySym,
        pub binary: ExprBinary,
        pub unary: ExprUnary,
        pub field_ref: ExprFieldRef,
        pub array_ref: ExprArrayRef,
        pub action: ExprAction,
        pub actions: ExprActionList,
        pub keysym_list: ExprKeysymList,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeysymList {
        pub common: ParseCommon,
        pub syms: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_1 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprActionList {
        pub common: ParseCommon,
        pub actions: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprAction {
        pub common: ParseCommon,
        pub name: xkb_atom_t,
        pub args: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprArrayRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
        pub entry: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprFieldRef {
        pub common: ParseCommon,
        pub element: xkb_atom_t,
        pub field: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprUnary {
        pub common: ParseCommon,
        pub child: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBinary {
        pub common: ParseCommon,
        pub left: *mut ExprDef,
        pub right: *mut ExprDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeySym {
        pub common: ParseCommon,
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprKeyName {
        pub common: ParseCommon,
        pub key_name: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprInteger {
        pub common: ParseCommon,
        pub ival: i64,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprBoolean {
        pub common: ParseCommon,
        pub set: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprString {
        pub common: ParseCommon,
        pub str: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ExprIdent {
        pub common: ParseCommon,
        pub ident: xkb_atom_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VModDef {
        pub common: ParseCommon,
        pub merge: merge_mode,
        pub name: xkb_atom_t,
        pub value: *mut ExprDef,
    }
    use super::atom_h::xkb_atom_t;
    use super::darray_h::darray_size_t;
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod limits_h {
    pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
pub mod text_h {
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_mod_set};
    use super::xkbcommon_h::xkb_mod_mask_t;
    extern "C" {
        pub fn ModMaskText(
            ctx: *mut xkb_context,
            type_0: mod_type,
            mods: *const xkb_mod_set,
            mask: xkb_mod_mask_t,
        ) -> *const ::core::ffi::c_char;
    }
}
pub mod expr_h {
    use super::ast_h::ExprDef;
    use super::context_h::xkb_context;
    use super::keymap_h::{mod_type, xkb_mod_set};
    use super::xkbcommon_h::xkb_mod_mask_t;
    extern "C" {
        pub fn ExprResolveModMask(
            ctx: *mut xkb_context,
            expr: *const ExprDef,
            mod_type: mod_type,
            mods: *const xkb_mod_set,
            mask_rtrn: *mut xkb_mod_mask_t,
        ) -> bool;
    }
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

use self::assert_h::__assert_fail;
pub use self::ast_h::{
    _ParseCommon, merge_mode, stmt_type, C2Rust_Unnamed_1, ExprAction, ExprActionList,
    ExprArrayRef, ExprBinary, ExprBoolean, ExprDef, ExprFieldRef, ExprIdent, ExprInteger,
    ExprKeyName, ExprKeySym, ExprKeysymList, ExprString, ExprUnary, ParseCommon, VModDef,
    _MERGE_MODE_NUM_ENTRIES, _STMT_NUM_VALUES, MERGE_AUGMENT, MERGE_DEFAULT, MERGE_OVERRIDE,
    MERGE_REPLACE, STMT_ALIAS, STMT_EXPR_ACTION_DECL, STMT_EXPR_ACTION_LIST, STMT_EXPR_ADD,
    STMT_EXPR_ARRAY_REF, STMT_EXPR_ASSIGN, STMT_EXPR_BOOLEAN_LITERAL, STMT_EXPR_DIVIDE,
    STMT_EXPR_EMPTY_LIST, STMT_EXPR_FIELD_REF, STMT_EXPR_FLOAT_LITERAL, STMT_EXPR_IDENT,
    STMT_EXPR_INTEGER_LITERAL, STMT_EXPR_INVERT, STMT_EXPR_KEYNAME_LITERAL, STMT_EXPR_KEYSYM_LIST,
    STMT_EXPR_KEYSYM_LITERAL, STMT_EXPR_MULTIPLY, STMT_EXPR_NEGATE, STMT_EXPR_NOT,
    STMT_EXPR_STRING_LITERAL, STMT_EXPR_SUBTRACT, STMT_EXPR_UNARY_PLUS, STMT_GROUP_COMPAT,
    STMT_INCLUDE, STMT_INTERP, STMT_KEYCODE, STMT_LED_MAP, STMT_LED_NAME, STMT_MODMAP,
    STMT_SYMBOLS, STMT_TYPE, STMT_UNKNOWN, STMT_UNKNOWN_COMPOUND, STMT_UNKNOWN_DECLARATION,
    STMT_VAR, STMT_VMOD,
};
pub use self::atom_h::{atom_table, xkb_atom_t};
pub use self::context_h::{xkb_atom_text, xkb_context, xkb_log, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::darray_size_t;
use self::expr_h::ExprResolveModMask;
pub use self::internal::{__va_list_tag, __CHAR_BIT__};
pub use self::keymap_h::{
    mod_type, xkb_mod, xkb_mod_set, MOD_BOTH, MOD_REAL, MOD_VIRT, XKB_MAX_MODS,
};
pub use self::limits_h::CHAR_BIT;
pub use self::messages_codes_h::{
    xkb_log_verbosity, XKB_LOG_VERBOSITY_BRIEF, XKB_LOG_VERBOSITY_COMPREHENSIVE,
    XKB_LOG_VERBOSITY_DEFAULT, XKB_LOG_VERBOSITY_DETAILED, XKB_LOG_VERBOSITY_MINIMAL,
    XKB_LOG_VERBOSITY_SILENT, XKB_LOG_VERBOSITY_VERBOSE,
};
pub use self::stdbool_h::{false_0, true_0};
use self::text_h::ModMaskText;
pub use self::xkbcommon_h::{
    xkb_keysym_t, xkb_log_level, xkb_mod_index_t, xkb_mod_mask_t, xkb_rule_names,
    XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG, XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO,
    XKB_LOG_LEVEL_WARNING,
};
#[no_mangle]
pub unsafe extern "C" fn InitVMods(
    mut info: *mut xkb_mod_set,
    mut mods: *const xkb_mod_set,
    mut reset: bool,
) {
    unsafe {
        *info = *mods;
        if !reset {
            return;
        }
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        let mut vmod: xkb_mod_index_t = 0 as xkb_mod_index_t;
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw mut (*info).mods as *mut xkb_mod;
        while vmod < (*info).num_mods {
            (*mod_0).mapping = 0 as xkb_mod_mask_t;
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*info).explicit_vmods = 0 as xkb_mod_mask_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MergeModSets(
    mut ctx: *mut xkb_context,
    mut into: *mut xkb_mod_set,
    mut from: *const xkb_mod_set,
    mut merge: merge_mode,
) {
    unsafe {
        let clobber: bool = merge as ::core::ffi::c_uint
            != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
        let mut vmod: xkb_mod_index_t = 0;
        let mut mod_0: *const xkb_mod = ::core::ptr::null::<xkb_mod>();
        if (*into).num_mods <= (*from).num_mods {
        } else {
            __assert_fail(
                b"into->num_mods <= from->num_mods\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"../src/xkbcomp/vmod.c\0".as_ptr() as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_uint,
                b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw const (*from).mods as *const xkb_mod;
        while vmod < (*from).num_mods {
            let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << vmod;
            if (*mod_0).type_0 as ::core::ffi::c_uint
                != MOD_VIRT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*mod_0).type_0 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
                    && (*mod_0).name == 0 as xkb_atom_t
                    || (*mod_0).type_0 as ::core::ffi::c_uint
                        & MOD_REAL as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                        && (*into).mods[vmod as usize].type_0 as ::core::ffi::c_uint
                            == (*mod_0).type_0 as ::core::ffi::c_uint
                        && (*mod_0).name != 0 as xkb_atom_t
                        && (*into).mods[vmod as usize].name == (*mod_0).name
                {
                } else {
                    __assert_fail(
                        b"(mod->type == 0 && mod->name == XKB_ATOM_NONE) || ((mod->type & MOD_REAL) && into->mods[vmod].type == mod->type && mod->name != XKB_ATOM_NONE && into->mods[vmod].name == mod->name)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        51 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            } else if (*into).mods[vmod as usize].type_0 as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            {
                if (*into).mods[vmod as usize].name == 0 as xkb_atom_t {
                } else {
                    __assert_fail(
                        b"into->mods[vmod].name == XKB_ATOM_NONE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        55 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*mod_0).type_0 as ::core::ffi::c_uint
                    == MOD_VIRT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        b"mod->type == MOD_VIRT\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        56 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*mod_0).name != 0 as xkb_atom_t {
                } else {
                    __assert_fail(
                        b"mod->name != XKB_ATOM_NONE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        57 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if vmod >= (*into).num_mods {
                } else {
                    __assert_fail(
                        b"vmod >= into->num_mods\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        58 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                (*into).mods[vmod as usize] = *mod_0;
                if (*from).explicit_vmods & mask != 0 {
                    (*into).explicit_vmods |= mask;
                }
            } else {
                if (*mod_0).type_0 as ::core::ffi::c_uint
                    == MOD_VIRT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        b"mod->type == MOD_VIRT\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        64 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*mod_0).name != 0 as xkb_atom_t {
                } else {
                    __assert_fail(
                        b"mod->name != XKB_ATOM_NONE\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*into).mods[vmod as usize].type_0 as ::core::ffi::c_uint
                    == (*mod_0).type_0 as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        b"into->mods[vmod].type == mod->type\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*into).mods[vmod as usize].name == (*mod_0).name {
                } else {
                    __assert_fail(
                        b"into->mods[vmod].name == mod->name\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"../src/xkbcomp/vmod.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        67 as ::core::ffi::c_uint,
                        b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if (*from).explicit_vmods & mask == 0 {
                    if (*mod_0).mapping == 0 as xkb_mod_mask_t {
                    } else {
                        __assert_fail(
                            b"mod->mapping == 0\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../src/xkbcomp/vmod.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            71 as ::core::ffi::c_uint,
                            b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                } else if (*into).explicit_vmods & mask == 0 {
                    if (*into).mods[vmod as usize].mapping == 0 as xkb_mod_mask_t {
                    } else {
                        __assert_fail(
                            b"into->mods[vmod].mapping == 0\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"../src/xkbcomp/vmod.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            74 as ::core::ffi::c_uint,
                            b"void MergeModSets(struct xkb_context *, struct xkb_mod_set *, const struct xkb_mod_set *, enum merge_mode)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    (*into).mods[vmod as usize].mapping = (*mod_0).mapping;
                    (*into).explicit_vmods |= mask;
                } else if (*mod_0).mapping != (*into).mods[vmod as usize].mapping {
                    let use_0: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*mod_0).mapping
                    } else {
                        (*into).mods[vmod as usize].mapping
                    };
                    let ignore: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*into).mods[vmod as usize].mapping
                    } else {
                        (*mod_0).mapping
                    };
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Virtual modifier %s mapping defined multiple times; Using %s, ignoring %s\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        xkb_atom_text(ctx, (*mod_0).name),
                        ModMaskText(ctx, MOD_REAL, from, use_0),
                        ModMaskText(ctx, MOD_REAL, from, ignore),
                    );
                    (*into).mods[vmod as usize].mapping = use_0;
                }
            }
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        (*into).num_mods = (*from).num_mods;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HandleVModDef(
    mut ctx: *mut xkb_context,
    mut mods: *mut xkb_mod_set,
    mut stmt: *mut VModDef,
) -> bool {
    unsafe {
        let mut mapping: xkb_mod_mask_t = 0 as xkb_mod_mask_t;
        if !(*stmt).value.is_null() {
            if !ExprResolveModMask(ctx, (*stmt).value, MOD_REAL, mods, &raw mut mapping) {
                xkb_log(
                    ctx,
                    XKB_LOG_LEVEL_ERROR,
                    XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                    b"Declaration of %s ignored\n\0".as_ptr() as *const ::core::ffi::c_char,
                    xkb_atom_text(ctx, (*stmt).name),
                );
                return false_0 != 0;
            }
        }
        let mut vmod: xkb_mod_index_t = 0;
        let mut mod_0: *mut xkb_mod = ::core::ptr::null_mut::<xkb_mod>();
        vmod = 0 as xkb_mod_index_t;
        mod_0 = &raw mut (*mods).mods as *mut xkb_mod;
        while vmod < (*mods).num_mods {
            if (*mod_0).name == (*stmt).name {
                if (*mod_0).type_0 as ::core::ffi::c_uint
                    != MOD_VIRT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_ERROR,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Can't add a virtual modifier named \"%s\"; there is already a non-virtual modifier with this name! Ignored\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        xkb_atom_text(ctx, (*mod_0).name),
                    );
                    return false_0 != 0;
                }
                let mask: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << vmod;
                if (*stmt).value.is_null() {
                    return true_0 != 0;
                } else if (*mods).explicit_vmods & mask == 0 {
                    (*mod_0).mapping = mapping;
                } else if (*mod_0).mapping != mapping {
                    let clobber: bool = (*stmt).merge as ::core::ffi::c_uint
                        != MERGE_AUGMENT as ::core::ffi::c_int as ::core::ffi::c_uint;
                    let use_0: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        mapping
                    } else {
                        (*mod_0).mapping
                    };
                    let ignore: xkb_mod_mask_t = if clobber as ::core::ffi::c_int != 0 {
                        (*mod_0).mapping
                    } else {
                        mapping
                    };
                    xkb_log(
                        ctx,
                        XKB_LOG_LEVEL_WARNING,
                        XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                        b"Virtual modifier %s mapping defined multiple times; Using %s, ignoring %s\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        xkb_atom_text(ctx, (*stmt).name),
                        ModMaskText(ctx, MOD_REAL, mods, use_0),
                        ModMaskText(ctx, MOD_REAL, mods, ignore),
                    );
                    (*mod_0).mapping = use_0;
                }
                (*mods).explicit_vmods |= mask;
                return true_0 != 0;
            }
            vmod = vmod.wrapping_add(1);
            mod_0 = mod_0.offset(1);
        }
        if (*mods).num_mods >= XKB_MAX_MODS {
            xkb_log(
                ctx,
                XKB_LOG_LEVEL_ERROR,
                XKB_LOG_VERBOSITY_MINIMAL as ::core::ffi::c_int,
                b"Cannot define virtual modifier %s: too many modifiers defined (maximum %u)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                xkb_atom_text(ctx, (*stmt).name),
                (::core::mem::size_of::<xkb_mod_mask_t>() as usize).wrapping_mul(8 as usize)
                    as xkb_mod_index_t,
            );
            return false_0 != 0;
        }
        (*mods).mods[(*mods).num_mods as usize].name = (*stmt).name;
        (*mods).mods[(*mods).num_mods as usize].type_0 = MOD_VIRT;
        (*mods).mods[(*mods).num_mods as usize].mapping = mapping;
        if !(*stmt).value.is_null() {
            let mask_0: xkb_mod_mask_t = (1 as xkb_mod_mask_t) << (*mods).num_mods;
            (*mods).explicit_vmods |= mask_0;
        }
        (*mods).num_mods = (*mods).num_mods.wrapping_add(1);
        return true_0 != 0;
    }
}
