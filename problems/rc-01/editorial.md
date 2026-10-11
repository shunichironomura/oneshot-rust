An `Account` cannot hold a `&Ledger`: it may be moved away from the ledger or outlive it, and the
signature `open(&self, ...) -> Account` has no lifetime to tie them together. Instead, the ledger and
its accounts **share ownership** of the log through `Rc`. `Rc::clone` only adds one more handle to
the same `Vec`.

`Rc` gives shared access only, but `deposit` must push to the log. `RefCell` moves the borrow check
to run time: `borrow_mut()` hands out a mutable borrow through a shared handle (and would panic if
another borrow were active). Keep each borrow short, as in `self.log.borrow_mut().push(...)`.

`Rc::strong_count` counts the handles that still exist. The ledger holds one and each live account
holds one, so `open_accounts` is the count minus 1. Dropping an account drops its handle, so the
count goes down on its own.

### Alternatives

Count the accounts yourself and decrement the count in a `Drop` impl. Putting all the shared state in
one struct keeps a single `RefCell`.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Shared {
    log: Vec<String>,
    open: usize,
}

pub struct Ledger {
    shared: Rc<RefCell<Shared>>,
}

pub struct Account {
    name: String,
    balance: u32,
    shared: Rc<RefCell<Shared>>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { shared: Rc::default() }
    }

    pub fn open(&self, name: &str) -> Account {
        self.shared.borrow_mut().open += 1;
        Account { name: name.to_string(), balance: 0, shared: Rc::clone(&self.shared) }
    }

    pub fn log(&self) -> Vec<String> {
        self.shared.borrow().log.clone()
    }

    pub fn open_accounts(&self) -> usize {
        self.shared.borrow().open
    }
}

impl Account {
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
        self.shared.borrow_mut().log.push(format!("{} +{}", self.name, amount));
    }

    pub fn withdraw(&mut self, amount: u32) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        self.shared.borrow_mut().log.push(format!("{} -{}", self.name, amount));
        true
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        self.shared.borrow_mut().open -= 1;
    }
}
```
