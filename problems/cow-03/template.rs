use std::rc::Rc;

#[derive(Clone)]
pub struct Doc {
    lines: Rc<Vec<String>>,
}

impl Doc {
    pub fn new(lines: &[&str]) -> Doc {
        todo!()
    }

    pub fn line(&self, i: usize) -> Option<&str> {
        todo!()
    }

    pub fn set_line(&mut self, i: usize, text: &str) {
        todo!()
    }

    pub fn shares_storage(&self, other: &Doc) -> bool {
        todo!()
    }
}
