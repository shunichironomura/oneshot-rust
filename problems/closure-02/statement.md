The bodies of the five functions in the editor are already written, but the code does not compile.
Change **only the signatures** (the `pub fn` lines, plus `where` clauses if you want them) so that
the test harness (the `tests.rs` tab) compiles and passes. Do not change the bodies.

- `apply_n(f, n)` calls `f` `n` times (not at all if `n` is `0`).
- `consume(f)` calls `f` once and returns its result.
- `map_pair(f, (x, y))` returns `(f(x), f(y))`, calling `f(x)` first.
- `call_shared(f)` calls `f` twice and returns the sum of the two results, a `u32`.
- `each_digit(n, f)` calls `f` once for each decimal digit of `n`, from the most significant digit
  to the least significant one. `0` has a single digit, `0`.

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

call_shared(|| 2)          // 4
let k = 5;
call_shared(|| k + 1)      // 12

let mut digits = Vec::new();
each_digit(4096, &mut |d| digits.push(d));
// digits is now [4, 0, 9, 6]
```
