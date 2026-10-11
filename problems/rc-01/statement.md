Several accounts write to one ledger. Implement `Ledger` and `Account`.

```rust
pub struct Ledger { /* up to you */ }
pub struct Account { /* up to you */ }

impl Ledger {
    pub fn new() -> Self;
    pub fn open(&self, name: &str) -> Account;
    pub fn log(&self) -> Vec<String>;
    pub fn open_accounts(&self) -> usize;
}

impl Account {
    pub fn deposit(&mut self, amount: u32);
    pub fn withdraw(&mut self, amount: u32) -> bool;
    pub fn balance(&self) -> u32;
}
```

- `open` creates a new account with balance 0. Every account has its own balance, even if two
  accounts have the same name.
- `deposit` adds `amount` and records `"<name> +<amount>"` in the ledger's log.
- `withdraw` subtracts `amount` and records `"<name> -<amount>"`, then returns `true`. If the balance
  is smaller than `amount`, it changes nothing, records nothing, and returns `false`.
- `log` returns everything recorded by this ledger's accounts, oldest first.
- `open_accounts` returns how many accounts opened from this ledger still exist (have not been
  dropped).
- An account may be moved anywhere and may even outlive its ledger.

### Example

```rust
let ledger = Ledger::new();
let mut ann = ledger.open("ann");
let mut bob = ledger.open("bob");
ann.deposit(50);
bob.deposit(10);
ann.withdraw(20); // true
bob.withdraw(30); // false
// ledger.log() == ["ann +50", "bob +10", "ann -20"]
// ledger.open_accounts() == 2
```
