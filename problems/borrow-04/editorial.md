The code holds two `&mut` into one slice at the same time, which is error E0499. To the borrow
checker, `&mut balances[from]` borrows "some element" of the slice without knowing which. It cannot
prove that `from != to`, and if they were equal, the two references would point at the same value.

`<[T]>::get_disjoint_mut([from, to])` (stable since Rust 1.86) checks that at run time and returns
several `&mut` at once. It returns a `Result` and gives `Err` for overlapping (or out-of-range)
indices. Here `from == to` is exactly the case where nothing should happen, so `if let Ok(..)`
covers it.

Fixing the code line by line also compiles, but watch the case `from == to`:
`balances[to] += balances[from]; balances[from] = 0;` doubles the balance and then sets it to 0.
The case the borrow checker was guarding against, two references to the same element, is exactly
the one that breaks.

### Alternatives

`std::mem::take` leaves 0 behind and returns the old balance, so when `from == to` the balance is
simply put back.

```rust
pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let amount = std::mem::take(&mut balances[from]);
    balances[to] += amount;
}
```

`split_at_mut`, which the compiler suggests, also gives two `&mut` into the slice, but you have to
handle `from < to`, `from > to` and `from == to` separately.
