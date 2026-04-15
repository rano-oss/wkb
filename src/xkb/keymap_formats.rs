use crate::xkb::shared_types::*;
use crate::xkb::utils::cstr_as_bytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_label {
    pub label: *const i8,
    pub format: u32,
}
static mut keymap_formats: [u32; 2] = [XKB_KEYMAP_FORMAT_TEXT_V1, XKB_KEYMAP_FORMAT_TEXT_V2];
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

pub unsafe fn xkb_keymap_is_supported_format(mut format: u32) -> bool {
    unsafe {
        if (format as u32) < keymap_formats[0 as usize] as u32 {
            return false;
        }
        let mut k: usize = 0 as usize;
        while k < (std::mem::size_of::<[u32; 2]>()).wrapping_div(std::mem::size_of::<u32>()) {
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
pub unsafe fn xkb_keymap_parse_format(mut raw: *const i8) -> u32 {
    unsafe {
        if raw.is_null() {
            return 0 as u32;
        }
        let mut format: u32 = 0 as u32;
        let raw_bytes = std::ffi::CStr::from_ptr(raw).to_bytes();
        let (val_parsed, count) = crate::xkb::utils::parse_hex_u32(raw_bytes);
        format = val_parsed;
        if count > 0 as i32 {
            return (if xkb_keymap_is_supported_format(format as u32) as i32 != 0 {
                format
            } else {
                0 as u32
            }) as u32;
        } else {
            let mut k: usize = 0 as usize;
            while k
                < (std::mem::size_of::<[format_label; 4]>())
                    .wrapping_div(std::mem::size_of::<format_label>())
            {
                if cstr_as_bytes(raw) == cstr_as_bytes(keymap_formats_labels[k as usize].label) {
                    return keymap_formats_labels[k as usize].format;
                }
                k = k.wrapping_add(1);
            }
            return 0 as u32;
        };
    }
}
pub unsafe fn xkb_keymap_get_format_label(mut format: u32) -> *const i8 {
    unsafe {
        if (format as u32) < keymap_formats_labels[0 as usize].format as u32 {
            return std::ptr::null();
        }
        let mut k: usize = 0 as usize;
        while k
            < (std::mem::size_of::<[format_label; 4]>())
                .wrapping_div(std::mem::size_of::<format_label>())
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
