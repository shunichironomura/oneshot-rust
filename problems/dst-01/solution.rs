use std::fmt::Debug;
use std::rc::Rc;

pub fn byte_size<T: ?Sized>(x: &T) -> usize {
    std::mem::size_of_val(x)
}

pub fn show_pair<A: Debug + ?Sized, B: Debug + ?Sized>(a: &A, b: &B) -> String {
    format!("{a:?} & {b:?}")
}

pub fn share<T: ?Sized>(b: Box<T>) -> Rc<T> {
    Rc::from(b)
}
