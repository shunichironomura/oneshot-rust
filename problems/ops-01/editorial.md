Inside `impl<T>`, nothing is known about `T`, so `self.x + rhs.x` does not compile. Each impl has to
ask for exactly the operations its body uses, **including what they produce**:

- `Add` needs `T: Add<Output = T>`. A plain `T: Add` is not enough, because `T + T` could then be some
  other type, while the template promises `type Output = Vec2<T>`.
- `Mul<T>` needs `T: Mul<Output = T> + Copy`. The scalar `k` is used twice, and without `Copy` the
  first multiplication would move it.
- `Neg` needs `T: Neg<Output = T>`, and `AddAssign` needs `T: AddAssign`.
- `dot` needs both `Add<Output = T>` and `Mul<Output = T>`. Putting it in its own
  `impl<T: ...> Vec2<T>` block keeps `new` available for every `T`.

`self` and `rhs` are taken by value, so their fields can be moved out; no `Clone` is required for
`Add` or `Neg`.

### Alternatives

The output type does not have to be `Vec2<T>`. Using the associated type `T::Output` makes the impl
work for any `T: Add`, even when adding two `T` gives a different type.

```rust
impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T::Output> {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
```

The bounds can also be written in a `where` clause, which reads better once there are several.
