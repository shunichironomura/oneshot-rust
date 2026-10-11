use std::borrow::Cow;
use std::rc::Rc;

pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    todo!()
}

pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    todo!()
}

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

    pub fn storage_id(&self) -> usize {
        todo!()
    }
}
