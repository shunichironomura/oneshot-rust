use std::cell::UnsafeCell;
use std::hint;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

// The lock hands out access to `T` to one thread at a time.
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        SpinLock { locked: AtomicBool::new(false), data: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> SpinGuard<'_, T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
            hint::spin_loop();
        }
    }

    pub fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
        // One atomic step: "if unlocked, lock it". On success, Acquire makes everything the
        // previous holder wrote before its Release visible to this thread.
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| SpinGuard { lock: self })
    }

    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// A shared `&SpinGuard` only gives out `&T`, so sharing the guard needs `T: Sync`.
unsafe impl<T: Sync> Sync for SpinGuard<'_, T> {}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // The guard proves that this thread holds the lock.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        // Release publishes this critical section's writes to the next thread that locks.
        self.lock.locked.store(false, Ordering::Release);
    }
}
