//! Native Rust atom table - string interning system
//!
//! Converted from C darray to Vec<T> for safety and simplicity.

pub mod types_h {
    pub type __uint8_t = u8;
    pub type __uint32_t = u32;
}
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type u32 = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
}
pub mod atom_h {
    pub type xkb_atom_t = darray_size_t;
    pub const XKB_ATOM_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::darray_h::darray_size_t;
}
pub mod string_h {

    extern "C" {
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: usize,
        ) -> ::core::ffi::c_int;
        pub fn strndup(
            __string: *const ::core::ffi::c_char,
            __n: usize,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
    }
}
pub mod stdlib_h {

    extern "C" {
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}

pub use self::atom_h::{xkb_atom_t, XKB_ATOM_NONE};
pub use self::darray_h::darray_size_t;
pub use self::stdint_uintn_h::{u32, uint8_t};
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::{strlen, strncmp, strndup};
pub use self::types_h::{__uint32_t, __uint8_t};

/// Atom table - string interning system
///
/// NOTE: Still uses raw C pointers for index hash table and string storage
/// to maintain FFI compatibility. Strings Vec replaces darray manual management.
#[repr(C)]
pub struct atom_table {
    pub index: *mut xkb_atom_t,
    pub index_size: usize,
    /// Vector of interned strings (raw C char pointers for FFI compat)
    /// Index 0 is always NULL (XKB_ATOM_NONE)
    pub strings: Vec<*mut ::core::ffi::c_char>,
}

/// FNV-1a hash function for strings
#[inline]
fn hash_buf(string: *const ::core::ffi::c_char, len: usize) -> u32 {
    unsafe {
        let mut hash: u32 = 2166136261;
        for i in 0..(len + 1) / 2 {
            hash ^= *string.offset(i as isize) as u8 as u32;
            hash = hash.wrapping_mul(0x1000193);
            hash ^= *string.offset((len - 1 - i) as isize) as u8 as u32;
            hash = hash.wrapping_mul(0x1000193);
        }
        hash
    }
}

/// Create new atom table
#[no_mangle]
pub unsafe extern "C" fn atom_table_new() -> *mut atom_table {
    unsafe {
        let mut table: *mut atom_table =
            calloc(1, ::core::mem::size_of::<atom_table>()) as *mut atom_table;
        if table.is_null() {
            return ::core::ptr::null_mut();
        }

        // Initialize strings vector with capacity 4, push NULL for index 0
        let mut strings = Vec::with_capacity(4);
        strings.push(::core::ptr::null_mut());

        // Move Vec into atom_table struct
        std::ptr::write(&mut (*table).strings, strings);

        // Initialize hash table index
        (*table).index_size = 4;
        (*table).index =
            calloc((*table).index_size, ::core::mem::size_of::<xkb_atom_t>()) as *mut xkb_atom_t;

        table
    }
}

/// Free atom table and all interned strings
#[no_mangle]
pub unsafe extern "C" fn atom_table_free(mut table: *mut atom_table) {
    unsafe {
        if table.is_null() {
            return;
        }

        // Free all interned strings (skip index 0 which is NULL)
        for string_ptr in (*table).strings.iter().skip(1) {
            if !string_ptr.is_null() {
                free(*string_ptr as *mut ::core::ffi::c_void);
            }
        }

        // Drop the Vec (automatically frees its buffer)
        std::ptr::drop_in_place(&mut (*table).strings);

        // Free hash table index
        free((*table).index as *mut ::core::ffi::c_void);

        // Free the table itself
        free(table as *mut ::core::ffi::c_void);
    }
}

/// Get number of atoms in table
#[no_mangle]
pub unsafe extern "C" fn atom_table_size(table: *mut atom_table) -> darray_size_t {
    unsafe { (*table).strings.len() as darray_size_t }
}

/// Get text for an atom (returns pointer to interned string)
#[no_mangle]
pub unsafe extern "C" fn atom_text(
    table: *mut atom_table,
    atom: xkb_atom_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let strings_ptr = (*table).strings.as_ptr();
        assert!(
            (atom as usize) < (*table).strings.len(),
            "atom index out of bounds"
        );
        *strings_ptr.add(atom as usize)
    }
}

