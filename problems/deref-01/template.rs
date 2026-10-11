use std::ops::Deref;

/// A vector whose elements are always in ascending order.
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, value: T) -> usize {
        todo!()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        todo!()
    }
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(items: Vec<T>) -> Self {
        todo!()
    }
}

impl<T> SortedVec<T> {
    pub fn into_vec(self) -> Vec<T> {
        todo!()
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        todo!()
    }
}
