use crate::judge::{case, check_eq};
use crate::user::{Bag, Container, Stack, total};

/// A container of the harness.
struct Shelf<T> {
    slots: Vec<T>,
}

impl<T> Container for Shelf<T> {
    type Item = T;

    fn items(&self) -> &[T] {
        &self.slots
    }
}

pub fn run() {
    case("example-1", || {
        check_eq(total(&Bag { bytes: vec![3, 1, 2] }), 6)?;
        check_eq(total(&Stack { elems: vec![-5i32, 2] }), -3)?;
        check_eq(total(&Stack { elems: vec![40_000u16, 30_000] }), 70_000)
    });
    case("case-1", || {
        // Empty containers.
        check_eq(total(&Bag { bytes: Vec::new() }), 0)?;
        check_eq(total(&Stack { elems: Vec::<i32>::new() }), 0)?;
        check_eq(total(&Shelf { slots: Vec::<i8>::new() }), 0)
    });
    case("case-2", || {
        // Every value is converted to i64 before adding.
        check_eq(total(&Bag { bytes: vec![255, 255, 255, 255] }), 1020)?;
        check_eq(total(&Stack { elems: vec![u32::MAX, u32::MAX] }), 8_589_934_590)?;
        check_eq(total(&Stack { elems: vec![i64::MAX, -1] }), i64::MAX - 1)?;
        check_eq(total(&Shelf { slots: vec![-128i8, 127, -1] }), -2)?;
        check_eq(total(&Stack { elems: vec![true, false, true] }), 2)
    });
}
