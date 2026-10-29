The code below does not compile. Fix it.

A `Bank` keeps the balance of each account by name (the names given to `new` are all different).
`balance` returns the balance of an account, or `None` for an unknown name. `transfer` moves
`amount` from account `from` to account `to`, checking in this order:

1. If `from == to`, it returns `Err(TransferError::SameAccount)`.
2. If `from` or `to` is not an existing account, it returns `Err(TransferError::UnknownAccount)`.
3. If the balance of `from` is less than `amount`, it returns
   `Err(TransferError::InsufficientFunds)`.
4. Otherwise, it subtracts `amount` from `from`, adds it to `to` and returns `Ok(())`.

When `transfer` returns an error, no balance changes.

The total of all balances fits in a `u64`, so no addition overflows.

```rust
#[derive(Debug, PartialEq)]
pub enum TransferError {
    UnknownAccount,
    SameAccount,
    InsufficientFunds,
}

impl Bank {
    pub fn new(accounts: &[(&str, u64)]) -> Self;
    pub fn balance(&self, name: &str) -> Option<u64>;
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), TransferError>;
}
```

Do not change these signatures.

### Examples

- With `let mut bank = Bank::new(&[("a", 10), ("b", 0)]);`, `bank.transfer("a", "b", 7)` returns
  `Ok(())`, and then `bank.balance("a")` is `Some(3)` and `bank.balance("b")` is `Some(7)`.
  `bank.transfer("a", "a", 1)` returns `Err(TransferError::SameAccount)`.
