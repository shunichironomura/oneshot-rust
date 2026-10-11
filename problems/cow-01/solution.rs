use std::borrow::Cow;
use std::rc::Rc;

pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim_ascii();
    // A run needs replacing if it is longer than one character or is not a plain space.
    let needs_copy =
        trimmed.contains("  ") || trimmed.contains(|c: char| c.is_ascii_whitespace() && c != ' ');
    if needs_copy {
        Cow::Owned(trimmed.split_ascii_whitespace().collect::<Vec<_>>().join(" "))
    } else {
        Cow::Borrowed(trimmed)
    }
}

pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    // Look first: `to_mut` would copy a borrowed slice even if nothing changes.
    if v.iter().any(|&x| x > max) {
        for x in v.to_mut().iter_mut() {
            *x = (*x).min(max);
        }
    }
}

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
        // Check before `make_mut`, which would already have copied a shared Vec.
        assert!(i < self.lines.len(), "line {i} is out of range");
        Rc::make_mut(&mut self.lines)[i] = text.to_string();
    }

    pub fn shares_storage(&self, other: &Doc) -> bool {
        Rc::ptr_eq(&self.lines, &other.lines)
    }

    pub fn storage_id(&self) -> usize {
        Rc::as_ptr(&self.lines) as usize
    }
}
