//! Native Rust atom table - string interning system
//!
//! Fully safe internals. Functions are normal Rust — no FFI wrappers.

use std::ffi::CString;

/// Atom table - string interning system
///
/// Fully safe Rust internals. No C allocations.
#[derive(Debug, Clone)]
pub struct atom_table {
    /// Hash index for O(1) lookups (open addressing, linear probing)
    index: Vec<u32>,
    /// Interned strings. Index 0 is None (XKB_ATOM_NONE).
    /// Stored as CString so `.as_ptr()` returns a valid `*const i8`.
    strings: Vec<Option<CString>>,
}

/// FNV-1a hash function for byte slices
#[inline]
fn hash_buf(bytes: &[u8]) -> u32 {
    let len = bytes.len();
    let mut hash: u32 = 2166136261;
    for i in 0..len.div_ceil(2) {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x1000193);
        hash ^= bytes[len - 1 - i] as u32;
        hash = hash.wrapping_mul(0x1000193);
    }
    hash
}

/// Create new atom table
pub fn atom_table_new() -> atom_table {
    atom_table {
        index: vec![0; 4],
        strings: vec![None], // index 0 = XKB_ATOM_NONE
    }
}

/// Get number of atoms in table
pub fn atom_table_size(table: &atom_table) -> u32 {
    table.strings.len() as u32
}

/// Get text for an atom (returns pointer to interned string)
pub fn atom_text(table: &atom_table, atom: u32) -> *const i8 {
    assert!(
        (atom as usize) < table.strings.len(),
        "atom index out of bounds"
    );
    match &table.strings[atom as usize] {
        Some(cstr) => cstr.as_ptr(),
        None => std::ptr::null(),
    }
}

/// Get text for an atom as a byte slice (safe alternative to atom_text)
///
/// Returns `b""` for `XKB_ATOM_NONE` or invalid atoms.
/// The returned slice is valid as long as the atom table is alive.
pub fn atom_text_bytes(table: &atom_table, atom: u32) -> &[u8] {
    if (atom as usize) >= table.strings.len() {
        return b"";
    }
    match &table.strings[atom as usize] {
        Some(cstr) => cstr.as_bytes(),
        None => b"",
    }
}

/// Intern a string or look up existing atom
///
/// If `add` is true, adds string to table if not found.
/// Returns atom ID or XKB_ATOM_NONE if not found and add=false.
pub unsafe fn atom_intern(table: &mut atom_table, string: *const i8, len: usize, add: bool) -> u32 {
    unsafe {
        // Convert input C string to a byte slice (not including null terminator)
        let input_bytes = std::slice::from_raw_parts(string as *const u8, len);

        let t = table;

        // Resize hash table if load factor > 0.8
        if t.strings.len() > t.index.len() * 4 / 5 {
            let new_size = t.index.len() * 2;
            t.index = vec![0; new_size];

            // Rehash all strings (skip index 0)
            for j in 1..t.strings.len() {
                if let Some(ref s) = t.strings[j] {
                    let s_bytes = s.as_bytes(); // does not include null terminator
                    let hash = hash_buf(s_bytes);

                    for i in 0..t.index.len() {
                        let index_pos = ((hash as usize) + i) & (t.index.len() - 1);
                        if index_pos == 0 {
                            continue;
                        }
                        if t.index[index_pos] == XKB_ATOM_NONE {
                            t.index[index_pos] = j as u32;
                            break;
                        }
                    }
                }
            }
        }

        // Look up or insert string
        let hash = hash_buf(input_bytes);

        for i in 0..t.index.len() {
            let index_pos = ((hash as usize) + i) & (t.index.len() - 1);
            if index_pos == 0 {
                continue;
            }

            let existing_atom = t.index[index_pos];

            // Empty slot - not found
            if existing_atom == XKB_ATOM_NONE {
                if add {
                    let new_atom = t.strings.len() as u32;

                    // Create CString from bytes
                    let new_string =
                        CString::new(input_bytes.to_vec()).expect("string contains interior null");
                    t.strings.push(Some(new_string));

                    // Update hash table
                    t.index[index_pos] = new_atom;

                    return new_atom;
                } else {
                    return XKB_ATOM_NONE;
                }
            }

            // Check if existing string matches
            if let Some(ref existing_cstr) = t.strings[existing_atom as usize] {
                let existing_bytes = existing_cstr.as_bytes();
                if existing_bytes == input_bytes {
                    return existing_atom;
                }
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
            let mut table = atom_table_new();

            // Initially should have 1 atom (NONE at index 0)
            assert_eq!(atom_table_size(&table), 1);

            // Intern a string
            let s1 = b"hello\0".as_ptr() as *const i8;
            let atom1 = atom_intern(&mut table, s1, 5, true);
            assert_ne!(atom1, XKB_ATOM_NONE as u32);
            assert_eq!(atom_table_size(&table), 2);

            // Look up same string again
            let atom2 = atom_intern(&mut table, s1, 5, false);
            assert_eq!(atom1, atom2);
            assert_eq!(atom_table_size(&table), 2); // No new atom

            // Intern different string
            let s2 = b"world\0".as_ptr() as *const i8;
            let atom3 = atom_intern(&mut table, s2, 5, true);
            assert_ne!(atom3, atom1);
            assert_eq!(atom_table_size(&table), 3);

            // Get text back
            let text1 = atom_text(&table, atom1);
            assert_eq!(std::ffi::CStr::from_ptr(text1).to_str().unwrap(), "hello");
        }
    }

    #[test]
    fn test_atom_not_found() {
        unsafe {
            let mut table = atom_table_new();

            let s = b"test\0".as_ptr() as *const i8;
            let atom = atom_intern(&mut table, s, 4, false);
            assert_eq!(atom, XKB_ATOM_NONE as u32);
        }
    }
}
use crate::xkb::shared_types::*;
