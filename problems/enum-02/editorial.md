Iterating over a `&[Cell]` gives `&Cell`s, and matching a reference against `Cell::Int(n)` binds the
fields by reference: `n` is a `&i64` and `s` a `&String` (match ergonomics). Most compile errors in
this problem come from forgetting that.

- In a `match` that produces a value, the arm `Cell::Int(n) => n` gives a `&i64` where an `i64` is
  expected. Write `*n`. (`sum += n` happens to compile, because `i64` also implements
  `AddAssign<&i64>`.)
- In `texts`, `Some(s)` gives an iterator over `&String`, and `collect` cannot build a `Vec<&str>`
  from it. Deref coercion turns `&String` into `&str` only where the target type is already known,
  not inside `collect`. `s.as_str()` converts explicitly.

The slices that `texts` returns point into the `String`s inside `row`. With a single reference
parameter, lifetime elision ties the result to it, as in
`fn texts<'a>(row: &'a [Cell]) -> Vec<&'a str>`, so nothing is copied and the result can be used for
as long as `row` lives.

In `parse_cell`, `split(',')` always yields at least one piece, which is why `""` gives `[Empty]`.
`Cell::Text` owns a `String`, so the trimmed `&str` needs `to_string()`.

### Alternatives

Matching on `*cell` instead of `cell` binds the fields by value. This works because `i64` is `Copy`
and the `_` arm binds nothing. An arm like `Cell::Text(s)` would try to move the `String` out of the
borrowed row and would not compile.

```rust
pub fn numeric_sum(row: &[Cell]) -> i64 {
    row.iter()
        .map(|cell| match *cell {
            Cell::Int(n) => n,
            _ => 0,
        })
        .sum()
}
```

`texts` can be written as a loop. With `let mut out: Vec<&str>`, even `out.push(s)` would compile,
because the parameter type of `push` is known and `&String` is coerced to `&str`. Without the
annotation, the first `push` makes `out` a `Vec<&String>`, which does not match the return type.

```rust
pub fn texts(row: &[Cell]) -> Vec<&str> {
    let mut out = Vec::new();
    for cell in row {
        if let Cell::Text(s) = cell {
            out.push(s.as_str());
        }
    }
    out
}
```

A tuple variant's name is also a function, such as `Cell::Int: fn(i64) -> Cell`, so it can be passed
to `Result::map`. This also tells `parse` which type to produce, without a turbofish.

```rust
fn parse_cell(field: &str) -> Cell {
    if field.is_empty() {
        return Cell::Empty;
    }
    field
        .parse()
        .map(Cell::Int)
        .unwrap_or_else(|_| Cell::Text(field.to_string()))
}
```
