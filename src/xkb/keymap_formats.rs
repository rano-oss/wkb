
pub use crate::xkb::utils::is_xdigit;
pub use crate::xkb::utils::{digits__, parse_hex_to_uint32_t};
use crate::xkb::shared_types::*;
use crate::xkb::utils::cstr_cmp;
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
        if (format as u32) < keymap_formats[0 as usize] as u32 {
            return false;
        }
        let mut k: usize = 0 as usize;
        while k
            < (::core::mem::size_of::<[xkb_keymap_format; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<xkb_keymap_format>() as usize)
        {
            if keymap_formats[k as usize] as u32 == format as u32 {
                return true;
            }
            if keymap_formats[k as usize] as u32 > format as u32 {
                return false;
            }
            k = k.wrapping_add(1);
        }
        return false;
    }
}
pub unsafe fn xkb_keymap_parse_format(mut raw: *const i8) -> xkb_keymap_format {
    unsafe {
        if raw.is_null() {
            return 0 as xkb_keymap_format;
        }
        let mut format: u32 = 0 as u32;
        if parse_hex_to_uint32_t(raw, usize::MAX as usize, &raw mut format) > 0 as i32 {
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
                if cstr_cmp(raw, keymap_formats_labels[k as usize].label) == 0 as i32 {
                    return keymap_formats_labels[k as usize].format;
                }
                k = k.wrapping_add(1);
            }
            return 0 as xkb_keymap_format;
        };
    }
}
pub unsafe fn xkb_keymap_get_format_label(mut format: xkb_keymap_format) -> *const i8 {
    unsafe {
        if (format as u32) < keymap_formats_labels[0 as usize].format as u32 {
            return std::ptr::null();
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
                return std::ptr::null();
            }
            k = k.wrapping_add(1);
        }
        return std::ptr::null();
    }
}
