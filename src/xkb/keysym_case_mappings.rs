pub const XKB_KEYSYM_UNICODE_OFFSET: i32 = 0x1000000;
pub const XKB_KEYSYM_UNICODE_MIN: i32 = 0x1000100;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaseMappings {
    pub lower: bool,
    pub upper: bool,
    pub offset: i32,
}
static LEGACY_KEYSYM_DATA: [CaseMappings; 47] = [
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x1001dbf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x717_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x12bf_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xfff89b_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x12bf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x240_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x270_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x21_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
];
static LEGACY_KEYSYM_OFFSETS1: [u8; 540] = [
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x3, 0x6, 0x6, 0x6, 0x6, 0x6,
    0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x7, 0x2, 0x2, 0xb, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc,
    0xc, 0xc, 0xc, 0xc, 0xa, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x10, 0x11, 0x11, 0x10, 0x11, 0x10, 0x2, 0x2, 0x14, 0x1a, 0x1a, 0x19, 0x1a, 0x19,
    0x2, 0x2, 0x3, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x7, 0x6, 0x6, 0x6, 0x2, 0x2, 0x2, 0xb,
    0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0x2b, 0xc, 0xc, 0xc, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x13, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2d, 0x20, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0, 0x10, 0x1, 0x2, 0x11, 0x1,
    0x2, 0x2, 0x14, 0x14, 0x15, 0x2, 0x1a, 0x1b, 0x27, 0x7, 0x2, 0x2, 0x3, 0x2, 0x2, 0x7, 0x3, 0x3,
    0x6, 0x2, 0x2, 0x3, 0x2, 0x3, 0x7, 0xa, 0x2, 0x2, 0xb, 0x2, 0x2, 0xa, 0xb, 0xb, 0xc, 0x2, 0x2,
    0xb, 0x2, 0xb, 0xa, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0xf, 0x2, 0x2, 0x2, 0x2, 0x8, 0x2, 0x2, 0xf, 0x2, 0x2, 0x6, 0x6,
    0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x6, 0x7, 0x6, 0x6, 0x6, 0x4, 0xc, 0xc, 0xc, 0xc, 0xc,
    0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xa, 0xc, 0xc, 0xc, 0xd, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x10, 0x2, 0x2, 0x1, 0x25, 0x10, 0x1, 0x2, 0x14, 0x2,
    0x2, 0x15, 0x29, 0x14, 0x15, 0x2, 0x2, 0x2, 0x3, 0x7, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x3, 0x2,
    0x7, 0x2, 0x3, 0x7, 0x2, 0x2, 0xb, 0xa, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0xb, 0x2, 0xa, 0x2, 0xb,
    0xa, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x10,
    0x10, 0x10, 0x1, 0x10, 0x11, 0x1, 0x11, 0x14, 0x14, 0x14, 0x15, 0x14, 0x1a, 0x15, 0x1a, 0x7,
    0x3, 0x3, 0x7, 0x7, 0x7, 0x7, 0x3, 0x6, 0x7, 0x3, 0x2, 0x6, 0x3, 0x2, 0x7, 0xa, 0xb, 0xb, 0xa,
    0xa, 0xa, 0xa, 0xb, 0xc, 0xa, 0xb, 0x2, 0xc, 0xb, 0x2, 0xa, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2,
    0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x2, 0x16, 0x17, 0x17, 0x17, 0x17, 0x17, 0x17, 0x17,
    0x1d, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23,
    0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21,
    0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21,
];
static LEGACY_KEYSYM_OFFSETS2: [u16; 40] = [
    0, 0x11c, 0x7b, 0x19c, 0x7b, 0x15c, 0x7b, 0xdc, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x1dc, 0x7b,
    0x3e, 0x7b, 0x80, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b,
    0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0x7b, 0xbc,
];
#[inline]
unsafe fn get_legacy_keysym_entry(ks: u32) -> *const CaseMappings {
    unsafe {
        (&raw const LEGACY_KEYSYM_DATA as *const CaseMappings).offset(
            (*(&raw const LEGACY_KEYSYM_OFFSETS1 as *const u8).offset(
                (*(&raw const LEGACY_KEYSYM_OFFSETS2 as *const u16).offset((ks >> 7_i32) as isize)
                    as u32)
                    .wrapping_add(ks >> 1_i32 & 0x3f_u32) as isize,
            ) as u32)
                .wrapping_add(ks & 0x1_u32) as isize,
        ) as *const CaseMappings
    }
}
static UNICODE_DATA: [CaseMappings; 1019] = [
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x80_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x80_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x70_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x70_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x7e_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x7e_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa515_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa512_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x61_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x38_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x1dbf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x64_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x64_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x4a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x4a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x9_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c25_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x79_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x2a1f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x2a1c_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x2a1e_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd2_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xce_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xcd_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xcd_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa54b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xcf_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa567_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa528_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa544_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x2e7_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x9_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x20bf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2046_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x89c2_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x26_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x80_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x80_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x7e_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x7e_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x9_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x3e_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x39_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x36_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x30_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa543_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8a38_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa3_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa641_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd3_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd5_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd6_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xc7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xe8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xf_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2a1c_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x29fd_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2a1f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x7_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x4f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa544_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa54f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa54b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa541_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa544_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xca_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xcb_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa54f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x3a0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2a3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2a3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x22_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x9_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xdb_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1a_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x54_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1b_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa528_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x3f_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x3b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x1dbf_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8a04_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xee6_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1c60_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x82_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x74_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa567_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1c_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa512_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa52a_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa515_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x3a0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x28_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x27_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x2a1e_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x38_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x45_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd9_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd9_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x47_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x29e7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x8a38_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x29fd_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd5_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd6_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2a2b_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa3_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x2a28_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x2a3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xc3_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x45_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x47_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xa641_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x1d5d_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xbc0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x30_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8a04_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x29f7_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0xee6_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x29e7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2a2b_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x2a28_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x79_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x12c_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x9_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x8_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x70_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x70_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x7_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xc3_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd2_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xce_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x4a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x4a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x56_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x64_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x64_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xcd_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xcd_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x4f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xca_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd9_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd9_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xdb_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x56_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x74_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: -0x3c_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x60_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd3_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa544_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x29f7_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa541_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xd3_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa543_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0xda_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0xa52a_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x186e_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x186d_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1864_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1862_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1862_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1863_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x185c_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1825_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x10_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x20_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x26_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x25_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x40_i32,
    },
    CaseMappings {
        lower: false,
        upper: false,
        offset: 0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x3f_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x50_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xcb_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: 0x1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xcd_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xcf_i32,
    },
    CaseMappings {
        lower: false,
        upper: true,
        offset: -0x61_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd3_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0xd1_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
    CaseMappings {
        lower: true,
        upper: false,
        offset: 0x97d0_i32,
    },
];
static UNICODE_OFFSETS1: [u16; 908] = [
    0xa6, 0x9, 0xe, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0xad, 0xb3,
    0xb3, 0xb3, 0x1ca, 0x1ca, 0x1ca, 0x1d0, 0x9, 0x9, 0x18e, 0x18e, 0x18e, 0x18e, 0x190, 0x1f9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x170, 0x170, 0x170, 0x170, 0x172, 0x281, 0x327, 0x327,
    0x327, 0x327, 0x327, 0x32c, 0xa5, 0x165, 0xa5, 0xa6, 0xc, 0xaa, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x23, 0x23, 0x23, 0x23, 0x42, 0x42, 0x42, 0x42, 0x9, 0x9, 0x9, 0x9, 0x201, 0x201, 0x201,
    0x208, 0x20b, 0x20b, 0x20f, 0x9, 0x9, 0x9, 0x9, 0x9, 0x149, 0x84, 0x82, 0x84, 0x84, 0x84, 0x84,
    0x84, 0x84, 0x84, 0xa5, 0x34d, 0x84, 0x214, 0x348, 0x84, 0x84, 0x152, 0x29f, 0x84, 0xfd, 0x28f,
    0x84, 0x316, 0x9, 0x9, 0x1c2, 0xaa, 0x239, 0x239, 0x239, 0x231, 0xa5, 0xa5, 0x235, 0x239,
    0x239, 0x24e, 0x162, 0xa5, 0xa5, 0x239, 0x239, 0x239, 0x231, 0xa5, 0xa5, 0x12c, 0xe4, 0x241,
    0x24e, 0x1be, 0x163, 0xa5, 0x239, 0x239, 0x239, 0x231, 0xa5, 0xa5, 0x162, 0x235, 0x239, 0x239,
    0x9f, 0xa5, 0xa5, 0xa6, 0x233, 0x239, 0x239, 0x23a, 0xa5, 0xa5, 0xa5, 0x166, 0x239, 0x239,
    0x239, 0xa3, 0xa5, 0xa5, 0x164, 0x1e9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0xa5, 0xa5, 0xa5, 0xa5,
    0x15e, 0x164, 0xa5, 0xab, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3, 0x3c3,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa7, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x3bb,
    0xbb, 0x334, 0x334, 0x334, 0x334, 0x334, 0x339, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x1d3,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x145, 0x287, 0xbe, 0x3db, 0x59, 0x23, 0x20, 0x3d3, 0x279, 0x42,
    0x21c, 0x220, 0xd7, 0x84, 0x84, 0x84, 0x3a3, 0x13a, 0x198, 0x198, 0x198, 0x198, 0x19e, 0x1a0,
    0x1a0, 0x1a0, 0x1a4, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x1ba, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0x166, 0xaa, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x22, 0x23, 0x23, 0x5e, 0x41, 0x42, 0x42, 0x27e, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x6, 0x61, 0x6, 0x23, 0x23, 0x5a, 0x24, 0x42, 0x42, 0x27a, 0x43, 0x31c, 0x22d,
    0x74, 0x246, 0x31f, 0x93, 0x132, 0x99, 0x1e3, 0x297, 0x9, 0x9, 0x3cb, 0x3cb, 0x393, 0x393,
    0xee, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x1db,
    0x1db, 0x1db, 0x1db, 0x1db, 0x1db, 0x1e0, 0x9, 0x1f1, 0x1f1, 0x1f1, 0x1f1, 0x1f1, 0x1f1, 0x1f6,
    0x9, 0x3e3, 0x3e3, 0x23, 0x23, 0x23, 0x23, 0x42, 0x42, 0x42, 0x42, 0x39b, 0x39b, 0x84, 0x84,
    0x84, 0x84, 0xbc, 0x149, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x116, 0x85, 0x84, 0x84, 0x84,
    0x84, 0x84, 0x84, 0xcd, 0xce, 0xce, 0xce, 0xcf, 0x9, 0x25b, 0x25c, 0x25c, 0x25c, 0x25d, 0x8,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x7, 0xa, 0x9, 0x9, 0xa5,
    0xa8, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x239, 0x239, 0x239, 0xe9,
    0x239, 0x239, 0x239, 0xe9, 0x239, 0x239, 0x239, 0x71, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5,
    0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0x272, 0xa5, 0x2fc, 0xa5, 0xa5, 0xa5, 0xa5,
    0xa5, 0xa5, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x84, 0x84, 0x84, 0x84, 0x84, 0x143, 0x9,
    0x9, 0x84, 0x84, 0x84, 0x293, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x23,
    0x23, 0x5b, 0x9, 0x42, 0x42, 0x27b, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x2c7, 0x2c3, 0x2c7, 0x2c3, 0x2cc, 0x2d6, 0x2d3, 0x2d6, 0x2d3, 0x2dc, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x22, 0x23, 0x23, 0x5e, 0x41, 0x42, 0x42, 0x27e, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x3f3,
    0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x3f3, 0x227, 0xde, 0x6d, 0x249, 0x23c,
    0x9f, 0xa5, 0xa5, 0xa5, 0x23f, 0x8d, 0x24c, 0xa3, 0xa5, 0xa5, 0x235, 0x239, 0x239, 0x24e, 0xa5,
    0xa5, 0xa5, 0x239, 0x239, 0x239, 0x231, 0xa5, 0xa5, 0x235, 0x239, 0x239, 0x24e, 0xa5, 0xa5,
    0xa5, 0x239, 0x239, 0x239, 0x231, 0xa5, 0xa5, 0xa7, 0x239, 0x239, 0x239, 0xa3, 0xa5, 0xa5,
    0x164, 0x237, 0x239, 0x239, 0xa1, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84,
    0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x126, 0x254, 0x84, 0x84, 0x84, 0x84, 0x84,
    0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x10c, 0x83, 0x118, 0x124, 0x84, 0x84, 0x84, 0x84,
    0x84, 0x35b, 0x373, 0x383, 0x3eb, 0x104, 0x10e, 0x341, 0x38b, 0x2e9, 0xf3, 0xf8, 0x118, 0x14c,
    0x84, 0x84, 0x18, 0x84, 0x84, 0x84, 0x84, 0x141, 0x84, 0x128, 0x309, 0x310, 0x84, 0x4b, 0x159,
    0x52, 0x3ab, 0x303, 0x2f6, 0x3b3, 0x2f1, 0x1b7, 0x11, 0xa5, 0xa5, 0xa5, 0x8, 0xab, 0x9, 0x9,
    0x9, 0xa8, 0x9, 0x9, 0x9, 0xce, 0xce, 0xce, 0xce, 0xce, 0xce, 0x25c, 0x25c, 0x25c, 0x25c,
    0x25c, 0x25c, 0x353, 0x11c, 0x2e4, 0x168, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84, 0x84,
    0x84, 0x84, 0x84, 0x2a3, 0x1a9, 0xef, 0x9, 0x17a, 0x184, 0x17c, 0x186, 0x17a, 0x184, 0x17a,
    0x184, 0x17c, 0x186, 0x264, 0x33, 0x17a, 0x184, 0x37b, 0, 0x17a, 0x184, 0x17a, 0x184, 0x17a,
    0x184, 0x1b1, 0x3a, 0x67, 0x363, 0x26b, 0x2c, 0x7c, 0x36b, 0x67, 0xc6, 0x2ab, 0x2ab, 0x2ab,
    0x2ab, 0x2ab, 0x2b7, 0x2b7, 0x2b7, 0x2b7, 0x2b7, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9,
    0x9, 0x9, 0x9, 0x2ab, 0x2ab, 0x2ab, 0x2ab, 0x2af, 0x2b7, 0x2b7, 0x2b7, 0x2b7, 0x2bb,
];
static UNICODE_OFFSETS2: [u16; 498] = [
    0x16f, 0x2ed, 0x30c, 0x127, 0x1c0, 0x1da, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x39, 0x3, 0x3, 0x280, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x10f, 0x218, 0x2d3, 0x34c,
    0x1ec, 0x18f, 0x3, 0x3, 0xd, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x32c, 0x2d, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x230, 0x8d, 0x3, 0x3, 0x3, 0xe5, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0, 0x3, 0x3, 0x3, 0x274, 0x3,
    0x3, 0x3, 0x3, 0x36c, 0x258, 0x3, 0x157, 0x3, 0x3, 0x3, 0x3, 0x1a0, 0x246, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x65, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x71, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3,
    0x3, 0x3, 0x3, 0x3, 0xad, 0x2a0, 0x2b3, 0xcb, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x59, 0xff,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x147, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x206,
];
#[inline]
unsafe fn get_unicode_entry(ks: u32) -> *const CaseMappings {
    unsafe {
        (&raw const UNICODE_DATA as *const CaseMappings).offset(
            (*(&raw const UNICODE_OFFSETS1 as *const u16).offset(
                (*(&raw const UNICODE_OFFSETS2 as *const u16).offset((ks >> 8_i32) as isize) as u32)
                    .wrapping_add(ks >> 3_i32 & 0x1f_u32) as isize,
            ) as u32)
                .wrapping_add(ks & 0x7_u32) as isize,
        ) as *const CaseMappings
    }
}
pub unsafe fn xkb_keysym_to_upper(mut ks: u32) -> u32 {
    unsafe {
        if ks <= 0x13be_u32 {
            let m: *const CaseMappings = get_legacy_keysym_entry(ks);
            if (*m).upper as i32 != 0 {
                ks.wrapping_sub((*m).offset as u32)
            } else {
                ks
            }
        } else if XKB_KEYSYM_UNICODE_MIN as u32 <= ks && ks <= 0x101f189_u32 {
            let m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as u32));
            if (*m_0).upper {
                ks = ks.wrapping_sub((*m_0).offset as u32);
                if ks < XKB_KEYSYM_UNICODE_MIN as u32 {
                    ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as u32)
                } else {
                    ks
                }
            } else {
                ks
            }
        } else {
            ks
        }
    }
}
pub unsafe fn xkb_keysym_is_lower(ks: u32) -> bool {
    unsafe {
        if ks <= 0x13be_u32 {
            let m: *const CaseMappings = get_legacy_keysym_entry(ks);
            (*m).upper as i32 != 0 && !(*m).lower
        } else if XKB_KEYSYM_UNICODE_MIN as u32 <= ks && ks <= 0x101f189_u32 {
            let m_0: *const CaseMappings =
                get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as u32));
            (*m_0).upper as i32 != 0 && !(*m_0).lower
        } else {
            false
        }
    }
}
pub unsafe fn xkb_keysym_is_upper_or_title(ks: u32) -> bool {
    unsafe {
        if ks <= 0x13be_u32 {
            (*get_legacy_keysym_entry(ks)).lower
        } else if XKB_KEYSYM_UNICODE_MIN as u32 <= ks && ks <= 0x101f189_u32 {
            (*get_unicode_entry(ks.wrapping_sub(XKB_KEYSYM_UNICODE_OFFSET as u32))).lower
        } else {
            false
        }
    }
}
