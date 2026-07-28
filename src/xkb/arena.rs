use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

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
    static BUMP: RefCell<bumpalo::Bump> = RefCell::new(bumpalo::Bump::new());
}

pub(crate) fn alloc_arena<T>(value: T) -> ArenaBox<T> {
    BUMP.with(|b| {
        let guard = b.borrow_mut();
        let ptr = guard.alloc(value) as *mut T;
        ArenaBox {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    })
}

pub(crate) fn reset_arena() {
    BUMP.with(|b| {
        b.replace(bumpalo::Bump::new());
    });
}
