Each slip needs only a small fix, but the compiler does not show all five at once.

1. **A `const` needs a type.** Unlike `let`, a `const` (or `static`) item never infers its type, so
   `const BASE = 10;` is an error. The type must be `u64`: `BASE` is compared with `n: u64` and used
   in `rest % BASE`, and Rust never converts between integer types implicitly. The compiler suggests
   `: i32`, the default type of an integer literal, which only leads to new type errors.
2. **Parameters are immutable.** A parameter is a binding just like `let n`, so `n = sum` is
   rejected. Write `mut n: u64` in the parameter list (the function's type stays the same), or shadow
   it with `let mut n = n;` at the start of the body. Writing `let n = sum;` inside the loop compiles,
   but it declares a new `n` that disappears at the end of each iteration: the condition `n >= BASE`
   keeps seeing the original value, and the loop never ends.
3. **`n;` is a statement.** A function returns the value of its final expression, which has no
   semicolon. With `n;` the body ends with a statement and evaluates to `()`.
4. **All branches of an `if` must have the same type.** Three branches are `&str` and the last one is
   a `String`. Turn the literals into `String`s with `.to_string()` (or `String::from`). The `if`
   can then be the final expression of the function by itself. The compiler suggests borrowing the
   last branch as `&n.to_string()` instead, but then the whole `if` is a `&str`, and the error only
   moves to `label`, which must be returned as a `String`.
5. **`break` can carry a value.** A `loop` is an expression, and `break value` gives its value. A
   bare `break;` gives `()`, so it must be `break x;`.

The first compile reports only four errors. Assigning to the immutable `n` is caught by the borrow
checker, which looks at a function only after it passes type checking, and `digital_root` does not
pass it yet: it uses the untyped `BASE` and ends with `n;`. Fixing exactly what the compiler shows
costs one more submission.

### Alternatives

Some of the slips can be avoided altogether. A `match` on both remainders states the FizzBuzz rules
directly, and the next multiple can be computed without a loop: `limit / step * step` is the largest
multiple of `step` that is not greater than `limit`, so the answer is one `step` more.

```rust
pub fn fizz_label(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

pub fn first_multiple_above(step: u64, limit: u64) -> u64 {
    (limit / step + 1) * step
}
```

The digital root also has a closed form. A number and the sum of its digits leave the same remainder
when divided by 9, so for `n > 0` the result is `1 + (n - 1) % 9`. `0` needs its own branch, because
`n - 1` would underflow.

```rust
pub fn digital_root(n: u64) -> u64 {
    if n == 0 { 0 } else { 1 + (n - 1) % 9 }
}
```
