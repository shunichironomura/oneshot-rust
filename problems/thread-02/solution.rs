use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Arc::new(words);
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let processed = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Arc::clone(&words);
        let counts = Arc::clone(&counts);
        let processed = Arc::clone(&processed);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                for c in word.chars() {
                    *counts.lock().unwrap().entry(c).or_insert(0) += 1;
                }
                processed.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // Every clone moved into a thread is gone once the threads are joined.
    let counts = Arc::try_unwrap(counts).unwrap().into_inner().unwrap();
    (counts, processed.load(Ordering::Relaxed))
}
