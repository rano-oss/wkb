//! Native Rust atom table - string interning system
//!
//! Fully safe internals. Functions are normal Rust — no FFI wrappers.

/// Atom table - string interning system
///
/// Fully safe Rust internals. No C allocations.
#[derive(Debug, Clone)]
pub struct atom_table {
    /// Hash index for O(1) lookups (open addressing, linear probing)
    index: Vec<u32>,
    /// Interned strings. Index 0 is None (XKB_ATOM_NONE).
    strings: Vec<Option<String>>,
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

/// Get text for an atom as a byte slice.
///
/// Returns `b""` for `XKB_ATOM_NONE` or invalid atoms.
/// The returned slice is valid as long as the atom table is alive.
pub fn atom_text_bytes(table: &atom_table, atom: u32) -> &[u8] {
    if (atom as usize) >= table.strings.len() {
        return b"";
    }
    match &table.strings[atom as usize] {
        Some(s) => s.as_bytes(),
        None => b"",
    }
}

/// Intern a string or look up existing atom
///
/// If `add` is true, adds string to table if not found.
/// Returns atom ID or XKB_ATOM_NONE if not found and add=false.
pub fn atom_intern(table: &mut atom_table, input_bytes: &[u8], add: bool) -> u32 {
    let t = table;

    // Resize hash table if load factor > 0.8
    if t.strings.len() > t.index.len() * 4 / 5 {
        let new_size = t.index.len() * 2;
        t.index = vec![0; new_size];

        // Rehash all strings (skip index 0)
        for j in 1..t.strings.len() {
            if let Some(ref s) = t.strings[j] {
                let s_bytes = s.as_bytes();
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

                let new_string = String::from_utf8(input_bytes.to_vec())
                    .expect("atom string is not valid UTF-8");
                t.strings.push(Some(new_string));

                // Update hash table
                t.index[index_pos] = new_atom;

                return new_atom;
            } else {
                return XKB_ATOM_NONE;
            }
        }

        // Check if existing string matches
        if let Some(ref existing) = t.strings[existing_atom as usize] {
            if existing.as_bytes() == input_bytes {
                return existing_atom;
            }
        }
    }

    // Should never reach here - hash table is kept sparse enough
    panic!("couldn't find an empty slot during probing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_table_basic() {
        let mut table = atom_table_new();

        // Initially should have 1 atom (NONE at index 0)
        assert_eq!(atom_table_size(&table), 1);

        // Intern a string
        let atom1 = atom_intern(&mut table, b"hello", true);
        assert_ne!(atom1, XKB_ATOM_NONE as u32);
        assert_eq!(atom_table_size(&table), 2);

        // Look up same string again
        let atom2 = atom_intern(&mut table, b"hello", false);
        assert_eq!(atom1, atom2);
        assert_eq!(atom_table_size(&table), 2); // No new atom

        // Intern different string
        let atom3 = atom_intern(&mut table, b"world", true);
        assert_ne!(atom3, atom1);
        assert_eq!(atom_table_size(&table), 3);

        // Get text back
        assert_eq!(atom_text_bytes(&table, atom1), b"hello");
    }

    #[test]
    fn test_atom_not_found() {
        let mut table = atom_table_new();
        let atom = atom_intern(&mut table, b"test", false);
        assert_eq!(atom, XKB_ATOM_NONE as u32);
    }
}
use crate::xkb::shared_types::*;
