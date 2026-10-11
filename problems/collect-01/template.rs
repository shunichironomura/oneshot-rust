use std::collections::BTreeMap;
use std::marker::PhantomData;

/// A multiset: each distinct element is stored once, with the number of times it was added.
pub struct Bag<T> {
    counts: BTreeMap<T, usize>,
}

impl<T: Ord> Bag<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn count(&self, x: &T) -> usize {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}

impl<T: Ord> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

impl<T: Ord> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
}

impl<'a, T: Ord + Copy + 'a> Extend<&'a T> for Bag<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        todo!()
    }
}

/// Iterator over the `(element, count)` pairs of a borrowed bag.
pub struct Iter<'a, T> {
    // Placeholder: replace it with the fields you need.
    placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = (&'a T, usize);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        todo!()
    }
}

/// Iterator that moves the elements out of a bag.
pub struct IntoIter<T> {
    // Placeholder: replace it with the fields you need.
    placeholder: PhantomData<T>,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        todo!()
    }
}
