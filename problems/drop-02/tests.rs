use std::cell::RefCell;
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{Log, Transaction};

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Returns the lines logged since the last call, and clears the log.
fn drain(log: &Log) -> Vec<String> {
    std::mem::take(&mut *log.borrow_mut())
}

fn nothing() -> Vec<String> {
    Vec::new()
}

fn with_ops(log: &Log, ops: &[&str]) -> Transaction {
    let mut t = Transaction::begin(log);
    for op in ops {
        t.push(op);
    }
    t
}

/// No transaction keeps the log alive any more.
fn alone(log: &Log) -> Result<(), String> {
    check(Rc::strong_count(log) == 1, "the log is still kept alive")
}

pub fn run() {
    case("sample-1", || {
        let log = new_log();
        let mut t = Transaction::begin(&log);
        t.push("debit");
        t.push("credit");
        check_eq(drain(&log), nothing())?;
        check_eq(t.commit(), 2)?;
        check_eq(drain(&log), ["commit:debit,credit"])?;
        alone(&log)
    });
    case("sample-2", || {
        let log = new_log();
        let mut a = Transaction::begin(&log);
        a.push("x");
        a.push("y");
        check_eq(a.into_ops(), ["x", "y"])?;
        check_eq(drain(&log), ["rollback:2"])?;
        {
            let mut b = Transaction::begin(&log);
            b.push("z");
            let _c = Transaction::begin(&log);
        }
        check_eq(drain(&log), ["rollback:0", "rollback:1"])?;
        alone(&log)
    });
    case("hidden-1", || {
        // Committing an empty transaction, then one with a single operation.
        let log = new_log();
        let t = Transaction::begin(&log);
        check_eq(t.commit(), 0)?;
        check_eq(drain(&log), ["commit:"])?;
        alone(&log)?;
        let t = with_ops(&log, &["only"]);
        check_eq(t.commit(), 1)?;
        check_eq(drain(&log), ["commit:only"])?;
        alone(&log)
    });
    case("hidden-2", || {
        // Dropping uncommitted transactions, including an empty one.
        let log = new_log();
        let empty = Transaction::begin(&log);
        drop(empty);
        check_eq(drain(&log), ["rollback:0"])?;
        alone(&log)?;
        let t = with_ops(&log, &["a", "b", "c"]);
        drop(t);
        check_eq(drain(&log), ["rollback:3"])?;
        alone(&log)
    });
    case("hidden-3", || {
        // into_ops on an empty transaction and on one with unusual operations.
        let log = new_log();
        let t = Transaction::begin(&log);
        check_eq(t.into_ops(), nothing())?;
        check_eq(drain(&log), ["rollback:0"])?;
        alone(&log)?;
        let t = with_ops(&log, &["élan", "x,y", "x,y"]);
        check_eq(t.into_ops(), ["élan", "x,y", "x,y"])?;
        check_eq(drain(&log), ["rollback:3"])?;
        alone(&log)
    });
    case("hidden-4", || {
        // Retry the operations of a rolled-back transaction while another one is open.
        let log = new_log();
        let first = with_ops(&log, &["a", "b"]);
        let other = with_ops(&log, &["o"]);
        let ops = first.into_ops();
        let mut retry = Transaction::begin(&log);
        for op in &ops {
            retry.push(op);
        }
        retry.push("c");
        check_eq(retry.commit(), 3)?;
        check_eq(drain(&log), ["rollback:2", "commit:a,b,c"])?;
        check_eq(other.commit(), 1)?;
        check_eq(drain(&log), ["commit:o"])?;
        alone(&log)
    });
    case("hidden-5", || {
        // Two logs, nested scopes: locals are dropped in reverse order of declaration.
        let (log1, log2) = (new_log(), new_log());
        {
            let _a = with_ops(&log1, &["a"]);
            let _b = with_ops(&log2, &["b1", "b2"]);
            let c = with_ops(&log1, &["c"]);
            let _d = Transaction::begin(&log1);
            {
                let _e = with_ops(&log2, &["e"]);
                check_eq(c.commit(), 1)?;
            }
            check_eq(drain(&log2), ["rollback:1"])?;
        }
        check_eq(drain(&log1), ["commit:c", "rollback:0", "rollback:1"])?;
        check_eq(drain(&log2), ["rollback:2"])?;
        alone(&log1)?;
        alone(&log2)
    });
    case("hidden-6", || {
        // A transaction that is overwritten, and transactions kept in a Vec.
        let log = new_log();
        let mut t = with_ops(&log, &["p", "q"]);
        t.push("p");
        t = with_ops(&log, &["r"]);
        check_eq(drain(&log), ["rollback:3"])?;
        check_eq(t.commit(), 1)?;
        let mut v = vec![
            with_ops(&log, &["a", "b", "c"]),
            Transaction::begin(&log),
            with_ops(&log, &["z"]),
        ];
        let last = v.pop().map(Transaction::into_ops);
        check_eq(last, Some(vec!["z".to_string()]))?;
        v[1].push("w");
        drop(v);
        check_eq(drain(&log), ["commit:r", "rollback:1", "rollback:3", "rollback:1"])?;
        alone(&log)
    });
}
