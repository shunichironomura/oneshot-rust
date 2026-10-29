`Container` is a trait with an **associated type** `Item`, the type of the values a container holds.
`Stack<T>` holds values of any type `T`, and `Bag` holds bytes (`u8`). The trait, the two types and
their impls are already in the editor.

```rust
pub trait Container {
    type Item;
    fn items(&self) -> &[Self::Item];  // all the values, in order
}

pub struct Stack<T> { pub elems: Vec<T> }  // impl<T> Container for Stack<T>: Item = T
pub struct Bag { pub bytes: Vec<u8> }      // impl Container for Bag: Item = u8
```

The body of the function below is already written, but the code does not compile, and the
signature accepts only `Bag`. Change **only the signature** so that the test harness (the
`tests.rs` tab) compiles and passes. Do not change the body, the trait, the two types or their
impls.

```rust
pub fn total(c: &Bag) -> i64
```

- `total(c)` converts every value to an `i64` and returns their sum (`0` for an empty container;
  `true` counts as 1 and `false` as 0). The harness calls it on `Bag`, on stacks of `i32`, `u16`,
  `u32`, `i64` and `bool`, and on its own container type `Shelf<i8>` (its `Item` is `i8`).

### Examples

```rust
total(&Bag { bytes: vec![3, 1, 2] })              // 6
total(&Stack { elems: vec![-5i32, 2] })           // -3
total(&Stack { elems: vec![40_000u16, 30_000] })  // 70000
```
