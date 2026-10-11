`Describe` is a trait of your own crate, so you may implement it for any type, including types of
the standard library such as `str`, `Vec<T>`, `Option<T>`, tuples and references. The orphan rule
only forbids implementing a trait from another crate for a type from another crate (such as
`Display` for `Vec<T>`).

A generic impl reads "for every `T` that satisfies the bounds, this type implements `Describe`".
The container bodies call `describe` on their elements, so each container impl needs a type
parameter with a `Describe` bound:

```rust
impl<T: Describe> Describe for [T]
impl<T: Describe> Describe for Vec<T>
impl<T: Describe> Describe for Option<T>
impl<A: Describe, B: Describe> Describe for (A, B)
```

The compiler chains these impls for nested types: `Vec<Option<i32>>` implements `Describe` because
`Option<i32>` does, because `i32` does.

- The body of the `Vec` impl calls `self.as_slice().describe()`, so it needs `[T]: Describe`. The
  harness also describes slices directly, but even without that the slice impl would have to be
  generic: with `impl Describe for [i32]`, the generic `Vec` impl does not compile.
- The pair needs two type parameters. With `(T, T)`, `(true, vec!["a", "b"])` would not match: its
  parts are a `bool` and a `Vec<&str>`.
- The bound is what lets a body call `describe` on its elements. In
  `impl<T> Describe for Option<T>`, nothing is known about `T`, so `value.describe()` fails (E0599).
- The type parameter must be declared after `impl`. `impl Describe for Vec<T>` alone fails with
  "cannot find type `T` in this scope".

The `&T` impl was already generic. Its `?Sized` removes the implicit `Sized` bound that every type
parameter has, so that it also covers `&str`, `&[i32]` and `&dyn Describe`: `str`, `[i32]` and
`dyn Describe` are dynamically sized types. `vec!["a", "b"]` is a `Vec<&str>`, so describing it uses
the `&T` impl with `T = str`. For a trait like `Describe` that can be made into a trait object, the
compiler itself implements `Describe` for `dyn Describe`, which is why a `Vec<&dyn Describe>` can
hold values of different types.

`?Sized` does not fit the other impls. A slice and a `Vec<T>` lay out their `T` values one after
another, and an `Option<T>` holds its `T` value inline, so `T` must have a size known at compile
time, and `impl<T: Describe + ?Sized> Describe for Vec<T>` is rejected (E0277). (In a tuple, only
the last element may be unsized: `B: ?Sized` is accepted, `A: ?Sized` is not. Neither is needed.)

### Alternatives

The bounds can also go in `where` clauses. The `Vec` impl can ask for exactly what its body uses,
`[T]: Describe`, instead of `T: Describe`.

```rust
impl<T> Describe for Vec<T>
where
    [T]: Describe,
{
    fn describe(&self) -> String {
        self.as_slice().describe()
    }
}

impl<A, B> Describe for (A, B)
where
    A: Describe,
    B: Describe,
{
    fn describe(&self) -> String {
        format!("({}, {})", self.0.describe(), self.1.describe())
    }
}
```
