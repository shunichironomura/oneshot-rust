A `Vec<T>` already behaves like a stack: `push` adds at the end, `pop` removes from the end and
returns an `Option<T>`, and `last` returns an `Option<&T>`. Every method just delegates to it.

### Alternatives

A type with `new()` and no arguments usually implements `Default` too (Clippy suggests it with
`new_without_default`). `#[derive(Default)]` would add a `T: Default` bound, which `Vec<T>` does not
need, so write the impl by hand.

```rust
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
```
