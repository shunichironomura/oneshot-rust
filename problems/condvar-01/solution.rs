use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BoundedQueue<T> {
    capacity: usize,
    /// The items and the closed flag change together, under one lock.
    state: Mutex<State<T>>,
    /// Notified when an item is added or the queue is closed.
    not_empty: Condvar,
    /// Notified when an item is removed or the queue is closed.
    not_full: Condvar,
}

struct State<T> {
    items: VecDeque<T>,
    closed: bool,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedQueue {
            capacity,
            state: Mutex::new(State { items: VecDeque::new(), closed: false }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        let state = self.state.lock().unwrap();
        // `wait_while` checks the condition again after every wakeup, and sleeps again if needed.
        let mut state = self
            .not_full
            .wait_while(state, |s| !s.closed && s.items.len() == self.capacity)
            .unwrap();
        if state.closed {
            return Err(item);
        }
        state.items.push_back(item);
        drop(state);
        self.not_empty.notify_one();
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let state = self.state.lock().unwrap();
        let mut state = self
            .not_empty
            .wait_while(state, |s| !s.closed && s.items.is_empty())
            .unwrap();
        // Empty here means closed and empty.
        let item = state.items.pop_front()?;
        drop(state);
        self.not_full.notify_one();
        Some(item)
    }

    pub fn close(&self) {
        self.state.lock().unwrap().closed = true;
        // Every waiting call has to see the change, not just one of them.
        self.not_empty.notify_all();
        self.not_full.notify_all();
    }

    pub fn len(&self) -> usize {
        self.state.lock().unwrap().items.len()
    }
}
