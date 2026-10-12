use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn push(&mut self, x: T) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.as_slice().get(i)
    }

    pub fn as_slice(&self) -> &[T] {
        todo!()
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        todo!()
    }
}
