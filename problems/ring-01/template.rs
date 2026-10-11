pub struct Ring<T, const N: usize> {
    buf: [Option<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, x: T) -> Option<T> {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_full(&self) -> bool {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        // A placeholder so that the code compiles. Replace it.
        std::iter::empty()
    }
}
