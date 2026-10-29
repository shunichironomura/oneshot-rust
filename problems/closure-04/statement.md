The function in the editor visits the decimal digits of a number, calling a closure for each of
them. Its body is already written, but the code does not compile. Change **only the signature**
(the `pub fn` line, plus a `where` clause if you want one) so that the test harness (the `tests.rs`
tab) compiles and passes. Do not change the body.

```rust
pub fn each_digit(n: u64, f: impl Fn(u64))
```

- `each_digit(n, f)` calls `f` once for each decimal digit of `n`, from the most significant digit
  to the least significant one. `0` has a single digit, `0`.

The harness passes `&mut` references to closures that push to a `Vec` or update counters, and
passes the same closure to several calls.

### Examples

```rust
let mut digits = Vec::new();
each_digit(4096, &mut |d| digits.push(d));
// digits is now [4, 0, 9, 6]
```
