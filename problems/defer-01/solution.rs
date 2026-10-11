/// A guard that runs its action when it is dropped.
pub struct Defer<F: FnOnce()> {
    /// `None` once the action has run or has been cancelled.
    action: Option<F>,
}

pub fn defer<F: FnOnce()>(f: F) -> Defer<F> {
    Defer { action: Some(f) }
}

impl<F: FnOnce()> Defer<F> {
    pub fn cancel(mut self) {
        // Drop the action without calling it: what it captured is released now, and `drop`
        // finds nothing to run.
        self.action = None;
    }

    pub fn run_now(self) {
        // Dropping the guard is exactly what runs the action.
        drop(self);
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            action();
        }
    }
}

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    defer(move || {
        // One guard per action, the last action first. A `Vec` drops its elements from front to
        // back, and if one of them panics, it still drops the rest before the panic goes on.
        let guards: Vec<_> = actions.into_iter().rev().map(defer).collect();
        drop(guards);
    })
}
