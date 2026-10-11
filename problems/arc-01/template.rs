use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

/// One heap allocation shared by all the handles: the number of handles and the value.
/// (A suggestion: you may change or remove it.)
struct Inner<T> {
    count: AtomicUsize,
    value: T,
}

pub struct MyArc<T> {
    // Replace this placeholder with your own fields.
    placeholder: PhantomData<T>,
}

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self {
        todo!()
    }

    pub fn strong_count(this: &Self) -> usize {
        todo!()
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        todo!()
    }

    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        todo!()
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!()
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        // Runs every time a handle is dropped.
    }
}
