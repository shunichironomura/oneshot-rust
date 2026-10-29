Each slip needs only a small fix, but the compiler does not show all three at once.

1. **A `const` needs a type.** Unlike `let`, a `const` (or `static`) item never infers its type, so
   `const BASE = 10;` is an error. The type must be `u64`: `BASE` is compared with `n: u64` and used
   in `rest % BASE`, and Rust never converts between integer types implicitly. The compiler suggests
   `: i32`, the default type of an integer literal, which only leads to new type errors.
2. **Parameters are immutable.** A parameter is a binding just like `let n`, so `n = sum` is
   rejected. Write `mut n: u64` in the parameter list, or shadow it with `let mut n = n;` at the start
   of the body. `mut n: u64` does not change the signature: a `mut` on a parameter only makes the
   binding `n` inside the function mutable and is not part of the function's type, which stays
   `fn(u64) -> u64`. The argument is passed by value, so callers cannot tell the difference, and
   rustdoc leaves this `mut` out. Writing `let n = sum;` inside the loop compiles,
   but it declares a new `n` that disappears at the end of each iteration: the condition `n >= BASE`
   keeps seeing the original value, and the loop never ends.
3. **`n;` is a statement.** A function returns the value of its final expression, which has no
   semicolon. With `n;` the body ends with a statement and evaluates to `()`.

The first compile reports only two errors. Assigning to the immutable `n` is caught by the borrow
checker, which looks at a function only after it passes type checking, and `digital_root` does not
pass it yet: it uses the untyped `BASE` and ends with `n;`. Fixing exactly what the compiler shows
costs one more submission.

### Alternatives

The digital root also has a closed form. A number and the sum of its digits leave the same remainder
when divided by 9, so for `n > 0` the result is `1 + (n - 1) % 9`. `0` needs its own branch, because
`n - 1` would underflow.

```rust
pub fn digital_root(n: u64) -> u64 {
    if n == 0 { 0 } else { 1 + (n - 1) % 9 }
}
```
