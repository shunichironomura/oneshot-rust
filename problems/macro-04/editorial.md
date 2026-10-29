`matches_any!` uses the matcher `$($p:pat)|+`, one or more patterns separated by `|`. That is how
`matches!` and similar macros were written before Rust 2021, when a `pat` fragment did not match a
`|` at the top level. Since edition 2021, `pat` matches a whole or-pattern: `1 | 4..=6` is a single
`pat`. A `|` right after a `pat` fragment would be ambiguous, so rustc rejects the definition itself
("`$p:pat` is followed by `|`, which is not allowed for `pat` fragments"). A `pat` may only be
followed by `=>`, `,`, `=`, `if` or `in`. The fix is to take the whole pattern as one `$p:pat`,
which is also how `matches!` is defined today.

### Alternatives

`pat_param` keeps the old meaning of `pat`: a pattern without a top-level `|`. It may be followed by
`|`, so the original repetition works once the fragment specifier is changed.

```rust
macro_rules! matches_any {
    ($value:expr, $($p:pat_param)|+) => {
        match $value {
            $($p)|+ => true,
            _ => false,
        }
    };
}
pub(crate) use matches_any;
```
