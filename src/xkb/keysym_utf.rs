//! Native Rust keysym to UTF-8/UTF-32 conversion
//!
//! Converted from C FFI to idiomatic Rust.
// Re-export type alias
// XKB key symbol constants
pub const XKB_KEY_NO_SYMBOL: u32 = 0;
pub const XKB_KEY_BACKSPACE: u32 = 0xff08;
pub const XKB_KEY_CLEAR: u32 = 0xff0b;
pub const XKB_KEY_RETURN: u32 = 0xff0d;
pub const XKB_KEY_ESCAPE: u32 = 0xff1b;
pub const XKB_KEY_DELETE: u32 = 0xffff;
pub const XKB_KEY_KP_SPACE: u32 = 0xff80;
pub const XKB_KEY_KP_TAB: u32 = 0xff89;
pub const XKB_KEY_KP_ENTER: u32 = 0xff8d;
pub const XKB_KEY_KP_EQUAL: u32 = 0xffbd;
pub const XKB_KEY_KP_MULTIPLY: u32 = 0xffaa;
pub const XKB_KEY_KP_9: u32 = 0xffb9;
pub const XKB_KEY_SPACE: u32 = 0x20;
pub const XKB_KEY_XF86_NUMERIC_0: u32 = 0x10081200;
pub const XKB_KEY_XF86_NUMERIC_9: u32 = 0x10081209;
pub const XKB_KEY_XF86_NUMERIC_STAR: u32 = 268964362;
pub const XKB_KEY_XF86_NUMERIC_POUND: u32 = 268964363;

pub const XKB_KEYSYM_UNICODE_OFFSET: u32 = 0x1000000;
pub const XKB_KEYSYM_UNICODE_SURROGATE_MIN: u32 = 0x100d800;
pub const XKB_KEYSYM_UNICODE_SURROGATE_MAX: u32 = 0x100dfff;
pub const XKB_KEYSYM_UNICODE_MAX: u32 = 0x110ffff;

/// Keysym lookup table entry
#[derive(Copy, Clone, Debug)]
struct KeysymEntry {
    keysym: u16,
    ucs: u16,
    deprecated: bool,
}

