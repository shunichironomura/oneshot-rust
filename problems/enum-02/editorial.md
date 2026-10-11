Iterating over a `&[Cell]` gives `&Cell`s, and matching a reference against `Cell::Int(n)` binds the
fields by reference: `n` is a `&i64`, `x` a `&f64` and `s` a `&String` (match ergonomics). Most
compile errors in this problem come from forgetting that.

- `n as f64` is rejected: casting `&i64` as `f64` is invalid. Write `*n as f64`. Likewise, in a
  `match` that produces a value, the arm `Cell::Float(x) => x` gives a `&f64` where an `f64` is
  expected.
- In `texts`, `Some(s)` gives an iterator over `&String`, and `collect` cannot build a `Vec<&str>`
  from it. Deref coercion turns `&String` into `&str` only where the target type is already known,
  not inside `collect`. `s.as_str()` converts explicitly.

The slices that `texts` returns point into the `String`s inside `row`. With a single reference
parameter, lifetime elision ties the result to it, as in
`fn texts<'a>(row: &'a [Cell]) -> Vec<&'a str>`, so nothing is copied and the result can be used for
as long as `row` lives.

In `parse_cell`, the order of the checks matters: every string that the `i64` parser accepts is also
accepted by the `f64` parser, so trying `f64` first turns `3` into `Float(3.0)` and `-0` into
`Float(-0.0)`. The standard parsers already handle signs, exponents, `.5`, `5.` and `inf`, and
reject `1_000` and `0x10`, so no hand-written checks are needed. `split(',')` always yields at least
one piece, which is why `""` gives `[Empty]`. `Cell::Text` owns a `String`, so the trimmed `&str`
needs `to_string()`.

In `numeric_sum`, convert each integer to `f64` before adding it. Summing the `Int`s as `i64` first
overflows (a panic in debug builds) for values near `i64::MAX`.

### Alternatives

Matching on `*cell` instead of `cell` binds the fields by value. This works because `i64` and `f64`
are `Copy` and the `_` arm binds nothing. An arm like `Cell::Text(s)` would try to move the `String`
out of the borrowed row and would not compile.

```rust
pub fn numeric_sum(row: &[Cell]) -> f64 {
    row.iter()
        .map(|cell| match *cell {
            Cell::Int(n) => n as f64,
            Cell::Float(x) => x,
            _ => 0.0,
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
        .or_else(|_| field.parse().map(Cell::Float))
        .unwrap_or_else(|_| Cell::Text(field.to_string()))
}
```
