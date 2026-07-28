use std::alloc::Layout;
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

/// A simple bump arena allocator.
/// Allocates memory in `Vec<u8>` chunks; all memory freed on drop.
/// ArenaBox pointers into chunks remain valid until Arena is dropped.
pub(crate) struct Arena {
    chunks: Vec<Vec<u8>>,
    cursor: usize,
    current: usize,
}

impl Arena {
    pub(crate) fn new() -> Self {
        Arena {
            chunks: Vec::new(),
            cursor: 0,
            current: 0,
        }
    }

    pub(crate) fn alloc<T>(&mut self, value: T) -> ArenaBox<T> {
        let layout = Layout::new::<T>();
        let ptr = self.alloc_raw(layout) as *mut T;
        unsafe {
            ptr::write(ptr, value);
        }
        ArenaBox {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }

    fn alloc_raw(&mut self, layout: Layout) -> *mut u8 {
        loop {
            if self.current >= self.chunks.len() {
                let size = (layout.size().max(65536)).next_power_of_two();
                self.chunks.push(vec![0u8; size]);
                self.cursor = 0;
            }
            let chunk = &mut self.chunks[self.current];
            let start = chunk.as_mut_ptr() as usize;
            let aligned = (start + self.cursor + layout.align() - 1) & !(layout.align() - 1);
            let end = aligned + layout.size();
            if end <= start + chunk.len() {
                self.cursor = end - start;
                return aligned as *mut u8;
            }
            self.current += 1;
            self.cursor = 0;
        }
    }
}

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
    static ARENA_PTR: Cell<*mut Arena> = const { Cell::new(std::ptr::null_mut()) };
}

pub(crate) fn with_arena<R>(f: impl FnOnce(&mut Arena) -> R) -> R {
    ARENA_PTR.with(|cell| {
        let ptr = cell.get();
        assert!(!ptr.is_null(), "arena not initialized");
        let arena = unsafe { &mut *ptr };
        f(arena)
    })
}

pub(crate) fn set_arena(arena: &mut Arena) {
    ARENA_PTR.with(|cell| cell.set(arena));
}

pub(crate) fn clear_arena() {
    ARENA_PTR.with(|cell| cell.set(std::ptr::null_mut()));
}