// Include the static lookup table
const KEYSYM_TABLE: &[KeysymEntry] = &[
    KeysymEntry { keysym: 0x01A1, ucs: 0x0104, deprecated: false },
    KeysymEntry { keysym: 0x01A2, ucs: 0x02D8, deprecated: false },
    KeysymEntry { keysym: 0x01A3, ucs: 0x0141, deprecated: false },
    KeysymEntry { keysym: 0x01A5, ucs: 0x013D, deprecated: false },
    KeysymEntry { keysym: 0x01A6, ucs: 0x015A, deprecated: false },
    KeysymEntry { keysym: 0x01A9, ucs: 0x0160, deprecated: false },
    KeysymEntry { keysym: 0x01AA, ucs: 0x015E, deprecated: false },
    KeysymEntry { keysym: 0x01AB, ucs: 0x0164, deprecated: false },
    KeysymEntry { keysym: 0x01AC, ucs: 0x0179, deprecated: false },
    KeysymEntry { keysym: 0x01AE, ucs: 0x017D, deprecated: false },
    KeysymEntry { keysym: 0x01AF, ucs: 0x017B, deprecated: false },
    KeysymEntry { keysym: 0x01B1, ucs: 0x0105, deprecated: false },
    KeysymEntry { keysym: 0x01B2, ucs: 0x02DB, deprecated: false },
    KeysymEntry { keysym: 0x01B3, ucs: 0x0142, deprecated: false },
    KeysymEntry { keysym: 0x01B5, ucs: 0x013E, deprecated: false },
    KeysymEntry { keysym: 0x01B6, ucs: 0x015B, deprecated: false },
    KeysymEntry { keysym: 0x01B7, ucs: 0x02C7, deprecated: false },
    KeysymEntry { keysym: 0x01B9, ucs: 0x0161, deprecated: false },
    KeysymEntry { keysym: 0x01BA, ucs: 0x015F, deprecated: false },
    KeysymEntry { keysym: 0x01BB, ucs: 0x0165, deprecated: false },
    KeysymEntry { keysym: 0x01BC, ucs: 0x017A, deprecated: false },
    KeysymEntry { keysym: 0x01BD, ucs: 0x02DD, deprecated: false },
    KeysymEntry { keysym: 0x01BE, ucs: 0x017E, deprecated: false },
    KeysymEntry { keysym: 0x01BF, ucs: 0x017C, deprecated: false },
    KeysymEntry { keysym: 0x01C0, ucs: 0x0154, deprecated: false },
    KeysymEntry { keysym: 0x01C3, ucs: 0x0102, deprecated: false },
    KeysymEntry { keysym: 0x01C5, ucs: 0x0139, deprecated: false },
    KeysymEntry { keysym: 0x01C6, ucs: 0x0106, deprecated: false },
    KeysymEntry { keysym: 0x01C8, ucs: 0x010C, deprecated: false },
    KeysymEntry { keysym: 0x01CA, ucs: 0x0118, deprecated: false },
    KeysymEntry { keysym: 0x01CC, ucs: 0x011A, deprecated: false },
    KeysymEntry { keysym: 0x01CF, ucs: 0x010E, deprecated: false },
    KeysymEntry { keysym: 0x01D0, ucs: 0x0110, deprecated: false },
    KeysymEntry { keysym: 0x01D1, ucs: 0x0143, deprecated: false },
    KeysymEntry { keysym: 0x01D2, ucs: 0x0147, deprecated: false },
    KeysymEntry { keysym: 0x01D5, ucs: 0x0150, deprecated: false },
    KeysymEntry { keysym: 0x01D8, ucs: 0x0158, deprecated: false },
    KeysymEntry { keysym: 0x01D9, ucs: 0x016E, deprecated: false },
    KeysymEntry { keysym: 0x01DB, ucs: 0x0170, deprecated: false },
    KeysymEntry { keysym: 0x01DE, ucs: 0x0162, deprecated: false },
    KeysymEntry { keysym: 0x01E0, ucs: 0x0155, deprecated: false },
    KeysymEntry { keysym: 0x01E3, ucs: 0x0103, deprecated: false },
    KeysymEntry { keysym: 0x01E5, ucs: 0x013A, deprecated: false },
    KeysymEntry { keysym: 0x01E6, ucs: 0x0107, deprecated: false },
    KeysymEntry { keysym: 0x01E8, ucs: 0x010D, deprecated: false },
    KeysymEntry { keysym: 0x01EA, ucs: 0x0119, deprecated: false },
    KeysymEntry { keysym: 0x01EC, ucs: 0x011B, deprecated: false },
    KeysymEntry { keysym: 0x01EF, ucs: 0x010F, deprecated: false },
    KeysymEntry { keysym: 0x01F0, ucs: 0x0111, deprecated: false },
    KeysymEntry { keysym: 0x01F1, ucs: 0x0144, deprecated: false },
    KeysymEntry { keysym: 0x01F2, ucs: 0x0148, deprecated: false },
    KeysymEntry { keysym: 0x01F5, ucs: 0x0151, deprecated: false },
    KeysymEntry { keysym: 0x01F8, ucs: 0x0159, deprecated: false },
    KeysymEntry { keysym: 0x01F9, ucs: 0x016F, deprecated: false },
    KeysymEntry { keysym: 0x01FB, ucs: 0x0171, deprecated: false },
    KeysymEntry { keysym: 0x01FE, ucs: 0x0163, deprecated: false },
    KeysymEntry { keysym: 0x01FF, ucs: 0x02D9, deprecated: false },
    KeysymEntry { keysym: 0x02A1, ucs: 0x0126, deprecated: false },
    KeysymEntry { keysym: 0x02A6, ucs: 0x0124, deprecated: false },
    KeysymEntry { keysym: 0x02A9, ucs: 0x0130, deprecated: false },
    KeysymEntry { keysym: 0x02AB, ucs: 0x011E, deprecated: false },
    KeysymEntry { keysym: 0x02AC, ucs: 0x0134, deprecated: false },
    KeysymEntry { keysym: 0x02B1, ucs: 0x0127, deprecated: false },
    KeysymEntry { keysym: 0x02B6, ucs: 0x0125, deprecated: false },
    KeysymEntry { keysym: 0x02B9, ucs: 0x0131, deprecated: false },
    KeysymEntry { keysym: 0x02BB, ucs: 0x011F, deprecated: false },
    KeysymEntry { keysym: 0x02BC, ucs: 0x0135, deprecated: false },
    KeysymEntry { keysym: 0x02C5, ucs: 0x010A, deprecated: false },
    KeysymEntry { keysym: 0x02C6, ucs: 0x0108, deprecated: false },
    KeysymEntry { keysym: 0x02D5, ucs: 0x0120, deprecated: false },
    KeysymEntry { keysym: 0x02D8, ucs: 0x011C, deprecated: false },
    KeysymEntry { keysym: 0x02DD, ucs: 0x016C, deprecated: false },
    KeysymEntry { keysym: 0x02DE, ucs: 0x015C, deprecated: false },
    KeysymEntry { keysym: 0x02E5, ucs: 0x010B, deprecated: false },
    KeysymEntry { keysym: 0x02E6, ucs: 0x0109, deprecated: false },
    KeysymEntry { keysym: 0x02F5, ucs: 0x0121, deprecated: false },
    KeysymEntry { keysym: 0x02F8, ucs: 0x011D, deprecated: false },
    KeysymEntry { keysym: 0x02FD, ucs: 0x016D, deprecated: false },
    KeysymEntry { keysym: 0x02FE, ucs: 0x015D, deprecated: false },
    KeysymEntry { keysym: 0x03A2, ucs: 0x0138, deprecated: false },
    KeysymEntry { keysym: 0x03A3, ucs: 0x0156, deprecated: false },
    KeysymEntry { keysym: 0x03A5, ucs: 0x0128, deprecated: false },
    KeysymEntry { keysym: 0x03A6, ucs: 0x013B, deprecated: false },
    KeysymEntry { keysym: 0x03AA, ucs: 0x0112, deprecated: false },
    KeysymEntry { keysym: 0x03AB, ucs: 0x0122, deprecated: false },
    KeysymEntry { keysym: 0x03AC, ucs: 0x0166, deprecated: false },
    KeysymEntry { keysym: 0x03B3, ucs: 0x0157, deprecated: false },
    KeysymEntry { keysym: 0x03B5, ucs: 0x0129, deprecated: false },
    KeysymEntry { keysym: 0x03B6, ucs: 0x013C, deprecated: false },
    KeysymEntry { keysym: 0x03BA, ucs: 0x0113, deprecated: false },
    KeysymEntry { keysym: 0x03BB, ucs: 0x0123, deprecated: false },
    KeysymEntry { keysym: 0x03BC, ucs: 0x0167, deprecated: false },
    KeysymEntry { keysym: 0x03BD, ucs: 0x014A, deprecated: false },
    KeysymEntry { keysym: 0x03BF, ucs: 0x014B, deprecated: false },
    KeysymEntry { keysym: 0x03C0, ucs: 0x0100, deprecated: false },
    KeysymEntry { keysym: 0x03C7, ucs: 0x012E, deprecated: false },
    KeysymEntry { keysym: 0x03CC, ucs: 0x0116, deprecated: false },
    KeysymEntry { keysym: 0x03CF, ucs: 0x012A, deprecated: false },
    KeysymEntry { keysym: 0x03D1, ucs: 0x0145, deprecated: false },
    KeysymEntry { keysym: 0x03D2, ucs: 0x014C, deprecated: false },
    KeysymEntry { keysym: 0x03D3, ucs: 0x0136, deprecated: false },
    KeysymEntry { keysym: 0x03D9, ucs: 0x0172, deprecated: false },
    KeysymEntry { keysym: 0x03DD, ucs: 0x0168, deprecated: false },
    KeysymEntry { keysym: 0x03DE, ucs: 0x016A, deprecated: false },
    KeysymEntry { keysym: 0x03E0, ucs: 0x0101, deprecated: false },
    KeysymEntry { keysym: 0x03E7, ucs: 0x012F, deprecated: false },
    KeysymEntry { keysym: 0x03EC, ucs: 0x0117, deprecated: false },
    KeysymEntry { keysym: 0x03EF, ucs: 0x012B, deprecated: false },
    KeysymEntry { keysym: 0x03F1, ucs: 0x0146, deprecated: false },
    KeysymEntry { keysym: 0x03F2, ucs: 0x014D, deprecated: false },
    KeysymEntry { keysym: 0x03F3, ucs: 0x0137, deprecated: false },
    KeysymEntry { keysym: 0x03F9, ucs: 0x0173, deprecated: false },
    KeysymEntry { keysym: 0x03FD, ucs: 0x0169, deprecated: false },
    KeysymEntry { keysym: 0x03FE, ucs: 0x016B, deprecated: false },
    KeysymEntry { keysym: 0x047E, ucs: 0x203E, deprecated: false },
    KeysymEntry { keysym: 0x04A1, ucs: 0x3002, deprecated: false },
    KeysymEntry { keysym: 0x04A2, ucs: 0x300C, deprecated: false },
    KeysymEntry { keysym: 0x04A3, ucs: 0x300D, deprecated: false },
    KeysymEntry { keysym: 0x04A4, ucs: 0x3001, deprecated: false },
    KeysymEntry { keysym: 0x04A5, ucs: 0x30FB, deprecated: false },
    KeysymEntry { keysym: 0x04A6, ucs: 0x30F2, deprecated: false },
    KeysymEntry { keysym: 0x04A7, ucs: 0x30A1, deprecated: false },
    KeysymEntry { keysym: 0x04A8, ucs: 0x30A3, deprecated: false },
    KeysymEntry { keysym: 0x04A9, ucs: 0x30A5, deprecated: false },
    KeysymEntry { keysym: 0x04AA, ucs: 0x30A7, deprecated: false },
    KeysymEntry { keysym: 0x04AB, ucs: 0x30A9, deprecated: false },
    KeysymEntry { keysym: 0x04AC, ucs: 0x30E3, deprecated: false },
    KeysymEntry { keysym: 0x04AD, ucs: 0x30E5, deprecated: false },
    KeysymEntry { keysym: 0x04AE, ucs: 0x30E7, deprecated: false },
    KeysymEntry { keysym: 0x04AF, ucs: 0x30C3, deprecated: false },
    KeysymEntry { keysym: 0x04B0, ucs: 0x30FC, deprecated: false },
    KeysymEntry { keysym: 0x04B1, ucs: 0x30A2, deprecated: false },
    KeysymEntry { keysym: 0x04B2, ucs: 0x30A4, deprecated: false },
    KeysymEntry { keysym: 0x04B3, ucs: 0x30A6, deprecated: false },
    KeysymEntry { keysym: 0x04B4, ucs: 0x30A8, deprecated: false },
    KeysymEntry { keysym: 0x04B5, ucs: 0x30AA, deprecated: false },
    KeysymEntry { keysym: 0x04B6, ucs: 0x30AB, deprecated: false },
    KeysymEntry { keysym: 0x04B7, ucs: 0x30AD, deprecated: false },
    KeysymEntry { keysym: 0x04B8, ucs: 0x30AF, deprecated: false },
    KeysymEntry { keysym: 0x04B9, ucs: 0x30B1, deprecated: false },
    KeysymEntry { keysym: 0x04BA, ucs: 0x30B3, deprecated: false },
    KeysymEntry { keysym: 0x04BB, ucs: 0x30B5, deprecated: false },
    KeysymEntry { keysym: 0x04BC, ucs: 0x30B7, deprecated: false },
    KeysymEntry { keysym: 0x04BD, ucs: 0x30B9, deprecated: false },
    KeysymEntry { keysym: 0x04BE, ucs: 0x30BB, deprecated: false },
    KeysymEntry { keysym: 0x04BF, ucs: 0x30BD, deprecated: false },
    KeysymEntry { keysym: 0x04C0, ucs: 0x30BF, deprecated: false },
    KeysymEntry { keysym: 0x04C1, ucs: 0x30C1, deprecated: false },
    KeysymEntry { keysym: 0x04C2, ucs: 0x30C4, deprecated: false },
    KeysymEntry { keysym: 0x04C3, ucs: 0x30C6, deprecated: false },
    KeysymEntry { keysym: 0x04C4, ucs: 0x30C8, deprecated: false },
    KeysymEntry { keysym: 0x04C5, ucs: 0x30CA, deprecated: false },
    KeysymEntry { keysym: 0x04C6, ucs: 0x30CB, deprecated: false },
    KeysymEntry { keysym: 0x04C7, ucs: 0x30CC, deprecated: false },
    KeysymEntry { keysym: 0x04C8, ucs: 0x30CD, deprecated: false },
    KeysymEntry { keysym: 0x04C9, ucs: 0x30CE, deprecated: false },
    KeysymEntry { keysym: 0x04CA, ucs: 0x30CF, deprecated: false },
    KeysymEntry { keysym: 0x04CB, ucs: 0x30D2, deprecated: false },
    KeysymEntry { keysym: 0x04CC, ucs: 0x30D5, deprecated: false },
    KeysymEntry { keysym: 0x04CD, ucs: 0x30D8, deprecated: false },
    KeysymEntry { keysym: 0x04CE, ucs: 0x30DB, deprecated: false },
    KeysymEntry { keysym: 0x04CF, ucs: 0x30DE, deprecated: false },
    KeysymEntry { keysym: 0x04D0, ucs: 0x30DF, deprecated: false },
    KeysymEntry { keysym: 0x04D1, ucs: 0x30E0, deprecated: false },
    KeysymEntry { keysym: 0x04D2, ucs: 0x30E1, deprecated: false },
    KeysymEntry { keysym: 0x04D3, ucs: 0x30E2, deprecated: false },
    KeysymEntry { keysym: 0x04D4, ucs: 0x30E4, deprecated: false },
    KeysymEntry { keysym: 0x04D5, ucs: 0x30E6, deprecated: false },
    KeysymEntry { keysym: 0x04D6, ucs: 0x30E8, deprecated: false },
    KeysymEntry { keysym: 0x04D7, ucs: 0x30E9, deprecated: false },
    KeysymEntry { keysym: 0x04D8, ucs: 0x30EA, deprecated: false },
    KeysymEntry { keysym: 0x04D9, ucs: 0x30EB, deprecated: false },
    KeysymEntry { keysym: 0x04DA, ucs: 0x30EC, deprecated: false },
    KeysymEntry { keysym: 0x04DB, ucs: 0x30ED, deprecated: false },
    KeysymEntry { keysym: 0x04DC, ucs: 0x30EF, deprecated: false },
    KeysymEntry { keysym: 0x04DD, ucs: 0x30F3, deprecated: false },
    KeysymEntry { keysym: 0x04DE, ucs: 0x309B, deprecated: false },
    KeysymEntry { keysym: 0x04DF, ucs: 0x309C, deprecated: false },
    KeysymEntry { keysym: 0x05AC, ucs: 0x060C, deprecated: false },
    KeysymEntry { keysym: 0x05BB, ucs: 0x061B, deprecated: false },
    KeysymEntry { keysym: 0x05BF, ucs: 0x061F, deprecated: false },
    KeysymEntry { keysym: 0x05C1, ucs: 0x0621, deprecated: false },
    KeysymEntry { keysym: 0x05C2, ucs: 0x0622, deprecated: false },
    KeysymEntry { keysym: 0x05C3, ucs: 0x0623, deprecated: false },
    KeysymEntry { keysym: 0x05C4, ucs: 0x0624, deprecated: false },
    KeysymEntry { keysym: 0x05C5, ucs: 0x0625, deprecated: false },
    KeysymEntry { keysym: 0x05C6, ucs: 0x0626, deprecated: false },
    KeysymEntry { keysym: 0x05C7, ucs: 0x0627, deprecated: false },
    KeysymEntry { keysym: 0x05C8, ucs: 0x0628, deprecated: false },
    KeysymEntry { keysym: 0x05C9, ucs: 0x0629, deprecated: false },
    KeysymEntry { keysym: 0x05CA, ucs: 0x062A, deprecated: false },
    KeysymEntry { keysym: 0x05CB, ucs: 0x062B, deprecated: false },
    KeysymEntry { keysym: 0x05CC, ucs: 0x062C, deprecated: false },
    KeysymEntry { keysym: 0x05CD, ucs: 0x062D, deprecated: false },
    KeysymEntry { keysym: 0x05CE, ucs: 0x062E, deprecated: false },
    KeysymEntry { keysym: 0x05CF, ucs: 0x062F, deprecated: false },
    KeysymEntry { keysym: 0x05D0, ucs: 0x0630, deprecated: false },
    KeysymEntry { keysym: 0x05D1, ucs: 0x0631, deprecated: false },
    KeysymEntry { keysym: 0x05D2, ucs: 0x0632, deprecated: false },
    KeysymEntry { keysym: 0x05D3, ucs: 0x0633, deprecated: false },
    KeysymEntry { keysym: 0x05D4, ucs: 0x0634, deprecated: false },
    KeysymEntry { keysym: 0x05D5, ucs: 0x0635, deprecated: false },
    KeysymEntry { keysym: 0x05D6, ucs: 0x0636, deprecated: false },
    KeysymEntry { keysym: 0x05D7, ucs: 0x0637, deprecated: false },
    KeysymEntry { keysym: 0x05D8, ucs: 0x0638, deprecated: false },
    KeysymEntry { keysym: 0x05D9, ucs: 0x0639, deprecated: false },
    KeysymEntry { keysym: 0x05DA, ucs: 0x063A, deprecated: false },
    KeysymEntry { keysym: 0x05E0, ucs: 0x0640, deprecated: false },
    KeysymEntry { keysym: 0x05E1, ucs: 0x0641, deprecated: false },
    KeysymEntry { keysym: 0x05E2, ucs: 0x0642, deprecated: false },
    KeysymEntry { keysym: 0x05E3, ucs: 0x0643, deprecated: false },
    KeysymEntry { keysym: 0x05E4, ucs: 0x0644, deprecated: false },
    KeysymEntry { keysym: 0x05E5, ucs: 0x0645, deprecated: false },
    KeysymEntry { keysym: 0x05E6, ucs: 0x0646, deprecated: false },
    KeysymEntry { keysym: 0x05E7, ucs: 0x0647, deprecated: false },
    KeysymEntry { keysym: 0x05E8, ucs: 0x0648, deprecated: false },
    KeysymEntry { keysym: 0x05E9, ucs: 0x0649, deprecated: false },
    KeysymEntry { keysym: 0x05EA, ucs: 0x064A, deprecated: false },
    KeysymEntry { keysym: 0x05EB, ucs: 0x064B, deprecated: false },
    KeysymEntry { keysym: 0x05EC, ucs: 0x064C, deprecated: false },
    KeysymEntry { keysym: 0x05ED, ucs: 0x064D, deprecated: false },
    KeysymEntry { keysym: 0x05EE, ucs: 0x064E, deprecated: false },
    KeysymEntry { keysym: 0x05EF, ucs: 0x064F, deprecated: false },
    KeysymEntry { keysym: 0x05F0, ucs: 0x0650, deprecated: false },
    KeysymEntry { keysym: 0x05F1, ucs: 0x0651, deprecated: false },
    KeysymEntry { keysym: 0x05F2, ucs: 0x0652, deprecated: false },
    KeysymEntry { keysym: 0x06A1, ucs: 0x0452, deprecated: false },
    KeysymEntry { keysym: 0x06A2, ucs: 0x0453, deprecated: false },
    KeysymEntry { keysym: 0x06A3, ucs: 0x0451, deprecated: false },
    KeysymEntry { keysym: 0x06A4, ucs: 0x0454, deprecated: false },
    KeysymEntry { keysym: 0x06A5, ucs: 0x0455, deprecated: false },
    KeysymEntry { keysym: 0x06A6, ucs: 0x0456, deprecated: false },
    KeysymEntry { keysym: 0x06A7, ucs: 0x0457, deprecated: false },
    KeysymEntry { keysym: 0x06A8, ucs: 0x0458, deprecated: false },
    KeysymEntry { keysym: 0x06A9, ucs: 0x0459, deprecated: false },
    KeysymEntry { keysym: 0x06AA, ucs: 0x045A, deprecated: false },
    KeysymEntry { keysym: 0x06AB, ucs: 0x045B, deprecated: false },
    KeysymEntry { keysym: 0x06AC, ucs: 0x045C, deprecated: false },
    KeysymEntry { keysym: 0x06AD, ucs: 0x0491, deprecated: false },
    KeysymEntry { keysym: 0x06AE, ucs: 0x045E, deprecated: false },
    KeysymEntry { keysym: 0x06AF, ucs: 0x045F, deprecated: false },
    KeysymEntry { keysym: 0x06B0, ucs: 0x2116, deprecated: false },
    KeysymEntry { keysym: 0x06B1, ucs: 0x0402, deprecated: false },
    KeysymEntry { keysym: 0x06B2, ucs: 0x0403, deprecated: false },
    KeysymEntry { keysym: 0x06B3, ucs: 0x0401, deprecated: false },
    KeysymEntry { keysym: 0x06B4, ucs: 0x0404, deprecated: false },
    KeysymEntry { keysym: 0x06B5, ucs: 0x0405, deprecated: false },
    KeysymEntry { keysym: 0x06B6, ucs: 0x0406, deprecated: false },
    KeysymEntry { keysym: 0x06B7, ucs: 0x0407, deprecated: false },
    KeysymEntry { keysym: 0x06B8, ucs: 0x0408, deprecated: false },
    KeysymEntry { keysym: 0x06B9, ucs: 0x0409, deprecated: false },
    KeysymEntry { keysym: 0x06BA, ucs: 0x040A, deprecated: false },
    KeysymEntry { keysym: 0x06BB, ucs: 0x040B, deprecated: false },
    KeysymEntry { keysym: 0x06BC, ucs: 0x040C, deprecated: false },
    KeysymEntry { keysym: 0x06BD, ucs: 0x0490, deprecated: false },
    KeysymEntry { keysym: 0x06BE, ucs: 0x040E, deprecated: false },
    KeysymEntry { keysym: 0x06BF, ucs: 0x040F, deprecated: false },
    KeysymEntry { keysym: 0x06C0, ucs: 0x044E, deprecated: false },
    KeysymEntry { keysym: 0x06C1, ucs: 0x0430, deprecated: false },
    KeysymEntry { keysym: 0x06C2, ucs: 0x0431, deprecated: false },
    KeysymEntry { keysym: 0x06C3, ucs: 0x0446, deprecated: false },
    KeysymEntry { keysym: 0x06C4, ucs: 0x0434, deprecated: false },
    KeysymEntry { keysym: 0x06C5, ucs: 0x0435, deprecated: false },
    KeysymEntry { keysym: 0x06C6, ucs: 0x0444, deprecated: false },
    KeysymEntry { keysym: 0x06C7, ucs: 0x0433, deprecated: false },
    KeysymEntry { keysym: 0x06C8, ucs: 0x0445, deprecated: false },
    KeysymEntry { keysym: 0x06C9, ucs: 0x0438, deprecated: false },
    KeysymEntry { keysym: 0x06CA, ucs: 0x0439, deprecated: false },
    KeysymEntry { keysym: 0x06CB, ucs: 0x043A, deprecated: false },
    KeysymEntry { keysym: 0x06CC, ucs: 0x043B, deprecated: false },
    KeysymEntry { keysym: 0x06CD, ucs: 0x043C, deprecated: false },
    KeysymEntry { keysym: 0x06CE, ucs: 0x043D, deprecated: false },
    KeysymEntry { keysym: 0x06CF, ucs: 0x043E, deprecated: false },
    KeysymEntry { keysym: 0x06D0, ucs: 0x043F, deprecated: false },
    KeysymEntry { keysym: 0x06D1, ucs: 0x044F, deprecated: false },
    KeysymEntry { keysym: 0x06D2, ucs: 0x0440, deprecated: false },
    KeysymEntry { keysym: 0x06D3, ucs: 0x0441, deprecated: false },
    KeysymEntry { keysym: 0x06D4, ucs: 0x0442, deprecated: false },
    KeysymEntry { keysym: 0x06D5, ucs: 0x0443, deprecated: false },
    KeysymEntry { keysym: 0x06D6, ucs: 0x0436, deprecated: false },
    KeysymEntry { keysym: 0x06D7, ucs: 0x0432, deprecated: false },
    KeysymEntry { keysym: 0x06D8, ucs: 0x044C, deprecated: false },
    KeysymEntry { keysym: 0x06D9, ucs: 0x044B, deprecated: false },
    KeysymEntry { keysym: 0x06DA, ucs: 0x0437, deprecated: false },
    KeysymEntry { keysym: 0x06DB, ucs: 0x0448, deprecated: false },
    KeysymEntry { keysym: 0x06DC, ucs: 0x044D, deprecated: false },
    KeysymEntry { keysym: 0x06DD, ucs: 0x0449, deprecated: false },
    KeysymEntry { keysym: 0x06DE, ucs: 0x0447, deprecated: false },
    KeysymEntry { keysym: 0x06DF, ucs: 0x044A, deprecated: false },
    KeysymEntry { keysym: 0x06E0, ucs: 0x042E, deprecated: false },
    KeysymEntry { keysym: 0x06E1, ucs: 0x0410, deprecated: false },
    KeysymEntry { keysym: 0x06E2, ucs: 0x0411, deprecated: false },
    KeysymEntry { keysym: 0x06E3, ucs: 0x0426, deprecated: false },
    KeysymEntry { keysym: 0x06E4, ucs: 0x0414, deprecated: false },
    KeysymEntry { keysym: 0x06E5, ucs: 0x0415, deprecated: false },
    KeysymEntry { keysym: 0x06E6, ucs: 0x0424, deprecated: false },
    KeysymEntry { keysym: 0x06E7, ucs: 0x0413, deprecated: false },
    KeysymEntry { keysym: 0x06E8, ucs: 0x0425, deprecated: false },
    KeysymEntry { keysym: 0x06E9, ucs: 0x0418, deprecated: false },
    KeysymEntry { keysym: 0x06EA, ucs: 0x0419, deprecated: false },
    KeysymEntry { keysym: 0x06EB, ucs: 0x041A, deprecated: false },
    KeysymEntry { keysym: 0x06EC, ucs: 0x041B, deprecated: false },
    KeysymEntry { keysym: 0x06ED, ucs: 0x041C, deprecated: false },
    KeysymEntry { keysym: 0x06EE, ucs: 0x041D, deprecated: false },
    KeysymEntry { keysym: 0x06EF, ucs: 0x041E, deprecated: false },
    KeysymEntry { keysym: 0x06F0, ucs: 0x041F, deprecated: false },
    KeysymEntry { keysym: 0x06F1, ucs: 0x042F, deprecated: false },
    KeysymEntry { keysym: 0x06F2, ucs: 0x0420, deprecated: false },
    KeysymEntry { keysym: 0x06F3, ucs: 0x0421, deprecated: false },
    KeysymEntry { keysym: 0x06F4, ucs: 0x0422, deprecated: false },
    KeysymEntry { keysym: 0x06F5, ucs: 0x0423, deprecated: false },
    KeysymEntry { keysym: 0x06F6, ucs: 0x0416, deprecated: false },
    KeysymEntry { keysym: 0x06F7, ucs: 0x0412, deprecated: false },
    KeysymEntry { keysym: 0x06F8, ucs: 0x042C, deprecated: false },
    KeysymEntry { keysym: 0x06F9, ucs: 0x042B, deprecated: false },
    KeysymEntry { keysym: 0x06FA, ucs: 0x0417, deprecated: false },
    KeysymEntry { keysym: 0x06FB, ucs: 0x0428, deprecated: false },
    KeysymEntry { keysym: 0x06FC, ucs: 0x042D, deprecated: false },
    KeysymEntry { keysym: 0x06FD, ucs: 0x0429, deprecated: false },
    KeysymEntry { keysym: 0x06FE, ucs: 0x0427, deprecated: false },
    KeysymEntry { keysym: 0x06FF, ucs: 0x042A, deprecated: false },
    KeysymEntry { keysym: 0x07A1, ucs: 0x0386, deprecated: false },
    KeysymEntry { keysym: 0x07A2, ucs: 0x0388, deprecated: false },
    KeysymEntry { keysym: 0x07A3, ucs: 0x0389, deprecated: false },
    KeysymEntry { keysym: 0x07A4, ucs: 0x038A, deprecated: false },
    KeysymEntry { keysym: 0x07A5, ucs: 0x03AA, deprecated: false },
    KeysymEntry { keysym: 0x07A7, ucs: 0x038C, deprecated: false },
    KeysymEntry { keysym: 0x07A8, ucs: 0x038E, deprecated: false },
    KeysymEntry { keysym: 0x07A9, ucs: 0x03AB, deprecated: false },
    KeysymEntry { keysym: 0x07AB, ucs: 0x038F, deprecated: false },
    KeysymEntry { keysym: 0x07AE, ucs: 0x0385, deprecated: false },
    KeysymEntry { keysym: 0x07AF, ucs: 0x2015, deprecated: false },
    KeysymEntry { keysym: 0x07B1, ucs: 0x03AC, deprecated: false },
    KeysymEntry { keysym: 0x07B2, ucs: 0x03AD, deprecated: false },
    KeysymEntry { keysym: 0x07B3, ucs: 0x03AE, deprecated: false },
    KeysymEntry { keysym: 0x07B4, ucs: 0x03AF, deprecated: false },
    KeysymEntry { keysym: 0x07B5, ucs: 0x03CA, deprecated: false },
    KeysymEntry { keysym: 0x07B6, ucs: 0x0390, deprecated: false },
    KeysymEntry { keysym: 0x07B7, ucs: 0x03CC, deprecated: false },
    KeysymEntry { keysym: 0x07B8, ucs: 0x03CD, deprecated: false },
    KeysymEntry { keysym: 0x07B9, ucs: 0x03CB, deprecated: false },
    KeysymEntry { keysym: 0x07BA, ucs: 0x03B0, deprecated: false },
    KeysymEntry { keysym: 0x07BB, ucs: 0x03CE, deprecated: false },
    KeysymEntry { keysym: 0x07C1, ucs: 0x0391, deprecated: false },
    KeysymEntry { keysym: 0x07C2, ucs: 0x0392, deprecated: false },
    KeysymEntry { keysym: 0x07C3, ucs: 0x0393, deprecated: false },
    KeysymEntry { keysym: 0x07C4, ucs: 0x0394, deprecated: false },
    KeysymEntry { keysym: 0x07C5, ucs: 0x0395, deprecated: false },
    KeysymEntry { keysym: 0x07C6, ucs: 0x0396, deprecated: false },
    KeysymEntry { keysym: 0x07C7, ucs: 0x0397, deprecated: false },
    KeysymEntry { keysym: 0x07C8, ucs: 0x0398, deprecated: false },
    KeysymEntry { keysym: 0x07C9, ucs: 0x0399, deprecated: false },
    KeysymEntry { keysym: 0x07CA, ucs: 0x039A, deprecated: false },
    KeysymEntry { keysym: 0x07CB, ucs: 0x039B, deprecated: false },
    KeysymEntry { keysym: 0x07CC, ucs: 0x039C, deprecated: false },
    KeysymEntry { keysym: 0x07CD, ucs: 0x039D, deprecated: false },
    KeysymEntry { keysym: 0x07CE, ucs: 0x039E, deprecated: false },
    KeysymEntry { keysym: 0x07CF, ucs: 0x039F, deprecated: false },
    KeysymEntry { keysym: 0x07D0, ucs: 0x03A0, deprecated: false },
    KeysymEntry { keysym: 0x07D1, ucs: 0x03A1, deprecated: false },
    KeysymEntry { keysym: 0x07D2, ucs: 0x03A3, deprecated: false },
    KeysymEntry { keysym: 0x07D4, ucs: 0x03A4, deprecated: false },
    KeysymEntry { keysym: 0x07D5, ucs: 0x03A5, deprecated: false },
    KeysymEntry { keysym: 0x07D6, ucs: 0x03A6, deprecated: false },
    KeysymEntry { keysym: 0x07D7, ucs: 0x03A7, deprecated: false },
    KeysymEntry { keysym: 0x07D8, ucs: 0x03A8, deprecated: false },
    KeysymEntry { keysym: 0x07D9, ucs: 0x03A9, deprecated: false },
    KeysymEntry { keysym: 0x07E1, ucs: 0x03B1, deprecated: false },
    KeysymEntry { keysym: 0x07E2, ucs: 0x03B2, deprecated: false },
    KeysymEntry { keysym: 0x07E3, ucs: 0x03B3, deprecated: false },
    KeysymEntry { keysym: 0x07E4, ucs: 0x03B4, deprecated: false },
    KeysymEntry { keysym: 0x07E5, ucs: 0x03B5, deprecated: false },
    KeysymEntry { keysym: 0x07E6, ucs: 0x03B6, deprecated: false },
    KeysymEntry { keysym: 0x07E7, ucs: 0x03B7, deprecated: false },
    KeysymEntry { keysym: 0x07E8, ucs: 0x03B8, deprecated: false },
    KeysymEntry { keysym: 0x07E9, ucs: 0x03B9, deprecated: false },
    KeysymEntry { keysym: 0x07EA, ucs: 0x03BA, deprecated: false },
    KeysymEntry { keysym: 0x07EB, ucs: 0x03BB, deprecated: false },
    KeysymEntry { keysym: 0x07EC, ucs: 0x03BC, deprecated: false },
    KeysymEntry { keysym: 0x07ED, ucs: 0x03BD, deprecated: false },
    KeysymEntry { keysym: 0x07EE, ucs: 0x03BE, deprecated: false },
    KeysymEntry { keysym: 0x07EF, ucs: 0x03BF, deprecated: false },
    KeysymEntry { keysym: 0x07F0, ucs: 0x03C0, deprecated: false },
    KeysymEntry { keysym: 0x07F1, ucs: 0x03C1, deprecated: false },
    KeysymEntry { keysym: 0x07F2, ucs: 0x03C3, deprecated: false },
    KeysymEntry { keysym: 0x07F3, ucs: 0x03C2, deprecated: false },
    KeysymEntry { keysym: 0x07F4, ucs: 0x03C4, deprecated: false },
    KeysymEntry { keysym: 0x07F5, ucs: 0x03C5, deprecated: false },
    KeysymEntry { keysym: 0x07F6, ucs: 0x03C6, deprecated: false },
    KeysymEntry { keysym: 0x07F7, ucs: 0x03C7, deprecated: false },
    KeysymEntry { keysym: 0x07F8, ucs: 0x03C8, deprecated: false },
    KeysymEntry { keysym: 0x07F9, ucs: 0x03C9, deprecated: false },
    KeysymEntry { keysym: 0x08A1, ucs: 0x23B7, deprecated: false },
    KeysymEntry { keysym: 0x08A2, ucs: 0x250C, deprecated: true },
    KeysymEntry { keysym: 0x08A3, ucs: 0x2500, deprecated: true },
    KeysymEntry { keysym: 0x08A4, ucs: 0x2320, deprecated: false },
    KeysymEntry { keysym: 0x08A5, ucs: 0x2321, deprecated: false },
    KeysymEntry { keysym: 0x08A6, ucs: 0x2502, deprecated: true },
    KeysymEntry { keysym: 0x08A7, ucs: 0x23A1, deprecated: false },
    KeysymEntry { keysym: 0x08A8, ucs: 0x23A3, deprecated: false },
    KeysymEntry { keysym: 0x08A9, ucs: 0x23A4, deprecated: false },
    KeysymEntry { keysym: 0x08AA, ucs: 0x23A6, deprecated: false },
    KeysymEntry { keysym: 0x08AB, ucs: 0x239B, deprecated: false },
    KeysymEntry { keysym: 0x08AC, ucs: 0x239D, deprecated: false },
    KeysymEntry { keysym: 0x08AD, ucs: 0x239E, deprecated: false },
    KeysymEntry { keysym: 0x08AE, ucs: 0x23A0, deprecated: false },
    KeysymEntry { keysym: 0x08AF, ucs: 0x23A8, deprecated: false },
    KeysymEntry { keysym: 0x08B0, ucs: 0x23AC, deprecated: false },
    KeysymEntry { keysym: 0x08BC, ucs: 0x2264, deprecated: false },
    KeysymEntry { keysym: 0x08BD, ucs: 0x2260, deprecated: false },
    KeysymEntry { keysym: 0x08BE, ucs: 0x2265, deprecated: false },
    KeysymEntry { keysym: 0x08BF, ucs: 0x222B, deprecated: false },
    KeysymEntry { keysym: 0x08C0, ucs: 0x2234, deprecated: false },
    KeysymEntry { keysym: 0x08C1, ucs: 0x221D, deprecated: false },
    KeysymEntry { keysym: 0x08C2, ucs: 0x221E, deprecated: false },
    KeysymEntry { keysym: 0x08C5, ucs: 0x2207, deprecated: false },
    KeysymEntry { keysym: 0x08C8, ucs: 0x223C, deprecated: false },
    KeysymEntry { keysym: 0x08C9, ucs: 0x2243, deprecated: false },
    KeysymEntry { keysym: 0x08CD, ucs: 0x21D4, deprecated: false },
    KeysymEntry { keysym: 0x08CE, ucs: 0x21D2, deprecated: false },
    KeysymEntry { keysym: 0x08CF, ucs: 0x2261, deprecated: false },
    KeysymEntry { keysym: 0x08D6, ucs: 0x221A, deprecated: false },
    KeysymEntry { keysym: 0x08DA, ucs: 0x2282, deprecated: false },
    KeysymEntry { keysym: 0x08DB, ucs: 0x2283, deprecated: false },
    KeysymEntry { keysym: 0x08DC, ucs: 0x2229, deprecated: false },
    KeysymEntry { keysym: 0x08DD, ucs: 0x222A, deprecated: false },
    KeysymEntry { keysym: 0x08DE, ucs: 0x2227, deprecated: false },
    KeysymEntry { keysym: 0x08DF, ucs: 0x2228, deprecated: false },
    KeysymEntry { keysym: 0x08EF, ucs: 0x2202, deprecated: false },
    KeysymEntry { keysym: 0x08F6, ucs: 0x0192, deprecated: false },
    KeysymEntry { keysym: 0x08FB, ucs: 0x2190, deprecated: false },
    KeysymEntry { keysym: 0x08FC, ucs: 0x2191, deprecated: false },
    KeysymEntry { keysym: 0x08FD, ucs: 0x2192, deprecated: false },
    KeysymEntry { keysym: 0x08FE, ucs: 0x2193, deprecated: false },
    KeysymEntry { keysym: 0x09E0, ucs: 0x25C6, deprecated: false },
    KeysymEntry { keysym: 0x09E1, ucs: 0x2592, deprecated: false },
    KeysymEntry { keysym: 0x09E2, ucs: 0x2409, deprecated: false },
    KeysymEntry { keysym: 0x09E3, ucs: 0x240C, deprecated: false },
    KeysymEntry { keysym: 0x09E4, ucs: 0x240D, deprecated: false },
    KeysymEntry { keysym: 0x09E5, ucs: 0x240A, deprecated: false },
    KeysymEntry { keysym: 0x09E8, ucs: 0x2424, deprecated: false },
    KeysymEntry { keysym: 0x09E9, ucs: 0x240B, deprecated: false },
    KeysymEntry { keysym: 0x09EA, ucs: 0x2518, deprecated: false },
    KeysymEntry { keysym: 0x09EB, ucs: 0x2510, deprecated: false },
    KeysymEntry { keysym: 0x09EC, ucs: 0x250C, deprecated: false },
    KeysymEntry { keysym: 0x09ED, ucs: 0x2514, deprecated: false },
    KeysymEntry { keysym: 0x09EE, ucs: 0x253C, deprecated: false },
    KeysymEntry { keysym: 0x09EF, ucs: 0x23BA, deprecated: false },
    KeysymEntry { keysym: 0x09F0, ucs: 0x23BB, deprecated: false },
    KeysymEntry { keysym: 0x09F1, ucs: 0x2500, deprecated: false },
    KeysymEntry { keysym: 0x09F2, ucs: 0x23BC, deprecated: false },
    KeysymEntry { keysym: 0x09F3, ucs: 0x23BD, deprecated: false },
    KeysymEntry { keysym: 0x09F4, ucs: 0x251C, deprecated: false },
    KeysymEntry { keysym: 0x09F5, ucs: 0x2524, deprecated: false },
    KeysymEntry { keysym: 0x09F6, ucs: 0x2534, deprecated: false },
    KeysymEntry { keysym: 0x09F7, ucs: 0x252C, deprecated: false },
    KeysymEntry { keysym: 0x09F8, ucs: 0x2502, deprecated: false },
    KeysymEntry { keysym: 0x0AA1, ucs: 0x2003, deprecated: false },
    KeysymEntry { keysym: 0x0AA2, ucs: 0x2002, deprecated: false },
    KeysymEntry { keysym: 0x0AA3, ucs: 0x2004, deprecated: false },
    KeysymEntry { keysym: 0x0AA4, ucs: 0x2005, deprecated: false },
    KeysymEntry { keysym: 0x0AA5, ucs: 0x2007, deprecated: false },
    KeysymEntry { keysym: 0x0AA6, ucs: 0x2008, deprecated: false },
    KeysymEntry { keysym: 0x0AA7, ucs: 0x2009, deprecated: false },
    KeysymEntry { keysym: 0x0AA8, ucs: 0x200A, deprecated: false },
    KeysymEntry { keysym: 0x0AA9, ucs: 0x2014, deprecated: false },
    KeysymEntry { keysym: 0x0AAA, ucs: 0x2013, deprecated: false },
    KeysymEntry { keysym: 0x0AAC, ucs: 0x2423, deprecated: true },
    KeysymEntry { keysym: 0x0AAE, ucs: 0x2026, deprecated: false },
    KeysymEntry { keysym: 0x0AAF, ucs: 0x2025, deprecated: false },
    KeysymEntry { keysym: 0x0AB0, ucs: 0x2153, deprecated: false },
    KeysymEntry { keysym: 0x0AB1, ucs: 0x2154, deprecated: false },
    KeysymEntry { keysym: 0x0AB2, ucs: 0x2155, deprecated: false },
    KeysymEntry { keysym: 0x0AB3, ucs: 0x2156, deprecated: false },
    KeysymEntry { keysym: 0x0AB4, ucs: 0x2157, deprecated: false },
    KeysymEntry { keysym: 0x0AB5, ucs: 0x2158, deprecated: false },
    KeysymEntry { keysym: 0x0AB6, ucs: 0x2159, deprecated: false },
    KeysymEntry { keysym: 0x0AB7, ucs: 0x215A, deprecated: false },
    KeysymEntry { keysym: 0x0AB8, ucs: 0x2105, deprecated: false },
    KeysymEntry { keysym: 0x0ABB, ucs: 0x2012, deprecated: false },
    KeysymEntry { keysym: 0x0ABC, ucs: 0x27E8, deprecated: true },
    KeysymEntry { keysym: 0x0ABD, ucs: 0x002E, deprecated: true },
    KeysymEntry { keysym: 0x0ABE, ucs: 0x27E9, deprecated: true },
    KeysymEntry { keysym: 0x0AC3, ucs: 0x215B, deprecated: false },
    KeysymEntry { keysym: 0x0AC4, ucs: 0x215C, deprecated: false },
    KeysymEntry { keysym: 0x0AC5, ucs: 0x215D, deprecated: false },
    KeysymEntry { keysym: 0x0AC6, ucs: 0x215E, deprecated: false },
    KeysymEntry { keysym: 0x0AC9, ucs: 0x2122, deprecated: false },
    KeysymEntry { keysym: 0x0ACA, ucs: 0x2613, deprecated: true },
    KeysymEntry { keysym: 0x0ACC, ucs: 0x25C1, deprecated: true },
    KeysymEntry { keysym: 0x0ACD, ucs: 0x25B7, deprecated: true },
    KeysymEntry { keysym: 0x0ACE, ucs: 0x25CB, deprecated: true },
    KeysymEntry { keysym: 0x0ACF, ucs: 0x25AF, deprecated: true },
    KeysymEntry { keysym: 0x0AD0, ucs: 0x2018, deprecated: false },
    KeysymEntry { keysym: 0x0AD1, ucs: 0x2019, deprecated: false },
    KeysymEntry { keysym: 0x0AD2, ucs: 0x201C, deprecated: false },
    KeysymEntry { keysym: 0x0AD3, ucs: 0x201D, deprecated: false },
    KeysymEntry { keysym: 0x0AD4, ucs: 0x211E, deprecated: false },
    KeysymEntry { keysym: 0x0AD5, ucs: 0x2030, deprecated: false },
    KeysymEntry { keysym: 0x0AD6, ucs: 0x2032, deprecated: false },
    KeysymEntry { keysym: 0x0AD7, ucs: 0x2033, deprecated: false },
    KeysymEntry { keysym: 0x0AD9, ucs: 0x271D, deprecated: false },
    KeysymEntry { keysym: 0x0ADB, ucs: 0x25AC, deprecated: true },
    KeysymEntry { keysym: 0x0ADC, ucs: 0x25C0, deprecated: true },
    KeysymEntry { keysym: 0x0ADD, ucs: 0x25B6, deprecated: true },
    KeysymEntry { keysym: 0x0ADE, ucs: 0x25CF, deprecated: true },
    KeysymEntry { keysym: 0x0ADF, ucs: 0x25AE, deprecated: true },
    KeysymEntry { keysym: 0x0AE0, ucs: 0x25E6, deprecated: true },
    KeysymEntry { keysym: 0x0AE1, ucs: 0x25AB, deprecated: true },
    KeysymEntry { keysym: 0x0AE2, ucs: 0x25AD, deprecated: true },
    KeysymEntry { keysym: 0x0AE3, ucs: 0x25B3, deprecated: true },
    KeysymEntry { keysym: 0x0AE4, ucs: 0x25BD, deprecated: true },
    KeysymEntry { keysym: 0x0AE5, ucs: 0x2606, deprecated: true },
    KeysymEntry { keysym: 0x0AE6, ucs: 0x2022, deprecated: true },
    KeysymEntry { keysym: 0x0AE7, ucs: 0x25AA, deprecated: true },
    KeysymEntry { keysym: 0x0AE8, ucs: 0x25B2, deprecated: true },
    KeysymEntry { keysym: 0x0AE9, ucs: 0x25BC, deprecated: true },
    KeysymEntry { keysym: 0x0AEA, ucs: 0x261C, deprecated: true },
    KeysymEntry { keysym: 0x0AEB, ucs: 0x261E, deprecated: true },
    KeysymEntry { keysym: 0x0AEC, ucs: 0x2663, deprecated: false },
    KeysymEntry { keysym: 0x0AED, ucs: 0x2666, deprecated: false },
    KeysymEntry { keysym: 0x0AEE, ucs: 0x2665, deprecated: false },
    KeysymEntry { keysym: 0x0AF0, ucs: 0x2720, deprecated: false },
    KeysymEntry { keysym: 0x0AF1, ucs: 0x2020, deprecated: false },
    KeysymEntry { keysym: 0x0AF2, ucs: 0x2021, deprecated: false },
    KeysymEntry { keysym: 0x0AF3, ucs: 0x2713, deprecated: false },
    KeysymEntry { keysym: 0x0AF4, ucs: 0x2717, deprecated: false },
    KeysymEntry { keysym: 0x0AF5, ucs: 0x266F, deprecated: false },
    KeysymEntry { keysym: 0x0AF6, ucs: 0x266D, deprecated: false },
    KeysymEntry { keysym: 0x0AF7, ucs: 0x2642, deprecated: false },
    KeysymEntry { keysym: 0x0AF8, ucs: 0x2640, deprecated: false },
    KeysymEntry { keysym: 0x0AF9, ucs: 0x260E, deprecated: false },
    KeysymEntry { keysym: 0x0AFA, ucs: 0x2315, deprecated: false },
    KeysymEntry { keysym: 0x0AFB, ucs: 0x2117, deprecated: false },
    KeysymEntry { keysym: 0x0AFC, ucs: 0x2038, deprecated: false },
    KeysymEntry { keysym: 0x0AFD, ucs: 0x201A, deprecated: false },
    KeysymEntry { keysym: 0x0AFE, ucs: 0x201E, deprecated: false },
    KeysymEntry { keysym: 0x0BA3, ucs: 0x003C, deprecated: true },
    KeysymEntry { keysym: 0x0BA6, ucs: 0x003E, deprecated: true },
    KeysymEntry { keysym: 0x0BA8, ucs: 0x2228, deprecated: true },
    KeysymEntry { keysym: 0x0BA9, ucs: 0x2227, deprecated: true },
    KeysymEntry { keysym: 0x0BC0, ucs: 0x00AF, deprecated: true },
    KeysymEntry { keysym: 0x0BC2, ucs: 0x22A4, deprecated: false },
    KeysymEntry { keysym: 0x0BC3, ucs: 0x2229, deprecated: true },
    KeysymEntry { keysym: 0x0BC4, ucs: 0x230A, deprecated: false },
    KeysymEntry { keysym: 0x0BC6, ucs: 0x005F, deprecated: true },
    KeysymEntry { keysym: 0x0BCA, ucs: 0x2218, deprecated: false },
    KeysymEntry { keysym: 0x0BCC, ucs: 0x2395, deprecated: false },
    KeysymEntry { keysym: 0x0BCE, ucs: 0x22A5, deprecated: false },
    KeysymEntry { keysym: 0x0BCF, ucs: 0x25CB, deprecated: false },
    KeysymEntry { keysym: 0x0BD3, ucs: 0x2308, deprecated: false },
    KeysymEntry { keysym: 0x0BD6, ucs: 0x222A, deprecated: true },
    KeysymEntry { keysym: 0x0BD8, ucs: 0x2283, deprecated: true },
    KeysymEntry { keysym: 0x0BDA, ucs: 0x2282, deprecated: true },
    KeysymEntry { keysym: 0x0BDC, ucs: 0x22A3, deprecated: false },
    KeysymEntry { keysym: 0x0BFC, ucs: 0x22A2, deprecated: false },
    KeysymEntry { keysym: 0x0CDF, ucs: 0x2017, deprecated: false },
    KeysymEntry { keysym: 0x0CE0, ucs: 0x05D0, deprecated: false },
    KeysymEntry { keysym: 0x0CE1, ucs: 0x05D1, deprecated: false },
    KeysymEntry { keysym: 0x0CE2, ucs: 0x05D2, deprecated: false },
    KeysymEntry { keysym: 0x0CE3, ucs: 0x05D3, deprecated: false },
    KeysymEntry { keysym: 0x0CE4, ucs: 0x05D4, deprecated: false },
    KeysymEntry { keysym: 0x0CE5, ucs: 0x05D5, deprecated: false },
    KeysymEntry { keysym: 0x0CE6, ucs: 0x05D6, deprecated: false },
    KeysymEntry { keysym: 0x0CE7, ucs: 0x05D7, deprecated: false },
    KeysymEntry { keysym: 0x0CE8, ucs: 0x05D8, deprecated: false },
    KeysymEntry { keysym: 0x0CE9, ucs: 0x05D9, deprecated: false },
    KeysymEntry { keysym: 0x0CEA, ucs: 0x05DA, deprecated: false },
    KeysymEntry { keysym: 0x0CEB, ucs: 0x05DB, deprecated: false },
    KeysymEntry { keysym: 0x0CEC, ucs: 0x05DC, deprecated: false },
    KeysymEntry { keysym: 0x0CED, ucs: 0x05DD, deprecated: false },
    KeysymEntry { keysym: 0x0CEE, ucs: 0x05DE, deprecated: false },
    KeysymEntry { keysym: 0x0CEF, ucs: 0x05DF, deprecated: false },
    KeysymEntry { keysym: 0x0CF0, ucs: 0x05E0, deprecated: false },
    KeysymEntry { keysym: 0x0CF1, ucs: 0x05E1, deprecated: false },
    KeysymEntry { keysym: 0x0CF2, ucs: 0x05E2, deprecated: false },
    KeysymEntry { keysym: 0x0CF3, ucs: 0x05E3, deprecated: false },
    KeysymEntry { keysym: 0x0CF4, ucs: 0x05E4, deprecated: false },
    KeysymEntry { keysym: 0x0CF5, ucs: 0x05E5, deprecated: false },
    KeysymEntry { keysym: 0x0CF6, ucs: 0x05E6, deprecated: false },
    KeysymEntry { keysym: 0x0CF7, ucs: 0x05E7, deprecated: false },
    KeysymEntry { keysym: 0x0CF8, ucs: 0x05E8, deprecated: false },
    KeysymEntry { keysym: 0x0CF9, ucs: 0x05E9, deprecated: false },
    KeysymEntry { keysym: 0x0CFA, ucs: 0x05EA, deprecated: false },
    KeysymEntry { keysym: 0x0DA1, ucs: 0x0E01, deprecated: false },
    KeysymEntry { keysym: 0x0DA2, ucs: 0x0E02, deprecated: false },
    KeysymEntry { keysym: 0x0DA3, ucs: 0x0E03, deprecated: false },
    KeysymEntry { keysym: 0x0DA4, ucs: 0x0E04, deprecated: false },
    KeysymEntry { keysym: 0x0DA5, ucs: 0x0E05, deprecated: false },
    KeysymEntry { keysym: 0x0DA6, ucs: 0x0E06, deprecated: false },
    KeysymEntry { keysym: 0x0DA7, ucs: 0x0E07, deprecated: false },
    KeysymEntry { keysym: 0x0DA8, ucs: 0x0E08, deprecated: false },
    KeysymEntry { keysym: 0x0DA9, ucs: 0x0E09, deprecated: false },
    KeysymEntry { keysym: 0x0DAA, ucs: 0x0E0A, deprecated: false },
    KeysymEntry { keysym: 0x0DAB, ucs: 0x0E0B, deprecated: false },
    KeysymEntry { keysym: 0x0DAC, ucs: 0x0E0C, deprecated: false },
    KeysymEntry { keysym: 0x0DAD, ucs: 0x0E0D, deprecated: false },
    KeysymEntry { keysym: 0x0DAE, ucs: 0x0E0E, deprecated: false },
    KeysymEntry { keysym: 0x0DAF, ucs: 0x0E0F, deprecated: false },
    KeysymEntry { keysym: 0x0DB0, ucs: 0x0E10, deprecated: false },
    KeysymEntry { keysym: 0x0DB1, ucs: 0x0E11, deprecated: false },
    KeysymEntry { keysym: 0x0DB2, ucs: 0x0E12, deprecated: false },
    KeysymEntry { keysym: 0x0DB3, ucs: 0x0E13, deprecated: false },
    KeysymEntry { keysym: 0x0DB4, ucs: 0x0E14, deprecated: false },
    KeysymEntry { keysym: 0x0DB5, ucs: 0x0E15, deprecated: false },
    KeysymEntry { keysym: 0x0DB6, ucs: 0x0E16, deprecated: false },
    KeysymEntry { keysym: 0x0DB7, ucs: 0x0E17, deprecated: false },
    KeysymEntry { keysym: 0x0DB8, ucs: 0x0E18, deprecated: false },
    KeysymEntry { keysym: 0x0DB9, ucs: 0x0E19, deprecated: false },
    KeysymEntry { keysym: 0x0DBA, ucs: 0x0E1A, deprecated: false },
    KeysymEntry { keysym: 0x0DBB, ucs: 0x0E1B, deprecated: false },
    KeysymEntry { keysym: 0x0DBC, ucs: 0x0E1C, deprecated: false },
    KeysymEntry { keysym: 0x0DBD, ucs: 0x0E1D, deprecated: false },
    KeysymEntry { keysym: 0x0DBE, ucs: 0x0E1E, deprecated: false },
    KeysymEntry { keysym: 0x0DBF, ucs: 0x0E1F, deprecated: false },
    KeysymEntry { keysym: 0x0DC0, ucs: 0x0E20, deprecated: false },
    KeysymEntry { keysym: 0x0DC1, ucs: 0x0E21, deprecated: false },
    KeysymEntry { keysym: 0x0DC2, ucs: 0x0E22, deprecated: false },
    KeysymEntry { keysym: 0x0DC3, ucs: 0x0E23, deprecated: false },
    KeysymEntry { keysym: 0x0DC4, ucs: 0x0E24, deprecated: false },
    KeysymEntry { keysym: 0x0DC5, ucs: 0x0E25, deprecated: false },
    KeysymEntry { keysym: 0x0DC6, ucs: 0x0E26, deprecated: false },
    KeysymEntry { keysym: 0x0DC7, ucs: 0x0E27, deprecated: false },
    KeysymEntry { keysym: 0x0DC8, ucs: 0x0E28, deprecated: false },
    KeysymEntry { keysym: 0x0DC9, ucs: 0x0E29, deprecated: false },
    KeysymEntry { keysym: 0x0DCA, ucs: 0x0E2A, deprecated: false },
    KeysymEntry { keysym: 0x0DCB, ucs: 0x0E2B, deprecated: false },
    KeysymEntry { keysym: 0x0DCC, ucs: 0x0E2C, deprecated: false },
    KeysymEntry { keysym: 0x0DCD, ucs: 0x0E2D, deprecated: false },
    KeysymEntry { keysym: 0x0DCE, ucs: 0x0E2E, deprecated: false },
    KeysymEntry { keysym: 0x0DCF, ucs: 0x0E2F, deprecated: false },
    KeysymEntry { keysym: 0x0DD0, ucs: 0x0E30, deprecated: false },
    KeysymEntry { keysym: 0x0DD1, ucs: 0x0E31, deprecated: false },
    KeysymEntry { keysym: 0x0DD2, ucs: 0x0E32, deprecated: false },
    KeysymEntry { keysym: 0x0DD3, ucs: 0x0E33, deprecated: false },
    KeysymEntry { keysym: 0x0DD4, ucs: 0x0E34, deprecated: false },
    KeysymEntry { keysym: 0x0DD5, ucs: 0x0E35, deprecated: false },
    KeysymEntry { keysym: 0x0DD6, ucs: 0x0E36, deprecated: false },
    KeysymEntry { keysym: 0x0DD7, ucs: 0x0E37, deprecated: false },
    KeysymEntry { keysym: 0x0DD8, ucs: 0x0E38, deprecated: false },
    KeysymEntry { keysym: 0x0DD9, ucs: 0x0E39, deprecated: false },
    KeysymEntry { keysym: 0x0DDA, ucs: 0x0E3A, deprecated: false },
    KeysymEntry { keysym: 0x0DDE, ucs: 0x0E3E, deprecated: true },
    KeysymEntry { keysym: 0x0DDF, ucs: 0x0E3F, deprecated: false },
    KeysymEntry { keysym: 0x0DE0, ucs: 0x0E40, deprecated: false },
    KeysymEntry { keysym: 0x0DE1, ucs: 0x0E41, deprecated: false },
    KeysymEntry { keysym: 0x0DE2, ucs: 0x0E42, deprecated: false },
    KeysymEntry { keysym: 0x0DE3, ucs: 0x0E43, deprecated: false },
    KeysymEntry { keysym: 0x0DE4, ucs: 0x0E44, deprecated: false },
    KeysymEntry { keysym: 0x0DE5, ucs: 0x0E45, deprecated: false },
    KeysymEntry { keysym: 0x0DE6, ucs: 0x0E46, deprecated: false },
    KeysymEntry { keysym: 0x0DE7, ucs: 0x0E47, deprecated: false },
    KeysymEntry { keysym: 0x0DE8, ucs: 0x0E48, deprecated: false },
    KeysymEntry { keysym: 0x0DE9, ucs: 0x0E49, deprecated: false },
    KeysymEntry { keysym: 0x0DEA, ucs: 0x0E4A, deprecated: false },
    KeysymEntry { keysym: 0x0DEB, ucs: 0x0E4B, deprecated: false },
    KeysymEntry { keysym: 0x0DEC, ucs: 0x0E4C, deprecated: false },
    KeysymEntry { keysym: 0x0DED, ucs: 0x0E4D, deprecated: false },
    KeysymEntry { keysym: 0x0DF0, ucs: 0x0E50, deprecated: false },
    KeysymEntry { keysym: 0x0DF1, ucs: 0x0E51, deprecated: false },
    KeysymEntry { keysym: 0x0DF2, ucs: 0x0E52, deprecated: false },
    KeysymEntry { keysym: 0x0DF3, ucs: 0x0E53, deprecated: false },
    KeysymEntry { keysym: 0x0DF4, ucs: 0x0E54, deprecated: false },
    KeysymEntry { keysym: 0x0DF5, ucs: 0x0E55, deprecated: false },
    KeysymEntry { keysym: 0x0DF6, ucs: 0x0E56, deprecated: false },
    KeysymEntry { keysym: 0x0DF7, ucs: 0x0E57, deprecated: false },
    KeysymEntry { keysym: 0x0DF8, ucs: 0x0E58, deprecated: false },
    KeysymEntry { keysym: 0x0DF9, ucs: 0x0E59, deprecated: false },
    KeysymEntry { keysym: 0x0EA1, ucs: 0x3131, deprecated: false },
    KeysymEntry { keysym: 0x0EA2, ucs: 0x3132, deprecated: false },
    KeysymEntry { keysym: 0x0EA3, ucs: 0x3133, deprecated: false },
    KeysymEntry { keysym: 0x0EA4, ucs: 0x3134, deprecated: false },
    KeysymEntry { keysym: 0x0EA5, ucs: 0x3135, deprecated: false },
    KeysymEntry { keysym: 0x0EA6, ucs: 0x3136, deprecated: false },
    KeysymEntry { keysym: 0x0EA7, ucs: 0x3137, deprecated: false },
    KeysymEntry { keysym: 0x0EA8, ucs: 0x3138, deprecated: false },
    KeysymEntry { keysym: 0x0EA9, ucs: 0x3139, deprecated: false },
    KeysymEntry { keysym: 0x0EAA, ucs: 0x313A, deprecated: false },
    KeysymEntry { keysym: 0x0EAB, ucs: 0x313B, deprecated: false },
    KeysymEntry { keysym: 0x0EAC, ucs: 0x313C, deprecated: false },
    KeysymEntry { keysym: 0x0EAD, ucs: 0x313D, deprecated: false },
    KeysymEntry { keysym: 0x0EAE, ucs: 0x313E, deprecated: false },
    KeysymEntry { keysym: 0x0EAF, ucs: 0x313F, deprecated: false },
    KeysymEntry { keysym: 0x0EB0, ucs: 0x3140, deprecated: false },
    KeysymEntry { keysym: 0x0EB1, ucs: 0x3141, deprecated: false },
    KeysymEntry { keysym: 0x0EB2, ucs: 0x3142, deprecated: false },
    KeysymEntry { keysym: 0x0EB3, ucs: 0x3143, deprecated: false },
    KeysymEntry { keysym: 0x0EB4, ucs: 0x3144, deprecated: false },
    KeysymEntry { keysym: 0x0EB5, ucs: 0x3145, deprecated: false },
    KeysymEntry { keysym: 0x0EB6, ucs: 0x3146, deprecated: false },
    KeysymEntry { keysym: 0x0EB7, ucs: 0x3147, deprecated: false },
    KeysymEntry { keysym: 0x0EB8, ucs: 0x3148, deprecated: false },
    KeysymEntry { keysym: 0x0EB9, ucs: 0x3149, deprecated: false },
    KeysymEntry { keysym: 0x0EBA, ucs: 0x314A, deprecated: false },
    KeysymEntry { keysym: 0x0EBB, ucs: 0x314B, deprecated: false },
    KeysymEntry { keysym: 0x0EBC, ucs: 0x314C, deprecated: false },
    KeysymEntry { keysym: 0x0EBD, ucs: 0x314D, deprecated: false },
    KeysymEntry { keysym: 0x0EBE, ucs: 0x314E, deprecated: false },
    KeysymEntry { keysym: 0x0EBF, ucs: 0x314F, deprecated: false },
    KeysymEntry { keysym: 0x0EC0, ucs: 0x3150, deprecated: false },
    KeysymEntry { keysym: 0x0EC1, ucs: 0x3151, deprecated: false },
    KeysymEntry { keysym: 0x0EC2, ucs: 0x3152, deprecated: false },
    KeysymEntry { keysym: 0x0EC3, ucs: 0x3153, deprecated: false },
    KeysymEntry { keysym: 0x0EC4, ucs: 0x3154, deprecated: false },
    KeysymEntry { keysym: 0x0EC5, ucs: 0x3155, deprecated: false },
    KeysymEntry { keysym: 0x0EC6, ucs: 0x3156, deprecated: false },
    KeysymEntry { keysym: 0x0EC7, ucs: 0x3157, deprecated: false },
    KeysymEntry { keysym: 0x0EC8, ucs: 0x3158, deprecated: false },
    KeysymEntry { keysym: 0x0EC9, ucs: 0x3159, deprecated: false },
    KeysymEntry { keysym: 0x0ECA, ucs: 0x315A, deprecated: false },
    KeysymEntry { keysym: 0x0ECB, ucs: 0x315B, deprecated: false },
    KeysymEntry { keysym: 0x0ECC, ucs: 0x315C, deprecated: false },
    KeysymEntry { keysym: 0x0ECD, ucs: 0x315D, deprecated: false },
    KeysymEntry { keysym: 0x0ECE, ucs: 0x315E, deprecated: false },
    KeysymEntry { keysym: 0x0ECF, ucs: 0x315F, deprecated: false },
    KeysymEntry { keysym: 0x0ED0, ucs: 0x3160, deprecated: false },
    KeysymEntry { keysym: 0x0ED1, ucs: 0x3161, deprecated: false },
    KeysymEntry { keysym: 0x0ED2, ucs: 0x3162, deprecated: false },
    KeysymEntry { keysym: 0x0ED3, ucs: 0x3163, deprecated: false },
    KeysymEntry { keysym: 0x0ED4, ucs: 0x11A8, deprecated: false },
    KeysymEntry { keysym: 0x0ED5, ucs: 0x11A9, deprecated: false },
    KeysymEntry { keysym: 0x0ED6, ucs: 0x11AA, deprecated: false },
    KeysymEntry { keysym: 0x0ED7, ucs: 0x11AB, deprecated: false },
    KeysymEntry { keysym: 0x0ED8, ucs: 0x11AC, deprecated: false },
    KeysymEntry { keysym: 0x0ED9, ucs: 0x11AD, deprecated: false },
    KeysymEntry { keysym: 0x0EDA, ucs: 0x11AE, deprecated: false },
    KeysymEntry { keysym: 0x0EDB, ucs: 0x11AF, deprecated: false },
    KeysymEntry { keysym: 0x0EDC, ucs: 0x11B0, deprecated: false },
    KeysymEntry { keysym: 0x0EDD, ucs: 0x11B1, deprecated: false },
    KeysymEntry { keysym: 0x0EDE, ucs: 0x11B2, deprecated: false },
    KeysymEntry { keysym: 0x0EDF, ucs: 0x11B3, deprecated: false },
    KeysymEntry { keysym: 0x0EE0, ucs: 0x11B4, deprecated: false },
    KeysymEntry { keysym: 0x0EE1, ucs: 0x11B5, deprecated: false },
    KeysymEntry { keysym: 0x0EE2, ucs: 0x11B6, deprecated: false },
    KeysymEntry { keysym: 0x0EE3, ucs: 0x11B7, deprecated: false },
    KeysymEntry { keysym: 0x0EE4, ucs: 0x11B8, deprecated: false },
    KeysymEntry { keysym: 0x0EE5, ucs: 0x11B9, deprecated: false },
    KeysymEntry { keysym: 0x0EE6, ucs: 0x11BA, deprecated: false },
    KeysymEntry { keysym: 0x0EE7, ucs: 0x11BB, deprecated: false },
    KeysymEntry { keysym: 0x0EE8, ucs: 0x11BC, deprecated: false },
    KeysymEntry { keysym: 0x0EE9, ucs: 0x11BD, deprecated: false },
    KeysymEntry { keysym: 0x0EEA, ucs: 0x11BE, deprecated: false },
    KeysymEntry { keysym: 0x0EEB, ucs: 0x11BF, deprecated: false },
    KeysymEntry { keysym: 0x0EEC, ucs: 0x11C0, deprecated: false },
    KeysymEntry { keysym: 0x0EED, ucs: 0x11C1, deprecated: false },
    KeysymEntry { keysym: 0x0EEE, ucs: 0x11C2, deprecated: false },
    KeysymEntry { keysym: 0x0EEF, ucs: 0x316D, deprecated: false },
    KeysymEntry { keysym: 0x0EF0, ucs: 0x3171, deprecated: false },
    KeysymEntry { keysym: 0x0EF1, ucs: 0x3178, deprecated: false },
    KeysymEntry { keysym: 0x0EF2, ucs: 0x317F, deprecated: false },
    KeysymEntry { keysym: 0x0EF3, ucs: 0x3181, deprecated: false },
    KeysymEntry { keysym: 0x0EF4, ucs: 0x3184, deprecated: false },
    KeysymEntry { keysym: 0x0EF5, ucs: 0x3186, deprecated: false },
    KeysymEntry { keysym: 0x0EF6, ucs: 0x318D, deprecated: false },
    KeysymEntry { keysym: 0x0EF7, ucs: 0x318E, deprecated: false },
    KeysymEntry { keysym: 0x0EF8, ucs: 0x11EB, deprecated: false },
    KeysymEntry { keysym: 0x0EF9, ucs: 0x11F0, deprecated: false },
    KeysymEntry { keysym: 0x0EFA, ucs: 0x11F9, deprecated: false },
    KeysymEntry { keysym: 0x0EFF, ucs: 0x20A9, deprecated: true },
    KeysymEntry { keysym: 0x13BC, ucs: 0x0152, deprecated: false },
    KeysymEntry { keysym: 0x13BD, ucs: 0x0153, deprecated: false },
    KeysymEntry { keysym: 0x13BE, ucs: 0x0178, deprecated: false },
    KeysymEntry { keysym: 0x20AC, ucs: 0x20AC, deprecated: false },
];

