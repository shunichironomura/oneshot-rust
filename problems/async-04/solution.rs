use std::future::poll_fn;
use std::sync::Mutex;
use std::task::Poll;

/// Returns `Pending` once (after waking the task), then `Ready(())`.
pub async fn yield_now() {
    let mut yielded = false;
    poll_fn(|cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}

/// Pushes the items to `log` one at a time, yielding after each push.
/// Returns the length of `log` right after the last push.
pub async fn record_all(log: &Mutex<Vec<String>>, items: Vec<String>) -> usize {
    let mut len = log.lock().unwrap().len();
    for item in items {
        // The guard only exists inside this block, so it is gone before the `.await`.
        // (`drop(guard)` would unlock in time, but the future would still not be `Send`.)
        len = {
            let mut guard = log.lock().unwrap();
            guard.push(item);
            guard.len()
        };
        yield_now().await;
    }
    len
}

/// Moves all the messages of `inbox` to the end of `log`.
/// Returns the length of `log` after that.
pub async fn forward_all(inbox: &Mutex<Vec<String>>, log: &Mutex<Vec<String>>) -> usize {
    // The temporary guard is dropped at the end of this `let` statement.
    let messages = std::mem::take(&mut *inbox.lock().unwrap());
    record_all(log, messages).await
}
