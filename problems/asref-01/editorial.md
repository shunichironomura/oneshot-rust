No concrete element type fits every call, and deref coercion does not help. It can turn `&String`
into `&str`, but it never changes the elements inside a slice, an array or a `Vec`: `&Vec<String>`
can become `&[String]`, never `&[&str]`. So the function keeps the slice and makes its element
generic: `&[S]`, bounded by the trait the body uses. `as_ref()` comes from `AsRef<str>`, which both
`&str` and `String` implement. `&["ab", "c"]` coerces to `&[&str]` and `&Vec<String>` to
`&[String]`, and `S` is inferred from the element type.

- `words: &[impl AsRef<str>]` would **not** work: a type written as `impl Trait` in an argument has
  no name, so the function has no generic parameter that `join_upper::<String>` could set ("function
  takes 0 generic arguments"). The harness needs the turbofish because `&[]` says nothing about its
  element type.
- The bound is also what makes `w.as_ref()` unambiguous. In the template, `w: &String` gives
  `as_ref` several candidates (`String` implements `AsRef<str>`, `AsRef<[u8]>`, `AsRef<OsStr>` and
  `AsRef<Path>`), so even the body fails with "type annotations needed". All a generic function
  knows about `S` is `S: AsRef<str>`, so `as_ref` returns `&str`.
