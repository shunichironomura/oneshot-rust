The tokens are slices of the **input string**, not of the `Tokenizer`. Write `Option<&'a str>`, using
the struct's lifetime parameter, so that a token stays valid after the next call to `next_token`.

If you leave the return type elided (`Option<&str>`), it is tied to the `&mut self` borrow. Each
token then keeps the tokenizer mutably borrowed, and the harness, which stores tokens while it keeps
calling `next_token`, fails to compile.

### Alternatives

Implementing `Iterator` gives you `collect`, `for` loops and every other iterator adapter for free.

```rust
impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.next_token()
    }
}
```
