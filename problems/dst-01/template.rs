use std::fmt::Debug;
use std::rc::Rc;

pub fn byte_size(x: &u64) -> usize {
    std::mem::size_of_val(x)
}

pub fn show_pair(a: &str, b: &str) -> String {
    format!("{a:?} & {b:?}")
}

pub fn share(b: Box<i32>) -> Rc<i32> {
    Rc::from(b)
}