/// Intern a string or look up existing atom
///
/// If `add` is true, adds string to table if not found.
/// Returns atom ID or XKB_ATOM_NONE if not found and add=false.
#[no_mangle]
pub unsafe extern "C" fn atom_intern(
    table: *mut atom_table,
    string: *const ::core::ffi::c_char,
    len: usize,
    add: bool,
) -> xkb_atom_t {
    unsafe {
        // Resize hash table if load factor > 0.8
        if (*table).strings.len() > (*table).index_size * 4 / 5 {
            (*table).index_size *= 2;
            (*table).index = realloc(
                (*table).index as *mut ::core::ffi::c_void,
                (*table).index_size * ::core::mem::size_of::<xkb_atom_t>(),
            ) as *mut xkb_atom_t;

            // Clear hash table
            std::ptr::write_bytes((*table).index, 0, (*table).index_size);

            // Rehash all strings (skip index 0)
            let strings_ptr = (*table).strings.as_ptr();
            for j in 1..(*table).strings.len() {
                let s = *strings_ptr.add(j);
                let hash = hash_buf(s, strlen(s));

                // Linear probing to find empty slot
                for i in 0..(*table).index_size {
                    let index_pos = ((hash as usize) + i) & ((*table).index_size - 1);
                    if index_pos == 0 {
                        continue; // Skip slot 0 (reserved for NONE)
                    }

                    let atom = *(*table).index.offset(index_pos as isize);
                    if atom == XKB_ATOM_NONE as xkb_atom_t {
                        *(*table).index.offset(index_pos as isize) = j as xkb_atom_t;
                        break;
                    }
                }
            }
        }

        // Look up or insert string
        let hash = hash_buf(string, len);

        for i in 0..(*table).index_size {
            let index_pos = ((hash as usize) + i) & ((*table).index_size - 1);
            if index_pos == 0 {
                continue; // Skip slot 0
            }

            let existing_atom = *(*table).index.offset(index_pos as isize);

            // Empty slot - not found
            if existing_atom == XKB_ATOM_NONE as xkb_atom_t {
                if add {
                    let new_atom = (*table).strings.len() as xkb_atom_t;

                    // Add string to Vec
                    let new_string = strndup(string, len);
                    (*table).strings.push(new_string);

                    // Update hash table
                    *(*table).index.offset(index_pos as isize) = new_atom;

                    return new_atom;
                } else {
                    return XKB_ATOM_NONE as xkb_atom_t;
                }
            }

            // Check if existing string matches
            let strings_ptr = (*table).strings.as_ptr();
            let existing_value = *strings_ptr.add(existing_atom as usize);
            if strncmp(existing_value, string, len) == 0
                && *existing_value.offset(len as isize) as ::core::ffi::c_int == '\0' as i32
            {
                return existing_atom;
            }
        }

        // Should never reach here - hash table is kept sparse enough
        panic!("couldn't find an empty slot during probing");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_table_basic() {
        unsafe {
            let table = atom_table_new();
            assert!(!table.is_null());

            // Initially should have 1 atom (NONE at index 0)
            assert_eq!(atom_table_size(table), 1);

            // Intern a string
            let s1 = b"hello\0".as_ptr() as *const i8;
            let atom1 = atom_intern(table, s1, 5, true);
            assert_ne!(atom1, XKB_ATOM_NONE as u32);
            assert_eq!(atom_table_size(table), 2);

            // Look up same string again
            let atom2 = atom_intern(table, s1, 5, false);
            assert_eq!(atom1, atom2);
            assert_eq!(atom_table_size(table), 2); // No new atom

            // Intern different string
            let s2 = b"world\0".as_ptr() as *const i8;
            let atom3 = atom_intern(table, s2, 5, true);
            assert_ne!(atom3, atom1);
            assert_eq!(atom_table_size(table), 3);

            // Get text back
            let text1 = atom_text(table, atom1);
            assert_eq!(std::ffi::CStr::from_ptr(text1).to_str().unwrap(), "hello");

            atom_table_free(table);
        }
    }

    #[test]
    fn test_atom_not_found() {
        unsafe {
            let table = atom_table_new();

            let s = b"test\0".as_ptr() as *const i8;
            let atom = atom_intern(table, s, 4, false);
            assert_eq!(atom, XKB_ATOM_NONE as u32);

            atom_table_free(table);
        }
    }
}
