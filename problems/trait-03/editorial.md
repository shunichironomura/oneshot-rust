The body of a default method is type-checked once, for every type that could ever implement the
trait, not separately for `Point` and `Tags`. In the template, `Outline` says nothing about `Self`,
so `self.to_string()` or `format!("{self}")` inside `outline` does not compile: the compiler cannot
assume that `Self` implements `Display`. Declaring `pub trait Outline: fmt::Display` makes `Display`
a **supertrait**: every type that implements `Outline` must also implement `Display`, and in return
the trait's own methods may use it. `to_string` then comes from the standard blanket impl
`impl<T: Display + ?Sized> ToString for T`. The harness's own type implements `Display`, so its empty
`impl Outline` is accepted, just like those of `Point` and `Tags`.

The width is the number of `char`s, not `len()`: `len()` counts UTF-8 bytes, and `é` takes two
bytes, so `len()` would make the frame around `héllo` one column too wide.

`Tags` is a **newtype**. `impl fmt::Display for Vec<String>` is rejected by the orphan rule (E0117),
because both the trait and the type come from another crate. A tuple struct around the `Vec` is a
local type, so it may implement `Display`. Inside, `self.0` is the vector, and `join(", ")` builds
the list, which is empty for no strings, so `[]` needs no special case.

### Alternatives

A bound on the method itself instead of a supertrait also works. The difference is that `Outline`
can then be implemented for types without `Display` (they just cannot call `outline`), and generic
code bounded only by `T: Outline` can neither call `outline` nor print the value. The frame lines
can also be built with a fill character and a width in the format string.

```rust
pub trait Outline {
    fn outline(&self) -> String
    where
        Self: fmt::Display,
    {
        let text = self.to_string();
        let width = text.chars().count();
        let border = format!("{:*<n$}", "", n = width + 4);
        let blank = format!("*{:n$}*", "", n = width + 2);
        format!("{border}\n{blank}\n* {text} *\n{blank}\n{border}")
    }
}
```

The compiler's own suggestion, `pub trait Outline: ToString`, passes too, because every `Display`
type implements `ToString`.

`Tags` can also be written piece by piece, without building the joined `String` first.

```rust
impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (i, tag) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_str(tag)?;
        }
        f.write_str("]")
    }
}
```
