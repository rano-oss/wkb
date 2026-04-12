use c2rust_bitfields;
pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
}
pub mod stdint_intn_h {
    pub type i32 = __int32_t;
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
    pub const false_0: i32 = 0 as i32;
}
pub mod keysym_h {
    pub const XKB_KEYSYM_UNICODE_OFFSET: i32 = 0x1000000 as i32;
    pub const XKB_KEYSYM_UNICODE_MIN: i32 = 0x1000100 as i32;
}
pub use self::keysym_h::{XKB_KEYSYM_UNICODE_MIN, XKB_KEYSYM_UNICODE_OFFSET};
pub use self::stdbool_h::false_0;
pub use self::stdint_intn_h::i32;
pub use self::stdint_uintn_h::{u32, uint16_t, uint8_t};
pub use self::types_h::{__int32_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::xkbcommon_h::xkb_keysym_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct CaseMappings {
    #[bitfield(name = "lower", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "upper", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "offset", ty = "i32", bits = "2..=31")]
    pub lower_upper_offset: [u8; 4],
}
static mut legacy_keysym_data: [CaseMappings; 47] = [CaseMappings {
    lower_upper_offset: [0; 4],
}; 47];
static mut legacy_keysym_offsets1: [uint8_t; 540] = [
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0x2b as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x13 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2d as i32 as uint8_t,
    0x20 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x1b as i32 as uint8_t,
    0x27 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x8 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x4 as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xd as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x25 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x29 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x16 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x1d as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
];
static mut legacy_keysym_offsets2: [uint16_t; 40] = [
    0 as i32 as uint16_t,
    0x11c as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x19c as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x15c as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0xdc as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x1dc as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x3e as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x80 as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0x7b as i32 as uint16_t,
    0xbc as i32 as uint16_t,
];
#[inline]
unsafe fn get_legacy_keysym_entry(mut ks: xkb_keysym_t) -> *const CaseMappings {
    unsafe {
        return (&raw const legacy_keysym_data as *const CaseMappings).offset(
            (*(&raw const legacy_keysym_offsets1 as *const uint8_t).offset(
                (*(&raw const legacy_keysym_offsets2 as *const uint16_t)
                    .offset((ks >> 7 as i32) as isize) as xkb_keysym_t)
                    .wrapping_add(ks >> 1 as i32 & 0x3f as xkb_keysym_t) as isize,
            ) as xkb_keysym_t)
                .wrapping_add(ks & 0x1 as xkb_keysym_t) as isize,
        ) as *const CaseMappings;
    }
}
static mut unicode_data: [CaseMappings; 1019] = [CaseMappings {
    lower_upper_offset: [0; 4],
}; 1019];
static mut unicode_offsets1: [uint16_t; 908] = [
    0xa6 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xe as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xad as i32 as uint16_t,
    0xb3 as i32 as uint16_t,
    0xb3 as i32 as uint16_t,
    0xb3 as i32 as uint16_t,
    0x1ca as i32 as uint16_t,
    0x1ca as i32 as uint16_t,
    0x1ca as i32 as uint16_t,
    0x1d0 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x18e as i32 as uint16_t,
    0x18e as i32 as uint16_t,
    0x18e as i32 as uint16_t,
    0x18e as i32 as uint16_t,
    0x190 as i32 as uint16_t,
    0x1f9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x170 as i32 as uint16_t,
    0x170 as i32 as uint16_t,
    0x170 as i32 as uint16_t,
    0x170 as i32 as uint16_t,
    0x172 as i32 as uint16_t,
    0x281 as i32 as uint16_t,
    0x327 as i32 as uint16_t,
    0x327 as i32 as uint16_t,
    0x327 as i32 as uint16_t,
    0x327 as i32 as uint16_t,
    0x327 as i32 as uint16_t,
    0x32c as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x165 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa6 as i32 as uint16_t,
    0xc as i32 as uint16_t,
    0xaa as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x201 as i32 as uint16_t,
    0x201 as i32 as uint16_t,
    0x201 as i32 as uint16_t,
    0x208 as i32 as uint16_t,
    0x20b as i32 as uint16_t,
    0x20b as i32 as uint16_t,
    0x20f as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x149 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x82 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x34d as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x214 as i32 as uint16_t,
    0x348 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x152 as i32 as uint16_t,
    0x29f as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0xfd as i32 as uint16_t,
    0x28f as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x316 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x1c2 as i32 as uint16_t,
    0xaa as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x231 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x235 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x24e as i32 as uint16_t,
    0x162 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x231 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x12c as i32 as uint16_t,
    0xe4 as i32 as uint16_t,
    0x241 as i32 as uint16_t,
    0x24e as i32 as uint16_t,
    0x1be as i32 as uint16_t,
    0x163 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x231 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x162 as i32 as uint16_t,
    0x235 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x9f as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa6 as i32 as uint16_t,
    0x233 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x23a as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x166 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0xa3 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x164 as i32 as uint16_t,
    0x1e9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x15e as i32 as uint16_t,
    0x164 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xab as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x3c3 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa7 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x3bb as i32 as uint16_t,
    0xbb as i32 as uint16_t,
    0x334 as i32 as uint16_t,
    0x334 as i32 as uint16_t,
    0x334 as i32 as uint16_t,
    0x334 as i32 as uint16_t,
    0x334 as i32 as uint16_t,
    0x339 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x1d3 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x145 as i32 as uint16_t,
    0x287 as i32 as uint16_t,
    0xbe as i32 as uint16_t,
    0x3db as i32 as uint16_t,
    0x59 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x20 as i32 as uint16_t,
    0x3d3 as i32 as uint16_t,
    0x279 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x21c as i32 as uint16_t,
    0x220 as i32 as uint16_t,
    0xd7 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x3a3 as i32 as uint16_t,
    0x13a as i32 as uint16_t,
    0x198 as i32 as uint16_t,
    0x198 as i32 as uint16_t,
    0x198 as i32 as uint16_t,
    0x198 as i32 as uint16_t,
    0x19e as i32 as uint16_t,
    0x1a0 as i32 as uint16_t,
    0x1a0 as i32 as uint16_t,
    0x1a0 as i32 as uint16_t,
    0x1a4 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x1ba as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x166 as i32 as uint16_t,
    0xaa as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x22 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x5e as i32 as uint16_t,
    0x41 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x27e as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x6 as i32 as uint16_t,
    0x61 as i32 as uint16_t,
    0x6 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x5a as i32 as uint16_t,
    0x24 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x27a as i32 as uint16_t,
    0x43 as i32 as uint16_t,
    0x31c as i32 as uint16_t,
    0x22d as i32 as uint16_t,
    0x74 as i32 as uint16_t,
    0x246 as i32 as uint16_t,
    0x31f as i32 as uint16_t,
    0x93 as i32 as uint16_t,
    0x132 as i32 as uint16_t,
    0x99 as i32 as uint16_t,
    0x1e3 as i32 as uint16_t,
    0x297 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x3cb as i32 as uint16_t,
    0x3cb as i32 as uint16_t,
    0x393 as i32 as uint16_t,
    0x393 as i32 as uint16_t,
    0xee as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1db as i32 as uint16_t,
    0x1e0 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f1 as i32 as uint16_t,
    0x1f6 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x3e3 as i32 as uint16_t,
    0x3e3 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x39b as i32 as uint16_t,
    0x39b as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0xbc as i32 as uint16_t,
    0x149 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x116 as i32 as uint16_t,
    0x85 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0xcd as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xcf as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x25b as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25d as i32 as uint16_t,
    0x8 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x7 as i32 as uint16_t,
    0xa as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa8 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0xe9 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0xe9 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x71 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x272 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x2fc as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x143 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x293 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x5b as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x27b as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x2c7 as i32 as uint16_t,
    0x2c3 as i32 as uint16_t,
    0x2c7 as i32 as uint16_t,
    0x2c3 as i32 as uint16_t,
    0x2cc as i32 as uint16_t,
    0x2d6 as i32 as uint16_t,
    0x2d3 as i32 as uint16_t,
    0x2d6 as i32 as uint16_t,
    0x2d3 as i32 as uint16_t,
    0x2dc as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x22 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x23 as i32 as uint16_t,
    0x5e as i32 as uint16_t,
    0x41 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x42 as i32 as uint16_t,
    0x27e as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x3f3 as i32 as uint16_t,
    0x227 as i32 as uint16_t,
    0xde as i32 as uint16_t,
    0x6d as i32 as uint16_t,
    0x249 as i32 as uint16_t,
    0x23c as i32 as uint16_t,
    0x9f as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x23f as i32 as uint16_t,
    0x8d as i32 as uint16_t,
    0x24c as i32 as uint16_t,
    0xa3 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x235 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x24e as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x231 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x235 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x24e as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x231 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa7 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0xa3 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x164 as i32 as uint16_t,
    0x237 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0x239 as i32 as uint16_t,
    0xa1 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x126 as i32 as uint16_t,
    0x254 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x10c as i32 as uint16_t,
    0x83 as i32 as uint16_t,
    0x118 as i32 as uint16_t,
    0x124 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x35b as i32 as uint16_t,
    0x373 as i32 as uint16_t,
    0x383 as i32 as uint16_t,
    0x3eb as i32 as uint16_t,
    0x104 as i32 as uint16_t,
    0x10e as i32 as uint16_t,
    0x341 as i32 as uint16_t,
    0x38b as i32 as uint16_t,
    0x2e9 as i32 as uint16_t,
    0xf3 as i32 as uint16_t,
    0xf8 as i32 as uint16_t,
    0x118 as i32 as uint16_t,
    0x14c as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x18 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x141 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x128 as i32 as uint16_t,
    0x309 as i32 as uint16_t,
    0x310 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x4b as i32 as uint16_t,
    0x159 as i32 as uint16_t,
    0x52 as i32 as uint16_t,
    0x3ab as i32 as uint16_t,
    0x303 as i32 as uint16_t,
    0x2f6 as i32 as uint16_t,
    0x3b3 as i32 as uint16_t,
    0x2f1 as i32 as uint16_t,
    0x1b7 as i32 as uint16_t,
    0x11 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0xa5 as i32 as uint16_t,
    0x8 as i32 as uint16_t,
    0xab as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xa8 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0xce as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x25c as i32 as uint16_t,
    0x353 as i32 as uint16_t,
    0x11c as i32 as uint16_t,
    0x2e4 as i32 as uint16_t,
    0x168 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x84 as i32 as uint16_t,
    0x2a3 as i32 as uint16_t,
    0x1a9 as i32 as uint16_t,
    0xef as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x17c as i32 as uint16_t,
    0x186 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x17c as i32 as uint16_t,
    0x186 as i32 as uint16_t,
    0x264 as i32 as uint16_t,
    0x33 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x37b as i32 as uint16_t,
    0 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x17a as i32 as uint16_t,
    0x184 as i32 as uint16_t,
    0x1b1 as i32 as uint16_t,
    0x3a as i32 as uint16_t,
    0x67 as i32 as uint16_t,
    0x363 as i32 as uint16_t,
    0x26b as i32 as uint16_t,
    0x2c as i32 as uint16_t,
    0x7c as i32 as uint16_t,
    0x36b as i32 as uint16_t,
    0x67 as i32 as uint16_t,
    0xc6 as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2ab as i32 as uint16_t,
    0x2af as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2b7 as i32 as uint16_t,
    0x2bb as i32 as uint16_t,
];
static mut unicode_offsets2: [uint16_t; 498] = [
    0x16f as i32 as uint16_t,
    0x2ed as i32 as uint16_t,
    0x30c as i32 as uint16_t,
    0x127 as i32 as uint16_t,
    0x1c0 as i32 as uint16_t,
    0x1da as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x39 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x280 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x10f as i32 as uint16_t,
    0x218 as i32 as uint16_t,
    0x2d3 as i32 as uint16_t,
    0x34c as i32 as uint16_t,
    0x1ec as i32 as uint16_t,
    0x18f as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0xd as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x32c as i32 as uint16_t,
    0x2d as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x230 as i32 as uint16_t,
    0x8d as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0xe5 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x274 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x36c as i32 as uint16_t,
    0x258 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x157 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x1a0 as i32 as uint16_t,
    0x246 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x65 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x71 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0xad as i32 as uint16_t,
    0x2a0 as i32 as uint16_t,
    0x2b3 as i32 as uint16_t,
    0xcb as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x59 as i32 as uint16_t,
    0xff as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x147 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x3 as i32 as uint16_t,
    0x206 as i32 as uint16_t,
];
#[inline]
unsafe fn get_unicode_entry(mut ks: xkb_keysym_t) -> *const CaseMappings {
    unsafe {
        return (&raw const unicode_data as *const CaseMappings).offset(
            (*(&raw const unicode_offsets1 as *const uint16_t).offset(
                (*(&raw const unicode_offsets2 as *const uint16_t).offset((ks >> 8 as i32) as isize)
                    as xkb_keysym_t)
                    .wrapping_add(ks >> 3 as i32 & 0x1f as xkb_keysym_t) as isize,
            ) as xkb_keysym_t)
                .wrapping_add(ks & 0x7 as xkb_keysym_t) as isize,
        ) as *const CaseMappings;
    }
}

pub unsafe fn xkb_keysym_to_lower(mut ks: xkb_keysym_t) -> xkb_keysym_t {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return if (*m).lower() as i32 != 0 {
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
pub unsafe fn xkb_keysym_to_upper(mut ks: xkb_keysym_t) -> xkb_keysym_t {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return if (*m).upper() as i32 != 0 {
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
pub unsafe fn xkb_keysym_is_lower(mut ks: xkb_keysym_t) -> bool {
    unsafe {
        if ks <= 0x13be as xkb_keysym_t {
            let mut m: *const CaseMappings = get_legacy_keysym_entry(ks);
            return (*m).upper() as i32 != 0 && !(*m).lower();
        } else if XKB_KEYSYM_UNICODE_MIN as xkb_keysym_t <= ks && ks <= 0x101f189 as xkb_keysym_t {
            let mut m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as xkb_keysym_t));
            return (*m_0).upper() as i32 != 0 && !(*m_0).lower();
        } else {
            return false_0 != 0;
        };
    }
}
pub unsafe fn xkb_keysym_is_upper_or_title(mut ks: xkb_keysym_t) -> bool {
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
unsafe fn c2rust_run_static_initializers() {
    unsafe {
        unicode_data = [
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x80 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x80 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x70 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x70 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa515 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa512 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x61 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x38 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x1dbf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x64 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x64 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x4a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x4a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x79 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a1e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xce as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa54b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa567 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa528 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa544 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2e7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20bf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2046 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x89c2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x26 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x80 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x80 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x39 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x36 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa543 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8a38 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa641 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd5 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd6 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xc7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xe8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29fd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x4f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa544 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa54f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa54b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa541 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa544 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xca as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xcb as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa54f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3a0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x22 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xdb as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x54 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa528 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x3b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x1dbf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8a04 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xee6 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1c60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x82 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x74 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa567 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa512 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa52a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa515 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3a0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x27 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x2a1e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x38 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x45 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x47 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29e7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x8a38 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29fd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd5 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd6 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2a2b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2a28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x2a3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xc3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x45 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x47 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xa641 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x1d5d as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xbc0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x30 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8a04 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29f7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0xee6 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x29e7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2a2b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2a28 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x79 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x12c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x8 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x70 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x70 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xc3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xce as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x4a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x4a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x64 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x64 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x4f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xca as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd9 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xdb as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x56 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x74 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x3c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x60 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa544 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x29f7 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa541 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xd3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa543 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0xda as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xa52a as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x186e as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x186d as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1864 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1862 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1862 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1863 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x185c as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1825 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x26 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x25 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x40 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x3f as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x50 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcb as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcd as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xcf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x61 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd3 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0xd1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x97d0 as i32);
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
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x1001dbf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x717 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x12bf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0xfff89b as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x10 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x12bf as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(-0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(-0x240 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x2 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(false);
                init.set_offset(0 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x270 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x20 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x21 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(true);
                init.set_upper(false);
                init.set_offset(0x1 as i32);
                init
            },
            {
                let mut init = CaseMappings {
                    lower_upper_offset: [0; 4],
                };
                init.set_lower(false);
                init.set_upper(true);
                init.set_offset(0x1 as i32);
                init
            },
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [c2rust_run_static_initializers];
