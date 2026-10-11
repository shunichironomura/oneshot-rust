In the template, `'a` is a lifetime parameter of `map_lines`, so the **caller** picks it, and it
outlives the whole call. But the lines passed to `f` borrow `normalized`, a local `String` that is
dropped when the function returns. No single caller-chosen `'a` fits those borrows.

What `f` must really promise is "for **any** lifetime of the input, return a slice with that same
lifetime". With elision, `F: Fn(&str) -> &str` says exactly that: it is shorthand for the
higher-ranked bound `F: for<'a> Fn(&'a str) -> &'a str`. Closures like `|s| s.trim()` and functions
like `first_word` satisfy it.

Other tempting signatures fail too. `F: Fn(&str) -> String` rejects closures that return `&str`, and
`F: Fn(&str) -> R` with a generic `R` fails for `first_word`, because its return type depends on the
input lifetime and cannot be a single type `R`.

### Alternatives

Write the higher-ranked bound explicitly.

```rust
pub fn map_lines<F>(text: &str, f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> &'a str,
```

Or use `impl Trait` in argument position.

```rust
pub fn map_lines(text: &str, f: impl Fn(&str) -> &str) -> Vec<String>
```
