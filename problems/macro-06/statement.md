Write a declarative macro (`macro_rules!`) `count!` that counts token trees. It is already in the
editor as a placeholder rule that accepts any input, followed by a `pub(crate) use` line that lets
other modules call it by path. Replace the rule and keep the `pub(crate) use` line.

```rust
macro_rules! count { /* ... */ }
pub(crate) use count;
```

- `count!(...)` evaluates to the number of token trees in its input, as a `usize`. It must be a
  constant expression, usable in `const N: usize = count!(...);` or as an array length.
- A token tree is either a single token (an identifier or keyword, a literal, a lifetime, or a
  punctuation token such as `+`, `,`, `::` or `=>`) or a group in `()`, `[]` or `{}` together with
  everything inside it. `count!()` is `0`.

The tests call the macro by its path, `crate::user::count!`, from another module that has no `use`
declaration for it.

### Examples

```rust
const N: usize = crate::user::count!(a + (b c)); // 3: `a`, `+` and `(b c)`
const Z: usize = crate::user::count!(); // 0
```
