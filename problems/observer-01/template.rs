pub struct Subject {
    // Add the fields you need.
}

/// Dropping it unsubscribes the listener.
pub struct Subscription {
    // Add the fields you need.
}

impl Subject {
    pub fn new() -> Subject {
        todo!()
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        todo!()
    }

    pub fn notify(&self, msg: &str) -> usize {
        todo!()
    }

    pub fn listener_count(&self) -> usize {
        todo!()
    }
}
