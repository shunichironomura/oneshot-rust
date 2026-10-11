use std::cell::Cell;

use crate::judge::{case, check, check_eq};
use crate::user::{Handle, fan_out};

/// Implements neither `Clone` nor `Default` on purpose.
struct Counter {
    n: u32,
}

/// Sets the flag when it is dropped. Not `Clone` either, and it borrows.
struct Probe<'a> {
    dropped: &'a Cell<bool>,
}

impl Drop for Probe<'_> {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

/// Clones in generic code, where nothing is known about `T`.
fn twins<T>(h: &Handle<T>) -> (Handle<T>, Handle<T>) {
    (h.clone(), h.share())
}

pub fn run() {
    case("sample-1", || {
        let h = Handle::new(Counter { n: 0 });
        let v = fan_out(&h, 2);
        v[0].update(|c| c.n += 5);
        check_eq(h.read(|c| c.n), 5)?;
        check_eq(h.handles(), 3)
    });
    case("sample-2", || {
        let a = Handle::new(Counter { n: 1 });
        let b = a.clone();
        b.update(|c| c.n *= 10);
        check_eq(a.read(|c| c.n), 10)?;
        check_eq(a.handles(), 2)?;
        drop(b);
        check_eq(a.handles(), 1)
    });
    case("hidden-1", || {
        // `share` and `clone` (also from generic code) give handles to the same value,
        // which outlives the original handle.
        let h = Handle::new(Counter { n: 2 });
        let s = h.share();
        let (a, b) = twins(&s);
        a.update(|c| c.n += 3);
        b.update(|c| c.n *= 4);
        check_eq(h.read(|c| c.n), 20)?;
        check_eq((h.handles(), s.handles()), (4, 4))?;
        drop(h);
        drop(a);
        check_eq(s.handles(), 2)?;
        check_eq(b.read(|c| c.n), 20)
    });
    case("hidden-2", || {
        // `fan_out` with 0 and with several handles; dropping them lowers the count again.
        let h = Handle::new(Counter { n: 0 });
        let none = fan_out(&h, 0);
        check(none.is_empty(), "fan_out(&h, 0) should be empty")?;
        check_eq(h.handles(), 1)?;
        let v = fan_out(&h, 4);
        check_eq(v.len(), 4)?;
        for (i, x) in v.iter().enumerate() {
            x.update(|c| c.n += i as u32 + 1);
        }
        check_eq(h.read(|c| c.n), 10)?;
        check_eq(v[3].handles(), 5)?;
        drop(v);
        check_eq(h.handles(), 1)
    });
    case("hidden-3", || {
        // `Default` makes a new, independent value. Even when `T` is `Clone`, cloning a handle
        // shares the value instead of copying it.
        let a: Handle<Vec<u32>> = Handle::default();
        let b = Handle::<Vec<u32>>::default();
        let c = a.clone();
        c.update(|v| v.push(3));
        a.share().update(|v| v.push(4));
        check_eq(a.read(|v| v.clone()), vec![3, 4])?;
        check_eq(b.read(|v| v.len()), 0)?;
        check_eq((a.handles(), b.handles()), (2, 1))
    });
    case("hidden-4", || {
        // The value is dropped together with its last handle, not before.
        let dropped = Cell::new(false);
        let h = Handle::new(Probe { dropped: &dropped });
        let v = fan_out(&h, 2);
        drop(h);
        let w = v[1].share();
        drop(v);
        check(!dropped.get(), "the value was dropped while a handle was alive")?;
        check_eq(w.handles(), 1)?;
        drop(w);
        check(dropped.get(), "the value was not dropped with its last handle")?;
        // A handle to a handle: `read` returns a clone of the inner handle.
        let inner = Handle::new(Counter { n: 1 });
        let outer = Handle::new(inner.share());
        let copy = outer.read(|h| h.clone());
        copy.update(|c| c.n = 9);
        check_eq(inner.read(|c| c.n), 9)?;
        check_eq((inner.handles(), outer.handles()), (3, 1))
    });
}
