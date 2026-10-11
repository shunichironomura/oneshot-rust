pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self {
        todo!()
    }

    pub fn set_value(&mut self, value: usize) {
        todo!()
    }
}

/// A mock messenger that remembers every message it receives.
pub struct Recorder {
    // Add the fields you need.
}

impl Recorder {
    pub fn new() -> Recorder {
        todo!()
    }

    pub fn messages(&self) -> Vec<String> {
        todo!()
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        todo!()
    }
}
