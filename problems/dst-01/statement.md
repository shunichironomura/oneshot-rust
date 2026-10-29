The body of the function below is already written, but its signature accepts only one concrete
type, so the test harness (the `tests.rs` tab) does not compile. Change **only the signature** so
that the harness compiles and passes. Do not change the body.

```rust
pub fn byte_size(x: &u64) -> usize
```

- `byte_size` returns the size in bytes of the value that `x` points to, using
  `std::mem::size_of_val`. For a string slice this is its length in UTF-8 bytes, and for a slice it
  is the number of elements times the size of one element. The harness passes references to
  numbers, arrays, string slices (`&str`), slices (`&[T]`), trait objects (`&dyn Debug`), and values
  of its own type `Rgb`, which has no `derive` and no `impl` at all.

### Examples

```rust
byte_size("héllo")            // 6 ("é" takes 2 bytes)
byte_size(&[1u16, 2, 3][..])  // 6
byte_size(&7u64)              // 8
```
