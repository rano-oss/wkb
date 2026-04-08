use c2rust_bitfields;
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
}
pub mod stdint_intn_h {
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
pub mod xkbcommon_h {
    pub type xkb_keysym_t = u32;
    use super::stdint_uintn_h::u32;
}
pub mod stdbool_h {
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_UNICODE_OFFSET: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
    pub const XKB_KEYSYM_UNICODE_MIN: ::core::ffi::c_int = 0x1000100 as ::core::ffi::c_int;
}
pub use self::keysym_h::{XKB_KEYSYM_UNICODE_MIN, XKB_KEYSYM_UNICODE_OFFSET};
pub use self::stdbool_h::false_0;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::types_h::{__int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::xkbcommon_h::xkb_keysym_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct CaseMappings {
    #[bitfield(name = "lower", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "upper", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "offset", ty = "int32_t", bits = "2..=31")]
    pub lower_upper_offset: [u8; 4],
}
static mut legacy_keysym_data: [CaseMappings; 47] = [CaseMappings {
    lower_upper_offset: [0; 4],
}; 47];
static mut legacy_keysym_offsets1: [uint8_t; 540] = [
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x13 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2d as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x1b as ::core::ffi::c_int as uint8_t,
    0x27 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x29 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x1d as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
];
static mut legacy_keysym_offsets2: [uint16_t; 40] = [
    0 as ::core::ffi::c_int as uint16_t,
    0x11c as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x19c as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x15c as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0xdc as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x1dc as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x3e as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x80 as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0x7b as ::core::ffi::c_int as uint16_t,
    0xbc as ::core::ffi::c_int as uint16_t,
];
#[inline]
unsafe extern "C" fn get_legacy_keysym_entry(mut ks: xkb_keysym_t) -> *const CaseMappings {
    unsafe {
        return (&raw const legacy_keysym_data as *const CaseMappings).offset(
            (*(&raw const legacy_keysym_offsets1 as *const uint8_t).offset(
                (*(&raw const legacy_keysym_offsets2 as *const uint16_t)
                    .offset((ks >> 7 as ::core::ffi::c_int) as isize)
                    as xkb_keysym_t)
                    .wrapping_add(ks >> 1 as ::core::ffi::c_int & 0x3f as xkb_keysym_t)
                    as isize,
            ) as xkb_keysym_t)
                .wrapping_add(ks & 0x1 as xkb_keysym_t) as isize,
        ) as *const CaseMappings;
    }
}
static mut unicode_data: [CaseMappings; 1019] = [CaseMappings {
    lower_upper_offset: [0; 4],
}; 1019];
static mut unicode_offsets1: [uint16_t; 908] = [
    0xa6 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xe as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xad as ::core::ffi::c_int as uint16_t,
    0xb3 as ::core::ffi::c_int as uint16_t,
    0xb3 as ::core::ffi::c_int as uint16_t,
    0xb3 as ::core::ffi::c_int as uint16_t,
    0x1ca as ::core::ffi::c_int as uint16_t,
    0x1ca as ::core::ffi::c_int as uint16_t,
    0x1ca as ::core::ffi::c_int as uint16_t,
    0x1d0 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x18e as ::core::ffi::c_int as uint16_t,
    0x18e as ::core::ffi::c_int as uint16_t,
    0x18e as ::core::ffi::c_int as uint16_t,
    0x18e as ::core::ffi::c_int as uint16_t,
    0x190 as ::core::ffi::c_int as uint16_t,
    0x1f9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x170 as ::core::ffi::c_int as uint16_t,
    0x170 as ::core::ffi::c_int as uint16_t,
    0x170 as ::core::ffi::c_int as uint16_t,
    0x170 as ::core::ffi::c_int as uint16_t,
    0x172 as ::core::ffi::c_int as uint16_t,
    0x281 as ::core::ffi::c_int as uint16_t,
    0x327 as ::core::ffi::c_int as uint16_t,
    0x327 as ::core::ffi::c_int as uint16_t,
    0x327 as ::core::ffi::c_int as uint16_t,
    0x327 as ::core::ffi::c_int as uint16_t,
    0x327 as ::core::ffi::c_int as uint16_t,
    0x32c as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x165 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa6 as ::core::ffi::c_int as uint16_t,
    0xc as ::core::ffi::c_int as uint16_t,
    0xaa as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x201 as ::core::ffi::c_int as uint16_t,
    0x201 as ::core::ffi::c_int as uint16_t,
    0x201 as ::core::ffi::c_int as uint16_t,
    0x208 as ::core::ffi::c_int as uint16_t,
    0x20b as ::core::ffi::c_int as uint16_t,
    0x20b as ::core::ffi::c_int as uint16_t,
    0x20f as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x149 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x82 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x34d as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x214 as ::core::ffi::c_int as uint16_t,
    0x348 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x152 as ::core::ffi::c_int as uint16_t,
    0x29f as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0xfd as ::core::ffi::c_int as uint16_t,
    0x28f as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x316 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x1c2 as ::core::ffi::c_int as uint16_t,
    0xaa as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x231 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x235 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x24e as ::core::ffi::c_int as uint16_t,
    0x162 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x231 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x12c as ::core::ffi::c_int as uint16_t,
    0xe4 as ::core::ffi::c_int as uint16_t,
    0x241 as ::core::ffi::c_int as uint16_t,
    0x24e as ::core::ffi::c_int as uint16_t,
    0x1be as ::core::ffi::c_int as uint16_t,
    0x163 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x231 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x162 as ::core::ffi::c_int as uint16_t,
    0x235 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x9f as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa6 as ::core::ffi::c_int as uint16_t,
    0x233 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x23a as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x166 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0xa3 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x164 as ::core::ffi::c_int as uint16_t,
    0x1e9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x15e as ::core::ffi::c_int as uint16_t,
    0x164 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xab as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x3c3 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa7 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x3bb as ::core::ffi::c_int as uint16_t,
    0xbb as ::core::ffi::c_int as uint16_t,
    0x334 as ::core::ffi::c_int as uint16_t,
    0x334 as ::core::ffi::c_int as uint16_t,
    0x334 as ::core::ffi::c_int as uint16_t,
    0x334 as ::core::ffi::c_int as uint16_t,
    0x334 as ::core::ffi::c_int as uint16_t,
    0x339 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x1d3 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x145 as ::core::ffi::c_int as uint16_t,
    0x287 as ::core::ffi::c_int as uint16_t,
    0xbe as ::core::ffi::c_int as uint16_t,
    0x3db as ::core::ffi::c_int as uint16_t,
    0x59 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x20 as ::core::ffi::c_int as uint16_t,
    0x3d3 as ::core::ffi::c_int as uint16_t,
    0x279 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x21c as ::core::ffi::c_int as uint16_t,
    0x220 as ::core::ffi::c_int as uint16_t,
    0xd7 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x3a3 as ::core::ffi::c_int as uint16_t,
    0x13a as ::core::ffi::c_int as uint16_t,
    0x198 as ::core::ffi::c_int as uint16_t,
    0x198 as ::core::ffi::c_int as uint16_t,
    0x198 as ::core::ffi::c_int as uint16_t,
    0x198 as ::core::ffi::c_int as uint16_t,
    0x19e as ::core::ffi::c_int as uint16_t,
    0x1a0 as ::core::ffi::c_int as uint16_t,
    0x1a0 as ::core::ffi::c_int as uint16_t,
    0x1a0 as ::core::ffi::c_int as uint16_t,
    0x1a4 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x1ba as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x166 as ::core::ffi::c_int as uint16_t,
    0xaa as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x22 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x5e as ::core::ffi::c_int as uint16_t,
    0x41 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x27e as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x6 as ::core::ffi::c_int as uint16_t,
    0x61 as ::core::ffi::c_int as uint16_t,
    0x6 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x5a as ::core::ffi::c_int as uint16_t,
    0x24 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x27a as ::core::ffi::c_int as uint16_t,
    0x43 as ::core::ffi::c_int as uint16_t,
    0x31c as ::core::ffi::c_int as uint16_t,
    0x22d as ::core::ffi::c_int as uint16_t,
    0x74 as ::core::ffi::c_int as uint16_t,
    0x246 as ::core::ffi::c_int as uint16_t,
    0x31f as ::core::ffi::c_int as uint16_t,
    0x93 as ::core::ffi::c_int as uint16_t,
    0x132 as ::core::ffi::c_int as uint16_t,
    0x99 as ::core::ffi::c_int as uint16_t,
    0x1e3 as ::core::ffi::c_int as uint16_t,
    0x297 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x3cb as ::core::ffi::c_int as uint16_t,
    0x3cb as ::core::ffi::c_int as uint16_t,
    0x393 as ::core::ffi::c_int as uint16_t,
    0x393 as ::core::ffi::c_int as uint16_t,
    0xee as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1db as ::core::ffi::c_int as uint16_t,
    0x1e0 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f1 as ::core::ffi::c_int as uint16_t,
    0x1f6 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x3e3 as ::core::ffi::c_int as uint16_t,
    0x3e3 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x39b as ::core::ffi::c_int as uint16_t,
    0x39b as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0xbc as ::core::ffi::c_int as uint16_t,
    0x149 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x116 as ::core::ffi::c_int as uint16_t,
    0x85 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0xcd as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xcf as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x25b as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25d as ::core::ffi::c_int as uint16_t,
    0x8 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x7 as ::core::ffi::c_int as uint16_t,
    0xa as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa8 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0xe9 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0xe9 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x71 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x272 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x2fc as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x143 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x293 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x5b as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x27b as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x2c7 as ::core::ffi::c_int as uint16_t,
    0x2c3 as ::core::ffi::c_int as uint16_t,
    0x2c7 as ::core::ffi::c_int as uint16_t,
    0x2c3 as ::core::ffi::c_int as uint16_t,
    0x2cc as ::core::ffi::c_int as uint16_t,
    0x2d6 as ::core::ffi::c_int as uint16_t,
    0x2d3 as ::core::ffi::c_int as uint16_t,
    0x2d6 as ::core::ffi::c_int as uint16_t,
    0x2d3 as ::core::ffi::c_int as uint16_t,
    0x2dc as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x22 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x23 as ::core::ffi::c_int as uint16_t,
    0x5e as ::core::ffi::c_int as uint16_t,
    0x41 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x42 as ::core::ffi::c_int as uint16_t,
    0x27e as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x3f3 as ::core::ffi::c_int as uint16_t,
    0x227 as ::core::ffi::c_int as uint16_t,
    0xde as ::core::ffi::c_int as uint16_t,
    0x6d as ::core::ffi::c_int as uint16_t,
    0x249 as ::core::ffi::c_int as uint16_t,
    0x23c as ::core::ffi::c_int as uint16_t,
    0x9f as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x23f as ::core::ffi::c_int as uint16_t,
    0x8d as ::core::ffi::c_int as uint16_t,
    0x24c as ::core::ffi::c_int as uint16_t,
    0xa3 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x235 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x24e as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x231 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x235 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x24e as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x231 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa7 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0xa3 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x164 as ::core::ffi::c_int as uint16_t,
    0x237 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0x239 as ::core::ffi::c_int as uint16_t,
    0xa1 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x126 as ::core::ffi::c_int as uint16_t,
    0x254 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x10c as ::core::ffi::c_int as uint16_t,
    0x83 as ::core::ffi::c_int as uint16_t,
    0x118 as ::core::ffi::c_int as uint16_t,
    0x124 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x35b as ::core::ffi::c_int as uint16_t,
    0x373 as ::core::ffi::c_int as uint16_t,
    0x383 as ::core::ffi::c_int as uint16_t,
    0x3eb as ::core::ffi::c_int as uint16_t,
    0x104 as ::core::ffi::c_int as uint16_t,
    0x10e as ::core::ffi::c_int as uint16_t,
    0x341 as ::core::ffi::c_int as uint16_t,
    0x38b as ::core::ffi::c_int as uint16_t,
    0x2e9 as ::core::ffi::c_int as uint16_t,
    0xf3 as ::core::ffi::c_int as uint16_t,
    0xf8 as ::core::ffi::c_int as uint16_t,
    0x118 as ::core::ffi::c_int as uint16_t,
    0x14c as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x18 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x141 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x128 as ::core::ffi::c_int as uint16_t,
    0x309 as ::core::ffi::c_int as uint16_t,
    0x310 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x4b as ::core::ffi::c_int as uint16_t,
    0x159 as ::core::ffi::c_int as uint16_t,
    0x52 as ::core::ffi::c_int as uint16_t,
    0x3ab as ::core::ffi::c_int as uint16_t,
    0x303 as ::core::ffi::c_int as uint16_t,
    0x2f6 as ::core::ffi::c_int as uint16_t,
    0x3b3 as ::core::ffi::c_int as uint16_t,
    0x2f1 as ::core::ffi::c_int as uint16_t,
    0x1b7 as ::core::ffi::c_int as uint16_t,
    0x11 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0xa5 as ::core::ffi::c_int as uint16_t,
    0x8 as ::core::ffi::c_int as uint16_t,
    0xab as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xa8 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0xce as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x25c as ::core::ffi::c_int as uint16_t,
    0x353 as ::core::ffi::c_int as uint16_t,
    0x11c as ::core::ffi::c_int as uint16_t,
    0x2e4 as ::core::ffi::c_int as uint16_t,
    0x168 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x84 as ::core::ffi::c_int as uint16_t,
    0x2a3 as ::core::ffi::c_int as uint16_t,
    0x1a9 as ::core::ffi::c_int as uint16_t,
    0xef as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x17c as ::core::ffi::c_int as uint16_t,
    0x186 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x17c as ::core::ffi::c_int as uint16_t,
    0x186 as ::core::ffi::c_int as uint16_t,
    0x264 as ::core::ffi::c_int as uint16_t,
    0x33 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x37b as ::core::ffi::c_int as uint16_t,
    0 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x17a as ::core::ffi::c_int as uint16_t,
    0x184 as ::core::ffi::c_int as uint16_t,
    0x1b1 as ::core::ffi::c_int as uint16_t,
    0x3a as ::core::ffi::c_int as uint16_t,
    0x67 as ::core::ffi::c_int as uint16_t,
    0x363 as ::core::ffi::c_int as uint16_t,
    0x26b as ::core::ffi::c_int as uint16_t,
    0x2c as ::core::ffi::c_int as uint16_t,
    0x7c as ::core::ffi::c_int as uint16_t,
    0x36b as ::core::ffi::c_int as uint16_t,
    0x67 as ::core::ffi::c_int as uint16_t,
    0xc6 as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x9 as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2ab as ::core::ffi::c_int as uint16_t,
    0x2af as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2b7 as ::core::ffi::c_int as uint16_t,
    0x2bb as ::core::ffi::c_int as uint16_t,
];
static mut unicode_offsets2: [uint16_t; 498] = [
    0x16f as ::core::ffi::c_int as uint16_t,
    0x2ed as ::core::ffi::c_int as uint16_t,
    0x30c as ::core::ffi::c_int as uint16_t,
    0x127 as ::core::ffi::c_int as uint16_t,
    0x1c0 as ::core::ffi::c_int as uint16_t,
    0x1da as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x39 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x280 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x10f as ::core::ffi::c_int as uint16_t,
    0x218 as ::core::ffi::c_int as uint16_t,
    0x2d3 as ::core::ffi::c_int as uint16_t,
    0x34c as ::core::ffi::c_int as uint16_t,
    0x1ec as ::core::ffi::c_int as uint16_t,
    0x18f as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0xd as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x32c as ::core::ffi::c_int as uint16_t,
    0x2d as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x230 as ::core::ffi::c_int as uint16_t,
    0x8d as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0xe5 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x274 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x36c as ::core::ffi::c_int as uint16_t,
    0x258 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x157 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x1a0 as ::core::ffi::c_int as uint16_t,
    0x246 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x65 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x71 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0xad as ::core::ffi::c_int as uint16_t,
    0x2a0 as ::core::ffi::c_int as uint16_t,
    0x2b3 as ::core::ffi::c_int as uint16_t,
    0xcb as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x59 as ::core::ffi::c_int as uint16_t,
    0xff as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x147 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x3 as ::core::ffi::c_int as uint16_t,
    0x206 as ::core::ffi::c_int as uint16_t,
];
#[inline]
unsafe extern "C" fn get_unicode_entry(mut ks: xkb_keysym_t) -> *const CaseMappings {
    unsafe {
        return (&raw const unicode_data as *const CaseMappings).offset(
            (*(&raw const unicode_offsets1 as *const uint16_t).offset(
                (*(&raw const unicode_offsets2 as *const uint16_t)
                    .offset((ks >> 8 as ::core::ffi::c_int) as isize)
                    as xkb_keysym_t)
                    .wrapping_add(ks >> 3 as ::core::ffi::c_int & 0x1f as xkb_keysym_t)
                    as isize,
            ) as xkb_keysym_t)
                .wrapping_add(ks & 0x7 as xkb_keysym_t) as isize,
        ) as *const CaseMappings;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_lower(mut ks: xkb_keysym_t) -> xkb_keysym_t {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return if (*m).lower() as ::core::ffi::c_int != 0 {
                ks.wrapping_add((*m).offset() as xkb_keysym_t)
            } else {
                ks
            };
        } else if XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks && ks <= 0x101f189 as xkb_keysym_t {
            let mut m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t));
            if (*m_0).lower() {
                ks = ks.wrapping_add((*m_0).offset() as xkb_keysym_t);
                return if ks < XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t {
                    ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t)
                } else {
                    ks
                };
            } else {
                return ks;
            }
        } else {
            return ks;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_to_upper(mut ks: xkb_keysym_t) -> xkb_keysym_t {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return if (*m).upper() as ::core::ffi::c_int != 0 {
                ks.wrapping_sub((*m).offset() as xkb_keysym_t)
            } else {
                ks
            };
        } else if XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks && ks <= 0x101f189 as xkb_keysym_t {
            let mut m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t));
            if (*m_0).upper() {
                ks = ks.wrapping_sub((*m_0).offset() as xkb_keysym_t);
                return if ks < XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t {
                    ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t)
                } else {
                    ks
                };
            } else {
                return ks;
            }
        } else {
            return ks;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_is_lower(mut ks: xkb_keysym_t) -> bool {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return (*m).upper() as ::core::ffi::c_int != 0 && !(*m).lower();
        } else if XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks && ks <= 0x101f189 as xkb_keysym_t {
            let mut m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t));
            return (*m_0).upper() as ::core::ffi::c_int != 0 && !(*m_0).lower();
        } else {
            return false_0 != 0;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keysym_is_upper_or_title(mut ks: xkb_keysym_t) -> bool {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            return (*get_legacy_keysym_entry(ks)).lower();
        } else if XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks && ks <= 0x101f189 as xkb_keysym_t {
            return (*get_unicode_entry(
                ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t),
            ))
            .lower();
        } else {
            return false_0 != 0;
        };
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        unicode_data = [
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x80 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x80 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x70 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x70 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa515 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa512 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x61 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x38 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x1dbf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x64 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x64 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x4a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x4a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x79 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xce as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa54b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa567 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa528 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa544 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2e7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20bf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2046 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x89c2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x26 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x80 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x80 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x39 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x36 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa543 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8a38 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa641 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd5 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd6 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xc7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xe8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29fd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x4f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa544 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa54f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa54b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa541 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa544 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xca as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcb as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa54f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3a0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xdb as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x54 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa528 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x1dbf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8a04 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xee6 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x74 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa567 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa512 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa52a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa515 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3a0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x38 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x45 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x47 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29e7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8a38 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29fd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd5 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd6 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2a2b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2a28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xc3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x45 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x47 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa641 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x1d5d as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x30 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8a04 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29f7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xee6 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29e7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2a2b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2a28 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x79 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x12c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x70 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x70 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xc3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xce as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x4a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x4a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x64 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x64 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x4f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xca as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd9 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xdb as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x56 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x74 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x3c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x60 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa544 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29f7 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa541 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa543 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa52a as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x186e as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x186d as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1864 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1862 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1862 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1863 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x185c as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1825 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x26 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3f as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcb as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x61 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd3 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as int32_t);
                init
            },
        ];
        legacy_keysym_data = [
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x1001dbf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x717 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x12bf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xfff89b as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x10 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x12bf as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x240 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x270 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x21 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as int32_t);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as int32_t);
                init
            },
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
