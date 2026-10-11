pub struct BoundedQueue<T> {
    // Choose the fields yourself. This one is only a placeholder.
    _todo: std::marker::PhantomData<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        todo!()
    }

    pub fn pop(&self) -> Option<T> {
        todo!()
    }

    pub fn close(&self) {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
