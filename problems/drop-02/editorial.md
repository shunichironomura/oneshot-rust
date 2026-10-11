A type that implements `Drop` cannot be taken apart. `let Transaction { log, ops } = self;` and
returning `self.ops` both move a field out of `self`, and the compiler rejects both (E0509): `drop`
still runs on the value later, and it gets `&mut self` with every field in place. Taking `self` by
value does not switch `Drop` off either. Inside the method, `self` is an ordinary local variable,
and it is dropped at the end of the method unless it is moved somewhere as a whole.

That is why the fixes the compiler suggests compile but are wrong here. With `ref log, ref ops` the
pattern only borrows, `self` is dropped at the end of `commit`, and `drop` appends `"rollback:2"`
right after `"commit:debit,credit"`. With `self.ops.clone()`, `into_ops` rolls back once by itself
and once more when `self` is dropped.

`drop` has to know that the transaction is already finished. The solution adds a `finished` flag
that `commit` and `into_ops` set, and `drop` rolls back only while it is `false`. The methods take
`mut self` so that they can set it (`mut` on a parameter is not part of the signature). `into_ops`
then moves the operations out with `std::mem::take`, which leaves an empty `Vec` behind. Log the
rollback **before** that: once the operations are taken, `self.ops.len()` is 0. For the same
reason, taking the operations and leaving the rollback to `drop` logs `"rollback:0"`.

Two other tempting fixes fail as well. `std::mem::forget(self)` skips `drop`, but it also skips
dropping the fields: the `Rc` in `log` is never released, `Rc::strong_count(&log)` stays above 1,
and Miri reports a memory leak. Using "no operations left" as the marker instead of a flag breaks
empty transactions, which must still log `"rollback:0"` when they are dropped.

### Alternatives

Make the log handle an `Option` and take it out to finish the transaction. `rollback` then logs at
most once on its own, so `drop` can call it unconditionally, and no flag can get out of sync.

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Transaction {
    /// `None` once the outcome has been logged.
    log: Option<Log>,
    ops: Vec<String>,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { log: Some(Rc::clone(log)), ops: Vec::new() }
    }

    pub fn push(&mut self, op: &str) {
        self.ops.push(op.to_string());
    }

    pub fn commit(mut self) -> usize {
        if let Some(log) = self.log.take() {
            log.borrow_mut().push(format!("commit:{}", self.ops.join(",")));
        }
        self.ops.len()
    }

    pub fn into_ops(mut self) -> Vec<String> {
        self.rollback();
        std::mem::take(&mut self.ops)
    }

    fn rollback(&mut self) {
        if let Some(log) = self.log.take() {
            log.borrow_mut().push(format!("rollback:{}", self.ops.len()));
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.rollback();
    }
}
```

To keep destructuring as in the original code, move the fields into an inner struct that does not
implement `Drop`, and keep it in an `Option`. `take()` hands out the whole inner value, which can be
destructured freely, and `drop` only rolls back while it is still `Some`. The `else` branches never
run: the value is `None` only inside `commit` and `into_ops`.

```rust
struct Inner {
    log: Log,
    ops: Vec<String>,
}

pub struct Transaction {
    /// `Some` until `commit` or `into_ops` takes it out.
    inner: Option<Inner>,
}

impl Transaction {
    pub fn begin(log: &Log) -> Transaction {
        Transaction { inner: Some(Inner { log: Rc::clone(log), ops: Vec::new() }) }
    }

    pub fn push(&mut self, op: &str) {
        if let Some(inner) = &mut self.inner {
            inner.ops.push(op.to_string());
        }
    }

    pub fn commit(mut self) -> usize {
        let Some(Inner { log, ops }) = self.inner.take() else { return 0 };
        log.borrow_mut().push(format!("commit:{}", ops.join(",")));
        ops.len()
    }

    pub fn into_ops(mut self) -> Vec<String> {
        let Some(Inner { log, ops }) = self.inner.take() else { return Vec::new() };
        log.borrow_mut().push(format!("rollback:{}", ops.len()));
        ops
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Some(Inner { log, ops }) = &self.inner {
            log.borrow_mut().push(format!("rollback:{}", ops.len()));
        }
    }
}
```
