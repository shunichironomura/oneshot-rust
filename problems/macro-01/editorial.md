A `macro_rules!` rule is a matcher and a transcriber. `$($key:expr => $value:expr),+` matches one
or more `key => value` pairs separated by commas, and `$(,)?` accepts an optional trailing comma.
In the transcriber, `$( map.insert($key, $value); )+` repeats once per matched pair.

A macro used as an expression must expand to one expression. To run several statements, expand to a
block: in `=> {{ ... }}`, the outer braces delimit the transcriber and the inner ones are the block.
A local variable declared there, such as `map`, is hygienic: it never clashes with the caller's
variables, even ones with the same name.

Paths in the expansion, on the other hand, are resolved where the macro is called. The tests do not
import `HashMap`, so a bare `HashMap::new()` fails to compile; `::std::collections::HashMap::new()`
works anywhere.

### Alternatives

`HashMap::from` takes an array of pairs. Array elements are evaluated from left to right. For equal
keys, the documentation only promises that one of the values is kept; the current standard library
inserts the pairs in order, so the last one wins, but the `insert` version above does not depend on
this. For `hashmap! {}`, the type of the empty array is inferred from the context, so a single rule
is enough.

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        ::std::collections::HashMap::from([$(($key, $value)),*])
    };
}
pub(crate) use hashmap;
```
