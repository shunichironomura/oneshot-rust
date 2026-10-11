Every value in this syntax is a single token tree: `null`, `true`, `3` and `"s"` are single tokens,
and `[...]`, `{...}` and `(...)` are each one group, whatever is inside. So the rule
`[$($element:tt),* $(,)?]` splits an array at its top-level commas, the rule
`{$($key:literal : $value:tt),* $(,)?}` splits an object into pairs, and each element goes back
into `json!`, which handles the nested arrays and objects. This is also why other expressions need
parentheses: `-1` and `x - 7` are several token trees, while `(x - 7)` is one.

`expr` fragments do not work here. `{ "a": 1 }` is not a Rust expression, and once the parser of a
fragment fails, `macro_rules!` reports the error instead of trying the next rule. Worse, a fragment
captured as `expr` is opaque when it is passed on to another macro call: in `json!($element)`, it
no longer matches the rules `(null)` or `([...])`, so `[true, null]` falls through to the last rule
as `Json::from([true, null])`. Only `tt`, `ident` and `lifetime` fragments can still be
matched by tokens. `expr` cannot even be used for the keys: an `expr` fragment may only be followed
by `=>`, `,` or `;`, so a macro with the matcher `$key:expr : $value:tt` does not compile at all.
`literal` and `tt` have no such restriction.

Rules are tried from top to bottom, and the first one that matches is used. `$other:tt` matches any
value, including `null`, `[...]` and `{...}`, so it must come last. Placed above `(null)`, it would
turn `json!(null)` into `Json::from(null)`: "cannot find value `null` in this scope".

The last rule needs no case for each kind of literal. With the given impls, `Json::from` converts
`true`, `3` (of the impls, only `From<i32>` takes an integer), `2.5`, `"s"` and every parenthesized
expression, and a `Json` goes through the standard `impl<T> From<T> for T`. Every element becomes
one element of `vec![...]`, and those are evaluated from left to right, so each expression is
evaluated once, in the written order.

The expansion is resolved where the macro is called, and the tests import neither `Json` nor
`json`. So the expansion names the type as `$crate::user::Json` and calls itself as
`$crate::user::json!`; a bare `json!(...)` gives "cannot find macro `json` in this scope".
`::std::vec!` and `::std::string::String` are absolute paths for the same reason: they keep
working even where the caller has its own `vec` or `String`.

### Alternatives

A rule for each kind of value. `(($e:expr))` matches a group in parentheses with one expression in
it, and drops the parentheses (in the array rule, the group is still one token tree).
`$lit:literal` matches number and string literals, and also `true` and `false`. These rules do not
overlap, so their order does not matter, and an expression without parentheses, such as `x`, is
reported as "no rules expected `x`".

```rust
macro_rules! json {
    (null) => {
        $crate::user::Json::Null
    };
    ([$($element:tt),* $(,)?]) => {
        $crate::user::Json::Arr(::std::vec![$($crate::user::json!($element)),*])
    };
    ({$($key:literal : $value:tt),* $(,)?}) => {
        $crate::user::Json::Obj(::std::vec![
            $((::std::string::String::from($key), $crate::user::json!($value))),*
        ])
    };
    (($e:expr)) => {
        $crate::user::Json::from($e)
    };
    ($lit:literal) => {
        $crate::user::Json::from($lit)
    };
}
pub(crate) use json;
```
