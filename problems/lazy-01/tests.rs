use std::cell::Cell;

use crate::judge::{case, check, check_eq};
use crate::user::Lazy;

// Neither Clone, Copy nor Default.
#[derive(Debug, PartialEq)]
struct Token(String);

fn answer() -> u64 {
    42
}

pub fn run() {
    case("example-1", || {
        let name = String::from("Ferris");
        let mut greeting = Lazy::new(move || name + "!");
        check_eq(greeting.is_ready(), false)?;
        check_eq(greeting.get(), "Ferris!")?;
        check_eq(greeting.is_ready(), true)?;
        check_eq(greeting.get(), "Ferris!")?;
        check_eq(greeting.into_value(), "Ferris!")
    });
    case("example-2", || {
        let calls = Cell::new(0);
        let lazy = Lazy::new(|| {
            calls.set(calls.get() + 1);
            vec![1, 2, 3]
        });
        check_eq(calls.get(), 0)?;
        check_eq(lazy.into_value(), vec![1, 2, 3])?;
        check_eq(calls.get(), 1)
    });
    case("case-1", || {
        // Neither `new` nor `is_ready` calls init, and dropping an unused Lazy does not either.
        let calls = Cell::new(0);
        let word = String::from("unused");
        let lazy = Lazy::new(|| {
            calls.set(calls.get() + 1);
            word
        });
        check_eq(calls.get(), 0)?;
        check(!lazy.is_ready(), "a new Lazy should not be ready")?;
        check(!lazy.is_ready(), "is_ready should not compute the value")?;
        check_eq(calls.get(), 0)?;
        drop(lazy);
        check_eq(calls.get(), 0)
    });
    case("case-2", || {
        // The value is computed by the first `get`, not by `new`, and only once.
        let calls = Cell::new(0);
        let base = Cell::new(1);
        let mut lazy = Lazy::new(|| {
            calls.set(calls.get() + 1);
            base.get() * 10
        });
        base.set(7);
        check_eq(*lazy.get(), 70)?;
        base.set(9);
        check_eq(*lazy.get(), 70)?;
        check_eq(*lazy.get(), 70)?;
        check_eq(calls.get(), 1)?;
        check(lazy.is_ready(), "the Lazy should be ready after get")?;
        check_eq(lazy.into_value(), 70)?;
        check_eq(calls.get(), 1)
    });
    case("case-3", || {
        // A type that is not Clone, moved out by a closure that can be called only once.
        let mut lazy = Lazy::new(move || Token(String::from("abc")));
        check_eq(lazy.get(), &Token(String::from("abc")))?;
        check_eq(lazy.into_value(), Token(String::from("abc")))?;
        let token = Token(String::from("xyz"));
        let lazy = Lazy::new(move || token);
        check(!lazy.is_ready(), "a new Lazy should not be ready")?;
        check_eq(lazy.into_value(), Token(String::from("xyz")))
    });
    case("case-4", || {
        // A function, a value that borrows from a local, and a boxed closure.
        let mut a = Lazy::new(answer);
        check_eq(*a.get(), 42)?;
        check_eq(a.into_value(), 42)?;
        let text = String::from("alpha beta gamma");
        let mut b = Lazy::new(|| text.split(' ').nth(1).unwrap());
        check_eq(b.get(), &"beta")?;
        check_eq(b.into_value(), "beta")?;
        let owned = String::from("boxed");
        let boxed: Box<dyn FnOnce() -> String> = Box::new(move || owned.to_uppercase());
        let mut c = Lazy::new(boxed);
        check(!c.is_ready(), "a new Lazy should not be ready")?;
        check_eq(c.get(), "BOXED")?;
        check(c.is_ready(), "the Lazy should be ready after get")?;
        check_eq(c.into_value(), "BOXED")
    });
    case("case-5", || {
        // Each Lazy is independent, the value may be `()`, and a Lazy may use another one.
        let calls = Cell::new(0);
        let mut unit = Lazy::new(|| calls.set(calls.get() + 1));
        let mut other = Lazy::new(|| {
            calls.set(calls.get() + 10);
            Some(5u8)
        });
        check_eq(*unit.get(), ())?;
        check_eq(*unit.get(), ())?;
        check_eq(calls.get(), 1)?;
        check(unit.is_ready(), "the Lazy should be ready after get")?;
        check(!other.is_ready(), "using one Lazy should not compute another")?;
        check_eq(*other.get(), Some(5))?;
        check_eq(calls.get(), 11)?;
        let inner = Lazy::new(|| vec![String::from("x"), String::from("y")]);
        let mut outer = Lazy::new(move || inner.into_value().concat());
        check_eq(outer.get(), "xy")?;
        check_eq(outer.into_value(), "xy")
    });
}