/// Binary search the keysym lookup table
fn bin_search_keysym(keysym: u32) -> Option<u32> {
    let keysym16 = if keysym <= 0xFFFF {
        keysym as u16
    } else {
        return None;
    };

    let idx = KEYSYM_TABLE
        .binary_search_by_key(&keysym16, |entry| entry.keysym)
        .ok()?;
    Some(KEYSYM_TABLE[idx].ucs as u32)
}

/// Convert an XKB keysym to a Unicode character
///
/// Returns `None` if the keysym cannot be converted to Unicode.
pub fn keysym_to_char(keysym: u32) -> Option<char> {
    let codepoint = keysym_to_codepoint(keysym)?;
    char::from_u32(codepoint)
}

/// Convert an XKB keysym to a Unicode codepoint (U+XXXX)
///
/// Returns `None` if the keysym cannot be converted.
fn keysym_to_codepoint(keysym: u32) -> Option<u32> {
    // ASCII printable and Latin-1
    if (0x20..=0x7E).contains(&keysym) || (0xA0..=0xFF).contains(&keysym) {
        return Some(keysym);
    }

    // Keypad space maps to regular space
    if keysym == XKB_KEY_KP_SPACE {
        return Some(XKB_KEY_SPACE & 0x7F);
    }

    // Special keys that map to ASCII control characters
    if (XKB_KEY_BACKSPACE..=XKB_KEY_CLEAR).contains(&keysym)
        || (XKB_KEY_KP_MULTIPLY..=XKB_KEY_KP_9).contains(&keysym)
        || keysym == XKB_KEY_RETURN
        || keysym == XKB_KEY_ESCAPE
        || keysym == XKB_KEY_DELETE
        || keysym == XKB_KEY_KP_TAB
        || keysym == XKB_KEY_KP_ENTER
        || keysym == XKB_KEY_KP_EQUAL
    {
        return Some(keysym & 0x7F);
    }

    // Reject surrogate pairs
    if (XKB_KEYSYM_UNICODE_SURROGATE_MIN..=XKB_KEYSYM_UNICODE_SURROGATE_MAX).contains(&keysym) {
        return None;
    }

    // Unicode keysyms
    if (XKB_KEYSYM_UNICODE_OFFSET..=XKB_KEYSYM_UNICODE_MAX).contains(&keysym) {
        return Some(keysym - XKB_KEYSYM_UNICODE_OFFSET);
    }

    // XF86 numeric keys
    if (XKB_KEY_XF86_NUMERIC_0..=XKB_KEY_XF86_NUMERIC_9).contains(&keysym) {
        return Some(keysym - XKB_KEY_XF86_NUMERIC_0 + 0x30); // '0' to '9'
    }

    // Special XF86 keys
    match keysym {
        XKB_KEY_XF86_NUMERIC_STAR => return Some(0x2A), // '*'
        XKB_KEY_XF86_NUMERIC_POUND => return Some(0x23), // '#'
        _ => {}
    }

    // Look up in the static table
    bin_search_keysym(keysym)
}

