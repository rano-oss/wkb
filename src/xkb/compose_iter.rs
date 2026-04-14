#[derive(Clone)]
pub struct xkb_compose_table {
    pub refcnt: i32,
    pub format: xkb_compose_format,
    pub flags: xkb_compose_compile_flags,
    pub ctx: *mut xkb_context,
    pub locale: *mut i8,
    pub utf8: Vec<i8>,
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
    pub lokid: u32,
    pub hikid: u32,
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
    #[bitfield(name = "utf8", ty = "u32", bits = "0..=30")]
    #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
    pub utf8_is_leaf: [u8; 4],
    pub keysym: xkb_keysym_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    #[bitfield(name = "_pad", ty = "u32", bits = "0..=30")]
    #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
    pub _pad_is_leaf: [u8; 4],
    pub eqkid: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    #[bitfield(name = "_pad", ty = "u32", bits = "0..=30")]
    #[bitfield(name = "is_leaf", ty = "bool", bits = "31..=31")]
    pub _pad_is_leaf: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_compose_table_entry {
    pub sequence_length: usize,
    pub sequence: *mut xkb_keysym_t,
    pub keysym: xkb_keysym_t,
    pub utf8: *const i8,
}

use crate::xkb::shared_types::darray_size_t;
use crate::xkb::shared_types::xkb_keysym_t;
pub type xkb_compose_compile_flags = u32;
pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;
pub type xkb_compose_format = u32;
pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;
pub type xkb_compose_table_iter_t =
    Option<unsafe fn(*mut xkb_compose_table_entry, *mut ::core::ffi::c_void) -> ()>;

unsafe fn for_each_helper(
    mut table: *mut xkb_compose_table,
    mut iter: xkb_compose_table_iter_t,
    mut data: *mut ::core::ffi::c_void,
    mut syms: *mut xkb_keysym_t,
    mut nsyms: usize,
    mut p: u32,
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
                    .as_ptr()
                    .offset((*node).c2rust_unnamed.leaf.utf8() as isize)
                    as *mut i8,
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

pub unsafe fn xkb_compose_table_for_each(
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
            0 as usize,
            1 as u32,
        );
    }
}
use crate::xkb::shared_types::*;
