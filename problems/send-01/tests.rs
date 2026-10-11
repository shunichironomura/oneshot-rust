use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, mpsc};

use crate::judge::{case, check_eq};
use crate::user::{par_map, spawn_job};

pub fn run() {
    case("sample-1", || {
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
    case("sample-2", || {
        let name = String::from("Ferris");
        let job = spawn_job(3, move |n: usize| name + &"!".repeat(n));
        check_eq(job.join().unwrap(), "Ferris!!!")
    });
    case("hidden-1", || {
        let no_bytes: Vec<Vec<u8>> = Vec::new();
        check_eq(par_map(no_bytes, |bytes| bytes.len()), Vec::<usize>::new())?;
        let no_channels: Vec<mpsc::Receiver<i32>> = Vec::new();
        check_eq(par_map(no_channels, |rx| rx.recv().unwrap()), Vec::<i32>::new())
    });
    case("hidden-2", || {
        // The item and the result borrow `text`.
        let text = String::from("  solo ");
        check_eq(par_map(vec![text.as_str()], |w| w.trim()), ["solo"])
    });
    case("hidden-3", || {
        // The closure takes the lock guard with it.
        let table = Mutex::new(vec![10, 20, 30, 40]);
        let guard = table.lock().unwrap();
        let picked = par_map(vec![3, 0], move |i: usize| guard[i]);
        check_eq(picked, [40, 10])?;
        check_eq(table.lock().unwrap().len(), 4)
    });
    case("hidden-4", || {
        // Each result is a `Cell`.
        check_eq(par_map(vec![7], |n| Cell::new(n * 10)), [Cell::new(70)])
    });
    case("hidden-5", || {
        // The job is already running while this thread sends the numbers.
        let (tx, rx) = mpsc::channel();
        let job = spawn_job(rx, |rx: mpsc::Receiver<i32>| rx.iter().sum::<i32>());
        for n in [1, 2, 3] {
            tx.send(n).unwrap();
        }
        drop(tx);
        check_eq(job.join().unwrap(), 6)
    });
    case("hidden-6", || {
        // The closure takes the receiver with it and gives it back.
        let (tx, rx) = mpsc::channel();
        let job = spawn_job(10, move |k: i32| (rx.recv().unwrap() * k, rx));
        tx.send(4).unwrap();
        tx.send(5).unwrap();
        drop(tx);
        let (first, rx) = job.join().unwrap();
        check_eq(first, 40)?;
        check_eq(rx.iter().collect::<Vec<i32>>(), [5])
    });
}
