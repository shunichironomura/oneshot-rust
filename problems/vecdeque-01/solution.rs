use std::collections::VecDeque;

pub struct History {
    /// The oldest action is at the front, the newest at the back.
    actions: VecDeque<String>,
    capacity: usize,
}

impl History {
    pub fn new(capacity: usize) -> History {
        History { actions: VecDeque::new(), capacity }
    }

    pub fn record(&mut self, action: &str) {
        self.actions.push_back(action.to_string());
        if self.actions.len() > self.capacity {
            self.actions.pop_front();
        }
    }

    pub fn undo(&mut self) -> Option<String> {
        self.actions.pop_back()
    }

    pub fn recent(&self, n: usize) -> Vec<&str> {
        self.actions.iter().rev().take(n).map(|s| s.as_str()).collect()
    }

    pub fn oldest(&self) -> Option<&str> {
        self.actions.front().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }
}
