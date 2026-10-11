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
impl<T: Describe + ?Sized> Describe for &T
```

The compiler chains these impls for nested types: `Vec<Option<i32>>` implements `Describe` because
`Option<i32>` does, because `i32` does.

- The body of the `Vec` impl calls `self.as_slice().describe()`, so it needs `[T]: Describe`. The
  slice impl must be generic too; with `impl Describe for [i32]`, the generic `Vec` impl does not
  compile.
- The pair needs two type parameters. With `(T, T)`, `(true, vec!["a", "b"])` would not match: its
  parts are a `bool` and a `Vec<&str>`.
- The real trap is the `&T` impl, which already looks generic. Every type parameter has an implicit
  `Sized` bound, so `impl<T: Describe> Describe for &T` covers `&i32` and `&String`, but not `&str`,
  `&[i32]` or `&dyn Describe`: `str`, `[i32]` and `dyn Describe` are dynamically sized types.
  `vec!["a", "b"]` is a `Vec<&str>`, so describing it needs `&str: Describe`, which is the `&T`
  impl with `T = str`. `T: ?Sized` removes the implicit bound. This is allowed because the body only
  uses `T` behind a reference: `**self` is a place of type `T`, and `.describe()` borrows it again.
- For a trait like `Describe` that can be made into a trait object, the compiler itself implements
  `Describe` for `dyn Describe`. With `?Sized`, `&dyn Describe` is `Describe` too, so a
  `Vec<&dyn Describe>` can hold values of different types.

`?Sized` does not fit the other impls. A slice, a `Vec<T>` and an `Option<T>` store `T` values
directly, so `T` must have a size known at compile time, and
`impl<T: Describe + ?Sized> Describe for Vec<T>` is rejected (E0277). (In a tuple, only the last
element may be unsized: `B: ?Sized` is accepted, `A: ?Sized` is not. Neither is needed.)

Adding `impl Describe for &str` instead is against the rules, and it would only handle `&str`, not
`&[i32]` or `&dyn Describe`. Once `&T` has `?Sized`, it covers `&str` as well, and a second impl for
`&str` is rejected as a conflicting implementation (E0119): coherence allows at most one impl of a
trait for each type. (Next to the template's `&T` impl without `?Sized`, it would be accepted,
because `str` is never `Sized`, so the two impls cannot overlap.)

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

impl<T> Describe for &T
where
    T: Describe + ?Sized,
{
    fn describe(&self) -> String {
        (**self).describe()
    }
}
```
