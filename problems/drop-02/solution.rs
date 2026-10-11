use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Transaction {
    log: Log,
    ops: Vec<String>,
    /// Set once `commit` or `into_ops` has logged the outcome, so that `drop` adds nothing.
    finished: bool,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { log: Rc::clone(log), ops: Vec::new(), finished: false }
    }

    pub fn push(&mut self, op: &str) {
        self.ops.push(op.to_string());
    }

    pub fn commit(mut self) -> usize {
        // A type that implements `Drop` cannot be destructured (E0509), and `self` is still
        // dropped at the end of this method: tell `drop` that there is nothing to roll back.
        self.log.borrow_mut().push(format!("commit:{}", self.ops.join(",")));
        self.finished = true;
        self.ops.len()
    }

    pub fn into_ops(mut self) -> Vec<String> {
        // Log first: after `take`, `self.ops` is empty.
        self.rollback();
        self.finished = true;
        std::mem::take(&mut self.ops)
    }

    fn rollback(&self) {
        self.log.borrow_mut().push(format!("rollback:{}", self.ops.len()));
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.finished {
            self.rollback();
        }
    }
}
