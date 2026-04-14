#[derive(Clone)]
pub struct xkb_compose_table {
    pub refcnt: i32,
    pub format: xkb_compose_format,
    pub flags: xkb_compose_compile_flags,
    pub ctx: *mut xkb_context,
    pub locale: *mut i8,
    pub utf8: Vec<i8>,
    pub nodes: Vec<compose_node>,
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
/// Leaf node: bits 0..30 = utf8 index, bit 31 = is_leaf (always true).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub utf8_is_leaf: u32,
    pub keysym: xkb_keysym_t,
}
impl C2Rust_Unnamed_3 {
    #[inline]
    pub fn utf8(&self) -> u32 {
        self.utf8_is_leaf & 0x7FFF_FFFF
    }
}
/// Internal node: bits 0..30 = pad, bit 31 = is_leaf (always false).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _pad_is_leaf: u32,
    pub eqkid: u32,
}
/// Tag-only accessor: bit 31 = is_leaf discriminant.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub _pad_is_leaf: u32,
}
impl C2Rust_Unnamed_5 {
    #[inline]
    pub fn is_leaf(&self) -> bool {
        (self._pad_is_leaf >> 31) != 0
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xkb_compose_table_entry {
    pub sequence_length: usize,
    pub sequence: *mut xkb_keysym_t,
    pub keysym: xkb_keysym_t,
    pub utf8: *const i8,
}

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
        let mut node: *const compose_node = (&(*table).nodes).as_ptr().offset(p as isize);
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
        if (&(*table).nodes).len() <= 1 {
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

extern "C" {
    pub fn xkb_compose_table_new_from_locale(
        context: *mut xkb_context,
        locale: *const i8,
        flags: xkb_compose_compile_flags,
    ) -> *mut xkb_compose_table;
    pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);
}
