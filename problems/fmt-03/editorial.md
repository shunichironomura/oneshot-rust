The trap is writing `Debug` like a `Display`, with `write!`. For `{:?}` it looks right, and
everything else breaks. `write!(f, "Rgb({:?}, {:?}, {:?})", self.r, self.g, self.b)` formats each
`{:?}` with the spec written in that string, not with the caller's: `{:x?}` still prints
`Rgb(255, 0, 16)`, and `{:#?}` stays on one line. The caller's flags are stored in `f`, and only
code that formats the fields **with `f`** sees them.

That is what the debug builders do, and what `#[derive(Debug)]` expands to. Start with
`f.debug_struct(name)` (or `f.debug_tuple(name)` for the tuple-like form), add each field with
`.field(...)` and end with `.finish()`: the builder writes the punctuation itself, on one line or in
the pretty form depending on `f.alternate()`. It calls the `Debug` of each field with a formatter
that has the same settings as `f`, so the field sees the same flags: `u8` and `u32` print in hex
under `{:x?}`, with `0x` when `#` is set too. In the pretty form the builder also indents every
line the field writes, which is why nested lists and maps come out right, and why the whole value is
indented correctly inside a `Vec`.

Three details:

- A field is printed with the `Debug` of its value. `&"<redacted>"` is a `&str`, so it would appear
  as `"<redacted>"`, quotes included. `format_args!("<redacted>")` builds a `fmt::Arguments`,
  whose `Debug` writes the text as is.
- `finish_non_exhaustive()` ends a struct with `..`: `, .. }` on one line, and a `..` line of its
  own in the pretty form.
- `field` returns `&mut DebugTuple`, a borrow of the builder. To add a field only sometimes, keep
  the builder itself in a `let mut` variable. `let t = f.debug_tuple("Rgba").field(&self.r);` does
  not compile (E0716): the builder is a temporary that is dropped at the end of that statement,
  while `t` still borrows it.

### Alternatives

A small type whose `Debug` writes the bare text works in place of `format_args!`.

```rust
struct Redacted;

impl fmt::Debug for Redacted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("password", &Redacted)
            .field("scopes", &self.scopes)
            .finish()
    }
}
```

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
