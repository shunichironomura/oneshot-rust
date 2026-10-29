**Only `loop` and labeled blocks can `break` with a value.** A `for` loop (like a `while` loop) can
also end on its own, when the iterator runs out, and then there is no value to give. So it always
evaluates to `()`: `break 'search Some((r, c))` is rejected (E0571), and so is a function body that
ends with the loop (E0308). The template never says what to return when nothing is found.

A labeled block `'search: { ... }` can carry a value: `break 'search value` leaves it with that
value, even from inside the nested `for` loops, and when the code reaches its end, the block
evaluates to its final expression, here `None`.

Do not follow the compiler's help blindly: `break 'search;` plus `None` at the end compiles, but
always returns `None`.

### Alternatives

When the labeled block is the whole function body, `return` does the same job, and it is what most
Rust code uses.

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == target {
                return Some((r, c));
            }
        }
    }
    None
}
```

A `break` without a value may name a `for` loop. To keep the template's loops, store the answer in
a variable and leave both loops with `break 'search;`. A plain `break` would leave only the inner
loop, and a match in a later row would overwrite the answer.

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    let mut found = None;
    'search: for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == target {
                found = Some((r, c));
                break 'search;
            }
        }
    }
    found
}
```

With iterators, `position` finds the column within one row, and `find_map` stops at the first row
that has one.

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&value| value == target).map(|c| (r, c)))
}
```
