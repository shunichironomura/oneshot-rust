use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Transaction {
    log: Log,
    ops: Vec<String>,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { log: Rc::clone(log), ops: Vec::new() }
    }

    pub fn push(&mut self, op: &str) {
        self.ops.push(op.to_string());
    }

    pub fn commit(self) -> usize {
        let Transaction { log, ops } = self;
        log.borrow_mut().push(format!("commit:{}", ops.join(",")));
        ops.len()
    }

    pub fn into_ops(self) -> Vec<String> {
        self.rollback();
        self.ops
    }

    fn rollback(&self) {
        self.log.borrow_mut().push(format!("rollback:{}", self.ops.len()));
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.rollback();
    }
}
