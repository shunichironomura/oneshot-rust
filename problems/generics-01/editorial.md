In `impl<X, Y> Point<X, Y>`, the `<X, Y>` right after `impl` **declares** two type parameters, and
`Point<X, Y>` **uses** them to say which type the methods belong to. The template's `impl Point`
declares nothing, so `X` and `Y` in `new` are unknown names ("cannot find type `X` in this scope"),
and a bare `Point` lacks its two type arguments. Each of the three blocks needs a different header.

- `new`, `mixup`, `swap` and `convert` work for every point, so their block is
  `impl<X, Y> Point<X, Y>` with no bounds. The first three bodies only move values around, `convert`
  states its own requirements (see below), and a bound such as `X: Clone` would reject the
  harness's `Grade`.
- In `mixup`, `other` has coordinate types of its own, so the method declares its own parameters:
  `mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2>`. The result mixes the two sets. With
  `other: Self`, both points would need the same types, and
  `Point::new(5, 10.4).mixup(Point::new("Hello", 'c'))` would not compile. The new names must differ
  from `X` and `Y`: `mixup<X, Y2>` is rejected because the `impl` already uses the name `X`.
- `swap` returns `Point<Y, X>`, not `Self`. Inside this block `Self` is `Point<X, Y>`, a different
  type unless `X` and `Y` happen to be the same.
- `convert` also declares `X2` and `Y2`, and its `.into()` calls need `X: Into<X2>` and
  `Y: Into<Y2>`. These bounds mention `X2` and `Y2`, which exist only in the method, so they cannot
  go on the `impl` line; they go in the method's `where` clause. A method's `where` clause may
  constrain the `impl`'s parameters too, and then only that method carries the requirement. The
  order `<X2, Y2>` matters for the harness's turbofish `convert::<String, u32>()`.
- `distance_from_origin` is only for `Point<f64, f64>`, so its block names that type and declares
  nothing: `impl Point<f64, f64>`. A generic `T` cannot work: `sqrt` is an inherent method of the
  float types, not part of any trait, and `self.x * self.x` copies the coordinates out of `&self`.
  Beware of `impl<f64> Point<f64, f64>`: it declares a type **parameter** named `f64` that hides
  the real `f64`, and the body then fails with the puzzling "cannot multiply `f64` by `f64`".
  `impl Point<f64>` (one argument, like the Book's `Point<f32>`) is an error too, because `Point`
  takes two.
- `larger` compares `x` with `y` and formats one of them, so both coordinates have one type `T`, and
  `T: PartialOrd + Display`: `>=` comes from `PartialOrd`, `{}` from `Display`. Ask for nothing
  more. `Ord` rejects `f64` and `Grade`, `Copy` rejects `String` and `Grade`, and `Clone` rejects
  `Grade`. None of them is needed: `self.x >= self.y` compares through references (it calls
  `PartialOrd::ge(&self.x, &self.y)`), and `format!` borrows its arguments.

In `Point::new(3.0, 4.0).distance_from_origin()`, the literals could still be `f32` or `f64`. The
only `distance_from_origin` is in the block for `Point<f64, f64>`, so they become `f64`. (With a
second block for `Point<f32, f32>`, the call would be ambiguous.)

### Alternatives

The bounds can go in a `where` clause on the `impl`.

```rust
impl<T> Point<T, T>
where
    T: PartialOrd + Display,
{
    // larger as before
}
```

`larger` can also allow two different types that can be compared with each other. `PartialOrd<Y>`
means "can be compared with a `Y`", and `T: PartialOrd` is short for `T: PartialOrd<T>`.

```rust
impl<X: PartialOrd<Y> + Display, Y: Display> Point<X, Y> {
    // larger as before
}
```

With those bounds in a method `where` clause, `larger` can even move into the first block, next to
`convert`; the separate block is then not needed.

```rust
    pub fn larger(&self) -> String
    where
        X: PartialOrd<Y> + Display,
        Y: Display,
    {
        if self.x >= self.y {
            format!("x = {}", self.x)
        } else {
            format!("y = {}", self.y)
        }
    }
```

`convert` can ask for `From` instead of `Into`. The standard library implements `Into<U> for T`
whenever `U: From<T>`, so `.into()` still works.

```rust
    pub fn convert<X2, Y2>(self) -> Point<X2, Y2>
    where
        X2: From<X>,
        Y2: From<Y>,
```
