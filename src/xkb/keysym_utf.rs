use c2rust_bitfields;
pub mod types_h {
    pub type __uint16_t = u16;
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod xkbcommon_h {
    pub type xkb_keysym_t = u32;
    use super::stdint_uintn_h::u32;
}
pub mod utf8_h {
    use super::stdint_uintn_h::u32;
    extern "C" {
        pub fn utf32_to_utf8(unichar: u32, buffer: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    }
}
pub mod xkbcommon_keysyms_h {
    pub const XKB_KEY_NoSymbol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const XKB_KEY_BackSpace: ::core::ffi::c_int = 0xff08 as ::core::ffi::c_int;
    pub const XKB_KEY_Clear: ::core::ffi::c_int = 0xff0b as ::core::ffi::c_int;
    pub const XKB_KEY_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
    pub const XKB_KEY_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
    pub const XKB_KEY_Delete: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
    pub const XKB_KEY_KP_Space: ::core::ffi::c_int = 0xff80 as ::core::ffi::c_int;
    pub const XKB_KEY_KP_Tab: ::core::ffi::c_int = 0xff89 as ::core::ffi::c_int;
    pub const XKB_KEY_KP_Enter: ::core::ffi::c_int = 0xff8d as ::core::ffi::c_int;
    pub const XKB_KEY_KP_Equal: ::core::ffi::c_int = 0xffbd as ::core::ffi::c_int;
    pub const XKB_KEY_KP_Multiply: ::core::ffi::c_int = 0xffaa as ::core::ffi::c_int;
    pub const XKB_KEY_KP_9: ::core::ffi::c_int = 0xffb9 as ::core::ffi::c_int;
    pub const XKB_KEY_space: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    pub const XKB_KEY_XF86Numeric0: ::core::ffi::c_int = 0x10081200 as ::core::ffi::c_int;
    pub const XKB_KEY_XF86Numeric9: ::core::ffi::c_int = 0x10081209 as ::core::ffi::c_int;
    pub const XKB_KEY_XF86NumericStar: xkb_keysym_t = 268964362 as xkb_keysym_t;
    pub const XKB_KEY_XF86NumericPound: xkb_keysym_t = 268964363 as xkb_keysym_t;
    use super::xkbcommon_h::xkb_keysym_t;
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn is_surrogate(mut cp: u32) -> bool {
        unsafe {
            return cp >= 0xd800 as u32 && cp <= 0xdfff as u32;
        }
    }
    use super::stdint_uintn_h::u32;
}
pub mod stdbool_h {
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_UNICODE_OFFSET: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UNICODE_SURROGATE_MIN: ::core::ffi::c_int =
        0x100d800 as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UNICODE_SURROGATE_MAX: ::core::ffi::c_int =
        0x100dfff as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UNICODE_MAX: ::core::ffi::c_int = 0x110ffff as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UTF8_MAX_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::keysym_h::{
    XKB_KEYSYM_UNICODE_MAX, XKB_KEYSYM_UNICODE_OFFSET, XKB_KEYSYM_UNICODE_SURROGATE_MAX,
    XKB_KEYSYM_UNICODE_SURROGATE_MIN, XKB_KEYSYM_UTF8_MAX_SIZE,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_uintn_h::{u32, uint16_t};
pub use self::types_h::{__uint16_t, __uint32_t};
use self::utf8_h::utf32_to_utf8;
pub use self::utils_h::is_surrogate;
pub use self::xkbcommon_h::xkb_keysym_t;
pub use self::xkbcommon_keysyms_h::{
    XKB_KEY_BackSpace, XKB_KEY_Clear, XKB_KEY_Delete, XKB_KEY_Escape, XKB_KEY_KP_Enter,
    XKB_KEY_KP_Equal, XKB_KEY_KP_Multiply, XKB_KEY_KP_Space, XKB_KEY_KP_Tab, XKB_KEY_NoSymbol,
    XKB_KEY_Return, XKB_KEY_XF86Numeric0, XKB_KEY_XF86Numeric9, XKB_KEY_XF86NumericPound,
    XKB_KEY_XF86NumericStar, XKB_KEY_space, XKB_KEY_KP_9,
};
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct codepair {
    #[bitfield(name = "keysym", ty = "uint16_t", bits = "0..=14")]
    #[bitfield(name = "deprecated", ty = "bool", bits = "15..=15")]
    pub keysym_deprecated: [u8; 2],
    pub ucs: uint16_t,
}
pub const NO_KEYSYM_UNICODE_CONVERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut keysymtab: [codepair; 763] = [codepair {
    keysym_deprecated: [0; 2],
    ucs: 0,
}; 763];
unsafe extern "C" fn bin_search(
    mut table: *const codepair,
    mut length: size_t,
    mut keysym: xkb_keysym_t,
) -> u32 {
    unsafe {
        let mut first: size_t = 0 as size_t;
        let mut last: size_t = length;
        if keysym < (*table.offset(0 as ::core::ffi::c_int as isize)).keysym() as xkb_keysym_t
            || keysym > (*table.offset(length as isize)).keysym() as xkb_keysym_t
        {
            return 0 as u32;
        }
        while last >= first {
            let mut mid: size_t = first.wrapping_add(last).wrapping_div(2 as size_t);
            if ((*table.offset(mid as isize)).keysym() as xkb_keysym_t) < keysym {
                first = mid.wrapping_add(1 as size_t);
            } else if (*table.offset(mid as isize)).keysym() as xkb_keysym_t > keysym {
                last = mid.wrapping_sub(1 as size_t);
            } else {
                return (*table.offset(mid as isize)).ucs as u32;
            }
        }
        return NO_KEYSYM_UNICODE_CONVERSION as u32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_utf32(mut keysym: xkb_keysym_t) -> u32 {
    unsafe {
        if keysym >= 0x20 as xkb_keysym_t && keysym <= 0x7e as xkb_keysym_t
            || keysym >= 0xa0 as xkb_keysym_t && keysym <= 0xff as xkb_keysym_t
        {
            return keysym as u32;
        }
        if keysym == XKB_KEY_KP_Space as xkb_keysym_t {
            return (XKB_KEY_space & 0x7f as ::core::ffi::c_int) as u32;
        }
        if keysym >= XKB_KEY_BackSpace as xkb_keysym_t && keysym <= XKB_KEY_Clear as xkb_keysym_t
            || keysym >= XKB_KEY_KP_Multiply as xkb_keysym_t
                && keysym <= XKB_KEY_KP_9 as xkb_keysym_t
            || keysym == XKB_KEY_Return as xkb_keysym_t
            || keysym == XKB_KEY_Escape as xkb_keysym_t
            || keysym == XKB_KEY_Delete as xkb_keysym_t
            || keysym == XKB_KEY_KP_Tab as xkb_keysym_t
            || keysym == XKB_KEY_KP_Enter as xkb_keysym_t
            || keysym == XKB_KEY_KP_Equal as xkb_keysym_t
        {
            return keysym as u32 & 0x7f as u32;
        }
        if keysym >= XKB_KEYSYM_UNICODE_SURROGATE_MIN as xkb_keysym_t
            && keysym <= XKB_KEYSYM_UNICODE_SURROGATE_MAX as xkb_keysym_t
        {
            return NO_KEYSYM_UNICODE_CONVERSION as u32;
        }
        if XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t <= keysym
            && keysym <= XKB_KEYSYM_UNICODE_MAX as xkb_keysym_t
        {
            return (keysym as u32).wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as u32);
        }
        if keysym >= XKB_KEY_XF86Numeric0 as xkb_keysym_t
            && keysym <= XKB_KEY_XF86Numeric9 as xkb_keysym_t
        {
            return (keysym as u32)
                .wrapping_sub(XKB_KEY_XF86Numeric0 as u32)
                .wrapping_add(0x30 as u32);
        }
        match keysym {
            268964362 => return 0x2a as u32,
            268964363 => return 0x23 as u32,
            _ => {
                return bin_search(
                    &raw const keysymtab as *const codepair,
                    (::core::mem::size_of::<[codepair; 763]>() as size_t)
                        .wrapping_div(::core::mem::size_of::<codepair>() as size_t)
                        .wrapping_sub(1 as size_t),
                    keysym,
                );
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_utf32_to_keysym(mut ucs: u32) -> xkb_keysym_t {
    unsafe {
        if ucs >= 0x20 as u32 && ucs <= 0x7e as u32 || ucs >= 0xa0 as u32 && ucs <= 0xff as u32 {
            return ucs as xkb_keysym_t;
        }
        if ucs >= (XKB_KEY_BackSpace & 0x7f as ::core::ffi::c_int) as u32
            && ucs <= (XKB_KEY_Clear & 0x7f as ::core::ffi::c_int) as u32
            || ucs == (XKB_KEY_Return & 0x7f as ::core::ffi::c_int) as u32
            || ucs == (XKB_KEY_Escape & 0x7f as ::core::ffi::c_int) as u32
        {
            return ucs as xkb_keysym_t | 0xff00 as xkb_keysym_t;
        }
        if ucs == (XKB_KEY_Delete & 0x7f as ::core::ffi::c_int) as u32 {
            return XKB_KEY_Delete as xkb_keysym_t;
        }
        if (ucs == 0 as u32
            || is_surrogate(ucs) as ::core::ffi::c_int != 0
            || ucs > 0x10ffff as u32) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return XKB_KEY_NoSymbol as xkb_keysym_t;
        }
        let mut i: size_t = 0 as size_t;
        while i
            < (::core::mem::size_of::<[codepair; 763]>() as usize)
                .wrapping_div(::core::mem::size_of::<codepair>() as usize)
        {
            if keysymtab[i as usize].ucs as u32 == ucs && !keysymtab[i as usize].deprecated() {
                return keysymtab[i as usize].keysym() as xkb_keysym_t;
            }
            i = i.wrapping_add(1);
        }
        return ucs as xkb_keysym_t | XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_utf8(
    mut keysym: xkb_keysym_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut codepoint: u32 = 0;
        if size < XKB_KEYSYM_UTF8_MAX_SIZE as size_t {
            return -1 as ::core::ffi::c_int;
        }
        codepoint = xkb_keysym_to_utf32(keysym);
        if codepoint == NO_KEYSYM_UNICODE_CONVERSION as u32 {
            return 0 as ::core::ffi::c_int;
        }
        return utf32_to_utf8(codepoint, buffer);
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        keysymtab = [
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x104 as uint16_t,
                };
                init.set_keysym(0x1a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2d8 as uint16_t,
                };
                init.set_keysym(0x1a2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x141 as uint16_t,
                };
                init.set_keysym(0x1a3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x13d as uint16_t,
                };
                init.set_keysym(0x1a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15a as uint16_t,
                };
                init.set_keysym(0x1a6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x160 as uint16_t,
                };
                init.set_keysym(0x1a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15e as uint16_t,
                };
                init.set_keysym(0x1aa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x164 as uint16_t,
                };
                init.set_keysym(0x1ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x179 as uint16_t,
                };
                init.set_keysym(0x1ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x17d as uint16_t,
                };
                init.set_keysym(0x1ae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x17b as uint16_t,
                };
                init.set_keysym(0x1af as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x105 as uint16_t,
                };
                init.set_keysym(0x1b1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2db as uint16_t,
                };
                init.set_keysym(0x1b2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x142 as uint16_t,
                };
                init.set_keysym(0x1b3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x13e as uint16_t,
                };
                init.set_keysym(0x1b5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15b as uint16_t,
                };
                init.set_keysym(0x1b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2c7 as uint16_t,
                };
                init.set_keysym(0x1b7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x161 as uint16_t,
                };
                init.set_keysym(0x1b9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15f as uint16_t,
                };
                init.set_keysym(0x1ba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x165 as uint16_t,
                };
                init.set_keysym(0x1bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x17a as uint16_t,
                };
                init.set_keysym(0x1bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2dd as uint16_t,
                };
                init.set_keysym(0x1bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x17e as uint16_t,
                };
                init.set_keysym(0x1be as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x17c as uint16_t,
                };
                init.set_keysym(0x1bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x154 as uint16_t,
                };
                init.set_keysym(0x1c0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x102 as uint16_t,
                };
                init.set_keysym(0x1c3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x139 as uint16_t,
                };
                init.set_keysym(0x1c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x106 as uint16_t,
                };
                init.set_keysym(0x1c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10c as uint16_t,
                };
                init.set_keysym(0x1c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x118 as uint16_t,
                };
                init.set_keysym(0x1ca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11a as uint16_t,
                };
                init.set_keysym(0x1cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10e as uint16_t,
                };
                init.set_keysym(0x1cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x110 as uint16_t,
                };
                init.set_keysym(0x1d0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x143 as uint16_t,
                };
                init.set_keysym(0x1d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x147 as uint16_t,
                };
                init.set_keysym(0x1d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x150 as uint16_t,
                };
                init.set_keysym(0x1d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x158 as uint16_t,
                };
                init.set_keysym(0x1d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16e as uint16_t,
                };
                init.set_keysym(0x1d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x170 as uint16_t,
                };
                init.set_keysym(0x1db as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x162 as uint16_t,
                };
                init.set_keysym(0x1de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x155 as uint16_t,
                };
                init.set_keysym(0x1e0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x103 as uint16_t,
                };
                init.set_keysym(0x1e3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x13a as uint16_t,
                };
                init.set_keysym(0x1e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x107 as uint16_t,
                };
                init.set_keysym(0x1e6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10d as uint16_t,
                };
                init.set_keysym(0x1e8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x119 as uint16_t,
                };
                init.set_keysym(0x1ea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b as uint16_t,
                };
                init.set_keysym(0x1ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10f as uint16_t,
                };
                init.set_keysym(0x1ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x111 as uint16_t,
                };
                init.set_keysym(0x1f0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x144 as uint16_t,
                };
                init.set_keysym(0x1f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x148 as uint16_t,
                };
                init.set_keysym(0x1f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x151 as uint16_t,
                };
                init.set_keysym(0x1f5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x159 as uint16_t,
                };
                init.set_keysym(0x1f8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16f as uint16_t,
                };
                init.set_keysym(0x1f9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x171 as uint16_t,
                };
                init.set_keysym(0x1fb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x163 as uint16_t,
                };
                init.set_keysym(0x1fe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2d9 as uint16_t,
                };
                init.set_keysym(0x1ff as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x126 as uint16_t,
                };
                init.set_keysym(0x2a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x124 as uint16_t,
                };
                init.set_keysym(0x2a6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x130 as uint16_t,
                };
                init.set_keysym(0x2a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11e as uint16_t,
                };
                init.set_keysym(0x2ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x134 as uint16_t,
                };
                init.set_keysym(0x2ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x127 as uint16_t,
                };
                init.set_keysym(0x2b1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x125 as uint16_t,
                };
                init.set_keysym(0x2b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x131 as uint16_t,
                };
                init.set_keysym(0x2b9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11f as uint16_t,
                };
                init.set_keysym(0x2bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x135 as uint16_t,
                };
                init.set_keysym(0x2bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10a as uint16_t,
                };
                init.set_keysym(0x2c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x108 as uint16_t,
                };
                init.set_keysym(0x2c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x120 as uint16_t,
                };
                init.set_keysym(0x2d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11c as uint16_t,
                };
                init.set_keysym(0x2d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16c as uint16_t,
                };
                init.set_keysym(0x2dd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15c as uint16_t,
                };
                init.set_keysym(0x2de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x10b as uint16_t,
                };
                init.set_keysym(0x2e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x109 as uint16_t,
                };
                init.set_keysym(0x2e6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x121 as uint16_t,
                };
                init.set_keysym(0x2f5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11d as uint16_t,
                };
                init.set_keysym(0x2f8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16d as uint16_t,
                };
                init.set_keysym(0x2fd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x15d as uint16_t,
                };
                init.set_keysym(0x2fe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x138 as uint16_t,
                };
                init.set_keysym(0x3a2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x156 as uint16_t,
                };
                init.set_keysym(0x3a3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x128 as uint16_t,
                };
                init.set_keysym(0x3a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x13b as uint16_t,
                };
                init.set_keysym(0x3a6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x112 as uint16_t,
                };
                init.set_keysym(0x3aa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x122 as uint16_t,
                };
                init.set_keysym(0x3ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x166 as uint16_t,
                };
                init.set_keysym(0x3ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x157 as uint16_t,
                };
                init.set_keysym(0x3b3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x129 as uint16_t,
                };
                init.set_keysym(0x3b5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x13c as uint16_t,
                };
                init.set_keysym(0x3b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x113 as uint16_t,
                };
                init.set_keysym(0x3ba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x123 as uint16_t,
                };
                init.set_keysym(0x3bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x167 as uint16_t,
                };
                init.set_keysym(0x3bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x14a as uint16_t,
                };
                init.set_keysym(0x3bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x14b as uint16_t,
                };
                init.set_keysym(0x3bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x100 as uint16_t,
                };
                init.set_keysym(0x3c0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x12e as uint16_t,
                };
                init.set_keysym(0x3c7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x116 as uint16_t,
                };
                init.set_keysym(0x3cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x12a as uint16_t,
                };
                init.set_keysym(0x3cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x145 as uint16_t,
                };
                init.set_keysym(0x3d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x14c as uint16_t,
                };
                init.set_keysym(0x3d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x136 as uint16_t,
                };
                init.set_keysym(0x3d3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x172 as uint16_t,
                };
                init.set_keysym(0x3d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x168 as uint16_t,
                };
                init.set_keysym(0x3dd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16a as uint16_t,
                };
                init.set_keysym(0x3de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x101 as uint16_t,
                };
                init.set_keysym(0x3e0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x12f as uint16_t,
                };
                init.set_keysym(0x3e7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x117 as uint16_t,
                };
                init.set_keysym(0x3ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x12b as uint16_t,
                };
                init.set_keysym(0x3ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x146 as uint16_t,
                };
                init.set_keysym(0x3f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x14d as uint16_t,
                };
                init.set_keysym(0x3f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x137 as uint16_t,
                };
                init.set_keysym(0x3f3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x173 as uint16_t,
                };
                init.set_keysym(0x3f9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x169 as uint16_t,
                };
                init.set_keysym(0x3fd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x16b as uint16_t,
                };
                init.set_keysym(0x3fe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x203e as uint16_t,
                };
                init.set_keysym(0x47e as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3002 as uint16_t,
                };
                init.set_keysym(0x4a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x300c as uint16_t,
                };
                init.set_keysym(0x4a2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x300d as uint16_t,
                };
                init.set_keysym(0x4a3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3001 as uint16_t,
                };
                init.set_keysym(0x4a4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30fb as uint16_t,
                };
                init.set_keysym(0x4a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30f2 as uint16_t,
                };
                init.set_keysym(0x4a6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a1 as uint16_t,
                };
                init.set_keysym(0x4a7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a3 as uint16_t,
                };
                init.set_keysym(0x4a8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a5 as uint16_t,
                };
                init.set_keysym(0x4a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a7 as uint16_t,
                };
                init.set_keysym(0x4aa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a9 as uint16_t,
                };
                init.set_keysym(0x4ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e3 as uint16_t,
                };
                init.set_keysym(0x4ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e5 as uint16_t,
                };
                init.set_keysym(0x4ad as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e7 as uint16_t,
                };
                init.set_keysym(0x4ae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30c3 as uint16_t,
                };
                init.set_keysym(0x4af as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30fc as uint16_t,
                };
                init.set_keysym(0x4b0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a2 as uint16_t,
                };
                init.set_keysym(0x4b1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a4 as uint16_t,
                };
                init.set_keysym(0x4b2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a6 as uint16_t,
                };
                init.set_keysym(0x4b3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30a8 as uint16_t,
                };
                init.set_keysym(0x4b4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30aa as uint16_t,
                };
                init.set_keysym(0x4b5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ab as uint16_t,
                };
                init.set_keysym(0x4b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ad as uint16_t,
                };
                init.set_keysym(0x4b7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30af as uint16_t,
                };
                init.set_keysym(0x4b8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30b1 as uint16_t,
                };
                init.set_keysym(0x4b9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30b3 as uint16_t,
                };
                init.set_keysym(0x4ba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30b5 as uint16_t,
                };
                init.set_keysym(0x4bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30b7 as uint16_t,
                };
                init.set_keysym(0x4bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30b9 as uint16_t,
                };
                init.set_keysym(0x4bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30bb as uint16_t,
                };
                init.set_keysym(0x4be as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30bd as uint16_t,
                };
                init.set_keysym(0x4bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30bf as uint16_t,
                };
                init.set_keysym(0x4c0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30c1 as uint16_t,
                };
                init.set_keysym(0x4c1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30c4 as uint16_t,
                };
                init.set_keysym(0x4c2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30c6 as uint16_t,
                };
                init.set_keysym(0x4c3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30c8 as uint16_t,
                };
                init.set_keysym(0x4c4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ca as uint16_t,
                };
                init.set_keysym(0x4c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30cb as uint16_t,
                };
                init.set_keysym(0x4c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30cc as uint16_t,
                };
                init.set_keysym(0x4c7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30cd as uint16_t,
                };
                init.set_keysym(0x4c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ce as uint16_t,
                };
                init.set_keysym(0x4c9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30cf as uint16_t,
                };
                init.set_keysym(0x4ca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30d2 as uint16_t,
                };
                init.set_keysym(0x4cb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30d5 as uint16_t,
                };
                init.set_keysym(0x4cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30d8 as uint16_t,
                };
                init.set_keysym(0x4cd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30db as uint16_t,
                };
                init.set_keysym(0x4ce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30de as uint16_t,
                };
                init.set_keysym(0x4cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30df as uint16_t,
                };
                init.set_keysym(0x4d0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e0 as uint16_t,
                };
                init.set_keysym(0x4d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e1 as uint16_t,
                };
                init.set_keysym(0x4d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e2 as uint16_t,
                };
                init.set_keysym(0x4d3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e4 as uint16_t,
                };
                init.set_keysym(0x4d4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e6 as uint16_t,
                };
                init.set_keysym(0x4d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e8 as uint16_t,
                };
                init.set_keysym(0x4d6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30e9 as uint16_t,
                };
                init.set_keysym(0x4d7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ea as uint16_t,
                };
                init.set_keysym(0x4d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30eb as uint16_t,
                };
                init.set_keysym(0x4d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ec as uint16_t,
                };
                init.set_keysym(0x4da as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ed as uint16_t,
                };
                init.set_keysym(0x4db as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30ef as uint16_t,
                };
                init.set_keysym(0x4dc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x30f3 as uint16_t,
                };
                init.set_keysym(0x4dd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x309b as uint16_t,
                };
                init.set_keysym(0x4de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x309c as uint16_t,
                };
                init.set_keysym(0x4df as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x60c as uint16_t,
                };
                init.set_keysym(0x5ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x61b as uint16_t,
                };
                init.set_keysym(0x5bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x61f as uint16_t,
                };
                init.set_keysym(0x5bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x621 as uint16_t,
                };
                init.set_keysym(0x5c1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x622 as uint16_t,
                };
                init.set_keysym(0x5c2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x623 as uint16_t,
                };
                init.set_keysym(0x5c3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x624 as uint16_t,
                };
                init.set_keysym(0x5c4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x625 as uint16_t,
                };
                init.set_keysym(0x5c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x626 as uint16_t,
                };
                init.set_keysym(0x5c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x627 as uint16_t,
                };
                init.set_keysym(0x5c7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x628 as uint16_t,
                };
                init.set_keysym(0x5c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x629 as uint16_t,
                };
                init.set_keysym(0x5c9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62a as uint16_t,
                };
                init.set_keysym(0x5ca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62b as uint16_t,
                };
                init.set_keysym(0x5cb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62c as uint16_t,
                };
                init.set_keysym(0x5cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62d as uint16_t,
                };
                init.set_keysym(0x5cd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62e as uint16_t,
                };
                init.set_keysym(0x5ce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x62f as uint16_t,
                };
                init.set_keysym(0x5cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x630 as uint16_t,
                };
                init.set_keysym(0x5d0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x631 as uint16_t,
                };
                init.set_keysym(0x5d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x632 as uint16_t,
                };
                init.set_keysym(0x5d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x633 as uint16_t,
                };
                init.set_keysym(0x5d3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x634 as uint16_t,
                };
                init.set_keysym(0x5d4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x635 as uint16_t,
                };
                init.set_keysym(0x5d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x636 as uint16_t,
                };
                init.set_keysym(0x5d6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x637 as uint16_t,
                };
                init.set_keysym(0x5d7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x638 as uint16_t,
                };
                init.set_keysym(0x5d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x639 as uint16_t,
                };
                init.set_keysym(0x5d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x63a as uint16_t,
                };
                init.set_keysym(0x5da as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x640 as uint16_t,
                };
                init.set_keysym(0x5e0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x641 as uint16_t,
                };
                init.set_keysym(0x5e1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x642 as uint16_t,
                };
                init.set_keysym(0x5e2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x643 as uint16_t,
                };
                init.set_keysym(0x5e3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x644 as uint16_t,
                };
                init.set_keysym(0x5e4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x645 as uint16_t,
                };
                init.set_keysym(0x5e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x646 as uint16_t,
                };
                init.set_keysym(0x5e6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x647 as uint16_t,
                };
                init.set_keysym(0x5e7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x648 as uint16_t,
                };
                init.set_keysym(0x5e8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x649 as uint16_t,
                };
                init.set_keysym(0x5e9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64a as uint16_t,
                };
                init.set_keysym(0x5ea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64b as uint16_t,
                };
                init.set_keysym(0x5eb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64c as uint16_t,
                };
                init.set_keysym(0x5ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64d as uint16_t,
                };
                init.set_keysym(0x5ed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64e as uint16_t,
                };
                init.set_keysym(0x5ee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x64f as uint16_t,
                };
                init.set_keysym(0x5ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x650 as uint16_t,
                };
                init.set_keysym(0x5f0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x651 as uint16_t,
                };
                init.set_keysym(0x5f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x652 as uint16_t,
                };
                init.set_keysym(0x5f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x452 as uint16_t,
                };
                init.set_keysym(0x6a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x453 as uint16_t,
                };
                init.set_keysym(0x6a2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x451 as uint16_t,
                };
                init.set_keysym(0x6a3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x454 as uint16_t,
                };
                init.set_keysym(0x6a4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x455 as uint16_t,
                };
                init.set_keysym(0x6a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x456 as uint16_t,
                };
                init.set_keysym(0x6a6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x457 as uint16_t,
                };
                init.set_keysym(0x6a7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x458 as uint16_t,
                };
                init.set_keysym(0x6a8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x459 as uint16_t,
                };
                init.set_keysym(0x6a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x45a as uint16_t,
                };
                init.set_keysym(0x6aa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x45b as uint16_t,
                };
                init.set_keysym(0x6ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x45c as uint16_t,
                };
                init.set_keysym(0x6ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x491 as uint16_t,
                };
                init.set_keysym(0x6ad as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x45e as uint16_t,
                };
                init.set_keysym(0x6ae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x45f as uint16_t,
                };
                init.set_keysym(0x6af as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2116 as uint16_t,
                };
                init.set_keysym(0x6b0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x402 as uint16_t,
                };
                init.set_keysym(0x6b1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x403 as uint16_t,
                };
                init.set_keysym(0x6b2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x401 as uint16_t,
                };
                init.set_keysym(0x6b3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x404 as uint16_t,
                };
                init.set_keysym(0x6b4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x405 as uint16_t,
                };
                init.set_keysym(0x6b5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x406 as uint16_t,
                };
                init.set_keysym(0x6b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x407 as uint16_t,
                };
                init.set_keysym(0x6b7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x408 as uint16_t,
                };
                init.set_keysym(0x6b8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x409 as uint16_t,
                };
                init.set_keysym(0x6b9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x40a as uint16_t,
                };
                init.set_keysym(0x6ba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x40b as uint16_t,
                };
                init.set_keysym(0x6bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x40c as uint16_t,
                };
                init.set_keysym(0x6bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x490 as uint16_t,
                };
                init.set_keysym(0x6bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x40e as uint16_t,
                };
                init.set_keysym(0x6be as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x40f as uint16_t,
                };
                init.set_keysym(0x6bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44e as uint16_t,
                };
                init.set_keysym(0x6c0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x430 as uint16_t,
                };
                init.set_keysym(0x6c1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x431 as uint16_t,
                };
                init.set_keysym(0x6c2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x446 as uint16_t,
                };
                init.set_keysym(0x6c3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x434 as uint16_t,
                };
                init.set_keysym(0x6c4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x435 as uint16_t,
                };
                init.set_keysym(0x6c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x444 as uint16_t,
                };
                init.set_keysym(0x6c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x433 as uint16_t,
                };
                init.set_keysym(0x6c7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x445 as uint16_t,
                };
                init.set_keysym(0x6c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x438 as uint16_t,
                };
                init.set_keysym(0x6c9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x439 as uint16_t,
                };
                init.set_keysym(0x6ca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43a as uint16_t,
                };
                init.set_keysym(0x6cb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43b as uint16_t,
                };
                init.set_keysym(0x6cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43c as uint16_t,
                };
                init.set_keysym(0x6cd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43d as uint16_t,
                };
                init.set_keysym(0x6ce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43e as uint16_t,
                };
                init.set_keysym(0x6cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x43f as uint16_t,
                };
                init.set_keysym(0x6d0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44f as uint16_t,
                };
                init.set_keysym(0x6d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x440 as uint16_t,
                };
                init.set_keysym(0x6d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x441 as uint16_t,
                };
                init.set_keysym(0x6d3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x442 as uint16_t,
                };
                init.set_keysym(0x6d4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x443 as uint16_t,
                };
                init.set_keysym(0x6d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x436 as uint16_t,
                };
                init.set_keysym(0x6d6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x432 as uint16_t,
                };
                init.set_keysym(0x6d7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44c as uint16_t,
                };
                init.set_keysym(0x6d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44b as uint16_t,
                };
                init.set_keysym(0x6d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x437 as uint16_t,
                };
                init.set_keysym(0x6da as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x448 as uint16_t,
                };
                init.set_keysym(0x6db as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44d as uint16_t,
                };
                init.set_keysym(0x6dc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x449 as uint16_t,
                };
                init.set_keysym(0x6dd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x447 as uint16_t,
                };
                init.set_keysym(0x6de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x44a as uint16_t,
                };
                init.set_keysym(0x6df as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42e as uint16_t,
                };
                init.set_keysym(0x6e0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x410 as uint16_t,
                };
                init.set_keysym(0x6e1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x411 as uint16_t,
                };
                init.set_keysym(0x6e2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x426 as uint16_t,
                };
                init.set_keysym(0x6e3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x414 as uint16_t,
                };
                init.set_keysym(0x6e4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x415 as uint16_t,
                };
                init.set_keysym(0x6e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x424 as uint16_t,
                };
                init.set_keysym(0x6e6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x413 as uint16_t,
                };
                init.set_keysym(0x6e7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x425 as uint16_t,
                };
                init.set_keysym(0x6e8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x418 as uint16_t,
                };
                init.set_keysym(0x6e9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x419 as uint16_t,
                };
                init.set_keysym(0x6ea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41a as uint16_t,
                };
                init.set_keysym(0x6eb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41b as uint16_t,
                };
                init.set_keysym(0x6ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41c as uint16_t,
                };
                init.set_keysym(0x6ed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41d as uint16_t,
                };
                init.set_keysym(0x6ee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41e as uint16_t,
                };
                init.set_keysym(0x6ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x41f as uint16_t,
                };
                init.set_keysym(0x6f0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42f as uint16_t,
                };
                init.set_keysym(0x6f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x420 as uint16_t,
                };
                init.set_keysym(0x6f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x421 as uint16_t,
                };
                init.set_keysym(0x6f3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x422 as uint16_t,
                };
                init.set_keysym(0x6f4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x423 as uint16_t,
                };
                init.set_keysym(0x6f5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x416 as uint16_t,
                };
                init.set_keysym(0x6f6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x412 as uint16_t,
                };
                init.set_keysym(0x6f7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42c as uint16_t,
                };
                init.set_keysym(0x6f8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42b as uint16_t,
                };
                init.set_keysym(0x6f9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x417 as uint16_t,
                };
                init.set_keysym(0x6fa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x428 as uint16_t,
                };
                init.set_keysym(0x6fb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42d as uint16_t,
                };
                init.set_keysym(0x6fc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x429 as uint16_t,
                };
                init.set_keysym(0x6fd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x427 as uint16_t,
                };
                init.set_keysym(0x6fe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x42a as uint16_t,
                };
                init.set_keysym(0x6ff as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x386 as uint16_t,
                };
                init.set_keysym(0x7a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x388 as uint16_t,
                };
                init.set_keysym(0x7a2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x389 as uint16_t,
                };
                init.set_keysym(0x7a3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x38a as uint16_t,
                };
                init.set_keysym(0x7a4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3aa as uint16_t,
                };
                init.set_keysym(0x7a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x38c as uint16_t,
                };
                init.set_keysym(0x7a7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x38e as uint16_t,
                };
                init.set_keysym(0x7a8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ab as uint16_t,
                };
                init.set_keysym(0x7a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x38f as uint16_t,
                };
                init.set_keysym(0x7ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x385 as uint16_t,
                };
                init.set_keysym(0x7ae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2015 as uint16_t,
                };
                init.set_keysym(0x7af as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ac as uint16_t,
                };
                init.set_keysym(0x7b1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ad as uint16_t,
                };
                init.set_keysym(0x7b2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ae as uint16_t,
                };
                init.set_keysym(0x7b3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3af as uint16_t,
                };
                init.set_keysym(0x7b4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ca as uint16_t,
                };
                init.set_keysym(0x7b5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x390 as uint16_t,
                };
                init.set_keysym(0x7b6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3cc as uint16_t,
                };
                init.set_keysym(0x7b7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3cd as uint16_t,
                };
                init.set_keysym(0x7b8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3cb as uint16_t,
                };
                init.set_keysym(0x7b9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b0 as uint16_t,
                };
                init.set_keysym(0x7ba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ce as uint16_t,
                };
                init.set_keysym(0x7bb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x391 as uint16_t,
                };
                init.set_keysym(0x7c1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x392 as uint16_t,
                };
                init.set_keysym(0x7c2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x393 as uint16_t,
                };
                init.set_keysym(0x7c3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x394 as uint16_t,
                };
                init.set_keysym(0x7c4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x395 as uint16_t,
                };
                init.set_keysym(0x7c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x396 as uint16_t,
                };
                init.set_keysym(0x7c6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x397 as uint16_t,
                };
                init.set_keysym(0x7c7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x398 as uint16_t,
                };
                init.set_keysym(0x7c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x399 as uint16_t,
                };
                init.set_keysym(0x7c9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39a as uint16_t,
                };
                init.set_keysym(0x7ca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39b as uint16_t,
                };
                init.set_keysym(0x7cb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39c as uint16_t,
                };
                init.set_keysym(0x7cc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39d as uint16_t,
                };
                init.set_keysym(0x7cd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39e as uint16_t,
                };
                init.set_keysym(0x7ce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x39f as uint16_t,
                };
                init.set_keysym(0x7cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a0 as uint16_t,
                };
                init.set_keysym(0x7d0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a1 as uint16_t,
                };
                init.set_keysym(0x7d1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a3 as uint16_t,
                };
                init.set_keysym(0x7d2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a4 as uint16_t,
                };
                init.set_keysym(0x7d4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a5 as uint16_t,
                };
                init.set_keysym(0x7d5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a6 as uint16_t,
                };
                init.set_keysym(0x7d6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a7 as uint16_t,
                };
                init.set_keysym(0x7d7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a8 as uint16_t,
                };
                init.set_keysym(0x7d8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3a9 as uint16_t,
                };
                init.set_keysym(0x7d9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b1 as uint16_t,
                };
                init.set_keysym(0x7e1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b2 as uint16_t,
                };
                init.set_keysym(0x7e2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b3 as uint16_t,
                };
                init.set_keysym(0x7e3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b4 as uint16_t,
                };
                init.set_keysym(0x7e4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b5 as uint16_t,
                };
                init.set_keysym(0x7e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b6 as uint16_t,
                };
                init.set_keysym(0x7e6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b7 as uint16_t,
                };
                init.set_keysym(0x7e7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b8 as uint16_t,
                };
                init.set_keysym(0x7e8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3b9 as uint16_t,
                };
                init.set_keysym(0x7e9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3ba as uint16_t,
                };
                init.set_keysym(0x7ea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3bb as uint16_t,
                };
                init.set_keysym(0x7eb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3bc as uint16_t,
                };
                init.set_keysym(0x7ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3bd as uint16_t,
                };
                init.set_keysym(0x7ed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3be as uint16_t,
                };
                init.set_keysym(0x7ee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3bf as uint16_t,
                };
                init.set_keysym(0x7ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c0 as uint16_t,
                };
                init.set_keysym(0x7f0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c1 as uint16_t,
                };
                init.set_keysym(0x7f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c3 as uint16_t,
                };
                init.set_keysym(0x7f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c2 as uint16_t,
                };
                init.set_keysym(0x7f3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c4 as uint16_t,
                };
                init.set_keysym(0x7f4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c5 as uint16_t,
                };
                init.set_keysym(0x7f5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c6 as uint16_t,
                };
                init.set_keysym(0x7f6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c7 as uint16_t,
                };
                init.set_keysym(0x7f7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c8 as uint16_t,
                };
                init.set_keysym(0x7f8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c9 as uint16_t,
                };
                init.set_keysym(0x7f9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23b7 as uint16_t,
                };
                init.set_keysym(0x8a1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x250c as uint16_t,
                };
                init.set_keysym(0x8a2 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2500 as uint16_t,
                };
                init.set_keysym(0x8a3 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2320 as uint16_t,
                };
                init.set_keysym(0x8a4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2321 as uint16_t,
                };
                init.set_keysym(0x8a5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2502 as uint16_t,
                };
                init.set_keysym(0x8a6 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a1 as uint16_t,
                };
                init.set_keysym(0x8a7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a3 as uint16_t,
                };
                init.set_keysym(0x8a8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a4 as uint16_t,
                };
                init.set_keysym(0x8a9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a6 as uint16_t,
                };
                init.set_keysym(0x8aa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x239b as uint16_t,
                };
                init.set_keysym(0x8ab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x239d as uint16_t,
                };
                init.set_keysym(0x8ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x239e as uint16_t,
                };
                init.set_keysym(0x8ad as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a0 as uint16_t,
                };
                init.set_keysym(0x8ae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23a8 as uint16_t,
                };
                init.set_keysym(0x8af as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23ac as uint16_t,
                };
                init.set_keysym(0x8b0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2264 as uint16_t,
                };
                init.set_keysym(0x8bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2260 as uint16_t,
                };
                init.set_keysym(0x8bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2265 as uint16_t,
                };
                init.set_keysym(0x8be as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x222b as uint16_t,
                };
                init.set_keysym(0x8bf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2234 as uint16_t,
                };
                init.set_keysym(0x8c0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x221d as uint16_t,
                };
                init.set_keysym(0x8c1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x221e as uint16_t,
                };
                init.set_keysym(0x8c2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2207 as uint16_t,
                };
                init.set_keysym(0x8c5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x223c as uint16_t,
                };
                init.set_keysym(0x8c8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2243 as uint16_t,
                };
                init.set_keysym(0x8c9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x21d4 as uint16_t,
                };
                init.set_keysym(0x8cd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x21d2 as uint16_t,
                };
                init.set_keysym(0x8ce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2261 as uint16_t,
                };
                init.set_keysym(0x8cf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x221a as uint16_t,
                };
                init.set_keysym(0x8d6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2282 as uint16_t,
                };
                init.set_keysym(0x8da as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2283 as uint16_t,
                };
                init.set_keysym(0x8db as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2229 as uint16_t,
                };
                init.set_keysym(0x8dc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x222a as uint16_t,
                };
                init.set_keysym(0x8dd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2227 as uint16_t,
                };
                init.set_keysym(0x8de as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2228 as uint16_t,
                };
                init.set_keysym(0x8df as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2202 as uint16_t,
                };
                init.set_keysym(0x8ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x192 as uint16_t,
                };
                init.set_keysym(0x8f6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2190 as uint16_t,
                };
                init.set_keysym(0x8fb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2191 as uint16_t,
                };
                init.set_keysym(0x8fc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2192 as uint16_t,
                };
                init.set_keysym(0x8fd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2193 as uint16_t,
                };
                init.set_keysym(0x8fe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25c6 as uint16_t,
                };
                init.set_keysym(0x9e0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2592 as uint16_t,
                };
                init.set_keysym(0x9e1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2409 as uint16_t,
                };
                init.set_keysym(0x9e2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x240c as uint16_t,
                };
                init.set_keysym(0x9e3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x240d as uint16_t,
                };
                init.set_keysym(0x9e4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x240a as uint16_t,
                };
                init.set_keysym(0x9e5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2424 as uint16_t,
                };
                init.set_keysym(0x9e8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x240b as uint16_t,
                };
                init.set_keysym(0x9e9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2518 as uint16_t,
                };
                init.set_keysym(0x9ea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2510 as uint16_t,
                };
                init.set_keysym(0x9eb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x250c as uint16_t,
                };
                init.set_keysym(0x9ec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2514 as uint16_t,
                };
                init.set_keysym(0x9ed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x253c as uint16_t,
                };
                init.set_keysym(0x9ee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23ba as uint16_t,
                };
                init.set_keysym(0x9ef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23bb as uint16_t,
                };
                init.set_keysym(0x9f0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2500 as uint16_t,
                };
                init.set_keysym(0x9f1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23bc as uint16_t,
                };
                init.set_keysym(0x9f2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x23bd as uint16_t,
                };
                init.set_keysym(0x9f3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x251c as uint16_t,
                };
                init.set_keysym(0x9f4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2524 as uint16_t,
                };
                init.set_keysym(0x9f5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2534 as uint16_t,
                };
                init.set_keysym(0x9f6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x252c as uint16_t,
                };
                init.set_keysym(0x9f7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2502 as uint16_t,
                };
                init.set_keysym(0x9f8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2003 as uint16_t,
                };
                init.set_keysym(0xaa1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2002 as uint16_t,
                };
                init.set_keysym(0xaa2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2004 as uint16_t,
                };
                init.set_keysym(0xaa3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2005 as uint16_t,
                };
                init.set_keysym(0xaa4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2007 as uint16_t,
                };
                init.set_keysym(0xaa5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2008 as uint16_t,
                };
                init.set_keysym(0xaa6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2009 as uint16_t,
                };
                init.set_keysym(0xaa7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x200a as uint16_t,
                };
                init.set_keysym(0xaa8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2014 as uint16_t,
                };
                init.set_keysym(0xaa9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2013 as uint16_t,
                };
                init.set_keysym(0xaaa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2423 as uint16_t,
                };
                init.set_keysym(0xaac as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2026 as uint16_t,
                };
                init.set_keysym(0xaae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2025 as uint16_t,
                };
                init.set_keysym(0xaaf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2153 as uint16_t,
                };
                init.set_keysym(0xab0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2154 as uint16_t,
                };
                init.set_keysym(0xab1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2155 as uint16_t,
                };
                init.set_keysym(0xab2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2156 as uint16_t,
                };
                init.set_keysym(0xab3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2157 as uint16_t,
                };
                init.set_keysym(0xab4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2158 as uint16_t,
                };
                init.set_keysym(0xab5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2159 as uint16_t,
                };
                init.set_keysym(0xab6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x215a as uint16_t,
                };
                init.set_keysym(0xab7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2105 as uint16_t,
                };
                init.set_keysym(0xab8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2012 as uint16_t,
                };
                init.set_keysym(0xabb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x27e8 as uint16_t,
                };
                init.set_keysym(0xabc as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2e as uint16_t,
                };
                init.set_keysym(0xabd as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x27e9 as uint16_t,
                };
                init.set_keysym(0xabe as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x215b as uint16_t,
                };
                init.set_keysym(0xac3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x215c as uint16_t,
                };
                init.set_keysym(0xac4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x215d as uint16_t,
                };
                init.set_keysym(0xac5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x215e as uint16_t,
                };
                init.set_keysym(0xac6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2122 as uint16_t,
                };
                init.set_keysym(0xac9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2613 as uint16_t,
                };
                init.set_keysym(0xaca as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25c1 as uint16_t,
                };
                init.set_keysym(0xacc as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25b7 as uint16_t,
                };
                init.set_keysym(0xacd as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25cb as uint16_t,
                };
                init.set_keysym(0xace as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25af as uint16_t,
                };
                init.set_keysym(0xacf as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2018 as uint16_t,
                };
                init.set_keysym(0xad0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2019 as uint16_t,
                };
                init.set_keysym(0xad1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x201c as uint16_t,
                };
                init.set_keysym(0xad2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x201d as uint16_t,
                };
                init.set_keysym(0xad3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x211e as uint16_t,
                };
                init.set_keysym(0xad4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2030 as uint16_t,
                };
                init.set_keysym(0xad5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2032 as uint16_t,
                };
                init.set_keysym(0xad6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2033 as uint16_t,
                };
                init.set_keysym(0xad7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x271d as uint16_t,
                };
                init.set_keysym(0xad9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25ac as uint16_t,
                };
                init.set_keysym(0xadb as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25c0 as uint16_t,
                };
                init.set_keysym(0xadc as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25b6 as uint16_t,
                };
                init.set_keysym(0xadd as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25cf as uint16_t,
                };
                init.set_keysym(0xade as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25ae as uint16_t,
                };
                init.set_keysym(0xadf as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25e6 as uint16_t,
                };
                init.set_keysym(0xae0 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25ab as uint16_t,
                };
                init.set_keysym(0xae1 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25ad as uint16_t,
                };
                init.set_keysym(0xae2 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25b3 as uint16_t,
                };
                init.set_keysym(0xae3 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25bd as uint16_t,
                };
                init.set_keysym(0xae4 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2606 as uint16_t,
                };
                init.set_keysym(0xae5 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2022 as uint16_t,
                };
                init.set_keysym(0xae6 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25aa as uint16_t,
                };
                init.set_keysym(0xae7 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25b2 as uint16_t,
                };
                init.set_keysym(0xae8 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25bc as uint16_t,
                };
                init.set_keysym(0xae9 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x261c as uint16_t,
                };
                init.set_keysym(0xaea as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x261e as uint16_t,
                };
                init.set_keysym(0xaeb as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2663 as uint16_t,
                };
                init.set_keysym(0xaec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2666 as uint16_t,
                };
                init.set_keysym(0xaed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2665 as uint16_t,
                };
                init.set_keysym(0xaee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2720 as uint16_t,
                };
                init.set_keysym(0xaf0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2020 as uint16_t,
                };
                init.set_keysym(0xaf1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2021 as uint16_t,
                };
                init.set_keysym(0xaf2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2713 as uint16_t,
                };
                init.set_keysym(0xaf3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2717 as uint16_t,
                };
                init.set_keysym(0xaf4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x266f as uint16_t,
                };
                init.set_keysym(0xaf5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x266d as uint16_t,
                };
                init.set_keysym(0xaf6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2642 as uint16_t,
                };
                init.set_keysym(0xaf7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2640 as uint16_t,
                };
                init.set_keysym(0xaf8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x260e as uint16_t,
                };
                init.set_keysym(0xaf9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2315 as uint16_t,
                };
                init.set_keysym(0xafa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2117 as uint16_t,
                };
                init.set_keysym(0xafb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2038 as uint16_t,
                };
                init.set_keysym(0xafc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x201a as uint16_t,
                };
                init.set_keysym(0xafd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x201e as uint16_t,
                };
                init.set_keysym(0xafe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3c as uint16_t,
                };
                init.set_keysym(0xba3 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3e as uint16_t,
                };
                init.set_keysym(0xba6 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2228 as uint16_t,
                };
                init.set_keysym(0xba8 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2227 as uint16_t,
                };
                init.set_keysym(0xba9 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xaf as uint16_t,
                };
                init.set_keysym(0xbc0 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x22a4 as uint16_t,
                };
                init.set_keysym(0xbc2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2229 as uint16_t,
                };
                init.set_keysym(0xbc3 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x230a as uint16_t,
                };
                init.set_keysym(0xbc4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5f as uint16_t,
                };
                init.set_keysym(0xbc6 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2218 as uint16_t,
                };
                init.set_keysym(0xbca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2395 as uint16_t,
                };
                init.set_keysym(0xbcc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x22a5 as uint16_t,
                };
                init.set_keysym(0xbce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x25cb as uint16_t,
                };
                init.set_keysym(0xbcf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2308 as uint16_t,
                };
                init.set_keysym(0xbd3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x222a as uint16_t,
                };
                init.set_keysym(0xbd6 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2283 as uint16_t,
                };
                init.set_keysym(0xbd8 as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2282 as uint16_t,
                };
                init.set_keysym(0xbda as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x22a3 as uint16_t,
                };
                init.set_keysym(0xbdc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x22a2 as uint16_t,
                };
                init.set_keysym(0xbfc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x2017 as uint16_t,
                };
                init.set_keysym(0xcdf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d0 as uint16_t,
                };
                init.set_keysym(0xce0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d1 as uint16_t,
                };
                init.set_keysym(0xce1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d2 as uint16_t,
                };
                init.set_keysym(0xce2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d3 as uint16_t,
                };
                init.set_keysym(0xce3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d4 as uint16_t,
                };
                init.set_keysym(0xce4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d5 as uint16_t,
                };
                init.set_keysym(0xce5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d6 as uint16_t,
                };
                init.set_keysym(0xce6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d7 as uint16_t,
                };
                init.set_keysym(0xce7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d8 as uint16_t,
                };
                init.set_keysym(0xce8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5d9 as uint16_t,
                };
                init.set_keysym(0xce9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5da as uint16_t,
                };
                init.set_keysym(0xcea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5db as uint16_t,
                };
                init.set_keysym(0xceb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5dc as uint16_t,
                };
                init.set_keysym(0xcec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5dd as uint16_t,
                };
                init.set_keysym(0xced as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5de as uint16_t,
                };
                init.set_keysym(0xcee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5df as uint16_t,
                };
                init.set_keysym(0xcef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e0 as uint16_t,
                };
                init.set_keysym(0xcf0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e1 as uint16_t,
                };
                init.set_keysym(0xcf1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e2 as uint16_t,
                };
                init.set_keysym(0xcf2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e3 as uint16_t,
                };
                init.set_keysym(0xcf3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e4 as uint16_t,
                };
                init.set_keysym(0xcf4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e5 as uint16_t,
                };
                init.set_keysym(0xcf5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e6 as uint16_t,
                };
                init.set_keysym(0xcf6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e7 as uint16_t,
                };
                init.set_keysym(0xcf7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e8 as uint16_t,
                };
                init.set_keysym(0xcf8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5e9 as uint16_t,
                };
                init.set_keysym(0xcf9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x5ea as uint16_t,
                };
                init.set_keysym(0xcfa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe01 as uint16_t,
                };
                init.set_keysym(0xda1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe02 as uint16_t,
                };
                init.set_keysym(0xda2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe03 as uint16_t,
                };
                init.set_keysym(0xda3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe04 as uint16_t,
                };
                init.set_keysym(0xda4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe05 as uint16_t,
                };
                init.set_keysym(0xda5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe06 as uint16_t,
                };
                init.set_keysym(0xda6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe07 as uint16_t,
                };
                init.set_keysym(0xda7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe08 as uint16_t,
                };
                init.set_keysym(0xda8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe09 as uint16_t,
                };
                init.set_keysym(0xda9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0a as uint16_t,
                };
                init.set_keysym(0xdaa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0b as uint16_t,
                };
                init.set_keysym(0xdab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0c as uint16_t,
                };
                init.set_keysym(0xdac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0d as uint16_t,
                };
                init.set_keysym(0xdad as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0e as uint16_t,
                };
                init.set_keysym(0xdae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe0f as uint16_t,
                };
                init.set_keysym(0xdaf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe10 as uint16_t,
                };
                init.set_keysym(0xdb0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe11 as uint16_t,
                };
                init.set_keysym(0xdb1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe12 as uint16_t,
                };
                init.set_keysym(0xdb2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe13 as uint16_t,
                };
                init.set_keysym(0xdb3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe14 as uint16_t,
                };
                init.set_keysym(0xdb4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe15 as uint16_t,
                };
                init.set_keysym(0xdb5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe16 as uint16_t,
                };
                init.set_keysym(0xdb6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe17 as uint16_t,
                };
                init.set_keysym(0xdb7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe18 as uint16_t,
                };
                init.set_keysym(0xdb8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe19 as uint16_t,
                };
                init.set_keysym(0xdb9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1a as uint16_t,
                };
                init.set_keysym(0xdba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1b as uint16_t,
                };
                init.set_keysym(0xdbb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1c as uint16_t,
                };
                init.set_keysym(0xdbc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1d as uint16_t,
                };
                init.set_keysym(0xdbd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1e as uint16_t,
                };
                init.set_keysym(0xdbe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe1f as uint16_t,
                };
                init.set_keysym(0xdbf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe20 as uint16_t,
                };
                init.set_keysym(0xdc0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe21 as uint16_t,
                };
                init.set_keysym(0xdc1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe22 as uint16_t,
                };
                init.set_keysym(0xdc2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe23 as uint16_t,
                };
                init.set_keysym(0xdc3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe24 as uint16_t,
                };
                init.set_keysym(0xdc4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe25 as uint16_t,
                };
                init.set_keysym(0xdc5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe26 as uint16_t,
                };
                init.set_keysym(0xdc6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe27 as uint16_t,
                };
                init.set_keysym(0xdc7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe28 as uint16_t,
                };
                init.set_keysym(0xdc8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe29 as uint16_t,
                };
                init.set_keysym(0xdc9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2a as uint16_t,
                };
                init.set_keysym(0xdca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2b as uint16_t,
                };
                init.set_keysym(0xdcb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2c as uint16_t,
                };
                init.set_keysym(0xdcc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2d as uint16_t,
                };
                init.set_keysym(0xdcd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2e as uint16_t,
                };
                init.set_keysym(0xdce as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe2f as uint16_t,
                };
                init.set_keysym(0xdcf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe30 as uint16_t,
                };
                init.set_keysym(0xdd0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe31 as uint16_t,
                };
                init.set_keysym(0xdd1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe32 as uint16_t,
                };
                init.set_keysym(0xdd2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe33 as uint16_t,
                };
                init.set_keysym(0xdd3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe34 as uint16_t,
                };
                init.set_keysym(0xdd4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe35 as uint16_t,
                };
                init.set_keysym(0xdd5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe36 as uint16_t,
                };
                init.set_keysym(0xdd6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe37 as uint16_t,
                };
                init.set_keysym(0xdd7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe38 as uint16_t,
                };
                init.set_keysym(0xdd8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe39 as uint16_t,
                };
                init.set_keysym(0xdd9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe3a as uint16_t,
                };
                init.set_keysym(0xdda as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe3e as uint16_t,
                };
                init.set_keysym(0xdde as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe3f as uint16_t,
                };
                init.set_keysym(0xddf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe40 as uint16_t,
                };
                init.set_keysym(0xde0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe41 as uint16_t,
                };
                init.set_keysym(0xde1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe42 as uint16_t,
                };
                init.set_keysym(0xde2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe43 as uint16_t,
                };
                init.set_keysym(0xde3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe44 as uint16_t,
                };
                init.set_keysym(0xde4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe45 as uint16_t,
                };
                init.set_keysym(0xde5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe46 as uint16_t,
                };
                init.set_keysym(0xde6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe47 as uint16_t,
                };
                init.set_keysym(0xde7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe48 as uint16_t,
                };
                init.set_keysym(0xde8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe49 as uint16_t,
                };
                init.set_keysym(0xde9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe4a as uint16_t,
                };
                init.set_keysym(0xdea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe4b as uint16_t,
                };
                init.set_keysym(0xdeb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe4c as uint16_t,
                };
                init.set_keysym(0xdec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe4d as uint16_t,
                };
                init.set_keysym(0xded as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe50 as uint16_t,
                };
                init.set_keysym(0xdf0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe51 as uint16_t,
                };
                init.set_keysym(0xdf1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe52 as uint16_t,
                };
                init.set_keysym(0xdf2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe53 as uint16_t,
                };
                init.set_keysym(0xdf3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe54 as uint16_t,
                };
                init.set_keysym(0xdf4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe55 as uint16_t,
                };
                init.set_keysym(0xdf5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe56 as uint16_t,
                };
                init.set_keysym(0xdf6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe57 as uint16_t,
                };
                init.set_keysym(0xdf7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe58 as uint16_t,
                };
                init.set_keysym(0xdf8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0xe59 as uint16_t,
                };
                init.set_keysym(0xdf9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3131 as uint16_t,
                };
                init.set_keysym(0xea1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3132 as uint16_t,
                };
                init.set_keysym(0xea2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3133 as uint16_t,
                };
                init.set_keysym(0xea3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3134 as uint16_t,
                };
                init.set_keysym(0xea4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3135 as uint16_t,
                };
                init.set_keysym(0xea5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3136 as uint16_t,
                };
                init.set_keysym(0xea6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3137 as uint16_t,
                };
                init.set_keysym(0xea7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3138 as uint16_t,
                };
                init.set_keysym(0xea8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3139 as uint16_t,
                };
                init.set_keysym(0xea9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313a as uint16_t,
                };
                init.set_keysym(0xeaa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313b as uint16_t,
                };
                init.set_keysym(0xeab as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313c as uint16_t,
                };
                init.set_keysym(0xeac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313d as uint16_t,
                };
                init.set_keysym(0xead as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313e as uint16_t,
                };
                init.set_keysym(0xeae as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x313f as uint16_t,
                };
                init.set_keysym(0xeaf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3140 as uint16_t,
                };
                init.set_keysym(0xeb0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3141 as uint16_t,
                };
                init.set_keysym(0xeb1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3142 as uint16_t,
                };
                init.set_keysym(0xeb2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3143 as uint16_t,
                };
                init.set_keysym(0xeb3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3144 as uint16_t,
                };
                init.set_keysym(0xeb4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3145 as uint16_t,
                };
                init.set_keysym(0xeb5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3146 as uint16_t,
                };
                init.set_keysym(0xeb6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3147 as uint16_t,
                };
                init.set_keysym(0xeb7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3148 as uint16_t,
                };
                init.set_keysym(0xeb8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3149 as uint16_t,
                };
                init.set_keysym(0xeb9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314a as uint16_t,
                };
                init.set_keysym(0xeba as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314b as uint16_t,
                };
                init.set_keysym(0xebb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314c as uint16_t,
                };
                init.set_keysym(0xebc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314d as uint16_t,
                };
                init.set_keysym(0xebd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314e as uint16_t,
                };
                init.set_keysym(0xebe as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x314f as uint16_t,
                };
                init.set_keysym(0xebf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3150 as uint16_t,
                };
                init.set_keysym(0xec0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3151 as uint16_t,
                };
                init.set_keysym(0xec1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3152 as uint16_t,
                };
                init.set_keysym(0xec2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3153 as uint16_t,
                };
                init.set_keysym(0xec3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3154 as uint16_t,
                };
                init.set_keysym(0xec4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3155 as uint16_t,
                };
                init.set_keysym(0xec5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3156 as uint16_t,
                };
                init.set_keysym(0xec6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3157 as uint16_t,
                };
                init.set_keysym(0xec7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3158 as uint16_t,
                };
                init.set_keysym(0xec8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3159 as uint16_t,
                };
                init.set_keysym(0xec9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315a as uint16_t,
                };
                init.set_keysym(0xeca as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315b as uint16_t,
                };
                init.set_keysym(0xecb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315c as uint16_t,
                };
                init.set_keysym(0xecc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315d as uint16_t,
                };
                init.set_keysym(0xecd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315e as uint16_t,
                };
                init.set_keysym(0xece as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x315f as uint16_t,
                };
                init.set_keysym(0xecf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3160 as uint16_t,
                };
                init.set_keysym(0xed0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3161 as uint16_t,
                };
                init.set_keysym(0xed1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3162 as uint16_t,
                };
                init.set_keysym(0xed2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3163 as uint16_t,
                };
                init.set_keysym(0xed3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11a8 as uint16_t,
                };
                init.set_keysym(0xed4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11a9 as uint16_t,
                };
                init.set_keysym(0xed5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11aa as uint16_t,
                };
                init.set_keysym(0xed6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11ab as uint16_t,
                };
                init.set_keysym(0xed7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11ac as uint16_t,
                };
                init.set_keysym(0xed8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11ad as uint16_t,
                };
                init.set_keysym(0xed9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11ae as uint16_t,
                };
                init.set_keysym(0xeda as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11af as uint16_t,
                };
                init.set_keysym(0xedb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b0 as uint16_t,
                };
                init.set_keysym(0xedc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b1 as uint16_t,
                };
                init.set_keysym(0xedd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b2 as uint16_t,
                };
                init.set_keysym(0xede as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b3 as uint16_t,
                };
                init.set_keysym(0xedf as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b4 as uint16_t,
                };
                init.set_keysym(0xee0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b5 as uint16_t,
                };
                init.set_keysym(0xee1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b6 as uint16_t,
                };
                init.set_keysym(0xee2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b7 as uint16_t,
                };
                init.set_keysym(0xee3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b8 as uint16_t,
                };
                init.set_keysym(0xee4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11b9 as uint16_t,
                };
                init.set_keysym(0xee5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11ba as uint16_t,
                };
                init.set_keysym(0xee6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11bb as uint16_t,
                };
                init.set_keysym(0xee7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11bc as uint16_t,
                };
                init.set_keysym(0xee8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11bd as uint16_t,
                };
                init.set_keysym(0xee9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11be as uint16_t,
                };
                init.set_keysym(0xeea as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11bf as uint16_t,
                };
                init.set_keysym(0xeeb as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11c0 as uint16_t,
                };
                init.set_keysym(0xeec as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11c1 as uint16_t,
                };
                init.set_keysym(0xeed as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11c2 as uint16_t,
                };
                init.set_keysym(0xeee as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x316d as uint16_t,
                };
                init.set_keysym(0xeef as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3171 as uint16_t,
                };
                init.set_keysym(0xef0 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3178 as uint16_t,
                };
                init.set_keysym(0xef1 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x317f as uint16_t,
                };
                init.set_keysym(0xef2 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3181 as uint16_t,
                };
                init.set_keysym(0xef3 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3184 as uint16_t,
                };
                init.set_keysym(0xef4 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x3186 as uint16_t,
                };
                init.set_keysym(0xef5 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x318d as uint16_t,
                };
                init.set_keysym(0xef6 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x318e as uint16_t,
                };
                init.set_keysym(0xef7 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11eb as uint16_t,
                };
                init.set_keysym(0xef8 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11f0 as uint16_t,
                };
                init.set_keysym(0xef9 as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x11f9 as uint16_t,
                };
                init.set_keysym(0xefa as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x20a9 as uint16_t,
                };
                init.set_keysym(0xeff as uint16_t);
                init.set_deprecated(true_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x152 as uint16_t,
                };
                init.set_keysym(0x13bc as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x153 as uint16_t,
                };
                init.set_keysym(0x13bd as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x178 as uint16_t,
                };
                init.set_keysym(0x13be as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
            {
                let mut init = codepair {
                    keysym_deprecated: [0; 2],
                    ucs: 0x20ac as uint16_t,
                };
                init.set_keysym(0x20ac as uint16_t);
                init.set_deprecated(false_0 != 0);
                init
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
