use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        // A dangling pointer is non-null and well aligned, which is all an empty buffer needs.
        // Nothing is allocated yet.
        MyVec { ptr: NonNull::dangling(), cap: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Grows the buffer: 0 -> 4, then doubling.
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { 2 * self.cap };
        let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
        let new_ptr = if self.cap == 0 {
            // SAFETY: T is not zero-sized and new_cap > 0, so the layout is not zero-sized.
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: `ptr` was allocated with `old_layout`, and the new size is not zero.
            unsafe { alloc::realloc(self.ptr.as_ptr().cast(), old_layout, new_layout.size()) }
        };
        self.ptr = match NonNull::new(new_ptr.cast::<T>()) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    pub fn push(&mut self, x: T) {
        if self.len == self.cap {
            self.grow();
        }
        // SAFETY: len < cap, so the slot is inside the buffer. It is uninitialized, so it must be
        // written without dropping its old "contents" (which `*slot = x` would do).
        unsafe { self.ptr.add(self.len).write(x) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: the slot was initialized, and it is no longer counted by `len`, so the value is
        // moved out exactly once.
        Some(unsafe { self.ptr.add(self.len).read() })
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.as_slice().get(i)
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: the first `len` slots are initialized, and `ptr` is non-null and aligned even
        // when nothing is allocated.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // SAFETY: the first `len` slots are initialized; each one is dropped exactly once here.
        unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len)) };
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: `ptr` was allocated with exactly this layout.
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast(), layout) };
        }
    }
}
