The error comes from **variance**: the rules that say when a type with one lifetime may be used as
the same type with another lifetime.

The helper `push_unique` is fine; the local variable is the problem. `all` is declared as
`Vec<&'static str>`, so `&mut all` is a `&mut Vec<&'static str>`. `&mut T` is *invariant* in `T`: it
cannot be passed as a `&mut Vec<&'a str>` with a shorter `'a`, because the callee could then push a
short-lived string into a vector that promises to hold only `'static` ones. So `'a` is forced to be
`'static`, and `word`, which borrows `extra`, does not live that long. No signature of `push_unique`
can fix this, because a `Vec<&'static str>` simply cannot hold `word`.

Write `Vec<&str>` instead, or leave the annotation out. Shared references and `Vec` are *covariant*:
a `&'static str` can be used as a `&'a str` for any shorter `'a`, so the `Vec<&'static str>` from
`defaults.to_vec()` can become a `Vec<&'a str>` whose lifetime fits both the defaults and the words of
`extra`. The `'static` in the type of `defaults` does not have to be carried into the local variable.

Making the words `'static` by leaking them (`Box::leak`, `String::leak`) also compiles, but Miri
reports the leaked memory.

### Alternatives

You can also do without borrowed strings by collecting owned `String`s from the start.

```rust
pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String> {
    let mut all: Vec<String> = defaults.iter().map(|d| d.to_string()).collect();
    for word in extra.split(',').map(str::trim).filter(|w| !w.is_empty()) {
        if !all.iter().any(|a| a == word) {
            all.push(word.to_string());
        }
    }
    all
}
```
