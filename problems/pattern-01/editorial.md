The pattern of a `for` loop must be **irrefutable**, one that matches every possible value, just
like the pattern of a plain `let` (E0005). `(i, Some(x))` does not match a slot that is `None`, and a
`for` loop never skips items, so the compiler cannot decide what to do with it. Bind the whole slot
and test it with `if let` in the body. `slot` is a `&Option<u8>`, so `Some(x)` binds `x` as a `&u8`,
and `*x` copies the value out.

Function and closure parameters follow the same rule as `let` and `for`: their patterns must be
irrefutable. Refutable patterns belong in `match`, `if let`, `while let` and let-else.

### Alternatives

let-else with `continue` skips the empty slots. The `else` block must not fall through: it has to
`return`, `break`, `continue` or panic, because after the statement `x` must hold a value.

```rust
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        let Some(x) = slot else { continue };
        filled.push((i, *x));
    }
    filled
}
```

With iterator adapters, `filter_map` does the test, and `Option::map` builds the pair. Enumerate
before filtering, so that the positions still count the empty slots.

```rust
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.map(|x| (i, x)))
        .collect()
}
```
