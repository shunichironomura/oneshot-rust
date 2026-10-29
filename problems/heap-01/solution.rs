use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Triage {
    /// `(severity, arrival number, name)`. `BinaryHeap` pops the largest tuple: the highest
    /// severity first, and among those the smallest arrival number, thanks to `Reverse`.
    heap: BinaryHeap<(u8, Reverse<u64>, String)>,
    arrivals: u64,
}

impl Triage {
    pub fn new() -> Self {
        Triage { heap: BinaryHeap::new(), arrivals: 0 }
    }

    pub fn arrive(&mut self, name: &str, severity: u8) {
        self.heap.push((severity, Reverse(self.arrivals), name.to_string()));
        self.arrivals += 1;
    }

    pub fn next(&mut self) -> Option<String> {
        self.heap.pop().map(|(_, _, name)| name)
    }

    pub fn peek(&self) -> Option<&str> {
        self.heap.peek().map(|(_, _, name)| name.as_str())
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}
