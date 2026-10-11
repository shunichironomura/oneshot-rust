`matches_any!` uses the matcher `$($p:pat)|+`, one or more patterns separated by `|`. That is how
`matches!` and similar macros were written before Rust 2021, when a `pat` fragment did not match a
`|` at the top level. Since edition 2021, `pat` matches a whole or-pattern: `1 | 4..=6` is a single
`pat`. A `|` right after a `pat` fragment would be ambiguous, so rustc rejects the definition itself
("`$p:pat` is followed by `|`, which is not allowed for `pat` fragments"). A `pat` may only be
followed by `=>`, `,`, `=`, `if` or `in`. The fix is to take the whole pattern as one `$p:pat`,
which is also how `matches!` is defined today.

`index_of!` is a tt muncher. The last rule evaluates `value` once with `match` and starts the
internal `@check` rules at index 0. Each `@check` step tests one pattern; if it does not match, the
macro calls itself with the remaining tokens and the index plus one, and an empty list gives `None`.
In the matcher of that step, `$(, $($rest)*)?` lacks a fragment specifier. In a matcher, every
metavariable needs one (`$rest:tt`); only the transcriber writes `$rest` alone. Calling a rule like
this has always failed, and since Rust 1.89 the definition is rejected even if the rule is never
used ("missing fragment specifier").

Leave `$p:pat` in `index_of!` as it is. It is followed by `,`, which is allowed, and since `pat`
accepts or-patterns, `'a'..='z' | 'A'..='Z', '0'..='9'` is split at the commas only. With
`pat_param`, `char_class` would no longer compile.

The rest of the template is already right. Binding `value` with `match` keeps the temporaries of the
expression alive until the end of the `match`, and the recursive call goes through `$crate`, so the
tests can call the macro by path from another module.

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

`index_of!` also works without recursion. Match the comma-separated patterns with
`$($p:pat),+ $(,)?`, test each of them into an array of `bool`s, and take the position of the first
`true`. Every pattern is tested, but testing a pattern has no side effects, and `value` is still
evaluated only once.

```rust
macro_rules! index_of {
    ($value:expr; $($p:pat),+ $(,)?) => {
        match $value {
            value => [$(matches!(value, $p)),+].iter().position(|&hit| hit),
        }
    };
}
pub(crate) use index_of;
```
