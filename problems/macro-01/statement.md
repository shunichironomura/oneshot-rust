Write a declarative macro (`macro_rules!`) `hashmap!` that builds a `std::collections::HashMap`.
It is already in the editor as a placeholder rule that accepts any input, followed by a
`pub(crate) use` line that lets other modules call it by path. Replace the rule and keep the
`pub(crate) use` line.

```rust
macro_rules! hashmap { /* ... */ }
pub(crate) use hashmap;
```

- `hashmap! { k1 => v1, k2 => v2, ... }` evaluates to a map with these entries. Keys and values
  are arbitrary expressions, and a trailing comma is allowed.
- `hashmap! {}` evaluates to an empty map. Its key and value types come from the context, such as
  a type annotation.
- The entries are inserted from left to right, so when a key appears more than once, its last
  value wins.
- Each key and each value is evaluated exactly once, in the order `k1`, `v1`, `k2`, `v2`, ...

The tests call the macro by its path, `crate::user::hashmap!`, from another module that has no
`use` declarations for the macro or for `HashMap`.

### Examples

```rust
let m: std::collections::HashMap<&str, i32> = crate::user::hashmap! { "a" => 1, "b" => 2, };
// m has 2 entries, and m["b"] is 2
```
