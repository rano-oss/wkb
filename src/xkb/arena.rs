use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

/// An owning pointer backed by arena memory.
/// Runs `drop_in_place` on drop but does NOT deallocate.
#[repr(transparent)]
pub(crate) struct ArenaBox<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Deref for ArenaBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for ArenaBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> Drop for ArenaBox<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.ptr.as_ptr()) }
    }
}

impl<T> ArenaBox<T> {
    pub(crate) fn into_inner(self) -> T {
        unsafe {
            let val = ptr::read(self.ptr.as_ptr());
            std::mem::forget(self);
            val
        }
    }
}

thread_local! {
    static BUMP: Cell<*mut bumpalo::Bump> = const { Cell::new(std::ptr::null_mut()) };
}

pub(crate) fn alloc_arena<T>(value: T) -> ArenaBox<T> {
    with_arena(|bump| {
        let ptr = bump.alloc(value) as *mut T;
        ArenaBox {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    })
}

pub(crate) fn with_arena<R>(f: impl FnOnce(&mut bumpalo::Bump) -> R) -> R {
    BUMP.with(|cell| {
        let ptr = cell.get();
        assert!(!ptr.is_null(), "arena not initialized");
        let bump = unsafe { &mut *ptr };
        f(bump)
    })
}

pub(crate) fn set_arena(bump: &mut bumpalo::Bump) {
    BUMP.with(|cell| cell.set(bump));
}

pub(crate) fn clear_arena() {
    BUMP.with(|cell| cell.set(std::ptr::null_mut()));
}