/// Convert a Unicode character to an XKB keysym
///
/// Returns `None` if the character has no corresponding keysym.
pub fn char_to_keysym(ch: char) -> Option<u32> {
    codepoint_to_keysym(ch as u32)
}

/// Convert a Unicode codepoint to an XKB keysym
fn codepoint_to_keysym(ucs: u32) -> Option<u32> {
    // ASCII printable and Latin-1
    if (0x20..=0x7E).contains(&ucs) || (0xA0..=0xFF).contains(&ucs) {
        return Some(ucs);
    }

    // Special control characters
    if (0x08..=0x0B).contains(&ucs) // BackSpace to Clear
        || ucs == 0x0D // Return
        || ucs == 0x1B
    // Escape
    {
        return Some(ucs | 0xFF00);
    }

    // Delete
    if ucs == 0x7F {
        return Some(XKB_KEY_DELETE);
    }

    // Invalid codepoints
    if ucs == 0 || (0xD800..=0xDFFF).contains(&ucs) || ucs > 0x10FFFF {
        return None;
    }

    // Search the table for non-deprecated entries
    for entry in KEYSYM_TABLE {
        if entry.ucs as u32 == ucs && !entry.deprecated {
            return Some(entry.keysym as u32);
        }
    }

    // Fallback: encode as Unicode keysym
    Some(ucs | XKB_KEYSYM_UNICODE_OFFSET)
}

