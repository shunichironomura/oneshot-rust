use std::pin::{Pin, pin};
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};
use std::thread;

use crate::judge::{case, check_eq};
use crate::user::{Fallback, MemStore, Store, total};

/// Polls `fut` with a waker that does nothing until it is ready.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..1000 {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return out;
        }
    }
    panic!("the future did not finish after 1000 polls");
}

/// Creates the future of `total` on this thread, then runs it to completion on another
/// thread, as a multi-threaded executor may do.
fn total_on_worker<S: Store>(store: &S, keys: &[&str]) -> u32 {
    let fut = total(store, keys);
    thread::scope(|sc| sc.spawn(move || block_on(fut)).join().unwrap())
}

/// Returns `Pending` the first time it is polled and `Ready(())` after that.
struct YieldOnce {
    yielded: bool,
}

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// The harness's own store: it logs every key it is asked for, and each lookup is not ready
/// the first time it is polled.
struct SlowStore {
    data: Vec<(&'static str, u32)>,
    log: Mutex<Vec<String>>,
}

impl SlowStore {
    fn new(data: &[(&'static str, u32)]) -> Self {
        SlowStore { data: data.to_vec(), log: Mutex::new(Vec::new()) }
    }

    /// The keys asked for so far, in order.
    fn keys_asked(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
}

impl Store for SlowStore {
    async fn get(&self, key: &str) -> Option<u32> {
        self.log.lock().unwrap().push(key.to_string());
        YieldOnce { yielded: false }.await;
        self.data.iter().find(|entry| entry.0 == key).map(|entry| entry.1)
    }
}

pub fn run() {
    case("sample-1", || {
        let store = MemStore::new(&[("a", 1), ("b", 2)]);
        check_eq(total_on_worker(&store, &["a", "b", "zz"]), 3)
    });
    case("sample-2", || {
        let store = Fallback {
            first: MemStore::new(&[("a", 1)]),
            second: MemStore::new(&[("a", 100), ("b", 20)]),
        };
        check_eq(total_on_worker(&store, &["a", "b"]), 21)
    });
    case("hidden-1", || {
        // No keys: the sum is 0 and nothing is looked up.
        let slow = SlowStore::new(&[("x", 5)]);
        check_eq(total_on_worker(&slow, &[]), 0)?;
        check_eq(slow.keys_asked(), Vec::<String>::new())
    });
    case("hidden-2", || {
        // One lookup per key, in order; a repeated key is looked up again.
        let slow = SlowStore::new(&[("x", 5), ("y", 7)]);
        check_eq(total_on_worker(&slow, &["y", "x", "nope", "y"]), 19)?;
        check_eq(slow.keys_asked(), vec!["y", "x", "nope", "y"])
    });
    case("hidden-3", || {
        // `second` is asked only when `first` does not have the key, even if `first` has 0.
        let store = Fallback {
            first: SlowStore::new(&[("a", 1)]),
            second: Fallback {
                first: MemStore::new(&[("zero", 0), ("b", 10)]),
                second: SlowStore::new(&[("a", 1000), ("zero", 50), ("c", 100)]),
            },
        };
        check_eq(total_on_worker(&store, &["c", "zero", "a", "b", "d"]), 111)?;
        check_eq(store.first.keys_asked(), vec!["c", "zero", "a", "b", "d"])?;
        check_eq(store.second.second.keys_asked(), vec!["c", "d"])
    });
}
