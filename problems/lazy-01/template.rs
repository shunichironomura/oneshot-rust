use std::marker::PhantomData;

pub struct Lazy<T, F: FnOnce() -> T> {
    // Replace this placeholder with the fields you need.
    _todo: PhantomData<(T, F)>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self {
        todo!()
    }

    pub fn get(&mut self) -> &T {
        todo!()
    }

    pub fn is_ready(&self) -> bool {
        todo!()
    }

    pub fn into_value(self) -> T {
        todo!()
    }
}
