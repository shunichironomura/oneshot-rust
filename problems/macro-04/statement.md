The code below does not compile. Fix it.

It defines a declarative macro that takes a pattern, and two functions that use it. The macro is
followed by a `pub(crate) use` line that lets other modules call it by path.

```rust
macro_rules! matches_any { /* ... */ }
pub(crate) use matches_any;

pub fn is_vowel(c: char) -> bool;
pub fn is_small(x: Option<u8>) -> bool;
```

- `matches_any!(value, PATTERN)` is `true` if `value` matches `PATTERN`, and `false` otherwise.
  `PATTERN` is any pattern that may appear in a `match` arm, including or-patterns at the top level
  (`1 | 4..=6`) and inside other patterns (`Some(1 | 2)`).
- `value` is an arbitrary expression and is evaluated exactly once.
- `is_vowel(c)` is `true` if `c` is one of the ASCII vowels `a`, `e`, `i`, `o` and `u`, in lowercase
  or uppercase.
- `is_small(x)` is `true` if `x` is `None` or `Some(n)` with `n` from 0 to 9.

The tests call the macro by path from another module, for example
`crate::user::matches_any!(c, 'a' | 'b')`. Do not change the names, the call syntax of the macro or
the signatures of the functions, and keep the `pub(crate) use` line.

### Examples

```rust
crate::user::matches_any!(5, 1 | 4..=6)                // true
crate::user::matches_any!(Some(7), Some(1..=3) | None) // false
is_vowel('E')                                          // true
is_small(Some(12))                                     // false
```
