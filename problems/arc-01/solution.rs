use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{self, AtomicUsize, Ordering};

/// One heap allocation shared by all the handles: the number of handles and the value.
struct Inner<T> {
    count: AtomicUsize,
    value: T,
}

pub struct MyArc<T> {
    // `NonNull<X>` is covariant in `X`, like `*const X`. A `*mut X` would be invariant.
    ptr: NonNull<Inner<T>>,
}

// A handle may drop the value on any thread (so `T: Send`), and handles on several threads give
// out `&T` at the same time (so `T: Sync`).
unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(Inner { count: AtomicUsize::new(1), value });
        MyArc { ptr: NonNull::from(Box::leak(inner)) }
    }

    fn inner(&self) -> &Inner<T> {
        // SAFETY: the allocation lives as long as any handle does, and `self` is one.
        unsafe { self.ptr.as_ref() }
    }

    pub fn strong_count(this: &Self) -> usize {
        this.inner().count.load(Ordering::Relaxed)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        // Acquire: whatever other threads did with the value before they dropped their handles
        // happens before we change it.
        if this.inner().count.load(Ordering::Acquire) == 1 {
            // SAFETY: `this` is the only handle, and it is borrowed mutably.
            Some(unsafe { &mut this.ptr.as_mut().value })
        } else {
            None
        }
    }

    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        a.ptr == b.ptr
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        // Relaxed: the new handle is made from a live one, so the value cannot go away here, and
        // no other memory access has to be ordered with the increment.
        self.inner().count.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        // Release: our uses of the value happen before our decrement.
        if self.inner().count.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        // Acquire: the other handles' uses of the value happen before we free it.
        atomic::fence(Ordering::Acquire);
        // SAFETY: this was the last handle, so nothing else can reach the allocation.
        drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
    }
}
