use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};

use crate::judge::{case, check, check_eq};
use crate::user::insert_sorted;

thread_local! {
    /// How many times each `Tracked` value has been dropped, 4 bits per id.
    static DROPS: Cell<u64> = const { Cell::new(0) };
}

fn drops() -> u64 {
    DROPS.with(Cell::get)
}

/// A value without heap memory that counts how many times it is dropped, so that a double drop
/// or a leak shows up in `DROPS` even where Miri has nothing to report.
struct Tracked {
    key: u32,
    id: u32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        DROPS.with(|d| d.set(d.get() + (1u64 << (4 * self.id))));
    }
}

/// The id of the value inserted by `trial`. The other values have the ids 0, 1, 2, ...
const X: u32 = 15;

/// A trap that `trial` never hits.
const NONE: u32 = 1000;

/// Panics from inside `less`. `resume_unwind` unwinds exactly like `panic!` but skips the panic
/// hook, which keeps the tests fast under Miri.
fn fail() -> ! {
    panic::resume_unwind(Box::new("less panicked"))
}

/// Runs `f` and tells whether it panicked.
fn panics(f: impl FnOnce()) -> bool {
    panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

fn by_key(a: &(u32, char), b: &(u32, char)) -> bool {
    a.0 < b.0
}

/// Inserts `x` into the strings `v` with a `less` that compares them, but panics when
/// `boom(n, a, b)` is true for its `n`-th call `less(a, b)` (counting from 1). The panic must
/// reach the caller, and the vector must then hold the strings of `v` in the same order, plus `x`
/// at most once. (`x` is not in `v`.)
fn string_panic(v: &[&str], x: &str, boom: impl Fn(usize, &str, &str) -> bool) -> Result<(), String> {
    let mut got: Vec<String> = v.iter().map(|s| s.to_string()).collect();
    let mut calls = 0;
    let panicked = panics(|| {
        insert_sorted(&mut got, x.to_string(), |a, b| {
            calls += 1;
            if boom(calls, a, b) {
                fail();
            }
            a < b
        })
    });
    check(panicked, "the panic of less did not reach the caller")?;
    check(got.iter().filter(|s| *s == x).count() <= 1, "x is in the vector twice")?;
    let rest: Vec<&str> = got.iter().map(String::as_str).filter(|s| *s != x).collect();
    check_eq(rest, v.to_vec())
}

/// Inserts a value with the key `key` and the id `X` into values with the keys `keys` and the
/// ids 0, 1, 2, ... `less` compares keys, but panics when one of its arguments has the id `trap`.
/// Checks every rule of the statement and returns whether `insert_sorted` panicked.
fn trial(keys: &[u32], key: u32, trap: u32) -> Result<bool, String> {
    DROPS.with(|d| d.set(0));
    let mut v = Vec::new();
    for (id, &key) in (0..).zip(keys) {
        v.push(Tracked { key, id });
    }
    let panicked = panics(|| {
        insert_sorted(&mut v, Tracked { key, id: X }, |a, b| {
            if a.id == trap || b.id == trap {
                fail();
            }
            a.key < b.key
        })
    });
    let mut ids: Vec<u32> = v.iter().map(|t| t.id).collect();
    let x_pos = ids.iter().position(|&id| id == X);
    if !panicked {
        // The new value goes after every value with a key not greater than its own.
        let pos = keys.iter().take_while(|&&k| k <= key).count();
        check_eq(x_pos, Some(pos))?;
    }
    // Apart from the new value, there are the original values, once each, in their order.
    if let Some(pos) = x_pos {
        ids.remove(pos);
    }
    let in_order = ids.len() == keys.len() && ids.iter().zip(0..).all(|(&id, i)| id == i);
    check(in_order, "the original values are not all there once each in their order")?;
    // Nothing has been dropped yet, except the new value if it is not in the vector.
    check_eq(drops(), if x_pos.is_some() { 0 } else { 1u64 << (4 * X) })?;
    // Dropping the vector drops each of its values once.
    drop(v);
    let all = (0..keys.len()).fold(1u64 << (4 * X), |acc, id| acc | 1 << (4 * id));
    check_eq(drops(), all)?;
    Ok(panicked)
}

/// `trial` where the panic is certain: every correct `insert_sorted` has to compare the new value
/// with the value `trap` (or with any value, if `trap` is `X`).
fn must_panic(keys: &[u32], key: u32, trap: u32) -> Result<(), String> {
    check(trial(keys, key, trap)?, "the panic of less did not reach the caller")
}

pub fn run() {
    case("sample-1", || {
        let mut v = vec!["b".to_string(), "d".to_string()];
        insert_sorted(&mut v, "c".to_string(), |a, b| a < b);
        check_eq(v, ["b", "c", "d"])?;
        let mut v = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        insert_sorted(&mut v, (2, 'x'), |a, b| a.0 < b.0);
        check_eq(v, [(1, 'a'), (2, 'b'), (2, 'x'), (3, 'c')])
    });
    case("sample-2", || string_panic(&["b", "d"], "c", |n, _, _| n == 2));
    case("hidden-1", || {
        // At the front, at the end, into an empty vector, and with a descending order.
        let mut v = vec![2, 4, 6];
        insert_sorted(&mut v, 1, |a, b| a < b);
        insert_sorted(&mut v, 9, |a, b| a < b);
        check_eq(v, [1, 2, 4, 6, 9])?;
        let mut v: Vec<i32> = Vec::new();
        insert_sorted(&mut v, 5, |a, b| a < b);
        check_eq(v, [5])?;
        let mut v = vec![9, 7, 3];
        for x in [8, 1, 10, 3] {
            insert_sorted(&mut v, x, |a, b| a > b);
        }
        check_eq(v, [10, 9, 8, 7, 3, 3, 1])
    });
    case("hidden-2", || {
        // Equal values: the new one goes after all of them.
        let mut v = vec![(1, 'a'), (2, 'b'), (2, 'c'), (3, 'd')];
        insert_sorted(&mut v, (2, 'x'), by_key);
        insert_sorted(&mut v, (1, 'y'), by_key);
        insert_sorted(&mut v, (3, 'z'), by_key);
        check_eq(v, [(1, 'a'), (1, 'y'), (2, 'b'), (2, 'c'), (2, 'x'), (3, 'd'), (3, 'z')])?;
        let mut v: Vec<(u32, char)> = "abcd".chars().map(|c| (5, c)).collect();
        insert_sorted(&mut v, (5, 'x'), by_key);
        check_eq(v.last(), Some(&(5, 'x')))?;
        let mut v = Vec::new();
        for item in [(3, 'a'), (1, 'b'), (3, 'c'), (2, 'd'), (1, 'e'), (3, 'f'), (2, 'g')] {
            insert_sorted(&mut v, item, by_key);
        }
        check_eq(v, [(1, 'b'), (1, 'e'), (2, 'd'), (2, 'g'), (3, 'a'), (3, 'c'), (3, 'f')])
    });
    case("hidden-3", || {
        // less panics on its first call.
        string_panic(&["a", "b", "c"], "bb", |n, _, _| n == 1)?;
        must_panic(&[10, 20, 30], 35, X)
    });
    case("hidden-4", || {
        // less panics in the middle of the work (when it gets "a", which every correct solution
        // compares with "b"), also among equal values.
        string_panic(&["a", "c", "e", "g", "i"], "b", |_, a, b| a == "a" || b == "a")?;
        must_panic(&[1, 2, 2, 2, 3], 2, 3)
    });
    case("hidden-5", || {
        // A panic when one of the two neighbors of the new value takes part in a comparison.
        let keys = [10, 20, 30, 40];
        must_panic(&keys, 25, 1)?;
        must_panic(&keys, 25, 2)
    });
    case("hidden-6", || {
        // Without a panic, every value is dropped exactly once.
        check(!trial(&[], 7, NONE)?, "less panicked")?;
        check(!trial(&[1, 3, 3, 3, 4], 3, NONE)?, "less panicked")
    });
}
