Each function breaks a different rule about leaving a block early. The compiler reports all of them
in the first compile, but in `find_cell` its suggestions lead to wrong code.

1. **Only `loop` and labeled blocks can `break` with a value.** A `for` loop (like a `while` loop)
   can also end on its own, when the iterator runs out, and then there is no value to give. So it
   always evaluates to `()`: `break 'search Some((r, c))` is rejected (E0571), and so is a function
   body that ends with the loop (E0308). The template never says what to return when nothing is
   found. A labeled block `'search: { ... }` can carry a value: `break 'search value` leaves it with
   that value, even from inside the nested `for` loops, and when the code reaches its end, the block
   evaluates to its final expression, here `None`. Do not follow the compiler's help blindly:
   `break 'search;` plus `None` at the end compiles, but always returns `None`.
2. **The `else` of let-else must diverge.** When the pattern does not match, `key` and `value` have
   no value, so the code after the statement must not run. The `else` block has to leave with
   `return`, `break` or `continue`, or panic: its type must be `!`. A bare `None` is only the value of
   the block, not a return from the function. Write `return None;`.
3. **An unlabeled `break` never leaves a labeled block.** A plain `break` (or `continue`) refers to
   the innermost loop. So that it is always clear what a `break` leaves, Rust rejects an unlabeled
   `break` that is inside a labeled block but not inside a loop within it (E0695), even when there is
   no loop around the block. Write `break 'check "empty"`. The `break 'check "negative"` inside the
   `for` loop was right all along: its target is the block, not the loop. A plain `break` there
   would compile, but it would leave only the `for` loop.

The key returned by `parse_setting` borrows `line`. By lifetime elision, the returned `&str` has the
lifetime of the only reference parameter, and `split_once` and `trim` return slices of the string
they are given, so no `String` is needed.

### Alternatives

When the labeled block is the whole function body, `return` does the same job, and it is what most
Rust code uses. `classify` also reads well as an `if` / `else if` chain.

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

pub fn classify(xs: &[i32]) -> &'static str {
    if xs.is_empty() {
        "empty"
    } else if xs.iter().any(|&x| x < 0) {
        "negative"
    } else if xs.contains(&0) {
        "zero"
    } else {
        "positive"
    }
}
```

`?` works on an `Option` too: it returns `None` from the function when the value is `None`. It can
replace the let-else in `parse_setting`, and the type of the parsed value is inferred from the
return type.

```rust
pub fn parse_setting(line: &str) -> Option<(&str, u32)> {
    let (key, value) = line.split_once('=')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    Some((key, value.trim().parse().ok()?))
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
