The code below does not compile. Fix it.

It defines two declarative macros that take patterns, and three functions that use them. Each
macro is followed by a `pub(crate) use` line that lets other modules call it by path.

```rust
macro_rules! matches_any { /* ... */ }
pub(crate) use matches_any;

macro_rules! index_of { /* ... */ }
pub(crate) use index_of;

pub fn is_vowel(c: char) -> bool;
pub fn is_small(x: Option<u8>) -> bool;
pub fn char_class(c: char) -> Option<usize>;
```

- `matches_any!(value, PATTERN)` is `true` if `value` matches `PATTERN`, and `false` otherwise.
  `PATTERN` is any pattern that may appear in a `match` arm, including or-patterns at the top level
  (`1 | 4..=6`) and inside other patterns (`Some(1 | 2)`).
- `index_of!(value; P0, P1, ...)` takes one or more such patterns separated by commas, and a
  trailing comma is allowed. The result is an `Option<usize>`: `Some(i)` for the first pattern
  `Pi` (counting from 0) that `value` matches, or `None` if `value` matches none of them.
- In both macros, `value` is an arbitrary expression and is evaluated exactly once.
- `is_vowel(c)` is `true` if `c` is one of the ASCII vowels `a`, `e`, `i`, `o` and `u`, in lowercase
  or uppercase.
- `is_small(x)` is `true` if `x` is `None` or `Some(n)` with `n` from 0 to 9.
- `char_class(c)` is `Some(0)` for an ASCII letter, `Some(1)` for an ASCII digit, `Some(2)` for a
  space (`' '`), a tab (`'\t'`) or a newline (`'\n'`), and `None` for any other character.

The tests call the macros by path from another module, for example
`crate::user::index_of!(c; 'a', 'b')`. Do not change the names, the call syntax of the macros or the
signatures of the functions, and keep the `pub(crate) use` lines.

### Examples

```rust
crate::user::matches_any!(5, 1 | 4..=6)                // true
crate::user::matches_any!(Some(7), Some(1..=3) | None) // false
crate::user::index_of!(7; 0..=4, 5..=9, _)             // Some(1)
is_vowel('E')                                          // true
is_small(Some(12))                                     // false
char_class('7')                                        // Some(1)
```
