use std::sync::mpsc;
use std::thread;

pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for group in groups {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            for n in group {
                tx.send(n).unwrap();
            }
        }));
    }
    // Only the clones in the threads may keep the channel open; otherwise
    // `rx.iter()` would wait forever for this `tx`.
    drop(tx);
    let mut all: Vec<u32> = rx.iter().collect();
    for handle in handles {
        handle.join().unwrap();
    }
    all.sort();
    all
}
