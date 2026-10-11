The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `Bank` keeps the balance of each account by name (the names given to `new` are all different).
`balance` returns the balance of an account, or `None` for an unknown name. `transfer` moves
`amount` from account `from` to account `to`, checking in this order:

1. If `from == to`, it returns `Err(TransferError::SameAccount)`.
2. If `from` or `to` is not an existing account, it returns `Err(TransferError::UnknownAccount)`.
3. If the balance of `from` is less than `amount`, it returns
   `Err(TransferError::InsufficientFunds)`.
4. Otherwise, it subtracts `amount` from `from`, adds it to `to` and returns `Ok(())`.

When `transfer` returns an error, no balance changes.

`merge_accounts` works on balances stored in a slice, where the index is the account number. It adds
`balances[from]` to `balances[to]` and sets `balances[from]` to 0. Both indices are less than
`balances.len()`. If `from == to`, the slice is left unchanged.

In a `Bank` and in the slice alike, the total of all balances fits in a `u64`, so no addition
overflows.

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

pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize);
```

Do not change these signatures.

### Examples

- With `let mut bank = Bank::new(&[("a", 10), ("b", 0)]);`, `bank.transfer("a", "b", 7)` returns
  `Ok(())`, and then `bank.balance("a")` is `Some(3)` and `bank.balance("b")` is `Some(7)`.
  `bank.transfer("a", "a", 1)` returns `Err(TransferError::SameAccount)`.
- `merge_accounts` with `from = 0` and `to = 2` turns `[5, 0, 7]` into `[0, 0, 12]`.
