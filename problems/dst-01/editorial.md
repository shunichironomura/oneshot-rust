`str`, slices `[T]` and trait objects `dyn Trait` are **dynamically sized types** (DSTs): the size
of a value is known only at run time. They are always used behind a pointer, such as `&str`,
`Box<[T]>` or `Rc<dyn Fn(i32) -> i32>`. Such a pointer is a "fat" pointer: next to the address it
stores the length (for `str` and `[T]`) or a pointer to the vtable (for `dyn Trait`).

Every type parameter has an implicit `Sized` bound: `fn byte_size<T>(x: &T)` really means
`fn byte_size<T: Sized>(x: &T)`. So the natural way to generalize the template fails as soon as the
harness calls `byte_size("héllo")`, which needs `T = str`: "the size for values of type `str` cannot
be known at compilation time" (E0277). `T: ?Sized` removes that default bound ("`T` may or may not
be `Sized`"). This is allowed here because the bodies only use `T` behind a pointer: `&T` and
`Box<T>` are sized even when `T` is not.

- `byte_size`: `std::mem::size_of_val` is itself declared as
  `fn size_of_val<T: ?Sized>(val: &T) -> usize`. It computes the size from the metadata of the fat
  pointer, so for `&dyn Debug` it returns the size of the value behind the trait object.
- `show_pair`: the two arguments have different types, so it needs two type parameters. With a
  single `T`, `show_pair("x", &[1, 2][..])` is a type mismatch. `?Sized` applies only to the
  parameter it is written on, so both need it. `{a:?}` formats `a: &A` through
  `impl<T: ?Sized + Debug> Debug for &T`, so both also need `Debug`.
- `share`: the standard library implements `From<Box<T>> for Rc<T>` for `T: ?Sized`, so
  `Rc::from(b)` turns a `Box<str>` into an `Rc<str>` and a `Box<dyn Fn(i32) -> i32>` into an
  `Rc<dyn Fn(i32) -> i32>`. It moves the value into a new allocation that also holds the reference
  counts.

Two other tempting fixes fail. Parameters of type `&dyn Debug` do not accept `"x"`: a `&dyn Debug`
pointing to a `str` would have to carry both its length and a vtable, but a fat pointer has room for
only one of them, so only sized types can be turned into trait objects. And bounds that the bodies
do not need reject some of the harness's arguments: `T: Debug + ?Sized` on `byte_size` or `share`
rejects `Rgb` and the boxed closures, and `T: ?Sized + 'static` on `share` rejects a `Box<&str>`
that borrows a local variable.

### Alternatives

`impl Trait` in argument position can be relaxed with `?Sized` too. In `show_pair`, the parentheses
are needed because of the `+`. `share` must keep a named type parameter, because its return type
mentions it.

```rust
pub fn byte_size(x: &impl ?Sized) -> usize

pub fn show_pair(a: &(impl Debug + ?Sized), b: &(impl Debug + ?Sized)) -> String
```

`show_pair` can also take the references themselves by value. Then the types are `&str`, `&[i32]`
and so on, which are sized, and `Debug` for a reference prints the value it points to, so the output
is the same.

```rust
pub fn show_pair(a: impl Debug, b: impl Debug) -> String
```

This trick does not work for the other two functions: the body of `byte_size` passes `x` to
`size_of_val`, which takes a reference, and the return type of `share` has to name the unsized type
itself, as in `Rc<str>`.
