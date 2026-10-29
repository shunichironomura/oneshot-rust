`$first:expr $(, $rest:expr)* $(,)?` matches one expression followed by any number of
comma-separated ones, and an optional trailing comma. In the transcriber, `$( ... )*` repeats once
per matched `$rest`.

A macro used as an expression must expand to one expression. To run several statements, expand to a
block: in `=> {{ ... }}`, the outer braces delimit the transcriber and the inner ones are the block.
Local variables declared there (`max`, `next`) are hygienic: they never clash with the caller's
variables, even ones with the same names.

The solution binds each argument to a local exactly once and replaces the current maximum only when
the next value is strictly greater, so the leftmost of equal values wins. Shadowing with
`let max = ...` avoids `let mut`, which would warn about an unused `mut` when there is only one
argument. The tempting `if $a > max_of!($($rest),+) { $a } else { max_of!($($rest),+) }` expands
the rest twice: side effects run twice, and a non-`Copy` variable is used after it has been moved.
`Iterator::max` and `std::cmp::max` need `Ord`, which `f64` does not implement, and both return the
*last* of equal maxima.

### Alternatives

`max_of!` can be recursive, too, if each argument is bound to a local once. The recursive call has
to go through `$crate`: paths in the expansion are resolved where the macro is called, and a bare
`max_of!(...)` is not found in the tests module, which calls the macro by path and does not import
it. `$crate` always names the crate that defines the macro.

```rust
macro_rules! max_of {
    ($only:expr $(,)?) => {
        $only
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let first = $first;
        let rest = $crate::user::max_of!($($rest),+);
        if rest > first { rest } else { first }
    }};
}
pub(crate) use max_of;
```
