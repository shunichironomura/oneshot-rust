The code below does not compile. Fix it.

Account balances are stored in a slice, where the index is the account number. `merge_accounts`
moves all the money of account `from` into account `to`.

```rust
pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize);
```

- It adds `balances[from]` to `balances[to]` and sets `balances[from]` to 0.
- Both indices are less than `balances.len()`.
- If `from == to`, the slice is left unchanged.
- The total of all balances fits in a `u64`, so no addition overflows.

Do not change the signature.

### Examples

- `merge_accounts` with `from = 0` and `to = 2` turns `[5, 0, 7]` into `[0, 0, 12]`.
