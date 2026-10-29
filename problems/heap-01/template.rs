use std::collections::BinaryHeap;

pub struct Triage {
    // Only a starting point: change the fields as you like.
    heap: BinaryHeap<(u8, String)>,
}

impl Triage {
    pub fn new() -> Self {
        todo!()
    }

    pub fn arrive(&mut self, name: &str, severity: u8) {
        todo!()
    }

    pub fn next(&mut self) -> Option<String> {
        todo!()
    }

    pub fn peek(&self) -> Option<&str> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
