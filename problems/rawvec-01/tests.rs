use std::cell::Cell;

use crate::judge::{case, check, check_eq};
use crate::user::MyVec;

/// Adds 1 to a shared counter when dropped.
struct Counted<'a> {
    id: u32,
    drops: &'a Cell<u32>,
}

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

/// An over-aligned type.
#[derive(Debug, PartialEq)]
#[repr(align(64))]
struct Wide(u8);

fn s(x: &str) -> String {
    String::from(x)
}

/// Pushes every item and returns the capacity after each push.
fn push_all<T>(v: &mut MyVec<T>, items: impl IntoIterator<Item = T>) -> Vec<usize> {
    items
        .into_iter()
        .map(|x| {
            v.push(x);
            v.capacity()
        })
        .collect()
}

pub fn run() {
    case("example-1", || {
        let mut v = MyVec::new();
        check_eq((v.len(), v.capacity()), (0, 0))?;
        let caps = push_all(&mut v, ["a", "b", "c", "d", "e"].map(String::from));
        check_eq(caps, vec![4, 4, 4, 4, 8])?;
        check_eq(v.len(), 5)?;
        check_eq(v.as_slice(), ["a", "b", "c", "d", "e"])?;
        check_eq(v.pop(), Some(s("e")))?;
        check_eq(v.pop(), Some(s("d")))?;
        check_eq(v.get(2), Some(&s("c")))?;
        check_eq(v.get(3), None)?;
        check_eq((v.len(), v.capacity()), (3, 8))
    });
    case("case-1", || {
        // Empty vectors, including ones that are dropped without ever allocating.
        let mut v: MyVec<String> = MyVec::new();
        check_eq((v.len(), v.capacity()), (0, 0))?;
        check_eq(v.pop(), None)?;
        check_eq(v.get(0), None)?;
        check(v.as_slice().is_empty(), "as_slice of an empty MyVec should be empty")?;
        drop(v);
        drop(MyVec::<u8>::new());
        drop(MyVec::<Wide>::new());
        // Emptied by pop: the buffer is still there and must be freed.
        let mut v = MyVec::new();
        v.push(s("only"));
        check_eq(v.pop(), Some(s("only")))?;
        check_eq(v.pop(), None)?;
        check_eq((v.len(), v.capacity()), (0, 4))
    });
    case("case-2", || {
        // Capacity grows only when a push finds the buffer full, and never shrinks.
        let mut v = MyVec::new();
        let caps = push_all(&mut v, 1..=17u64);
        check_eq(caps, vec![4, 4, 4, 4, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 32])?;
        check_eq(v.as_slice(), (1..=17u64).collect::<Vec<_>>())?;
        for want in (1..=17u64).rev() {
            check_eq(v.pop(), Some(want))?;
        }
        check_eq(v.pop(), None)?;
        check_eq((v.len(), v.capacity()), (0, 32))?;
        let caps = push_all(&mut v, [7, 8, 9]);
        check_eq(caps, vec![32, 32, 32])?;
        check_eq(v.as_slice(), [7, 8, 9])?;

        let mut w = MyVec::new();
        check_eq(push_all(&mut w, [1u32, 2, 3, 4]), vec![4, 4, 4, 4])?;
        check_eq(w.pop(), Some(4))?;
        check_eq(push_all(&mut w, [5, 6]), vec![4, 8])?;
        check_eq(w.as_slice(), [1, 2, 3, 5, 6])
    });
    case("case-3", || {
        // get and as_slice after mixed pushes and pops.
        let mut v = MyVec::new();
        for x in ["p", "q", "r", "s", "t", "u"] {
            v.push(s(x));
        }
        check_eq(v.pop(), Some(s("u")))?;
        check_eq(v.pop(), Some(s("t")))?;
        v.push(s("v"));
        check_eq(v.get(0), Some(&s("p")))?;
        check_eq(v.get(4), Some(&s("v")))?;
        check_eq(v.get(5), None)?;
        check_eq(v.get(usize::MAX), None)?;
        check_eq(v.as_slice(), ["p", "q", "r", "s", "v"])?;
        check_eq((v.len(), v.capacity()), (5, 8))
    });
    case("case-4", || {
        // Every element is dropped exactly once: by its owner after pop, or by the MyVec.
        let drops = Cell::new(0);
        let mut v = MyVec::new();
        for id in 0..10 {
            v.push(Counted { id, drops: &drops });
        }
        check_eq(drops.get(), 0)?;
        check_eq(v.pop().map(|c| c.id), Some(9))?;
        check_eq(drops.get(), 1)?;
        let kept = v.pop();
        check_eq(kept.as_ref().map(|c| c.id), Some(8))?;
        check_eq((drops.get(), v.len(), v.capacity()), (1, 8, 16))?;
        check_eq(v.get(7).map(|c| c.id), Some(7))?;
        check_eq(v.as_slice().iter().map(|c| c.id).sum::<u32>(), 28)?;
        drop(v);
        check_eq(drops.get(), 9)?;
        drop(kept);
        check_eq(drops.get(), 10)
    });
    case("case-5", || {
        // Small and over-aligned element types.
        let mut b = MyVec::new();
        let caps = push_all(&mut b, 1..=9u8);
        check_eq(caps, vec![4, 4, 4, 4, 8, 8, 8, 8, 16])?;
        check_eq(b.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9])?;
        let mut w = MyVec::new();
        let caps = push_all(&mut w, (0..6).map(Wide));
        check_eq(caps, vec![4, 4, 4, 4, 8, 8])?;
        check(w.as_slice().as_ptr() as usize % 64 == 0, "elements should be aligned")?;
        check_eq(w.pop(), Some(Wide(5)))?;
        check_eq(w.get(4), Some(&Wide(4)))?;
        check_eq(w.as_slice(), [Wide(0), Wide(1), Wide(2), Wide(3), Wide(4)])
    });
    case("case-6", || {
        // MyVecs inside a MyVec: moving and dropping elements that own memory.
        let mut outer = MyVec::new();
        for n in 0..6 {
            let mut inner = MyVec::new();
            for _ in 0..n {
                inner.push(s("x"));
            }
            outer.push(inner);
        }
        let popped = outer.pop();
        check_eq(popped.as_ref().map(|v| (v.len(), v.capacity())), Some((5, 8)))?;
        check_eq(outer.get(3).map(|v| (v.len(), v.capacity())), Some((3, 4)))?;
        check_eq(outer.get(0).map(|v| (v.len(), v.capacity())), Some((0, 0)))?;
        check_eq(outer.get(4).and_then(|v| v.get(3)), Some(&s("x")))?;
        check_eq((outer.len(), outer.capacity()), (5, 8))
    });
}
