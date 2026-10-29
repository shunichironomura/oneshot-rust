use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, mpsc};

use crate::judge::{case, check_eq};
use crate::user::par_map;

pub fn run() {
    case("example-1", || {
        let calls = AtomicUsize::new(0);
        let prefix = String::from("item-");
        let names = vec![String::from("a"), String::from("bb"), String::from("ccc")];
        let labels = par_map(names, |name| {
            calls.fetch_add(1, Ordering::Relaxed);
            format!("{prefix}{name}")
        });
        check_eq(labels, ["item-a", "item-bb", "item-ccc"])?;
        check_eq(calls.load(Ordering::Relaxed), 3)
    });
    case("case-1", || {
        let no_bytes: Vec<Vec<u8>> = Vec::new();
        check_eq(par_map(no_bytes, |bytes| bytes.len()), Vec::<usize>::new())?;
        let no_channels: Vec<mpsc::Receiver<i32>> = Vec::new();
        check_eq(par_map(no_channels, |rx| rx.recv().unwrap()), Vec::<i32>::new())?;
        let no_numbers: Vec<i32> = Vec::new();
        check_eq(par_map(no_numbers, |n| Cell::new(n * 2)), Vec::<Cell<i32>>::new())
    });
    case("case-2", || {
        // The items are receivers, and each thread takes its own.
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        tx1.send(4).unwrap();
        tx2.send(-1).unwrap();
        check_eq(par_map(vec![rx1, rx2], |rx| rx.recv().unwrap() * 10), [40, -10])
    });
    case("case-3", || {
        // The item and the result borrow `text`.
        let text = String::from("  solo ");
        check_eq(par_map(vec![text.as_str()], |w| w.trim()), ["solo"])
    });
    case("case-4", || {
        // The closure takes the lock guard with it.
        let table = Mutex::new(vec![10, 20, 30, 40]);
        let guard = table.lock().unwrap();
        let picked = par_map(vec![3, 0], move |i: usize| guard[i]);
        check_eq(picked, [40, 10])?;
        check_eq(table.lock().unwrap().len(), 4)
    });
}
