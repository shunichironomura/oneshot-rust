Write a declarative macro (`macro_rules!`) `max_of!` that returns the greatest of its arguments. It
is already in the editor as a placeholder rule that accepts any input, followed by a
`pub(crate) use` line that lets other modules call it by path. Replace the rule and keep the
`pub(crate) use` line.

```rust
macro_rules! max_of { /* ... */ }
pub(crate) use max_of;
```

- `max_of!(e1, e2, ...)` takes one or more expressions of the same type `T` and evaluates to the
  greatest of their values, compared with `PartialOrd`. `T` does not have to implement `Copy`,
  `Clone` or `Ord`. (The tests never pass values that cannot be compared, such as NaN.)
- If several arguments are equal to the greatest value, the result is the leftmost of them.
- Each argument is evaluated exactly once, from left to right. A trailing comma is allowed.

The tests call the macro by its path, `crate::user::max_of!`, from another module that has no
`use` declaration for it.

### Examples

```rust
let x: i32 = crate::user::max_of!(3, 9, 4); // 9
```
