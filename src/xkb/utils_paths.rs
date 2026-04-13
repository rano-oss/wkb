//! Path utilities - fully rustified

/// Path separator character (Unix-style)
pub const PATH_SEPARATOR: char = '/';

/// Check if a path is absolute (starts with /)
/// Safe Rust version using &str
pub fn is_absolute_path_safe(path: &str) -> bool {
    path.starts_with(PATH_SEPARATOR)
}

/// Raw pointer wrapper for callers that pass *const i8
pub unsafe fn is_absolute_path(path: *const i8) -> bool {
    if path.is_null() {
        return false;
    }
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(path);
        if let Ok(s) = c_str.to_str() {
            is_absolute_path_safe(s)
        } else {
            // If path is not valid UTF-8, fall back to checking first byte
            *path == b'/' as i8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_absolute_path_safe() {
        assert!(is_absolute_path_safe("/usr/share/xkb"));
        assert!(is_absolute_path_safe("/"));
        assert!(is_absolute_path_safe("/home/user"));
        assert!(!is_absolute_path_safe("relative/path"));
        assert!(!is_absolute_path_safe("file.txt"));
        assert!(!is_absolute_path_safe(""));
    }

    #[test]
    fn test_path_separator() {
        assert_eq!(PATH_SEPARATOR, '/');
    }

    #[test]
    fn test_is_absolute_path_ffi() {
        unsafe {
            let abs_path = std::ffi::CString::new("/usr/share").unwrap();
            assert!(is_absolute_path(abs_path.as_ptr()));

            let rel_path = std::ffi::CString::new("relative/path").unwrap();
            assert!(!is_absolute_path(rel_path.as_ptr()));

            // Test null pointer
            assert!(!is_absolute_path(std::ptr::null()));
        }
    }
}
use crate::xkb::shared_types::*;
