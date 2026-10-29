use std::fmt;
use std::ptr;

use crate::judge::{case, check, check_eq};
use crate::user::{Bag, Container, Stack, describe_all, largest, move_all, total};

/// A value type of the harness, ordered and displayed by its title.
/// It is not `Copy`, `Clone` or `Debug`.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Book(&'static str);

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

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
        check_eq(b.items(), [1, 2, 3])?;
        check_eq(total(&b), 6)?;
        check_eq(largest(&b), Some(&3))
    });
    case("example-2", || {
        let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
        let mut y = Stack { elems: vec![String::from("c")] };
        check_eq(move_all(&mut x, &mut y), 2)?;
        check_eq(x.elems.len(), 0)?;
        check_eq(y.elems, ["c", "b", "a"])
    });
    case("example-3", || {
        check_eq(total(&Stack { elems: vec![-5i32, 2] }), -3)?;
        check_eq(total(&Stack { elems: vec![40_000u16, 30_000] }), 70_000)?;
        check_eq(largest(&Stack { elems: vec!["a", "b"] }), Some(&"b"))?;
        check_eq(describe_all(&Stack { elems: vec!['h', 'i'] }), ["h", "i"])
    });
    case("case-1", || {
        // Empty containers.
        let mut empty = Stack { elems: Vec::<u8>::new() };
        let mut b = bag(&[5]);
        check_eq(move_all(&mut empty, &mut b), 0)?;
        check_eq(b.items(), [5])?;
        check_eq(move_all(&mut Bag::new(), &mut empty), 0)?;
        check_eq(empty.elems.len(), 0)?;
        check_eq(total(&Bag::new()), 0)?;
        check_eq(total(&Stack { elems: Vec::<i32>::new() }), 0)?;
        check_eq(largest(&Bag::new()), None)?;
        check_eq(largest(&Stack { elems: Vec::<String>::new() }), None)?;
        check_eq(describe_all(&Stack { elems: Vec::<char>::new() }), Vec::<String>::new())
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
        // The harness's own container, holding values that are not Copy, Clone or Debug.
        let mut shelf = Shelf { slots: vec![Book("Emma"), Book("Dune"), Book("Ulysses")] };
        let mut stack = Stack { elems: vec![Book("Beloved")] };
        check_eq(move_all(&mut shelf, &mut stack), 3)?;
        check_eq(titles(&shelf).len(), 0)?;
        check_eq(titles(&stack), ["Beloved", "Ulysses", "Dune", "Emma"])?;
        check_eq(move_all(&mut stack, &mut shelf), 4)?;
        check_eq(titles(&shelf), ["Emma", "Dune", "Ulysses", "Beloved"])?;
        check_eq(describe_all(&shelf), ["<Emma>", "<Dune>", "<Ulysses>", "<Beloved>"])?;
        let best = largest(&shelf);
        check(best.is_some_and(|b| ptr::eq(b, &shelf.slots[2])), "expected a reference to Ulysses")?;
        check(largest(&Shelf::<Book> { slots: Vec::new() }).is_none(), "expected None")
    });
    case("case-4", || {
        // total converts every value to i64 before adding.
        check_eq(total(&bag(&[255, 255, 255, 255])), 1020)?;
        check_eq(total(&Stack { elems: vec![u32::MAX, u32::MAX] }), 8_589_934_590)?;
        check_eq(total(&Stack { elems: vec![i64::MAX, -1] }), i64::MAX - 1)?;
        check_eq(total(&Shelf { slots: vec![-128i8, 127, -1] }), -2)?;
        check_eq(total(&Stack { elems: vec![true, false, true] }), 2)
    });
    case("case-5", || {
        // largest returns a reference into the container, to the last of equal largest values.
        let s = Stack { elems: vec![2, 7, 1, 7] };
        let got = largest(&s);
        check_eq(got, Some(&7))?;
        check(got.is_some_and(|r| ptr::eq(r, &s.elems[3])), "expected the last 7")?;
        let words = vec![String::from("pear"), String::from("Zebra"), String::from("apple")];
        let words = Stack { elems: words };
        check_eq(largest(&words).map(String::as_str), Some("pear"))?;
        let b = bag(&[4, 1, 4]);
        check(largest(&b).is_some_and(|r| ptr::eq(r, &b.items()[2])), "expected the last 4")
    });
    case("case-6", || {
        check_eq(describe_all(&bag(&[10, 2, 0])), ["0", "2", "10"])?;
        check_eq(describe_all(&Stack { elems: vec![1.5, -2.0] }), ["1.5", "-2"])?;
        check_eq(describe_all(&Stack { elems: vec!["x y", ""] }), ["x y", ""])?;
        check_eq(describe_all(&Shelf { slots: vec![-3i8] }), ["-3"])
    });
}
