Both functions hold two `&mut` into one collection at the same time, which is error E0499. To the
borrow checker, `self.accounts.get_mut(from)` borrows the whole map until `src` is last used, and
`&mut balances[from]` borrows "some element" of the slice without knowing which. It cannot prove
that `from != to`, and if they were equal, the two references would point at the same value.

`get_disjoint_mut` (stable since Rust 1.86) checks that at run time and returns several `&mut` at
once. The map and the slice versions handle a repeated key differently:

- `HashMap::get_disjoint_mut([from, to])` returns `[Option<&mut V>; 2]`, with `None` for a missing
  key, and **panics** if both keys name the same entry. The `SameAccount` check must stay in front
  of it.
- `<[T]>::get_disjoint_mut([from, to])` returns a `Result` and gives `Err` for overlapping (or
  out-of-range) indices. Here `from == to` is exactly the case where nothing should happen, so
  `if let Ok(..)` covers it.

Fixing the code by hand with one borrow at a time also works, but watch the order:

- `transfer` must finish every check before it changes anything. Subtracting from `from` and then
  looking up `to` with `?` loses the money when `to` is unknown. Checking the balance before looking
  up `to` returns `InsufficientFunds` where `UnknownAccount` is expected.
- In `merge_accounts`, the line-by-line rewrite `balances[to] += balances[from]; balances[from] = 0;`
  compiles, but when `from == to` it doubles the balance and then sets it to 0. The case the borrow
  checker was guarding against, two references to the same element, is exactly the one that breaks.

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

pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let amount = std::mem::take(&mut balances[from]);
    balances[to] += amount;
}
```

`std::mem::take` leaves 0 behind and returns the old balance, so when `from == to` the balance is
simply put back.

`split_at_mut`, which the compiler suggests, also gives two `&mut` into the slice, but you have to
handle `from < to`, `from > to` and `from == to` separately.
