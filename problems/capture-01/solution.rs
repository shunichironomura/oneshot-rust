use std::thread;

/// A raw pointer that may be sent to another thread.
#[derive(Clone, Copy)]
struct SendPtr(*mut u64);

// SAFETY: every thread writes to different elements, and all threads are
// joined before `fill_squares` returns.
unsafe impl Send for SendPtr {}

impl SendPtr {
    /// Takes `self` by value: a closure that calls `get` captures the whole
    /// `SendPtr` (which is `Send`), not only the raw pointer inside it.
    fn get(self) -> *mut u64 {
        self.0
    }
}

pub fn fill_squares(out: &mut [u64], workers: usize) {
    let len = out.len();
    let base = SendPtr(out.as_mut_ptr());
    let mut handles = Vec::new();
    for w in 0..workers {
        handles.push(thread::spawn(move || {
            for i in (w..len).step_by(workers) {
                // SAFETY: `i < len`, and no other thread writes to index `i`.
                unsafe { *base.get().add(i) = (i * i) as u64 };
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
