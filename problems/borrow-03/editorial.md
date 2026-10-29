`transfer` holds two `&mut` into one map at the same time, which is error E0499. To the borrow
checker, `self.accounts.get_mut(from)` borrows the whole map until `src` is last used. It cannot
prove that `from != to`, and if they were equal, the two references would point at the same value.

`HashMap::get_disjoint_mut` (stable since Rust 1.86) checks that at run time and returns several
`&mut` at once. `get_disjoint_mut([from, to])` returns `[Option<&mut V>; 2]`, with `None` for a
missing key, and **panics** if both keys name the same entry. The `SameAccount` check must stay in
front of it.

Fixing the code by hand with one borrow at a time also works, but watch the order: `transfer` must
finish every check before it changes anything. Subtracting from `from` and then looking up `to` with
`?` loses the money when `to` is unknown. Checking the balance before looking up `to` returns
`InsufficientFunds` where `UnknownAccount` is expected.

### Alternatives

Without `get_disjoint_mut`: borrow one entry at a time, and validate everything first.

```rust
pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if from == to {
        return Err(TransferError::SameAccount);
    }
    if !self.accounts.contains_key(to) {
        return Err(TransferError::UnknownAccount);
    }
    let src = self.accounts.get_mut(from).ok_or(TransferError::UnknownAccount)?;
    if *src < amount {
        return Err(TransferError::InsufficientFunds);
    }
    *src -= amount;
    *self.accounts.get_mut(to).unwrap() += amount;
    Ok(())
}
```
