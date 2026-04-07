pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod types_h {
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_char {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut ::core::ffi::c_char,
    }
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
        pub text_next: size_t,
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
    use super::__stddef_size_t_h::size_t;
    use super::atom_h::atom_table;
    use super::darray_h::darray_size_t;

    use super::xkbcommon_h::{xkb_log_level, xkb_rule_names};
}
pub mod atom_h {
    extern "C" {
        pub type atom_table;
    }
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
    pub type xkb_keysym_t = uint32_t;
    use super::stdint_uintn_h::uint32_t;
}
pub mod table_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_compose_table {
        pub refcnt: ::core::ffi::c_int,
        pub format: xkb_compose_format,
        pub flags: xkb_compose_compile_flags,
        pub ctx: *mut xkb_context,
        pub locale: *mut ::core::ffi::c_char,
        pub utf8: darray_char,
        pub nodes: C2Rust_Unnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_1 {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut compose_node,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct compose_node {
        pub keysym: xkb_keysym_t,
        pub lokid: uint32_t,
        pub hikid: uint32_t,
        pub c2rust_unnamed: C2Rust_Unnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union C2Rust_Unnamed_2 {
        pub c2rust_unnamed: C2Rust_Unnamed_5,
        pub internal: C2Rust_Unnamed_4,
        pub leaf: C2Rust_Unnamed_3,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_3 {
        #[bitfield(name = "utf8", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub utf8_is_leaf: [u8; 4],
        pub keysym: xkb_keysym_t,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        #[bitfield(name = "_pad", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
        pub eqkid: uint32_t,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        #[bitfield(name = "_pad", ty = "uint32_t", bits = "0..=30")]
        #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
        pub _pad_is_leaf: [u8; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct xkb_compose_table_entry {
        pub sequence_length: size_t,
        pub sequence: *mut xkb_keysym_t,
        pub keysym: xkb_keysym_t,
        pub utf8: *const ::core::ffi::c_char,
    }
    use super::__stddef_size_t_h::size_t;
    use super::context_h::xkb_context;
    use super::darray_h::{darray_char, darray_size_t};
    use super::stdint_uintn_h::uint32_t;
    use super::xkbcommon_compose_h::{xkb_compose_compile_flags, xkb_compose_format};
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod xkbcommon_compose_h {
    pub type xkb_compose_compile_flags = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
    pub type xkb_compose_format = ::core::ffi::c_uint;
    pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
}
pub mod compose_iter_h {
    pub type xkb_compose_table_iter_t =
        Option<unsafe extern "C" fn(*mut xkb_compose_table_entry, *mut ::core::ffi::c_void) -> ()>;
    use super::table_h::xkb_compose_table_entry;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::compose_iter_h::xkb_compose_table_iter_t;
pub use self::context_h::{xkb_context, C2Rust_Unnamed, C2Rust_Unnamed_0};
pub use self::darray_h::{darray_char, darray_size_t};
pub use self::internal::__va_list_tag;
pub use self::stdint_uintn_h::uint32_t;
pub use self::table_h::{
    compose_node, xkb_compose_table, xkb_compose_table_entry, C2Rust_Unnamed_1, C2Rust_Unnamed_2,
    C2Rust_Unnamed_3, C2Rust_Unnamed_4, C2Rust_Unnamed_5,
};
pub use self::types_h::__uint32_t;
pub use self::xkbcommon_compose_h::{
    xkb_compose_compile_flags, xkb_compose_format, XKB_COMPOSE_COMPILE_NO_FLAGS,
    XKB_COMPOSE_FORMAT_TEXT_V1,
};
pub use self::xkbcommon_h::{
    xkb_keysym_t, xkb_log_level, xkb_rule_names, XKB_LOG_LEVEL_CRITICAL, XKB_LOG_LEVEL_DEBUG,
    XKB_LOG_LEVEL_ERROR, XKB_LOG_LEVEL_INFO, XKB_LOG_LEVEL_WARNING,
};
unsafe extern "C" fn for_each_helper(
    mut table: *mut xkb_compose_table,
    mut iter: xkb_compose_table_iter_t,
    mut data: *mut ::core::ffi::c_void,
    mut syms: *mut xkb_keysym_t,
    mut nsyms: size_t,
    mut p: uint32_t,
) {
    unsafe {
        if p == 0 {
            return;
        }
        let mut node: *const compose_node =
            (*table).nodes.item.offset(p as isize) as *mut compose_node;
        for_each_helper(table, iter, data, syms, nsyms, (*node).lokid);
        let c2rust_fresh0 = nsyms;
        nsyms = nsyms.wrapping_add(1);
        *syms.offset(c2rust_fresh0 as isize) = (*node).keysym;
        if (*node).c2rust_unnamed.c2rust_unnamed.is_leaf() {
            let mut entry: xkb_compose_table_entry = xkb_compose_table_entry {
                sequence_length: nsyms,
                sequence: syms,
                keysym: (*node).c2rust_unnamed.leaf.keysym,
                utf8: (*table)
                    .utf8
                    .item
                    .offset((*node).c2rust_unnamed.leaf.utf8() as isize)
                    as *mut ::core::ffi::c_char,
            };
            iter.expect("non-null function pointer")(&raw mut entry, data);
        } else {
            for_each_helper(
                table,
                iter,
                data,
                syms,
                nsyms,
                (*node).c2rust_unnamed.internal.eqkid,
            );
        }
        nsyms = nsyms.wrapping_sub(1);
        for_each_helper(table, iter, data, syms, nsyms, (*node).hikid);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_compose_table_for_each(
    mut table: *mut xkb_compose_table,
    mut iter: xkb_compose_table_iter_t,
    mut data: *mut ::core::ffi::c_void,
) {
    unsafe {
        if (*table).nodes.size <= 1 as darray_size_t {
            return;
        }
        let mut syms: [xkb_keysym_t; 10] = [0; 10];
        for_each_helper(
            table,
            iter,
            data,
            &raw mut syms as *mut xkb_keysym_t,
            0 as size_t,
            1 as uint32_t,
        );
    }
}
