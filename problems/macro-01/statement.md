Write three declarative macros (`macro_rules!`). Each one is already in the editor as a placeholder
rule that accepts any input, followed by a `pub(crate) use` line that lets other modules call it by
path. Replace the rules and keep the `pub(crate) use` lines.

```rust
macro_rules! hashmap { /* ... */ }
pub(crate) use hashmap;

macro_rules! max_of { /* ... */ }
pub(crate) use max_of;

macro_rules! count { /* ... */ }
pub(crate) use count;
```

`hashmap!` builds a `std::collections::HashMap`.

- `hashmap! { k1 => v1, k2 => v2, ... }` evaluates to a map with these entries. Keys and values
  are arbitrary expressions, and a trailing comma is allowed.
- `hashmap! {}` evaluates to an empty map. Its key and value types come from the context, such as
  a type annotation.
- The entries are inserted from left to right, so when a key appears more than once, its last
  value wins.
- Each key and each value is evaluated exactly once, in the order `k1`, `v1`, `k2`, `v2`, ...

`max_of!` returns the greatest of its arguments.

- `max_of!(e1, e2, ...)` takes one or more expressions of the same type `T` and evaluates to the
  greatest of their values, compared with `PartialOrd`. `T` does not have to implement `Copy`,
  `Clone` or `Ord`. (The tests never pass values that cannot be compared, such as NaN.)
- If several arguments are equal to the greatest value, the result is the leftmost of them.
- Each argument is evaluated exactly once, from left to right. A trailing comma is allowed.

`count!` counts token trees.

- `count!(...)` evaluates to the number of token trees in its input, as a `usize`. It must be a
  constant expression, usable in `const N: usize = count!(...);` or as an array length.
- A token tree is either a single token (an identifier, a literal, a lifetime, or a punctuation
  token such as `+`, `,`, `::` or `=>`) or a group in `()`, `[]` or `{}` together with everything
  inside it. `count!()` is `0`.

The tests call each macro by its path, for example `crate::user::max_of!(3, 9, 4)`, from another
module that has no `use` declarations for the macros or for `HashMap`.

### Examples

```rust
let m: std::collections::HashMap<&str, i32> = crate::user::hashmap! { "a" => 1, "b" => 2, };
// m has 2 entries, and m["b"] is 2

let x: i32 = crate::user::max_of!(3, 9, 4); // 9

const N: usize = crate::user::count!(a + (b c)); // 3: `a`, `+` and `(b c)`
const Z: usize = crate::user::count!(); // 0
```
