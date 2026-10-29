use std::cell::RefCell;

use crate::judge::{case, check, check_eq};
use crate::user::Ring;

/// Implements none of `Copy`, `Clone`, `Default`, `Debug` and `PartialEq`.
struct Ticket(u32);

/// Records its id in `log` when it is dropped.
struct Noisy<'a> {
    id: u32,
    log: &'a RefCell<Vec<u32>>,
}

impl Drop for Noisy<'_> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.id);
    }
}

pub fn run() {
    case("example-1", || {
        let mut r: Ring<i32, 2> = Ring::new();
        check_eq(r.push(1), None)?;
        check_eq(r.push(2), None)?;
        check_eq(r.push(3), Some(1))?;
        check_eq((r.len(), r.is_full()), (2, true))?;
        check_eq(r.iter().collect::<Vec<_>>(), vec![&2, &3])?;
        check_eq(r.pop(), Some(2))?;
        check_eq(Ring::<i32, 3>::CAPACITY, 3)
    });
    case("case-1", || {
        // Capacity 1: every push to the full ring evicts its only element.
        let mut r: Ring<char, 1> = Ring::new();
        check_eq((r.len(), r.is_full()), (0, false))?;
        check_eq(r.push('a'), None)?;
        check_eq((r.len(), r.is_full()), (1, true))?;
        check_eq(r.push('b'), Some('a'))?;
        check_eq(r.push('c'), Some('b'))?;
        check_eq(r.iter().collect::<Vec<_>>(), vec![&'c'])?;
        check_eq(r.pop(), Some('c'))?;
        check_eq(r.pop(), None)?;
        check_eq((r.len(), r.is_full()), (0, false))?;
        check_eq(r.push('d'), None)?;
        check_eq(r.iter().collect::<Vec<_>>(), vec![&'d'])
    });
    case("case-2", || {
        // Popping an empty ring returns None and leaves the ring usable.
        let mut r: Ring<u8, 3> = Ring::new();
        check_eq(r.pop(), None)?;
        check_eq(r.iter().count(), 0)?;
        check_eq(r.push(1), None)?;
        check_eq(r.pop(), Some(1))?;
        check_eq(r.pop(), None)?;
        check_eq(r.pop(), None)?;
        check_eq((r.len(), r.is_full()), (0, false))?;
        check_eq(r.iter().count(), 0)?;
        check_eq(r.push(2), None)?;
        check_eq(r.push(3), None)?;
        check_eq(r.push(4), None)?;
        check_eq(r.push(5), Some(2))?;
        check_eq(r.iter().collect::<Vec<_>>(), vec![&3, &4, &5])
    });
    case("case-3", || {
        // Strings, with pops and pushes around the end of the array.
        let mut r: Ring<String, 3> = Ring::new();
        let evicted: Vec<Option<String>> =
            ["a", "b", "c", "d", "e"].into_iter().map(|s| r.push(s.to_string())).collect();
        check_eq(evicted, vec![None, None, None, Some("a".to_string()), Some("b".to_string())])?;
        check_eq(r.iter().map(String::as_str).collect::<Vec<_>>(), vec!["c", "d", "e"])?;
        check_eq(r.pop().as_deref(), Some("c"))?;
        check_eq((r.len(), r.is_full()), (2, false))?;
        check_eq(r.push("f".to_string()), None)?;
        check_eq((r.len(), r.is_full()), (3, true))?;
        check_eq(r.push("g".to_string()).as_deref(), Some("d"))?;
        check_eq(r.iter().map(String::as_str).collect::<Vec<_>>(), vec!["e", "f", "g"])?;
        // iter only borrows the ring.
        check_eq(r.iter().count(), 3)?;
        check_eq(r.pop().as_deref(), Some("e"))?;
        check_eq(r.pop().as_deref(), Some("f"))?;
        check_eq(r.push("h".to_string()), None)?;
        check_eq(r.iter().map(String::as_str).collect::<Vec<_>>(), vec!["g", "h"])
    });
    case("case-4", || {
        // Pushes and pops that wrap around the array several times. The values pushed are 0, 1,
        // 2, ..., so the ring always holds the values `oldest..next`.
        let mut r: Ring<u32, 5> = Ring::new();
        let (mut oldest, mut next) = (0u32, 0u32);
        let mut seed: u32 = 2025;
        for step in 0..45u32 {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            // Phases that mostly push alternate with phases that mostly pop.
            let push_percent = if (step / 9) % 2 == 0 { 75 } else { 25 };
            if (seed >> 16) % 100 < push_percent {
                let evicts = next - oldest == 5;
                check_eq(r.push(next), evicts.then_some(oldest))?;
                if evicts {
                    oldest += 1;
                }
                next += 1;
            } else {
                let non_empty = oldest < next;
                check_eq(r.pop(), non_empty.then_some(oldest))?;
                if non_empty {
                    oldest += 1;
                }
            }
            check_eq(r.len(), (next - oldest) as usize)?;
            check_eq(r.is_full(), next - oldest == 5)?;
            check(r.iter().copied().eq(oldest..next), "iter does not match")?;
        }
        Ok(())
    });
    case("case-5", || {
        // An element type that implements none of Copy, Clone, Default, Debug and PartialEq.
        let mut r: Ring<Ticket, 4> = Ring::new();
        let evicted: Vec<Option<u32>> = (1..=6).map(|id| r.push(Ticket(id)).map(|t| t.0)).collect();
        check_eq(evicted, vec![None, None, None, None, Some(1), Some(2)])?;
        check_eq(r.iter().map(|t| t.0).collect::<Vec<_>>(), vec![3, 4, 5, 6])?;
        let popped: Vec<Option<u32>> = (0..6).map(|_| r.pop().map(|t| t.0)).collect();
        check_eq(popped, vec![Some(3), Some(4), Some(5), Some(6), None, None])?;
        check_eq(r.len(), 0)
    });
    case("case-6", || {
        // An element type that borrows a local: the evicted element goes to the caller, and the
        // others are dropped with the ring.
        let log = RefCell::new(Vec::new());
        let mut r: Ring<Noisy<'_>, 2> = Ring::new();
        check(r.push(Noisy { id: 1, log: &log }).is_none(), "push 1 should return None")?;
        check(r.push(Noisy { id: 2, log: &log }).is_none(), "push 2 should return None")?;
        let evicted = r.push(Noisy { id: 3, log: &log });
        check_eq(evicted.as_ref().map(|n| n.id), Some(1))?;
        check_eq(log.borrow().len(), 0)?;
        drop(evicted);
        check_eq(log.borrow().clone(), vec![1])?;
        check_eq(r.iter().map(|n| n.id).collect::<Vec<_>>(), vec![2, 3])?;
        drop(r);
        let mut dropped = log.borrow().clone();
        dropped.sort();
        check_eq(dropped, vec![1, 2, 3])
    });
    case("case-7", || {
        // A larger ring, whose elements live in the struct itself.
        let mut r: Ring<u16, 40> = Ring::new();
        for x in 0..50u16 {
            check_eq(r.push(x), x.checked_sub(40))?;
        }
        check_eq((r.len(), r.is_full()), (40, true))?;
        check(r.iter().copied().eq(10..50), "iter does not match")?;
        check(
            std::mem::size_of::<Ring<u8, 64>>() >= 64 * std::mem::size_of::<Option<u8>>(),
            "the elements must be stored in the array `buf` inside the struct",
        )?;
        check_eq(Ring::<String, 7>::CAPACITY, 7)
    });
}
