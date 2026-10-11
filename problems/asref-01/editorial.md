No concrete type fits every call, and deref coercion does not help. It can turn `&String` into
`&str` when the parameter is `&str`, but a `String` passed by value is not a reference at all, and
coercion never changes the elements inside a slice, an array or a `Vec`: `&Vec<String>` can become
`&[String]`, never `&[&str]`. So each function gets a type parameter, bounded by the trait its body
uses: `as_ref()` comes from `AsRef<str>`, `as_mut()` from `AsMut<u32>`, `into_iter()` from
`IntoIterator`, and `Into::into` from `Into<i64>`.

- `char_len` takes `S` **by value**. `&str` and `String` implement `AsRef<str>`, and so does
  `&String`, through the blanket impl `impl<T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for &T`. With
  `s: &S` instead, a `String` argument would not fit, and `"Café"` would need `S = str`, which is
  unsized (type parameters are `Sized` unless you add `?Sized`).
- `join_upper` keeps the slice and makes its element generic: `&[S]`. `&["ab", "c"]` coerces to
  `&[&str]` and `&Vec<String>` to `&[String]`, and `S` is inferred from the element type.
  `words: &[impl AsRef<str>]` would **not** work: a type written as `impl Trait` in an argument has
  no name, so the function has no generic parameter that `join_upper::<String>` could set ("function
  takes 0 generic arguments"). The harness needs the turbofish because `&[]` says nothing about its
  element type.
- `total_chars` needs a bound on the items of the items. The solution writes it with associated
  type bounds (stable since Rust 1.79): `I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>`.
  `IntoIterator` accepts arrays, references and iterators as well as `Vec`s. Iterating over
  `&[Vec<String>; 1]` yields `&Vec<String>`, whose items are `&String`, which is `AsRef<str>` again.
- `square_in_place` takes `&mut T` with `T: AsMut<u32>`. `Box<u32>` implements `AsMut<u32>`, and so
  does `Score`.
- `widen` takes any `IntoIterator` whose items are `Into<i64>`. Every `u8`, `u16`, `i32` and `u32`
  fits in an `i64`, and `i64` converts to itself through `impl<T> From<T> for T`. (`u64` does not
  implement `Into<i64>`, because large values would not fit.)

The bound is also what makes `s.as_ref()` unambiguous. In the template, `s: &str` gives `as_ref`
several candidates (`str` implements `AsRef<str>`, `AsRef<[u8]>`, `AsRef<OsStr>` and
`AsRef<Path>`), so even the bodies fail with "type annotations needed". All a generic function
knows about `S` is `S: AsRef<str>`, so `as_ref` returns `&str`.

### Alternatives

`impl Trait` in argument position is shorter for every function except `join_upper`, whose callers
need to name the type parameter.

```rust
pub fn char_len(s: impl AsRef<str>) -> usize
pub fn total_chars(groups: impl IntoIterator<Item = impl IntoIterator<Item = impl AsRef<str>>>) -> usize
pub fn square_in_place(n: &mut impl AsMut<u32>)
pub fn widen(values: impl IntoIterator<Item = impl Into<i64>>) -> Vec<i64>
```

Without associated type bounds, write one bound per level in a `where` clause. The inner item type
must be written `<I::Item as IntoIterator>::Item`; `I::Item::Item` is rejected as an ambiguous
associated type.

```rust
pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator,
    I::Item: IntoIterator,
    <I::Item as IntoIterator>::Item: AsRef<str>,
```

Or give every level its own type parameter.

```rust
pub fn total_chars<I, G, S>(groups: I) -> usize
where
    I: IntoIterator<Item = G>,
    G: IntoIterator<Item = S>,
    S: AsRef<str>,
```

`square_in_place` can also take `T` by value. For `&mut b`, `T` is `&mut Box<u32>`, which implements
`AsMut<u32>` through the impl for `&mut T`. The parameter must be `mut n`, because `as_mut` borrows
`n` mutably.

```rust
pub fn square_in_place<T: AsMut<u32>>(mut n: T)
```
