use crate::judge::{case, check_eq};
use crate::user::{Bag, Container, Stack, move_all};

/// A value type of the harness. It is not `Copy` or `Clone`.
struct Book(&'static str);

/// A container of the harness: it keeps its values in the order they were put.
struct Shelf<T> {
    slots: Vec<T>,
}

impl<T> Container for Shelf<T> {
    type Item = T;

    fn put(&mut self, x: T) {
        self.slots.push(x);
    }

    fn take_last(&mut self) -> Option<T> {
        self.slots.pop()
    }

    fn items(&self) -> &[T] {
        &self.slots
    }
}

fn bag(bytes: &[u8]) -> Bag {
    let mut b = Bag::new();
    for &x in bytes {
        b.put(x);
    }
    b
}

fn titles<C: Container<Item = Book>>(c: &C) -> Vec<&'static str> {
    c.items().iter().map(|b| b.0).collect()
}

pub fn run() {
    case("example-1", || {
        let mut s = Stack { elems: vec![3, 1, 2] };
        let mut b = Bag::new();
        check_eq(move_all(&mut s, &mut b), 3)?;
        check_eq(s.elems.len(), 0)?;
        check_eq(b.items(), [1, 2, 3])
    });
    case("example-2", || {
        let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
        let mut y = Stack { elems: vec![String::from("c")] };
        check_eq(move_all(&mut x, &mut y), 2)?;
        check_eq(x.elems.len(), 0)?;
        check_eq(y.elems, ["c", "b", "a"])
    });
    case("case-1", || {
        // Empty containers.
        let mut empty = Stack { elems: Vec::<u8>::new() };
        let mut b = bag(&[5]);
        check_eq(move_all(&mut empty, &mut b), 0)?;
        check_eq(b.items(), [5])?;
        check_eq(move_all(&mut Bag::new(), &mut empty), 0)?;
        check_eq(empty.elems.len(), 0)
    });
    case("case-2", || {
        // Bag to Stack, Bag to Bag and i64 stacks; the values already in `to` stay.
        let mut b = bag(&[9, 4, 7]);
        let mut s = Stack { elems: vec![1] };
        check_eq(move_all(&mut b, &mut s), 3)?;
        check_eq(b.items().len(), 0)?;
        check_eq(&s.elems, &[1, 9, 7, 4])?;
        let mut from = bag(&[2, 8, 2]);
        let mut to = bag(&[5]);
        check_eq(move_all(&mut from, &mut to), 3)?;
        check_eq(to.items(), [2, 2, 5, 8])?;
        let mut a = Stack { elems: vec![-1i64, 7] };
        let mut c = Stack { elems: vec![0i64] };
        check_eq(move_all(&mut a, &mut c), 2)?;
        check_eq(c.elems, [0, 7, -1])
    });
    case("case-3", || {
        // The harness's own container, holding values that are not Copy or Clone.
        let mut shelf = Shelf { slots: vec![Book("Emma"), Book("Dune"), Book("Ulysses")] };
        let mut stack = Stack { elems: vec![Book("Beloved")] };
        check_eq(move_all(&mut shelf, &mut stack), 3)?;
        check_eq(titles(&shelf).len(), 0)?;
        check_eq(titles(&stack), ["Beloved", "Ulysses", "Dune", "Emma"])?;
        check_eq(move_all(&mut stack, &mut shelf), 4)?;
        check_eq(titles(&shelf), ["Emma", "Dune", "Ulysses", "Beloved"])
    });
}
