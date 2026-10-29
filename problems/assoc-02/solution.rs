/// Something that holds values.
pub trait Container {
    /// The type of the values it holds.
    type Item;

    /// All the values, in order.
    fn items(&self) -> &[Self::Item];
}

/// A list of values of any type.
pub struct Stack<T> {
    pub elems: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn items(&self) -> &[T] {
        &self.elems
    }
}

/// A bag of bytes.
pub struct Bag {
    pub bytes: Vec<u8>,
}

impl Container for Bag {
    type Item = u8;

    fn items(&self) -> &[u8] {
        &self.bytes
    }
}

pub fn total<C>(c: &C) -> i64
where
    C: Container,
    C::Item: Copy + Into<i64>,
{
    c.items().iter().map(|&x| x.into()).sum()
}
