The bodies of the five functions in the editor are already written, but the code does not compile.
Change **only the signatures** (the `pub fn` lines) so that the test harness (the `tests.rs` tab)
compiles and passes. Do not change the bodies.

- `apply_n(f, n)` calls `f` `n` times (not at all if `n` is `0`).
- `consume(f)` calls `f` once and returns its result.
- `map_pair(f, (x, y))` returns `(f(x), f(y))`, calling `f(x)` first.
- `call_shared(f)` calls `f` twice and returns the sum of the two results, a `u32`.
- `ops_table()` returns the symbols `'+'`, `'-'` and `'*'`, in this order, each paired with a
  function that adds, subtracts or multiplies two `i32` values.

The harness calls them with many different closures and a few ordinary functions. Every one of
these calls has to compile.

### Examples

```rust
let mut count = 0;
apply_n(|| count += 2, 3);
// count is now 6

let name = String::from("Ferris");
consume(move || name)      // String::from("Ferris")
consume(|| vec![1, 2])     // vec![1, 2]

map_pair(|s: &str| format!("{s}!"), ("a", "b"))  // (String::from("a!"), String::from("b!"))
map_pair(|n: i32| n * n, (3, -4))                // (9, 16)

let k = 5;
call_shared(|| k + 1)      // 12

for (op, f) in ops_table() {
    // f(7, 3) is 10 for '+', 4 for '-' and 21 for '*'
}
```
