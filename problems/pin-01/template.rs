use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// Returns `Pending` the first time it is polled (after waking the task)
/// and `Ready(())` the next time.
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

/// A task: an `async` block that outputs a `u32`.
type Task = Box<dyn Future<Output = u32>>;

/// Polls the unfinished tasks in rounds, each round in index order, until all of them
/// have finished. Returns the index and the output of each task, in the order they finished.
fn run_round_robin(tasks: Vec<Task>) -> Vec<(usize, u32)> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut unfinished: Vec<(usize, Task)> = tasks.into_iter().enumerate().collect();
    let mut finished = Vec::new();
    while !unfinished.is_empty() {
        // One round: poll each unfinished task once, and drop the ones that are done.
        unfinished.retain_mut(|(index, task)| match Pin::new(task.as_mut()).poll(&mut cx) {
            Poll::Ready(output) => {
                finished.push((*index, output));
                false
            }
            Poll::Pending => true,
        });
    }
    finished
}

/// Task `i` computes `i * i` as `i + i + ... + i`, yielding after each addition.
/// Returns the outputs in task order.
pub fn squares_round_robin(n: u32) -> Vec<u32> {
    let mut tasks: Vec<Task> = Vec::new();
    for i in 0..n {
        tasks.push(Box::new(async move {
            let mut square = 0;
            for _ in 0..i {
                square += i;
                yield_now().await;
            }
            square
        }));
    }
    let mut squares = vec![0; n as usize];
    for (index, output) in run_round_robin(tasks) {
        squares[index] = output;
    }
    squares
}

/// Task `i` yields `delays[i]` times. Returns the task indices in the order they finished.
pub fn finish_order(delays: &[u32]) -> Vec<usize> {
    let tasks = delays
        .iter()
        .map(|&delay| {
            Box::new(async move {
                for _ in 0..delay {
                    yield_now().await;
                }
                delay
            }) as Task
        })
        .collect();
    run_round_robin(tasks).into_iter().map(|(index, _)| index).collect()
}
