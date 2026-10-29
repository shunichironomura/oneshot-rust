use std::sync::mpsc::channel;

use crate::judge::{case, check_eq};
use crate::user::send_all;

/// Neither `Clone` nor `Copy`.
#[derive(Debug, PartialEq)]
struct Ticket(u32);

pub fn run() {
    case("example-1", || {
        let (tx, rx) = channel();
        drop(rx);
        check_eq(send_all(&tx, vec![1, 2]), vec![1, 2])
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
}
