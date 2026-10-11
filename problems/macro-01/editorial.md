A `macro_rules!` rule is a matcher and a transcriber. `$($key:expr => $value:expr),+` matches one or
more `key => value` pairs separated by commas, and `$(,)?` accepts an optional trailing comma. In the
transcriber, `$( map.insert($key, $value); )+` repeats once per matched pair. A repetition must
contain a metavariable from the matcher, which is why `0usize $(+ 1)*` is rejected ("attempted to
repeat an expression containing no syntax variables").

A macro used as an expression must expand to one expression. To run several statements, expand to a
block: in `=> {{ ... }}`, the outer braces delimit the transcriber and the inner ones are the block.
Local variables declared there (`map`, `max`, `next`) are hygienic: they never clash with the
caller's variables, even ones with the same names.

Paths in the expansion, on the other hand, are resolved where the macro is called. The tests do not
import `HashMap`, so a bare `HashMap::new()` fails to compile; `::std::collections::HashMap::new()`
works anywhere. Macro names behave the same way. A `macro_rules!` macro can be called by its bare
name only below its definition (in the same module or its submodules), or where it is imported. When
the tests call `crate::user::count!` by path, a recursive `count!(...)` in the expansion is looked
up in the tests module and is not found ("cannot find macro `count` in this scope"). `$crate` always
names the crate that defines the macro, so `$crate::user::count!` works from anywhere.

`max_of!` binds each argument to a local exactly once and replaces the current maximum only when the
next value is strictly greater, so the leftmost of equal values wins. Shadowing with `let max = ...`
avoids `let mut`, which would warn about an unused `mut` when there is only one argument. The
tempting `if $a > max_of!($($rest),+) { $a } else { max_of!($($rest),+) }` expands the rest twice:
side effects run twice, and a non-`Copy` variable is used after it has been moved. `Iterator::max`
and `std::cmp::max` need `Ord`, which `f64` does not implement, and both return the *last* of equal
maxima.

`count!` must be a constant expression, so `vec![...].len()` does not work: a `Vec` cannot be built
in a constant. `1usize + $crate::user::count!($($tail)*)` is constant. Each token tree adds one level
of recursion, and the default recursion limit is 128, which is plenty here. Matching with `$t:tt`
takes a whole group such as `(b c)` as one token tree.

### Alternatives

`HashMap::from` takes an array of pairs. Array elements are evaluated from left to right, and later
duplicates overwrite earlier ones. For `hashmap! {}`, the type of the empty array is inferred from
the context, so a single rule is enough.

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        ::std::collections::HashMap::from([$(($key, $value)),*])
    };
}
pub(crate) use hashmap;
```

`max_of!` can be recursive, too, if each argument is bound to a local once and the recursive call
goes through `$crate`.

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

`count!` without recursion: turn each token tree into a string with `stringify!` and take the length
of the slice. `<[&str]>::len` is a `const fn`. Writing `[...].len()` instead fails for `count!()`,
because the element type of an empty array is unknown.

```rust
macro_rules! count {
    ($($t:tt)*) => {
        <[&str]>::len(&[$(stringify!($t)),*])
    };
}
pub(crate) use count;
```
