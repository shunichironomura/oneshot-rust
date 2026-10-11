use std::fmt::Display;

/// Something that values can be put into and taken out of.
pub trait Container {
    /// The type of the values it holds.
    type Item;

    /// Adds `x` to the container.
    fn put(&mut self, x: Self::Item);
    /// Removes the last value and returns it, or returns `None` if the container is empty.
    fn take_last(&mut self) -> Option<Self::Item>;
    /// All the values, in order.
    fn items(&self) -> &[Self::Item];
}

/// A stack of values of any type: `put` pushes onto the end, `take_last` pops from the end.
#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    pub elems: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn put(&mut self, x: T) {
        self.elems.push(x);
    }

    fn take_last(&mut self) -> Option<T> {
        self.elems.pop()
    }

    fn items(&self) -> &[T] {
        &self.elems
    }
}

/// A bag of bytes that keeps them in ascending order.
#[derive(Debug, Default, PartialEq)]
pub struct Bag {
    bytes: Vec<u8>,
}

impl Bag {
    pub fn new() -> Self {
        Bag { bytes: Vec::new() }
    }
}

impl Container for Bag {
    type Item = u8;

    fn put(&mut self, x: u8) {
        let at = self.bytes.partition_point(|&b| b <= x);
        self.bytes.insert(at, x);
    }

    /// Removes the largest byte.
    fn take_last(&mut self) -> Option<u8> {
        self.bytes.pop()
    }

    fn items(&self) -> &[u8] {
        &self.bytes
    }
}

pub fn move_all<A, B>(from: &mut A, to: &mut B) -> usize
where
    A: Container,
    B: Container<Item = A::Item>,
{
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}

pub fn total<C>(c: &C) -> i64
where
    C: Container,
    C::Item: Copy + Into<i64>,
{
    c.items().iter().map(|&x| x.into()).sum()
}

pub fn largest<C>(c: &C) -> Option<&C::Item>
where
    C: Container,
    C::Item: Ord,
{
    c.items().iter().max()
}

pub fn describe_all<C>(c: &C) -> Vec<String>
where
    C: Container,
    C::Item: Display,
{
    c.items().iter().map(|x| x.to_string()).collect()
}
