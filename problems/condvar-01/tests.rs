use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::BoundedQueue;

/// Neither `Clone` nor `Copy`.
#[derive(Debug, PartialEq)]
struct Job(u32);

/// Gives the other threads many turns, so that the ones that are going to wait are waiting.
fn let_others_run() {
    for _ in 0..30 {
        thread::yield_now();
    }
}

pub fn run() {
    case("sample-1", || {
        let q = BoundedQueue::new(2);
        check_eq(q.push(1), Ok(()))?;
        check_eq(q.push(2), Ok(()))?;
        check_eq(q.len(), 2)?;
        check_eq(q.pop(), Some(1))?;
        check_eq(q.push(3), Ok(()))?;
        q.close();
        check_eq(q.push(4), Err(4))?;
        check_eq(q.pop(), Some(2))?;
        check_eq(q.pop(), Some(3))?;
        check_eq(q.pop(), None)
    });
    case("sample-2", || {
        // The main thread and a producer thread push 5 items each, while two consumer threads
        // pop until `None`. The queue has room for 2 items.
        let q = &BoundedQueue::new(2);
        let closing = &AtomicBool::new(false);
        let (pushed, received) = thread::scope(|s| {
            let consumers: Vec<_> = (0..2)
                .map(|_| {
                    s.spawn(move || {
                        let mut got = Vec::new();
                        while let Some(x) = q.pop() {
                            if q.len() > 2 {
                                return Err("the queue held more items than its capacity");
                            }
                            got.push(x);
                        }
                        if !closing.load(Ordering::SeqCst) {
                            return Err("pop returned None before the queue was closed");
                        }
                        Ok(got)
                    })
                })
                .collect();
            let producer = s.spawn(move || (200..205).map(|x| q.push(x)).collect::<Vec<_>>());
            let mut pushed: Vec<_> = (100..105).map(|x| q.push(x)).collect();
            pushed.extend(producer.join().unwrap());
            closing.store(true, Ordering::SeqCst);
            q.close();
            let received: Vec<_> = consumers.into_iter().map(|h| h.join().unwrap()).collect();
            (pushed, received)
        });
        check(pushed.iter().all(|r| r.is_ok()), "a push before close returned Err")?;
        let mut all = Vec::new();
        for got in received {
            let got = got?;
            for base in [100, 200] {
                let mine: Vec<u32> = got.iter().copied().filter(|x| x / 100 == base / 100).collect();
                check(mine.windows(2).all(|w| w[0] < w[1]), "a consumer got items out of order")?;
            }
            all.extend(got);
        }
        all.sort();
        check_eq(all, vec![100, 101, 102, 103, 104, 200, 201, 202, 203, 204])
    });
    case("hidden-1", || {
        // First in, first out, also when the queue fills up again after a pop.
        let q = BoundedQueue::new(3);
        check_eq(q.len(), 0)?;
        for x in [5, 6, 7] {
            check_eq(q.push(x), Ok(()))?;
        }
        check_eq(q.len(), 3)?;
        check_eq(q.pop(), Some(5))?;
        check_eq(q.push(8), Ok(()))?;
        check_eq(q.pop(), Some(6))?;
        check_eq(q.push(9), Ok(()))?;
        check_eq(q.len(), 3)?;
        check_eq([q.pop(), q.pop(), q.pop()], [Some(7), Some(8), Some(9)])?;
        check_eq(q.len(), 0)?;
        let one = BoundedQueue::new(1);
        check_eq(one.push(Job(1)), Ok(()))?;
        check_eq(one.len(), 1)?;
        check_eq(one.pop(), Some(Job(1)))?;
        check_eq(one.push(Job(2)), Ok(()))?;
        check_eq(one.pop(), Some(Job(2)))?;
        check_eq(one.len(), 0)
    });
    case("hidden-2", || {
        // A closed queue takes nothing, even with room left, and still hands out what it holds.
        let q = BoundedQueue::new(4);
        check_eq(q.push(Job(1)), Ok(()))?;
        check_eq(q.push(Job(2)), Ok(()))?;
        q.close();
        check_eq(q.push(Job(3)), Err(Job(3)))?;
        check_eq(q.len(), 2)?;
        q.close();
        check_eq(q.pop(), Some(Job(1)))?;
        check_eq(q.push(Job(4)), Err(Job(4)))?;
        check_eq(q.len(), 1)?;
        check_eq(q.pop(), Some(Job(2)))?;
        check_eq(q.pop(), None)?;
        check_eq(q.pop(), None)?;
        check_eq(q.len(), 0)?;
        let empty = BoundedQueue::new(1);
        empty.close();
        check_eq(empty.pop(), None)?;
        check_eq(empty.push(Job(5)), Err(Job(5)))?;
        check_eq(empty.len(), 0)
    });
    case("hidden-3", || {
        // Two threads pop until `None`, then wait in push on a full queue. The first queue has room
        // for 1 item. Three times, while both threads wait in pop, the main thread pushes two
        // items right away, so its second push often waits for room while the other thread still
        // waits for an item. The pop that takes the first item must then wake the waiting push
        // (waking only the other waiting pop loses the wakeup). Closing a queue wakes both threads.
        let q = &BoundedQueue::new(1);
        let full = &BoundedQueue::new(1);
        check_eq(full.push(Job(0)), Ok(()))?;
        let (pushed, [(a, a_pushed), (b, b_pushed)]) = thread::scope(|s| {
            let threads = [Job(1), Job(2)].map(|job| {
                s.spawn(move || {
                    let mut got = Vec::new();
                    while let Some(x) = q.pop() {
                        got.push(x);
                    }
                    (got, full.push(job))
                })
            });
            let pushed: Vec<_> = (1..=3)
                .map(|i| {
                    let_others_run();
                    [q.push(2 * i - 1), q.push(2 * i)]
                })
                .collect();
            let_others_run();
            q.close();
            let_others_run();
            full.close();
            (pushed, threads.map(|t| t.join().unwrap()))
        });
        check(pushed.iter().flatten().all(|r| r.is_ok()), "a push before close returned Err")?;
        check(a.is_sorted() && b.is_sorted(), "a thread got items out of order")?;
        let mut all = [a, b].concat();
        all.sort();
        check_eq(all, vec![1, 2, 3, 4, 5, 6])?;
        check_eq([a_pushed, b_pushed], [Err(Job(1)), Err(Job(2))])?;
        check_eq(full.len(), 1)?;
        check_eq(full.pop(), Some(Job(0)))?;
        check_eq(full.pop(), None)
    });
    case("hidden-4", || {
        // A pop waits on an empty queue. The main thread pushes 1 and pops it again at once, often
        // before the waiting pop gets to it. Then the waiting pop must keep waiting: the queue is
        // still open, and 2 is pushed a little later.
        let q = BoundedQueue::new(2);
        let (pushed, mine, theirs) = thread::scope(|s| {
            let waiting = s.spawn(|| {
                let item = q.pop();
                if item == Some(1) {
                    // The main thread is waiting for an item now.
                    let _ = q.push(2);
                }
                item
            });
            let_others_run();
            let first = q.push(1);
            let mine = q.pop();
            let_others_run();
            let second = if mine == Some(1) { q.push(2) } else { Ok(()) };
            ([first, second], mine, waiting.join().unwrap())
        });
        check_eq(pushed, [Ok(()), Ok(())])?;
        let mut got = [mine, theirs];
        got.sort();
        check_eq(got, [Some(1), Some(2)])?;
        check_eq(q.len(), 0)
    });
    case("hidden-5", || {
        // The same for push: a push of 2 waits on a full queue with room for 1 item. The main
        // thread pops 1 and pushes 3 at once, often before the waiting push gets the room. Then
        // the waiting push must keep waiting until the main thread pops again.
        let q = BoundedQueue::new(1);
        check_eq(q.push(1), Ok(()))?;
        let (mine, theirs) = thread::scope(|s| {
            let waiting = s.spawn(|| (q.push(2), q.pop()));
            let_others_run();
            let first = q.pop();
            let pushed = q.push(3);
            let_others_run();
            let len_seen = q.len();
            let last = q.pop();
            ((first, pushed, len_seen, last), waiting.join().unwrap())
        });
        check_eq(mine, (Some(1), Ok(()), 1, Some(3)))?;
        check_eq(theirs, (Ok(()), Some(2)))?;
        check_eq(q.len(), 0)
    });
}
