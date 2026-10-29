use std::rc::Rc;

#[derive(Clone)]
pub struct Doc {
    lines: Rc<Vec<String>>,
}

impl Doc {
    pub fn new(lines: &[&str]) -> Doc {
        Doc {
            lines: Rc::new(lines.iter().map(|s| s.to_string()).collect()),
        }
    }

    pub fn line(&self, i: usize) -> Option<&str> {
        self.lines.get(i).map(String::as_str)
    }

    pub fn set_line(&mut self, i: usize, text: &str) {
        // Copies the `Vec` first only if another `Rc` still points to it.
        Rc::make_mut(&mut self.lines)[i] = text.to_string();
    }

    pub fn shares_storage(&self, other: &Doc) -> bool {
        Rc::ptr_eq(&self.lines, &other.lines)
    }
}
