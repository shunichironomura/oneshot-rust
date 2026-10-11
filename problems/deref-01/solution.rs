use std::ops::Deref;

/// A vector whose elements are always in ascending order.
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        SortedVec { items: Vec::new() }
    }

    pub fn insert(&mut self, value: T) -> usize {
        // The first element greater than `value`, i.e. just after all the equal ones.
        let index = self.items.partition_point(|x| x <= &value);
        self.items.insert(index, value);
        index
    }

    pub fn remove(&mut self, value: &T) -> bool {
        // The first element not less than `value`: the leftmost equal one, if there is one.
        let index = self.items.partition_point(|x| x < value);
        if self.items.get(index) == Some(value) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(mut items: Vec<T>) -> Self {
        // `sort` is stable: equal elements keep their order.
        items.sort();
        SortedVec { items }
    }
}

impl<T> SortedVec<T> {
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // `&Vec<T>` coerces to `&[T]` here.
        &self.items
    }
}
