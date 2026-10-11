use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::DerefMut;

use crate::judge::{case, check, check_eq};
use crate::user::SortedVec;

/// Ordered by the number only; the letter tells equal keys apart.
#[derive(Debug)]
struct Key(u8, char);

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.0 == other.0
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn keys(pairs: &[(u8, char)]) -> Vec<Key> {
    pairs.iter().map(|&(n, c)| Key(n, c)).collect()
}

/// The letters of the keys, in order.
fn letters(keys: &[Key]) -> String {
    keys.iter().map(|k| k.1).collect()
}

fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn largest<T: Ord>(xs: &[T]) -> Option<&T> {
    xs.iter().max()
}

// Tells at compile time whether `S: DerefMut`, without requiring either answer to compile:
// the method of `HasDerefMut` is found first if it applies, otherwise the one of `NoDerefMut`.
struct Probe<S>(PhantomData<S>);

trait HasDerefMut {
    fn has_deref_mut(&self) -> bool {
        true
    }
}

impl<S: DerefMut> HasDerefMut for Probe<S> {}

trait NoDerefMut {
    fn has_deref_mut(&self) -> bool {
        false
    }
}

impl<S> NoDerefMut for &Probe<S> {}

pub fn run() {
    case("sample-1", || {
        let mut sv = SortedVec::from(vec![3, 1, 2]);
        check_eq(&*sv, [1, 2, 3])?;
        check_eq(sv.insert(2), 2)?;
        check_eq(&*sv, [1, 2, 2, 3])?;
        check_eq(sv.remove(&2), true)?;
        check_eq(&*sv, [1, 2, 3])?;
        check_eq(sv.remove(&5), false)?;
        check_eq(sv.len(), 3)?;
        check_eq(sv[0], 1)
    });
    case("sample-2", || {
        let mut sv = SortedVec::from(keys(&[(2, 'a'), (1, 'x'), (3, 'y')]));
        check_eq(letters(&sv), "xay")?;
        check_eq(sv.insert(Key(2, 'b')), 2)?;
        check_eq(letters(&sv), "xaby")?;
        check_eq(sv.remove(&Key(2, 'z')), true)?;
        check_eq(letters(&sv), "xby")
    });
    case("hidden-1", || {
        // An empty SortedVec.
        let mut sv: SortedVec<i32> = SortedVec::new();
        check_eq(sv.len(), 0)?;
        check(sv.is_empty(), "a new SortedVec should be empty")?;
        check_eq(sv.first(), None)?;
        check_eq(sv.remove(&1), false)?;
        check_eq(sv.insert(7), 0)?;
        check_eq(sv.remove(&7), true)?;
        check_eq(sv.remove(&7), false)?;
        check_eq(sv.into_vec(), Vec::<i32>::new())?;
        let sv = SortedVec::from(Vec::<i32>::new());
        check_eq(sv.len(), 0)?;
        check_eq(sv.last(), None)
    });
    case("hidden-2", || {
        // Each new key goes after the keys equal to it.
        let mut sv = SortedVec::new();
        check_eq(sv.insert(Key(2, 'a')), 0)?;
        check_eq(sv.insert(Key(2, 'b')), 1)?;
        check_eq(sv.insert(Key(1, 'c')), 0)?;
        check_eq(sv.insert(Key(2, 'd')), 3)?;
        check_eq(sv.insert(Key(3, 'e')), 4)?;
        check_eq(sv.insert(Key(1, 'f')), 1)?;
        check_eq(sv.insert(Key(0, 'g')), 0)?;
        check_eq(letters(&sv), "gcfabde")
    });
    case("hidden-3", || {
        // `from` keeps the input order of equal keys, also for a longer input.
        // The keys are lettered a-z, then A-Z, in input order.
        let input: Vec<Key> = "3141592653589793238462643383279502884197"
            .bytes()
            .zip(('a'..='z').chain('A'..='Z'))
            .map(|(d, c)| Key(d - b'0', c))
            .collect();
        let sv = SortedVec::from(input);
        check_eq(letters(&sv), "GbdLgqvCHajpryzBctxKeikFhuwnDNlsAIJfmoEM")?;
        let sv = SortedVec::from(keys(&[(5, 'a'), (5, 'b'), (5, 'c')]));
        check_eq(letters(&sv), "abc")
    });
    case("hidden-4", || {
        // `remove` takes the leftmost equal key, and only one.
        let mut sv = SortedVec::from(keys(&[(2, 'a'), (1, 'x'), (2, 'b'), (3, 'y'), (2, 'c')]));
        check_eq(letters(&sv), "xabcy")?;
        check_eq(sv.remove(&Key(2, '?')), true)?;
        check_eq(letters(&sv), "xbcy")?;
        check_eq(sv.remove(&Key(0, '?')), false)?;
        check_eq(sv.remove(&Key(4, '?')), false)?;
        check_eq(letters(&sv), "xbcy")?;
        check_eq(sv.remove(&Key(2, '?')), true)?;
        check_eq(letters(&sv), "xcy")?;
        check_eq(sv.remove(&Key(3, '?')), true)?;
        check_eq(sv.remove(&Key(2, '?')), true)?;
        check_eq(sv.remove(&Key(2, '?')), false)?;
        check_eq(letters(&sv), "x")?;
        // A value between two elements is absent too.
        let mut nums = SortedVec::from(vec![1, 3]);
        check_eq(nums.remove(&2), false)?;
        check_eq(&*nums, [1, 3])
    });
    case("hidden-5", || {
        // The slice API through deref, after a sequence of inserts.
        let mut sv = SortedVec::new();
        let mut indices = Vec::new();
        for x in [5, -3, 8, 5, 0, 12, -3, 7, 5, 1] {
            indices.push(sv.insert(x));
        }
        check_eq(indices, [0, 0, 2, 2, 1, 5, 1, 5, 5, 3])?;
        check(sv.windows(2).all(|w| w[0] <= w[1]), "elements are not in ascending order")?;
        check_eq(&*sv, [-3, -3, 0, 1, 5, 5, 5, 7, 8, 12])?;
        check_eq(total(&sv), 37)?;
        check_eq((sv.first(), sv.last()), (Some(&-3), Some(&12)))?;
        check_eq(sv[sv.len() - 1], 12)?;
        check_eq(&sv[2..4], [0, 1])?;
        check(sv.contains(&7) && !sv.contains(&6), "contains is wrong")?;
        check_eq(sv.binary_search(&8), Ok(8))?;
        check_eq(sv.iter().filter(|&&x| x == 5).count(), 3)?;
        check_eq(sv.remove(&5), true)?;
        check_eq(sv.into_vec(), [-3, -3, 0, 1, 5, 5, 7, 8, 12])
    });
    case("hidden-6", || {
        // Strings, and a generic function that takes a slice.
        let mut words = SortedVec::from(vec![
            String::from("pear"),
            String::from("apple"),
            String::from("fig"),
        ]);
        check_eq(words.insert(String::from("banana")), 1)?;
        check_eq(words.insert(String::from("zucchini")), 4)?;
        check_eq(largest(&words).map(String::as_str), Some("zucchini"))?;
        check(words.contains(&String::from("fig")), "fig should be there")?;
        check_eq(words.remove(&String::from("apple")), true)?;
        check_eq(words.remove(&String::from("grape")), false)?;
        check_eq(words.into_vec(), ["banana", "fig", "pear", "zucchini"])
    });
    case("hidden-7", || {
        // SortedVec gives no mutable access to its elements.
        check(
            !(&Probe::<SortedVec<i32>>(PhantomData)).has_deref_mut(),
            "SortedVec must not implement DerefMut",
        )
    });
}
