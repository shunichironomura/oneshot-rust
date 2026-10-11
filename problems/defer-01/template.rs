/// A guard that runs its action when it is dropped.
pub struct Defer<F> {
    // You may change the fields.
    action: F,
}

pub fn defer<F: FnOnce()>(f: F) -> Defer<F> {
    todo!()
}

impl<F: FnOnce()> Defer<F> {
    pub fn cancel(self) {
        todo!()
    }

    pub fn run_now(self) {
        todo!()
    }
}

impl<F> Drop for Defer<F> {
    fn drop(&mut self) {
        // Run the action here.
    }
}

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    // A placeholder so that the template compiles: replace it.
    defer(|| {})
}
