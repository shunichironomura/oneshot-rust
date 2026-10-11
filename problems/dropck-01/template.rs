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
    let mut tracers = Vec::new();
    let sink = RefCell::new(Vec::new());
    for name in names {
        tracers.push(Tracer::new(&sink, name));
    }
    sink.borrow_mut().push("run".to_string());
    sink.into_inner()
}
