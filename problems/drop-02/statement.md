The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `Transaction` collects operations and records its outcome in a shared log: either it is committed,
or it is rolled back. A transaction that is dropped without being committed is rolled back.

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

impl Transaction {
    pub fn begin(log: &Log) -> Transaction;
    pub fn push(&mut self, op: &str);
    pub fn commit(self) -> usize;
    pub fn into_ops(self) -> Vec<String>;
}

impl Drop for Transaction { /* ... */ }
```

- `begin` starts an empty transaction that writes to `log`. `begin` and `push` write nothing to the
  log.
- `push` adds the operation `op` (any string) to the transaction.
- `commit` appends `"commit:<ops>"` to the log and returns the number of operations. `<ops>` is the
  operations in the order they were pushed, joined by `,`. With no operations, the line is just
  `"commit:"`.
- `into_ops` rolls the transaction back and returns its operations in the order they were pushed.
- Rolling back appends `"rollback:<n>"` to the log, where `<n>` is the number of operations in the
  transaction (`"rollback:0"` for an empty one).
- Dropping a transaction that was neither committed nor consumed by `into_ops` rolls it back.
- Each transaction appends **exactly one** line to the log: a committed transaction is not rolled
  back as well, and `into_ops` rolls back only once.
- A transaction that is gone must not keep its log alive: once no transaction that writes to `log` is
  left, `Rc::strong_count(&log)` is `1` again.

Do not change `Log` or the public signatures. You may change the private fields and helpers of
`Transaction`.

### Examples

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let mut t = Transaction::begin(&log);
t.push("debit");
t.push("credit");
let n = t.commit(); // n == 2
// log: ["commit:debit,credit"]
```

```rust
let mut a = Transaction::begin(&log);
a.push("x");
a.push("y");
let ops = a.into_ops(); // ops == ["x", "y"], appends "rollback:2"
{
    let mut b = Transaction::begin(&log);
    b.push("z");
    let _c = Transaction::begin(&log);
} // _c is dropped first ("rollback:0"), then b ("rollback:1")
```
