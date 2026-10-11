use std::cell::Cell;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

use crate::judge::{case, check_eq};
use crate::user::{fetch_any, retry};

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

/// Returns `Pending` the first time it is polled and `Ready(())` after that.
struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
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

fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

/// Succeeds only for URLs on the mirror `ok`.
async fn probe(url: &str) -> Result<usize, &'static str> {
    yield_now().await;
    if url.starts_with("ok/") { Ok(url.len()) } else { Err("not found") }
}

pub fn run() {
    case("sample-1", || {
        let mut calls = 0;
        let result = block_on(retry(5, async || {
            calls += 1;
            yield_now().await;
            if calls < 3 { Err("busy") } else { Ok(calls) }
        }));
        check_eq((result, calls), (Ok(3), 3))
    });
    case("sample-2", || {
        let mut calls = 0;
        let result = block_on(retry(2, async || {
            calls += 1;
            yield_now().await;
            if calls < 3 { Err("busy") } else { Ok(calls) }
        }));
        check_eq((result, calls), (Err("busy"), 2))
    });
    case("sample-3", || {
        let mut tried = Vec::new();
        let result = block_on(fetch_any(&["a", "b", "c"], "x.txt", async |url: &str| {
            tried.push(url.to_string());
            yield_now().await;
            if url.starts_with("b/") { Ok(url.len()) } else { Err(url.to_string()) }
        }));
        check_eq(result, Ok(7))?;
        check_eq(tried, vec!["a/x.txt", "b/x.txt"])
    });
    case("hidden-1", || {
        // With times == 1, `f` is called exactly once.
        let mut calls = 0;
        let failed = block_on(retry(1, async || {
            calls += 1;
            Err::<(), _>(calls * 10)
        }));
        check_eq((failed, calls), (Err(10), 1))?;
        let succeeded = block_on(retry(1, async || {
            calls += 1;
            Ok::<_, ()>(calls)
        }));
        check_eq((succeeded, calls), (Ok(2), 2))
    });
    case("hidden-2", || {
        // A log that the closure appends to, read after `retry` returns.
        let mut log: Vec<String> = Vec::new();
        let result = block_on(retry(4, async || {
            log.push(String::from("try"));
            yield_now().await;
            if log.len() == 4 { Ok(log.len()) } else { Err(log.len()) }
        }));
        check_eq(result, Ok(4))?;
        check_eq(log, ["try", "try", "try", "try"])
    });
    case("hidden-3", || {
        // An `async move` closure that owns its state: no more calls after the first `Ok`.
        let mut responses = vec![Ok("done"), Err("timeout"), Err("busy")];
        let result = block_on(retry(5, async move || {
            yield_now().await;
            responses.pop().unwrap()
        }));
        check_eq(result, Ok("done"))
    });
    case("hidden-4", || {
        // A closure that only reads a borrowed table (and counts through a `Cell`).
        let scores = vec![("alice", 3), ("bob", 5)];
        let lookups = Cell::new(0);
        let key = String::from("bob");
        let found = block_on(retry(3, async || {
            lookups.set(lookups.get() + 1);
            yield_now().await;
            scores.iter().find(|entry| entry.0 == key).map(|entry| entry.1).ok_or("missing")
        }));
        check_eq((found, lookups.get()), (Ok(5), 1))?;
        let key = String::from("carol");
        let missing = block_on(retry(3, async || {
            lookups.set(lookups.get() + 1);
            scores.iter().find(|entry| entry.0 == key).map(|entry| entry.1).ok_or("missing")
        }));
        check_eq((missing, lookups.get()), (Err("missing"), 4))?;
        check_eq(scores.len(), 2)
    });
    case("hidden-5", || {
        // An `async fn` instead of a closure.
        check_eq(block_on(fetch_any(&["a", "ok", "b"], "index.html", probe)), Ok(13))?;
        check_eq(block_on(fetch_any(&["x", "y"], "p", probe)), Err("not found"))
    });
    case("hidden-6", || {
        // Stops at the first mirror that succeeds; a single mirror is tried once.
        let mut tried = Vec::new();
        let first = block_on(fetch_any(&["up", "down"], "v1", async |url: &str| {
            tried.push(url.to_string());
            Ok::<_, ()>(url.len())
        }));
        check_eq(first, Ok(5))?;
        check_eq(tried, vec!["up/v1"])?;
        let only = block_on(fetch_any(&["solo"], "a b", async |url: &str| {
            Err::<(), _>(url.to_uppercase())
        }));
        check_eq(only, Err("SOLO/A B".to_string()))
    });
    case("hidden-7", || {
        // `retry` inside the closure passed to `fetch_any`.
        let mut attempts = Vec::new();
        let result = block_on(fetch_any(&["a", "b"], "f", async |url: &str| {
            retry(2, async || {
                attempts.push(url.to_string());
                yield_now().await;
                Err::<(), _>(attempts.len())
            })
            .await
        }));
        check_eq(result, Err(4))?;
        check_eq(attempts, vec!["a/f", "a/f", "b/f", "b/f"])
    });
}
