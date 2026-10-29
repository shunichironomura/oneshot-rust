The code below does not compile. Fix it.

A row of slots may each hold a small number or be empty. `filled_slots` takes the row apart with a
pattern and lists the filled slots.

```rust
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)>;
```

- It returns the position and the value of every `Some` in `slots`, in order.
- Positions start at 0 and count the `None`s too.
- `Some(0)` is a filled slot holding `0`.

Do not change the signature.

### Examples

| Call | Result |
| --- | --- |
| `filled_slots(&[None, Some(4), Some(7)])` | `[(1, 4), (2, 7)]` |