// ====================================================================================
// FFI compatibility layer for internal C code (state.rs, scanner.rs, etc.)
// These wrappers maintain the old C API while using native Rust implementation
// ====================================================================================

/// FFI wrapper: Convert keysym to UTF-32 codepoint (C-compatible)
///
/// Returns 0 if conversion fails (matches old C behavior)
pub fn xkb_keysym_to_utf32(keysym: u32) -> u32 {
    keysym_to_codepoint(keysym).unwrap_or(0)
}

/// FFI wrapper: Convert keysym to UTF-8 string (C-compatible)
///
/// Writes UTF-8 bytes to `buffer` (must have space for at least 5 bytes).
/// Returns number of bytes written (including null terminator), or:
/// - 0 if conversion fails
/// - -1 if buffer too small
pub unsafe fn xkb_keysym_to_utf8(keysym: u32, buffer: *mut i8, size: usize) -> i32 {
    const MAX_UTF8_SIZE: usize = 5;

    if size < MAX_UTF8_SIZE {
        return -1;
    }

    if buffer.is_null() {
        return -1;
    }

    let Some(ch) = keysym_to_char(keysym) else {
        return 0;
    };

    // Encode to UTF-8
    let mut tmp = [0u8; 4];
    let utf8_bytes = ch.encode_utf8(&mut tmp).as_bytes();

    // Copy to buffer
    unsafe {
        std::ptr::copy_nonoverlapping(utf8_bytes.as_ptr(), buffer as *mut u8, utf8_bytes.len());
        // Null terminate
        *buffer.add(utf8_bytes.len()) = 0;
    }

    // Return length + 1 for null terminator (matches old C behavior)
    (utf8_bytes.len() + 1) as i32
}

