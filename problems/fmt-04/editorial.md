The output of `#[derive(Debug)]` is written by the debug builders of `Formatter`, and the way to
print exactly the same thing is to use them too. For a tuple struct, start with
`f.debug_tuple(name)`, add each field with `.field(...)` and end with `.finish()`. The builder
writes the punctuation itself, on one line or in the pretty form depending on `f.alternate()`, and
in the pretty form it indents every line that a field writes, which is why the whole value is
indented correctly inside an array. It also formats each field with the settings of `f`, so the `u8`
fields print in hex under `{:x?}`, with `0x` when `#` is set too.

`field` returns `&mut DebugTuple`, a borrow of the builder. To add the alpha only sometimes, keep
the builder itself in a `let mut` variable. `let t = f.debug_tuple("Rgba").field(&self.r);` does
not compile (E0716) once `t` is used in a later statement: the builder is a temporary that is
dropped at the end of the `let` statement, while `t` still borrows it.

Writing `Debug` like a `Display`, with
`write!(f, "Rgb({:?}, {:?}, {:?})", self.r, self.g, self.b)`, looks right only for `{:?}`. Each
`{:?}` in that string is formatted with the spec written there, not with the caller's: `{:x?}`
still prints `Rgb(255, 0, 16)`, and `{:#?}` stays on one line. The caller's flags are stored in
`f`, and only code that formats the fields with `f`, as the builders do, sees them.

### Alternatives

The `Debug` of a tuple prints the tuple form without a name, like `(255, 0, 16)`, so writing the
name first and then handing `f` to the tuple gives the same output, flags included. It must be
`fmt::Debug::fmt(&tuple, f)`, which reuses `f`; `write!(f, "{:?}", tuple)` would lose the flags
again.

```rust
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color { r, g, b, a } = *self;
        if a == 255 {
            f.write_str("Rgb")?;
            fmt::Debug::fmt(&(r, g, b), f)
        } else {
            f.write_str("Rgba")?;
            fmt::Debug::fmt(&(r, g, b, a), f)
        }
    }
}
```
