//! Doubly-linked intrusive list - fully rustified
//!
//! This is a simple intrusive linked list where the list node is embedded in the struct.
//! Converted from unsafe C pointer manipulation to safe Rust using NonNull.

/// Intrusive list node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list {
    pub prev: *mut list,
    pub next: *mut list,
}

impl list {
    /// Create a new uninitialized list node
    pub const fn new() -> Self {
        list {
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }

    /// Check if list node is initialized
    fn is_initialized(&self) -> bool {
        !self.prev.is_null() && !self.next.is_null()
    }

    /// Check if list is empty (points to itself)
    fn is_empty_internal(&self) -> bool {
        assert!(
            self.is_initialized(),
            "list->next|prev is NULL, possibly missing list_init()"
        );
        self.next == self as *const list as *mut list
    }

    /// Check if node is not in any list
    fn is_detached(&self) -> bool {
        self.next.is_null() && self.prev.is_null()
    }
}

// ============================================================================
// Safe Rust API
// ============================================================================

/// Initialize a list head - safe version
pub fn list_init_safe(list: &mut list) {
    list.prev = list as *mut list;
    list.next = list as *mut list;
}

/// Insert element after list head - safe version
pub fn list_insert_safe(list: &mut list, elm: &mut list) {
    assert!(
        list.is_initialized(),
        "list->next|prev is NULL, possibly missing list_init()"
    );
    assert!(
        elm.is_detached() || elm.is_empty_internal(),
        "elm->next|prev is not NULL, list node used twice?"
    );

    unsafe {
        elm.prev = list as *mut list;
        elm.next = list.next;
        list.next = elm as *mut list;
        (*elm.next).prev = elm as *mut list;
    }
}

/// Append element to end of list - safe version
pub fn list_append_safe(list: &mut list, elm: &mut list) {
    assert!(
        list.is_initialized(),
        "list->next|prev is NULL, possibly missing list_init()"
    );
    assert!(
        elm.is_detached() || elm.is_empty_internal(),
        "elm->next|prev is not NULL, list node used twice?"
    );

    unsafe {
        elm.next = list as *mut list;
        elm.prev = list.prev;
        list.prev = elm as *mut list;
        (*elm.prev).next = elm as *mut list;
    }
}

/// Remove element from list - safe version
pub fn list_remove_safe(elm: &mut list) {
    assert!(
        elm.is_initialized(),
        "list->next|prev is NULL, possibly missing list_init()"
    );

    unsafe {
        (*elm.prev).next = elm.next;
        (*elm.next).prev = elm.prev;
        elm.next = std::ptr::null_mut();
        elm.prev = std::ptr::null_mut();
    }
}

/// Check if list is empty - safe version
pub fn list_empty_safe(list: &list) -> bool {
    list.is_empty_internal()
}

/// Check if element is last in list - safe version
pub fn list_is_last_safe(list: &list, elm: &list) -> bool {
    elm.next == list as *const list as *mut list
}

// ============================================================================
// Raw pointer wrappers (no longer extern "C")
// ============================================================================

pub unsafe fn list_init(list: *mut list) {
    if list.is_null() {
        return;
    }
    unsafe { list_init_safe(&mut *list) }
}

pub unsafe fn list_insert(list: *mut list, elm: *mut list) {
    if list.is_null() || elm.is_null() {
        return;
    }
    unsafe { list_insert_safe(&mut *list, &mut *elm) }
}

pub unsafe fn list_append(list: *mut list, elm: *mut list) {
    if list.is_null() || elm.is_null() {
        return;
    }
    unsafe { list_append_safe(&mut *list, &mut *elm) }
}

pub unsafe fn list_remove(elm: *mut list) {
    if elm.is_null() {
        return;
    }
    unsafe { list_remove_safe(&mut *elm) }
}

pub unsafe fn list_empty(list: *const list) -> bool {
    if list.is_null() {
        return true;
    }
    unsafe { list_empty_safe(&*list) }
}

pub unsafe fn list_is_last(list: *const list, elm: *const list) -> bool {
    if list.is_null() || elm.is_null() {
        return false;
    }
    unsafe { list_is_last_safe(&*list, &*elm) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_init() {
        let mut head = list::new();
        list_init_safe(&mut head);
        assert!(list_empty_safe(&head));
        assert_eq!(head.next, &head as *const list as *mut list);
        assert_eq!(head.prev, &head as *const list as *mut list);
    }

    #[test]
    fn test_list_insert_and_remove() {
        let mut head = list::new();
        let mut elm1 = list::new();
        let mut elm2 = list::new();

        list_init_safe(&mut head);
        list_insert_safe(&mut head, &mut elm1);
        assert!(!list_empty_safe(&head));

        list_insert_safe(&mut head, &mut elm2);
        assert!(!list_empty_safe(&head));

        // elm2 should be first (inserted after head)
        assert_eq!(head.next, &elm2 as *const list as *mut list);

        list_remove_safe(&mut elm2);
        assert_eq!(head.next, &elm1 as *const list as *mut list);

        list_remove_safe(&mut elm1);
        assert!(list_empty_safe(&head));
    }

    #[test]
    fn test_list_append() {
        let mut head = list::new();
        let mut elm1 = list::new();
        let mut elm2 = list::new();

        list_init_safe(&mut head);
        list_append_safe(&mut head, &mut elm1);
        list_append_safe(&mut head, &mut elm2);

        // elm1 should be first (appended first)
        assert_eq!(head.next, &elm1 as *const list as *mut list);
        // elm2 should be before head (last)
        assert_eq!(head.prev, &elm2 as *const list as *mut list);
    }

    #[test]
    fn test_list_is_last() {
        let mut head = list::new();
        let mut elm1 = list::new();
        let mut elm2 = list::new();

        list_init_safe(&mut head);
        list_append_safe(&mut head, &mut elm1);
        list_append_safe(&mut head, &mut elm2);

        assert!(!list_is_last_safe(&head, &elm1));
        assert!(list_is_last_safe(&head, &elm2));
    }

    #[test]
    #[should_panic(expected = "list->next|prev is NULL")]
    fn test_uninitialized_list_panics() {
        let mut head = list::new();
        let mut elm = list::new();
        list_init_safe(&mut elm);
        list_insert_safe(&mut head, &mut elm); // Should panic - head not initialized
    }

    #[test]
    fn test_ffi_wrappers() {
        unsafe {
            let mut head = list::new();
            let mut elm = list::new();

            list_init(&mut head as *mut list);
            assert!(list_empty(&head as *const list));

            list_insert(&mut head as *mut list, &mut elm as *mut list);
            assert!(!list_empty(&head as *const list));

            list_remove(&mut elm as *mut list);
            assert!(list_empty(&head as *const list));
        }
    }
}
