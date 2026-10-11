use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// What a `Subscription` owns. Only the `Subscription` keeps an `Rc` to it, so the subject's
/// `Weak` can be upgraded exactly as long as the `Subscription` is alive.
struct Listener {
    callback: Rc<dyn Fn(&str)>,
}

pub struct Subject {
    listeners: RefCell<Vec<Weak<Listener>>>,
}

/// Dropping it unsubscribes the listener.
#[must_use = "dropping a Subscription unsubscribes the listener"]
pub struct Subscription {
    _listener: Rc<Listener>,
}

impl Subject {
    pub fn new() -> Subject {
        Subject { listeners: RefCell::new(Vec::new()) }
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        let listener = Rc::new(Listener { callback: Rc::new(f) });
        let mut listeners = self.listeners.borrow_mut();
        // Forget the listeners whose subscriptions are gone, so that the list does not grow.
        listeners.retain(|weak| weak.strong_count() > 0);
        listeners.push(Rc::downgrade(&listener));
        Subscription { _listener: listener }
    }

    pub fn notify(&self, msg: &str) -> usize {
        // Copy the list and release the borrow before calling anyone: a listener may subscribe,
        // and the listeners it subscribes are not in this copy.
        let snapshot = self.listeners.borrow().clone();
        let mut called = 0;
        for weak in snapshot {
            // Keep the callback alive during the call, but not the `Listener`: if the callback
            // drops its own subscription, it must stop counting as subscribed at once.
            let callback = match weak.upgrade() {
                Some(listener) => Rc::clone(&listener.callback),
                None => continue, // unsubscribed before its turn
            };
            callback(msg);
            called += 1;
        }
        called
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().iter().filter(|weak| weak.strong_count() > 0).count()
    }
}
