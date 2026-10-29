The body of the function in the editor is already written, but the code does not compile. Change
**only the signature** (the `pub fn` line, plus a `where` clause if you want one) so that the test
harness (the `tests.rs` tab) compiles and passes. Do not change the body.

```rust
pub fn map_pair(f: impl Fn(i32) -> i32, pair: (i32, i32)) -> (i32, i32)
```

- `map_pair(f, (x, y))` returns `(f(x), f(y))`, calling `f(x)` first.

The harness calls it with many different closures and an ordinary function. Some of the closures
update the variables they capture. Every one of these calls has to compile.

### Examples

```rust
map_pair(|s: &str| format!("{s}!"), ("a", "b"))  // (String::from("a!"), String::from("b!"))
map_pair(|n: i32| n * n, (3, -4))                // (9, 16)
```
