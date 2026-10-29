A `match` must be **exhaustive**: the compiler has to prove, from the type and the patterns alone,
that some arm matches every possible value. Otherwise it reports E0004. It does not run your code, so
it knows nothing about which values a function can return. `signum()` returns an `i32`, and the
compiler does not know that it is always `-1`, `0` or `1`, so `(2, 5)` looks uncovered.

`x.cmp(&0)` returns an `Ordering`, an enum with exactly three variants, so the seven arms become
exhaustive. Here `_ => unreachable!()` after the seven `signum` arms would also be correct, because
no other value can occur.

Adding `_ => ...` always silences E0004, but then the compiler no longer checks that case for you.
Before writing one, ask which values actually reach it and what they should produce.

### Alternatives

A fix closer to the template keeps `signum` and adds a catch-all arm, which is safe here.

```rust
pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x.signum(), y.signum()) {
        (1, 1) => "I",
        (-1, 1) => "II",
        (-1, -1) => "III",
        (1, -1) => "IV",
        (0, 0) => "origin",
        (0, _) => "y-axis",
        (_, 0) => "x-axis",
        _ => unreachable!("signum() is always -1, 0 or 1"),
    }
}
```

`quadrant` can also match `(x, y)` with range patterns directly. Once the axes are handled first,
the four quadrants are the only cases left, and the compiler can see that.

```rust
pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x, y) {
        (0, 0) => "origin",
        (0, _) => "y-axis",
        (_, 0) => "x-axis",
        (1.., 1..) => "I",
        (..0, 1..) => "II",
        (..0, ..0) => "III",
        (1.., ..0) => "IV",
    }
}
```
