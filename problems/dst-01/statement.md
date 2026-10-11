The bodies of the three functions below are already written, but each signature accepts only one
concrete type, so the test harness (the `tests.rs` tab) does not compile. Change **only the
signatures** so that the harness compiles and passes. Do not change the bodies.

```rust
pub fn byte_size(x: &u64) -> usize
pub fn show_pair(a: &str, b: &str) -> String
pub fn share(b: Box<i32>) -> Rc<i32>
```

- `byte_size` returns the size in bytes of the value that `x` points to, using
  `std::mem::size_of_val`. For a string slice this is its length in UTF-8 bytes, and for a slice it
  is the number of elements times the size of one element. The harness passes references to
  numbers, arrays, string slices (`&str`), slices (`&[T]`), trait objects (`&dyn Debug`), and values
  of its own type `Rgb`, which has no `derive` and no `impl` at all.
- `show_pair` formats `a` and `b` with `{:?}` and joins them with `" & "`. The two arguments may
  have different types. The harness passes references to numbers, string slices, slices and trait
  objects (`&dyn Debug`).
- `share` moves the value out of a `Box` into a new `Rc`. The harness passes boxes of ordinary
  values such as `Box<i32>`, as well as `Box<str>`, boxed slices (`Box<[T]>`) and boxed closures
  (`Box<dyn Fn(i32) -> i32>`), and uses each result as an `Rc` of the same type (`Rc<i32>`,
  `Rc<str>`, `Rc<[T]>`, `Rc<dyn Fn(i32) -> i32>`).

Some of the arguments borrow local variables of the harness.

### Examples

```rust
byte_size("héllo")            // 6 ("é" takes 2 bytes)
byte_size(&[1u16, 2, 3][..])  // 6
byte_size(&7u64)              // 8

show_pair("x", &[1, 2][..])   // "\"x\" & [1, 2]"
show_pair(&5, "y")            // "5 & \"y\""

let s: Rc<str> = share(Box::<str>::from("abc"));
let v: Rc<[i32]> = share(vec![1, 2].into_boxed_slice());
let n: Rc<i32> = share(Box::new(5));
```
