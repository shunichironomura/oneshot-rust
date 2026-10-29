The output of `#[derive(Debug)]` is written by the debug builders of `Formatter`, and the way to
print exactly the same thing is to use them too. Start with `f.debug_struct(name)`, add each field
with `.field(...)` and end with `.finish()`. The builder writes the punctuation itself, on one line
or in the pretty form depending on `f.alternate()`, and in the pretty form it indents every line
that a field writes, which is why the nested list comes out right, and why the whole value is
indented correctly inside a `Some`.

The redacted password is where the obvious code goes wrong. A field is printed with the `Debug` of
its value, and in `.field("password", &"<redacted>")` the value is a `&str`, whose `Debug` adds
quotes (and escapes special characters): `password: "<redacted>"`. The value has to be one whose
`Debug` writes the bare text. `format_args!("<redacted>")` builds a `fmt::Arguments`, whose
`Debug` writes the text as is; a small type with a hand-written `Debug` works too (see the
alternatives).

Writing `Debug` like a `Display`, with
`write!(f, "Credentials {{ user: {:?}, password: <redacted>, scopes: {:?} }}", self.user, self.scopes)`,
looks right only for `{:?}`. Each `{:?}` in that string is formatted with the spec written there,
not with the caller's, so `{:#?}` stays on one line. The caller's flags are stored in `f`, and only
code that formats the fields with `f`, as the builders do, sees them.

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
