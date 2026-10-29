`str`, slices `[T]` and trait objects `dyn Trait` are **dynamically sized types** (DSTs): the size
of a value is known only at run time. They are always used behind a pointer, such as `&str`,
`Box<[T]>` or `&dyn Debug`. Such a pointer is a "fat" pointer: next to the address it stores the
length (for `str` and `[T]`) or a pointer to the vtable (for `dyn Trait`).

Every type parameter has an implicit `Sized` bound: `fn byte_size<T>(x: &T)` really means
`fn byte_size<T: Sized>(x: &T)`. So the natural way to generalize the template fails as soon as the
harness calls `byte_size("héllo")`, which needs `T = str`: "the size for values of type `str` cannot
be known at compilation time" (E0277). `T: ?Sized` removes that default bound ("`T` may or may not
be `Sized`"). This is allowed here because the body only uses `T` behind a pointer: `&T` is sized
even when `T` is not.

`std::mem::size_of_val` is itself declared as `fn size_of_val<T: ?Sized>(val: &T) -> usize`. It
computes the size from the metadata of the fat pointer, so for `&dyn Debug` it returns the size of
the value behind the trait object.

Two other tempting fixes fail. A parameter of type `&dyn Debug` does not accept `"héllo"`: a
`&dyn Debug` pointing to a `str` would have to carry both its length and a vtable, but a fat pointer
has room for only one of them, so only sized types can be turned into trait objects. And a bound
that the body does not need rejects some of the harness's arguments: `T: Debug + ?Sized` rejects
`Rgb`.

### Alternatives

`impl Trait` in argument position can be relaxed with `?Sized` too.

```rust
pub fn byte_size(x: &impl ?Sized) -> usize {
    std::mem::size_of_val(x)
}
```
