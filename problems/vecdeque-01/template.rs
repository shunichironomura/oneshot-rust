use std::collections::VecDeque;

pub struct History {
    // Only a starting point: change the fields as you like.
    actions: VecDeque<String>,
}

impl History {
    pub fn new(capacity: usize) -> History {
        todo!()
    }

    pub fn record(&mut self, action: &str) {
        todo!()
    }

    pub fn undo(&mut self) -> Option<String> {
        todo!()
    }

    pub fn recent(&self, n: usize) -> Vec<&str> {
        todo!()
    }

    pub fn oldest(&self) -> Option<&str> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
