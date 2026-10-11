use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Rc::new(words);
    let counts = Rc::new(RefCell::new(HashMap::new()));
    let processed = Rc::new(Cell::new(0));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Rc::clone(&words);
        let counts = Rc::clone(&counts);
        let processed = Rc::clone(&processed);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                for c in word.chars() {
                    *counts.borrow_mut().entry(c).or_insert(0) += 1;
                }
                processed.set(processed.get() + 1);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let counts = Rc::try_unwrap(counts).unwrap().into_inner();
    (counts, processed.get())
}
