use std::cell::RefCell;

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
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        // "value is at least 90% of max" is value / max >= 90 / 100, that is
        // value * 100 >= max * 90: exact, with no division at all.
        if self.value >= self.max {
            self.messenger.send("Error: You are over your quota!");
        } else if self.value * 100 >= self.max * 90 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if self.value * 100 >= self.max * 75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/// A mock messenger that remembers every message it receives.
pub struct Recorder {
    sent: RefCell<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder { sent: RefCell::new(Vec::new()) }
    }

    pub fn messages(&self) -> Vec<String> {
        self.sent.borrow().clone()
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        self.sent.borrow_mut().push(msg.to_string());
    }
}
