use std::cell::RefCell;

pub struct Tracer<'a> {
    sink: &'a RefCell<Vec<String>>,
    name: String,
}

impl<'a> Tracer<'a> {
    pub fn new(sink: &'a RefCell<Vec<String>>, name: &str) -> Tracer<'a> {
        sink.borrow_mut().push(format!("begin {name}"));
        Tracer { sink, name: name.to_string() }
    }
}

impl Drop for Tracer<'_> {
    fn drop(&mut self) {
        self.sink.borrow_mut().push(format!("end {}", self.name));
    }
}

pub fn trace(names: &[&str]) -> Vec<String> {
    // Declared first, so it is dropped last: even if something panics before `drop(tracers)`,
    // the tracers are dropped (and write their "end" lines) while the sink is still alive.
    let sink = RefCell::new(Vec::new());
    let mut tracers = Vec::new();
    for name in names {
        tracers.push(Tracer::new(&sink, name));
    }
    sink.borrow_mut().push("run".to_string());
    // A `Vec` drops its elements front to back, so the "end" lines come in creation order.
    // After this, nothing borrows `sink` any more and it can be moved out.
    drop(tracers);
    sink.into_inner()
}