/// FFI wrapper: Convert UTF-32 codepoint to keysym (C-compatible)
///
/// Returns 0 (XKB_KEY_NO_SYMBOL) if conversion fails
pub fn xkb_utf32_to_keysym(ucs: u32) -> u32 {
    codepoint_to_keysym(ucs).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_conversion() {
        assert_eq!(keysym_to_char(0x41), Some('A'));
        assert_eq!(keysym_to_char(0x20), Some(' '));
        assert_eq!(char_to_keysym('A'), Some(0x41));
    }

    #[test]
    fn test_special_keys() {
        assert_eq!(keysym_to_char(XKB_KEY_RETURN), Some('\r'));
        assert_eq!(keysym_to_char(XKB_KEY_ESCAPE), Some('\x1B'));
    }

    #[test]
    fn test_unicode_keysyms() {
        // U+1F600 GRINNING FACE
        let keysym = 0x1000000 + 0x1F600;
        assert_eq!(keysym_to_char(keysym), Some('😀'));
    }

    #[test]
    fn test_invalid_keysyms() {
        assert_eq!(keysym_to_char(0), None);
        assert_eq!(keysym_to_char(XKB_KEYSYM_UNICODE_SURROGATE_MIN), None);
    }

    #[test]
    fn test_ffi_compat() {
        // Test xkb_keysym_to_utf32
        unsafe {
            assert_eq!(xkb_keysym_to_utf32(0x41), 0x41); // 'A'
            assert_eq!(xkb_keysym_to_utf32(0), 0); // Invalid
        }

        // Test xkb_keysym_to_utf8
        unsafe {
            let mut buf = [0i8; 10];
            let len = xkb_keysym_to_utf8(0x41, buf.as_mut_ptr(), 10);
            assert_eq!(len, 2); // 'A' + null = 2 bytes
            assert_eq!(buf[0], b'A' as i8);
            assert_eq!(buf[1], 0);
        }
    }
}
