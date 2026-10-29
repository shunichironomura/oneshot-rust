use std::sync::atomic::{AtomicUsize, Ordering};

use crate::judge::{case, check_eq};
use crate::user::Bag;

/// How many times a `Ticket` has been cloned so far.
static CLONES: AtomicUsize = AtomicUsize::new(0);

fn clones() -> usize {
    CLONES.load(Ordering::Relaxed)
}

/// An element that counts its clones.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Ticket(u32);

impl Clone for Ticket {
    fn clone(&self) -> Self {
        CLONES.fetch_add(1, Ordering::Relaxed);
        Ticket(self.0)
    }
}

/// The `(element, count)` pairs of a borrowed bag, read with a `for` loop.
fn pairs<T: Copy>(bag: &Bag<T>) -> Vec<(T, usize)> {
    let mut out = Vec::new();
    for (&x, n) in bag {
        out.push((x, n));
    }
    out
}

fn ids(items: &[Option<Ticket>]) -> Vec<Option<u32>> {
    items.iter().map(|t| t.as_ref().map(|t| t.0)).collect()
}

pub fn run() {
    case("example-1", || {
        let mut b: Bag<char> = "hello".chars().collect();
        check_eq((b.count(&'l'), b.len()), (2, 5))?;
        b.extend("lo".chars());
        b.extend(&['x', 'x']);
        check_eq(pairs(&b), vec![('e', 1), ('h', 1), ('l', 3), ('o', 2), ('x', 2)])?;
        check_eq(b.into_iter().collect::<String>(), "ehlllooxx")
    });
    case("example-2", || {
        let b: Bag<i32> = (1..=3).chain(2..=3).collect();
        check_eq((b.count(&3), b.len()), (2, 5))?;
        check_eq((&b).into_iter().next_back(), Some((&3, 2)))?;
        check_eq(b.into_iter().rev().collect::<Vec<_>>(), vec![3, 3, 2, 2, 1])?;
        check_eq(Bag::<i32>::new().into_iter().next(), None)
    });
    case("case-1", || {
        // Empty bags, and elements that were never added.
        let empty: Bag<i32> = Bag::new();
        check_eq((empty.len(), empty.count(&0)), (0, 0))?;
        check_eq(pairs(&empty), Vec::new())?;
        check_eq((&empty).into_iter().next_back(), None)?;
        check_eq(empty.into_iter().next_back(), None)?;
        let none: Bag<i32> = Vec::new().into_iter().collect();
        check_eq((none.len(), none.count(&7), pairs(&none)), (0, 0, Vec::new()))
    });
    case("case-2", || {
        // String elements.
        let mut b: Bag<String> = ["to", "be"].into_iter().map(String::from).collect();
        b.extend(vec![String::from("be"), String::from("a")]);
        check_eq((b.count(&String::from("be")), b.count(&String::from("Be")), b.len()), (2, 0, 4))?;
        let got: Vec<(&str, usize)> = (&b).into_iter().map(|(s, n)| (s.as_str(), n)).collect();
        check_eq(got, vec![("a", 1), ("be", 2), ("to", 1)])?;
        check_eq(b.into_iter().collect::<Vec<String>>(), vec!["a", "be", "be", "to"])
    });
    case("case-3", || {
        // Creating the owned iterator clones nothing. Using it up, from either end, clones each
        // element count - 1 times.
        let b: Bag<Ticket> =
            vec![Ticket(5), Ticket(9), Ticket(5), Ticket(9), Ticket(7)].into_iter().collect();
        let mut seen = Vec::new();
        for (t, n) in &b {
            seen.push((t.0, n));
        }
        check_eq(seen, vec![(5, 2), (7, 1), (9, 2)])?;
        let before = clones();
        let mut it = b.into_iter();
        check_eq(clones() - before, 0)?;
        let got = [
            it.next_back(),
            it.next(),
            it.next_back(),
            it.next_back(),
            it.next_back(),
            it.next(),
            it.next_back(),
        ];
        check_eq(ids(&got), vec![Some(9), Some(5), Some(9), Some(7), Some(5), None, None])?;
        check_eq(clones() - before, 2)
    });
    case("case-4", || {
        // Extending with references to Copy elements.
        let mut b: Bag<i32> = Bag::new();
        let v = vec![5, -3, 5];
        b.extend(&v);
        b.extend(v[1..].iter());
        check_eq(pairs(&b), vec![(-3, 2), (5, 3)])?;
        check_eq(b.len(), 5)
    });
    case("case-5", || {
        // Borrowed iteration: repeatable, reversible, and the bag stays usable.
        let mut b: Bag<i32> = vec![3, -1, 3, 0, -1].into_iter().collect();
        let back: Vec<(i32, usize)> = (&b).into_iter().rev().map(|(&x, n)| (x, n)).collect();
        check_eq(back, vec![(3, 2), (0, 1), (-1, 2)])?;
        let mut it = (&b).into_iter();
        check_eq(
            (it.next(), it.next_back(), it.next(), it.next_back(), it.next()),
            (Some((&-1, 2)), Some((&3, 2)), Some((&0, 1)), None, None),
        )?;
        b.extend([0]);
        check_eq((b.count(&0), b.len()), (2, 6))
    });
    case("case-6", || {
        // Extending a collected bag, and mixing next and next_back on the owned iterator.
        let mut b: Bag<char> = "ba".chars().collect();
        b.extend("b".chars());
        b.extend("".chars());
        check_eq((b.count(&'a'), b.count(&'b'), b.len()), (1, 2, 3))?;
        let mut it = b.into_iter();
        check_eq(
            (it.next_back(), it.next(), it.next(), it.next(), it.next_back()),
            (Some('b'), Some('a'), Some('b'), None, None),
        )?;
        let mut it = "xxx".chars().collect::<Bag<char>>().into_iter();
        check_eq(
            (it.next(), it.next_back(), it.next_back(), it.next(), it.next_back()),
            (Some('x'), Some('x'), Some('x'), None, None),
        )
    });
}
