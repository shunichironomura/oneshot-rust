use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::MyArc;

/// Adds 1 to `drops` when dropped. It does not implement `Clone`.
struct Counted<'a> {
    id: u32,
    drops: &'a AtomicUsize,
}

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

/// A log that threads append to under its lock. When dropped, it reports how many entries it
/// holds: the last handle must see every entry added through the other handles.
struct Log<'a> {
    name: String,
    entries: Mutex<Vec<u32>>,
    reports: &'a Mutex<Vec<usize>>,
}

impl Drop for Log<'_> {
    fn drop(&mut self) {
        let len = self.entries.get_mut().map_or(0, |v| v.len());
        if let Ok(mut reports) = self.reports.lock() {
            reports.push(len);
        }
    }
}

/// Covariance: a handle to a longer-lived value can be used as one to a shorter-lived value.
fn shorten<'a>(a: MyArc<&'static str>) -> MyArc<&'a str> {
    a
}

// `send_sync!(T)` is `(T: Send, T: Sync)`, decided at compile time without requiring either:
// method lookup tries the `Yes` impls on `Probe<T>` first, and falls back to the `No` impls on
// `&Probe<T>` when their bound does not hold.
struct Probe<T>(PhantomData<T>);

trait SendYes {
    fn send(&self) -> bool {
        true
    }
}
impl<T: Send> SendYes for Probe<T> {}

trait SendNo {
    fn send(&self) -> bool {
        false
    }
}
impl<T> SendNo for &Probe<T> {}

trait SyncYes {
    fn sync(&self) -> bool {
        true
    }
}
impl<T: Sync> SyncYes for Probe<T> {}

trait SyncNo {
    fn sync(&self) -> bool {
        false
    }
}
impl<T> SyncNo for &Probe<T> {}

macro_rules! send_sync {
    ($t:ty) => {{
        let probe = &Probe::<$t>(PhantomData);
        (probe.send(), probe.sync())
    }};
}

