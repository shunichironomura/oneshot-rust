use std::marker::PhantomData;

pub struct Closed;
pub struct Open;

pub struct Connection<State> {
    // Add more fields if you need them.
    _state: PhantomData<State>,
}

impl Connection<Closed> {
    pub fn new(host: &str) -> Connection<Closed> {
        todo!()
    }

    pub fn open(self) -> Connection<Open> {
        todo!()
    }
}

impl Connection<Open> {
    pub fn send(&mut self, message: &str) {
        todo!()
    }

    pub fn close(self) -> Connection<Closed> {
        todo!()
    }
}

impl<State> Connection<State> {
    pub fn host(&self) -> &str {
        todo!()
    }

    pub fn log(&self) -> &[String] {
        todo!()
    }
}
