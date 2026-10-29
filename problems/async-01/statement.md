The code below does not compile. Fix it.

The test harness runs the function below with its own small executor (only `std` is available, there
is no async runtime).

```rust
pub fn greet(name: String) -> impl Future<Output = String>;
```

- `greet(name)` returns a future that resolves to `Hello, {name}!`, with `{name}` replaced by
  `name`. `name` can be any string, including the empty string.
- The future may be awaited any time after `greet` has returned.

Do not change the signature.

### Examples

- `greet("Ann".to_string())` resolves to `"Hello, Ann!"`.