pub fn run() {
    case("sample-1", || {
        let a = MyArc::new(String::from("hello"));
        let b = MyArc::clone(&a);
        check_eq((b.as_str(), a.len()), ("hello", 5))?;
        check_eq(MyArc::strong_count(&a), 2)?;
        check(MyArc::ptr_eq(&a, &b), "ptr_eq(&a, &b) should be true")?;
        drop(a);
        check_eq(MyArc::strong_count(&b), 1)?;
        let c = MyArc::new(String::from("hello"));
        check(!MyArc::ptr_eq(&b, &c), "ptr_eq(&b, &c) should be false")
    });
    case("sample-2", || {
        let mut v = MyArc::new(vec![1, 2]);
        MyArc::get_mut(&mut v).ok_or("get_mut returned None")?.push(3);
        let w = MyArc::clone(&v);
        check(MyArc::get_mut(&mut v).is_none(), "get_mut should be None while w is alive")?;
        drop(w);
        MyArc::get_mut(&mut v).ok_or("get_mut returned None after drop(w)")?.push(4);
        check_eq(v.as_slice(), [1, 2, 3, 4])
    });
    case("sample-3", || {
        let words = MyArc::new(vec!["a", "bb", "ccc"]);
        let (n, joined) = thread::scope(|s| {
            let mine = MyArc::clone(&words);
            let t1 = s.spawn(move || mine.len());
            let t2 = s.spawn(|| words.concat());
            (t1.join().unwrap(), t2.join().unwrap())
        });
        check_eq((n, joined.as_str()), (3, "abbccc"))?;
        check_eq(MyArc::strong_count(&words), 1)
    });
    case("hidden-1", || {
        // The value is dropped exactly once, with the last handle. `Counted` is not `Clone`.
        let drops = AtomicUsize::new(0);
        let dropped = || drops.load(Ordering::Relaxed);
        let a = MyArc::new(Counted { id: 5, drops: &drops });
        let b = MyArc::clone(&a);
        let c = MyArc::clone(&b);
        check_eq((MyArc::strong_count(&a), MyArc::strong_count(&c), c.id), (3, 3, 5))?;
        drop(a);
        check_eq((MyArc::strong_count(&b), dropped()), (2, 0))?;
        drop(b);
        let mut c = c;
        check_eq((MyArc::strong_count(&c), dropped()), (1, 0))?;
        MyArc::get_mut(&mut c).ok_or("get_mut returned None for the last handle")?.id = 6;
        check_eq(c.id, 6)?;
        drop(c);
        check_eq(dropped(), 1)?;
        drop(MyArc::new(Counted { id: 0, drops: &drops }));
        check_eq(dropped(), 2)?;
        // Separate values are never the same, even when they are zero-sized.
        let x = MyArc::new(());
        let y = MyArc::new(());
        let z = MyArc::clone(&x);
        check_eq(
            (MyArc::ptr_eq(&x, &y), MyArc::ptr_eq(&z, &x), MyArc::ptr_eq(&x, &x)),
            (false, true, true),
        )
    });
    case("hidden-2", || {
        // `MyArc<T>` is `Send` and `Sync` exactly when `T` is both.
        check_eq(send_sync!(MyArc<u8>), (true, true))?;
        check_eq(send_sync!(MyArc<Mutex<Cell<u8>>>), (true, true))?;
        check_eq(send_sync!(MyArc<Cell<u8>>), (false, false))?;
        check_eq(send_sync!(MyArc<MutexGuard<'static, u8>>), (false, false))?;
        check_eq(send_sync!(MyArc<Rc<u8>>), (false, false))
    });
    case("hidden-3", || {
        // Covariance: handles to `&'static str` mix with handles to shorter-lived `&str`.
        let greeting: MyArc<&'static str> = MyArc::new("hello");
        let local = String::from("local");
        let mut all = vec![MyArc::new(local.as_str())];
        all.push(shorten(MyArc::clone(&greeting)));
        all.push(MyArc::clone(&greeting));
        let texts: Vec<&str> = all.iter().map(|a| **a).collect();
        check_eq(texts, ["local", "hello", "hello"])?;
        check(MyArc::ptr_eq(&all[1], &greeting), "a shortened handle is still the same value")?;
        check_eq(MyArc::strong_count(&greeting), 3)
    });
    case("hidden-4", || {
        // Two threads use the value and drop their handles at the same time. Whichever is last
        // drops the value, and its destructor sees the entries added through the other handle.
        let reports = Mutex::new(Vec::new());
        let log = MyArc::new(Log {
            name: String::from("log"),
            entries: Mutex::new(Vec::new()),
            reports: &reports,
        });
        let work = |h: MyArc<Log<'_>>, t: u32| {
            for i in 0..3 {
                if let Ok(mut entries) = h.entries.lock() {
                    entries.push(t * 10 + i);
                }
            }
            let named = h.name == "log";
            drop(h);
            named
        };
        let named = thread::scope(|s| {
            let h = MyArc::clone(&log);
            let other = s.spawn(move || work(h, 1));
            work(log, 0) && other.join().unwrap()
        });
        check(named, "both threads should read the name")?;
        check_eq(reports.into_inner().unwrap_or_default(), [6])
    });
    case("hidden-5", || {
        // get_mut succeeds once the other thread has finished with its handle.
        let mut a = MyArc::new(vec![1u32, 2, 3]);
        let sum = thread::scope(|s| {
            let b = MyArc::clone(&a);
            let t = s.spawn(move || b.iter().sum::<u32>());
            loop {
                if let Some(v) = MyArc::get_mut(&mut a) {
                    v.push(4);
                    break;
                }
                thread::yield_now();
            }
            t.join().unwrap()
        });
        check_eq(sum, 6)?;
        check_eq((a.as_slice(), MyArc::strong_count(&a)), (&[1, 2, 3, 4][..], 1))
    });
    case("hidden-6", || {
        // Handles cloned on two threads at the same time, and sent back from the other thread.
        let drops = AtomicUsize::new(0);
        let a = MyArc::new(Counted { id: 7, drops: &drops });
        let clone_some = || {
            let mut handles = Vec::new();
            for _ in 0..6 {
                handles.push(MyArc::clone(&a));
                thread::yield_now();
            }
            handles
        };
        let all = thread::scope(|s| {
            let other = s.spawn(clone_some);
            let mut mine = clone_some();
            mine.extend(other.join().unwrap());
            mine
        });
        check_eq((MyArc::strong_count(&a), all.len()), (13, 12))?;
        check(all.iter().all(|h| MyArc::ptr_eq(h, &a) && h.id == 7), "all handles share one value")?;
        drop(a);
        check_eq((MyArc::strong_count(&all[11]), drops.load(Ordering::Relaxed)), (12, 0))?;
        drop(all);
        check_eq(drops.load(Ordering::Relaxed), 1)
    });
}
