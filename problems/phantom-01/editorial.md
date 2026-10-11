The impl for a number on the left, `impl<U, K: Into<f64>> Mul<Length<U>> for K`, breaks the
**orphan rule** (E0210: "type parameter `K` must be covered by another type when it appears before
the first local type"). `Mul` comes from the standard library, so an impl of it needs a local type,
and the types of `impl<..> Mul<T1> for T0` are checked in the order `T0`, `T1`: no bare type
parameter may appear before the first local type. Here `T0` is `K` itself, which may be any type at
all, and only `T1 = Length<U>` is local. The rule keeps crates from stepping on each other: a crate
that defines a type `Meters` is allowed to write `impl<T> Mul<T> for Meters`, and then
`Meters * Length<U>` would have two impls, one in each crate.

So write one impl per number type:

```rust
impl<U> Mul<Length<U>> for f64 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}
```

and the same for `f32`, `i32` and `u32`. These impls are allowed even though `Mul` and `f64` both
come from the standard library: `T0 = f64` contains no type parameter, and `T1 = Length<U>` is a
local type (the `U` inside it is "covered" by `Length`). Keep `U` generic: impls for
`Length<Mm>` and `Length<Inch>` only would not cover the tests' own units. The body simply reuses
`l * k`, and `perimeter`, whose `2 * (w + h)` multiplies by an `i32` from the left, now compiles
unchanged.

The other direction needs no such care. In `impl<U, K: Into<f64>> Mul<K> for Length<U>`, `T0` is
the local `Length<U>`, so the type parameter `K` after it is fine: one impl makes `l * k` work for
every `K: Into<f64>`, while `k * l` needs one impl per type. (Deleting the generic impl and writing
`(w + h) * 2` in `perimeter` compiles too, but the tests write `3 * a` and `2.5 * l`.)

With several impls for numbers on the left, a literal such as `2` in `2 * (w + h)` could be an
`i32` or a `u32`. The compiler settles it with the default type, `i32` (and `f64` for `2.5`), but
only at the end of type checking, so a method call directly on the result, as in
`(2 * l).value()`, fails with "type annotations needed". Write `2i32 * l` there, or give the result
a type first.

The units derive `Debug`, `Clone`, `Copy` and `PartialEq` because the derives on `Length<U>`
require each of these traits of `U` as well, even though `U` only appears in `PhantomData<U>`.

### Alternatives

A macro writes the four impls. `f64::from(self)` converts each number type without loss.

```rust
macro_rules! scalar_times_length {
    ($($t:ty),*) => {
        $(
            impl<U> Mul<Length<U>> for $t {
                type Output = Length<U>;

                fn mul(self, l: Length<U>) -> Length<U> {
                    Length::new(f64::from(self) * l.value)
                }
            }
        )*
    };
}

scalar_times_length!(f64, f32, i32, u32);
```
