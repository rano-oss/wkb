//! Text utility functions - converted from darray to Vec
//!
//! These functions process strings with prefix stripping and comment removal.

pub mod utils_text_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct text_line {
        pub start: *const ::core::ffi::c_char,
        pub length: usize,
    }
}
pub mod stdlib_h {
    extern "C" {
        pub fn rand() -> ::core::ffi::c_int;
    }
}
pub mod string_h {
    extern "C" {
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
    }
}

use self::stdlib_h::rand;
use self::string_h::{strchr, strlen, strstr};
pub use self::utils_text_h::text_line;

/// Strip lines that end with prefix (typically line continuation markers)
///
/// Returns a newly allocated C string (caller must free)
#[no_mangle]
pub unsafe extern "C" fn strip_lines(
    input: *const ::core::ffi::c_char,
    input_length: usize,
    prefix: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut buf = Vec::<u8>::new();
        let prefix_len = strlen(prefix);
        let start_mut = input as *mut i8;
        let mut start = start_mut;
        let end = start_mut.add(input_length);
        let mut next = strstr(input, prefix);

        while start < end && !next.is_null() {
            let mut count = next.offset_from(start as *mut i8) as usize;
            next = (start as *mut i8).add(count).add(prefix_len);

            // Trim trailing whitespace before prefix
            let mut i = count;
            while i > 0 {
                let ch = *start.add(i - 1) as u8;
                if ch != b' ' && ch != b'\t' {
                    break;
                }
                i -= 1;
            }

            let mut dropped = false;
            if i == 0 || *start.add(i - 1) == b'\n' as i8 {
                count = i;
                dropped = true;
            }

            // Append data to buffer
            let slice = std::slice::from_raw_parts(start as *const u8, count);
            buf.extend_from_slice(slice);

            if next >= end {
                break;
            }

            // Find next line
            start = strchr(next, b'\n' as i32);
            if start.is_null() {
                break;
            }

            if dropped {
                start = start.add(1);
            }
            next = strstr(start, prefix);
        }

        // Append remaining data
        if start < end {
            let count = end.offset_from(start) as usize;
            let slice = std::slice::from_raw_parts(start as *const u8, count);
            buf.extend_from_slice(slice);
        }

        // Null terminate and return
        buf.push(0);
        Box::into_raw(buf.into_boxed_slice()) as *mut ::core::ffi::c_char
    }
}

/// Remove comments from input (lines starting with prefix)
///
/// Returns a newly allocated C string (caller must free)
#[no_mangle]
pub unsafe extern "C" fn uncomment(
    input: *const ::core::ffi::c_char,
    input_length: usize,
    prefix: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut buf = Vec::<u8>::new();
        let prefix_len = strlen(prefix);
        let mut start = input;
        let end = input.add(input_length);
        let mut next = strstr(start, prefix);

        while start < end && !next.is_null() {
            let count = next.offset_from(start as *mut i8) as usize;

            // Append data before comment
            let slice = std::slice::from_raw_parts(start as *const u8, count);
            buf.extend_from_slice(slice);

            // Skip to end of line
            next = strchr(next.add(prefix_len), b'\n' as i32);
            if next.is_null() {
                break;
            }

            start = next.add(1);
            next = strstr(start, prefix);
        }

        // Append remaining data
        if start < end {
            let count = end.offset_from(start) as usize;
            let slice = std::slice::from_raw_parts(start as *const u8, count);
            buf.extend_from_slice(slice);
        }

        // Null terminate and return
        buf.push(0);
        Box::into_raw(buf.into_boxed_slice()) as *mut ::core::ffi::c_char
    }
}

/// Split input into lines
///
/// Returns array of text_line structs with start/length for each line.
/// Result array is null-terminated. Caller must free returned array.
#[no_mangle]
pub unsafe extern "C" fn split_lines(
    input: *const ::core::ffi::c_char,
    input_length: usize,
) -> *mut text_line {
    unsafe {
        let mut lines = Vec::<text_line>::new();
        let start_mut = input as *mut i8;
        let mut start = start_mut;
        let end = start_mut.add(input_length);

        while start < end {
            let next = strchr(start, b'\n' as i32);
            let (line_end, next_start) = if next.is_null() || next >= end {
                (end, end)
            } else {
                (next, next.add(1))
            };

            let length = line_end.offset_from(start) as usize;
            lines.push(text_line {
                start: start as *const i8,
                length,
            });
            start = next_start;
        }

        // Null-terminate array
        lines.push(text_line {
            start: std::ptr::null(),
            length: 0,
        });

        Box::into_raw(lines.into_boxed_slice()) as *mut text_line
    }
}

/// Randomize text (for testing) - shuffles characters
///
/// Returns a newly allocated C string (caller must free)
#[no_mangle]
pub unsafe extern "C" fn randomize_text(
    input: *const ::core::ffi::c_char,
    input_length: usize,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut buf = Vec::<u8>::with_capacity(input_length + 1);

        // Copy input to buffer
        let slice = std::slice::from_raw_parts(input as *const u8, input_length);
        buf.extend_from_slice(slice);

        // Fisher-Yates shuffle
        for i in (1..buf.len()).rev() {
            let j = (rand() as usize) % (i + 1);
            buf.swap(i, j);
        }

        // Null terminate and return
        buf.push(0);
        Box::into_raw(buf.into_boxed_slice()) as *mut ::core::ffi::c_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncomment() {
        unsafe {
            let input = b"foo\n#bar\nbaz\0";
            let prefix = b"#\0";
            let result = uncomment(
                input.as_ptr() as *const i8,
                input.len() - 1,
                prefix.as_ptr() as *const i8,
            );

            let output = std::ffi::CStr::from_ptr(result).to_str().unwrap();
            assert_eq!(output, "foo\nbaz");

            let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                result as *mut u8,
                output.len() + 1,
            ));
        }
    }
}
