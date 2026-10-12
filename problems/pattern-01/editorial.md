Each function has its own pattern mistake, and the compiler reports all three in one go.

- `first_even`: a plain `let` needs an **irrefutable** pattern, one that matches every possible value.
  `find` returns an `Option<&i32>`, and `Some(x)` does not match `None` (E0005). The compiler will not
  guess what should happen then, so say it with let-else: `let Some(x) = ... else { return None };`.
  The `else` block must not fall through: it has to `return`, `break`, `continue` or panic, because
  after the statement `x` must hold a value.
- `title_or_default`: a leading underscore only silences the "unused variable" warning. `_t` is still
  a variable, so `Some(_t)` **moves** the `String` out of `title`, and `title.unwrap_or(...)` then
  uses a partially moved value (E0382). The bare `_` is different: it is not a binding at all, so
  `Some(_)` checks the variant without moving anything, and `title` stays whole.
- `filled_slots`: the pattern of a `for` loop must be irrefutable too, just like a `let` (E0005). A
  `for` loop never skips items, so it cannot decide what to do with a `None`. Bind the whole slot and
  test it with `if let` in the body. `slot` is a `&Option<u8>`, so `Some(x)` binds `x` as a `&u8`,
  and `*x` copies the value out.

Function and closure parameters follow the same rule as `let` and `for`: their patterns must be
irrefutable. Refutable patterns belong in `match`, `if let`, `while let` and let-else.

### Alternatives

The same functions written with `Option` methods instead of refutable patterns. In
`title_or_default`, the elements of a tuple are evaluated from left to right, so `is_some` borrows
`title` and is done before `unwrap_or_else` moves it.

```rust
pub fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().copied().find(|x| x % 2 == 0)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    (title.is_some(), title.unwrap_or_else(|| String::from("Untitled")))
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.map(|x| (i, x)))
        .collect()
}
```

Smaller fixes that keep the template's shape also work. `?` returns `None` early just like the
let-else. Matching on `&title` (or writing `Some(ref _t)`) borrows the title instead of moving it.
In the loop, let-else with `continue` skips the empty slots.

```rust
pub fn first_even(v: &[i32]) -> Option<i32> {
    let x = v.iter().find(|x| *x % 2 == 0)?;
    Some(*x)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    let mut given = false;
    if let Some(_t) = &title {
        given = true;
    }
    (given, title.unwrap_or(String::from("Untitled")))
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        let Some(x) = slot else { continue };
        filled.push((i, *x));
    }
    filled
}
```
