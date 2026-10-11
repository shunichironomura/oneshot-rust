use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T> {
    let mut unsent = Vec::new();
    for item in items {
        // `send` takes the item by value; on failure, the error hands it back.
        if let Err(err) = tx.send(item) {
            unsent.push(err.0);
        }
    }
    unsent
}

pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool) {
    let mut ready = Vec::new();
    loop {
        match rx.try_recv() {
            Ok(value) => ready.push(value),
            Err(TryRecvError::Empty) => return (ready, false),
            Err(TryRecvError::Disconnected) => return (ready, true),
        }
    }
}

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
