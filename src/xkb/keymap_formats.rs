pub mod types_h {
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type u32 = __uint32_t;
    use super::types_h::__uint32_t;
}

pub mod xkbcommon_h {
    pub use crate::xkb::shared_types::*;
}
pub mod utils_numbers_h {
    pub static mut digits__: [::core::ffi::c_uchar; 256] = [
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0 as i32 as ::core::ffi::c_uchar,
        1 as i32 as ::core::ffi::c_uchar,
        2 as i32 as ::core::ffi::c_uchar,
        3 as i32 as ::core::ffi::c_uchar,
        4 as i32 as ::core::ffi::c_uchar,
        5 as i32 as ::core::ffi::c_uchar,
        6 as i32 as ::core::ffi::c_uchar,
        7 as i32 as ::core::ffi::c_uchar,
        8 as i32 as ::core::ffi::c_uchar,
        9 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        10 as i32 as ::core::ffi::c_uchar,
        11 as i32 as ::core::ffi::c_uchar,
        12 as i32 as ::core::ffi::c_uchar,
        13 as i32 as ::core::ffi::c_uchar,
        14 as i32 as ::core::ffi::c_uchar,
        15 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        10 as i32 as ::core::ffi::c_uchar,
        11 as i32 as ::core::ffi::c_uchar,
        12 as i32 as ::core::ffi::c_uchar,
        13 as i32 as ::core::ffi::c_uchar,
        14 as i32 as ::core::ffi::c_uchar,
        15 as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
        0xff as i32 as ::core::ffi::c_uchar,
    ];
    #[inline]
    pub unsafe fn parse_hex_to_uint32_t(
        mut s: *const i8,
        mut len: usize,
        mut out: *mut u32,
    ) -> i32 {
        unsafe {
            let mut result: u32 = 0 as u32;
            let mut i: usize = 0 as usize;
            while i < len
                && (digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32)
                    < 16 as u32
                && result <= 4294967295 as u32 >> 4 as i32
            {
                result = result.wrapping_mul(16 as u32).wrapping_add(
                    digits__[*s.offset(i as isize) as ::core::ffi::c_uchar as usize] as u32,
                );
                i = i.wrapping_add(1);
            }
            *out = result as u32;
            return if i >= len || !is_xdigit(*s.offset(i as isize)) {
                i as i32
            } else {
                -1 as i32
            };
        }
    }

    use super::stdint_uintn_h::u32;
    use super::utils_h::is_xdigit;
}
pub mod stdint_h {
    pub const SIZE_MAX: u64 = 18446744073709551615 as u64;
}
pub mod string_h {
    extern "C" {
        pub fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    }
}
pub mod stdbool_h {
    pub const true_0: i32 = 1 as i32;
    pub const false_0: i32 = 0 as i32;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod utils_h {
    #[inline]
    pub unsafe fn is_xdigit(mut ch: i8) -> bool {
        unsafe {
            return ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32
                || ch as i32 >= 'a' as i32 && ch as i32 <= 'f' as i32
                || ch as i32 >= 'A' as i32 && ch as i32 <= 'F' as i32;
        }
    }
}
pub use self::__stddef_null_h::NULL;

pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::SIZE_MAX;
pub use self::stdint_uintn_h::u32;
use self::string_h::strcmp;
pub use self::types_h::__uint32_t;
pub use self::utils_h::is_xdigit;
pub use self::utils_numbers_h::{digits__, parse_hex_to_uint32_t};
pub use self::xkbcommon_h::{
    xkb_keymap_format, XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2,
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_label {
    pub label: *const i8,
    pub format: xkb_keymap_format,
}
static mut keymap_formats: [xkb_keymap_format; 2] =
    [XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2];
static mut keymap_formats_labels: [format_label; 4] = [
    format_label {
        label: b"xkb_v1\0".as_ptr() as *const i8,
        format: XKB_KEYMAP_FORMAT_TEXT_V1,
    },
    format_label {
        label: b"v1\0".as_ptr() as *const i8,
        format: XKB_KEYMAP_FORMAT_TEXT_V1,
    },
    format_label {
        label: b"xkb_v2\0".as_ptr() as *const i8,
        format: XKB_KEYMAP_FORMAT_TEXT_V2,
    },
    format_label {
        label: b"v2\0".as_ptr() as *const i8,
        format: XKB_KEYMAP_FORMAT_TEXT_V2,
    },
];

pub unsafe fn xkb_keymap_supported_formats(mut formats: *mut *const xkb_keymap_format) -> usize {
    unsafe {
        *formats = &raw const keymap_formats as *const xkb_keymap_format;
        return (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize);
    }
}

pub unsafe fn xkb_keymap_is_supported_format(mut format: xkb_keymap_format) -> bool {
    unsafe {
        if (format as u32) < keymap_formats[0 as i32 as usize] as u32 {
            return false_0 != 0;
        }
        let mut k: usize = 0 as usize;
        while k
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            if keymap_formats[k as usize] as u32 == format as u32 {
                return true_0 != 0;
            }
            if keymap_formats[k as usize] as u32 > format as u32 {
                return false_0 != 0;
            }
            k = k.wrapping_add(1);
        }
        return false_0 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_parse_format(mut raw: *const i8) -> xkb_keymap_format {
    unsafe {
        if raw.is_null() {
            return 0 as xkb_keymap_format;
        }
        let mut format: u32 = 0 as u32;
        if parse_hex_to_uint32_t(raw, SIZE_MAX as usize, &raw mut format) > 0 as i32 {
            return (if xkb_keymap_is_supported_format(format as xkb_keymap_format) as i32 != 0 {
                format
            } else {
                0 as u32
            }) as xkb_keymap_format;
        } else {
            let mut k: usize = 0 as usize;
            while k
                < (::core::mem::size_of::<[format_label; 4]>() as usize)
                    .wrapping_div(::core::mem::size_of::<format_label>() as usize)
            {
                if strcmp(raw, keymap_formats_labels[k as usize].label) == 0 as i32 {
                    return keymap_formats_labels[k as usize].format;
                }
                k = k.wrapping_add(1);
            }
            return 0 as xkb_keymap_format;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_get_format_label(mut format: xkb_keymap_format) -> *const i8 {
    unsafe {
        if (format as u32) < keymap_formats_labels[0 as i32 as usize].format as u32 {
            return ::core::ptr::null::<i8>();
        }
        let mut k: usize = 0 as usize;
        while k
            < (::core::mem::size_of::<[format_label; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<format_label>() as usize)
        {
            if keymap_formats_labels[k as usize].format as u32 == format as u32 {
                return keymap_formats_labels[k as usize].label;
            }
            if keymap_formats_labels[k as usize].format as u32 > format as u32 {
                return ::core::ptr::null::<i8>();
            }
            k = k.wrapping_add(1);
        }
        return ::core::ptr::null::<i8>();
    }
}
