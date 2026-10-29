use std::sync::mpsc::channel;

use crate::judge::{case, check_eq};
use crate::user::{drain_ready, fan_in, send_all};

/// Neither `Clone` nor `Copy`.
#[derive(Debug, PartialEq)]
struct Ticket(u32);

pub fn run() {
    case("example-1", || {
        let (tx, rx) = channel();
        drop(rx);
        check_eq(send_all(&tx, vec![1, 2]), vec![1, 2])
    });
    case("example-2", || {
        let (tx, rx) = channel();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        check_eq(drain_ready(&rx), (vec![1, 2], false))?;
        drop(tx);
        check_eq(drain_ready(&rx), (vec![], true))
    });
    case("case-1", || {
        // A live receiver gets everything, in order.
        let (tx, rx) = channel();
        check_eq(send_all(&tx, vec![10, 20]), vec![])?;
        check_eq(send_all(&tx, Vec::new()), vec![])?;
        drop(tx);
        check_eq(rx.iter().collect::<Vec<_>>(), vec![10, 20])
    });
    case("case-2", || {
        // Every undelivered value comes back, in order, even if it cannot be cloned.
        let (tx, rx) = channel();
        drop(rx);
        let items = vec![Ticket(3), Ticket(1), Ticket(2)];
        check_eq(send_all(&tx, items), vec![Ticket(3), Ticket(1), Ticket(2)])
    });
    case("case-3", || {
        let (tx, rx) = channel::<i32>();
        check_eq(drain_ready(&rx), (vec![], false))?;
        let tx2 = tx.clone();
        tx.send(7).unwrap();
        tx2.send(8).unwrap();
        drop(tx);
        // `tx2` is still alive.
        check_eq(drain_ready(&rx), (vec![7, 8], false))?;
        tx2.send(9).unwrap();
        drop(tx2);
        // Queued messages are still delivered after the last sender is gone.
        check_eq(drain_ready(&rx), (vec![9], true))?;
        check_eq(drain_ready(&rx), (vec![], true))
    });
    case("example-3", || check_eq(fan_in(vec![vec![3, 1], vec![2]]), vec![1, 2, 3]));
    case("case-4", || check_eq(fan_in(Vec::new()), vec![]));
    case("case-5", || check_eq(fan_in(vec![vec![9, 0, 9], vec![]]), vec![0, 9, 9]));
}
