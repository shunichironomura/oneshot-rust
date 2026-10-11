use std::collections::{BTreeMap, btree_map};
use std::iter::{self, FlatMap, RepeatN};

/// A multiset: each distinct element is stored once, with the number of times it was added.
pub struct Bag<T> {
    counts: BTreeMap<T, usize>,
}

impl<T: Ord> Bag<T> {
    pub fn new() -> Self {
        Bag { counts: BTreeMap::new() }
    }

    pub fn count(&self, x: &T) -> usize {
        self.counts.get(x).copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.counts.values().sum()
    }
}

impl<T: Ord> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bag = Bag::new();
        bag.extend(iter);
        bag
    }
}

impl<T: Ord> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            *self.counts.entry(x).or_insert(0) += 1;
        }
    }
}

impl<'a, T: Ord + Copy + 'a> Extend<&'a T> for Bag<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

/// Iterator over the `(element, count)` pairs of a borrowed bag.
pub struct Iter<'a, T> {
    // `btree_map::Iter`, not `Iter`: that name is taken by this struct.
    inner: btree_map::Iter<'a, T, usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(x, &n)| (x, n))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(x, &n)| (x, n))
    }
}

impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = (&'a T, usize);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        Iter { inner: self.counts.iter() }
    }
}

/// Iterator that moves the elements out of a bag.
///
/// A closure type cannot be written down, so the function that turns an entry into its copies is
/// stored as a plain function pointer. `FlatMap` needs `RepeatN<T>: Iterator`, hence `T: Clone`.
pub struct IntoIter<T: Clone> {
    inner: FlatMap<btree_map::IntoIter<T, usize>, RepeatN<T>, fn((T, usize)) -> RepeatN<T>>,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.next()
    }
}

impl<T: Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.next_back()
    }
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        // The closure captures nothing, so it coerces to the fn pointer type of the field.
        // `repeat_n` clones the element n - 1 times and hands out the original last.
        IntoIter { inner: self.counts.into_iter().flat_map(|(x, n)| iter::repeat_n(x, n)) }
    }
}
