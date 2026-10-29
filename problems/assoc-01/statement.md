`Container` is a trait with an **associated type** `Item`, the type of the values a container holds.
`Stack<T>` holds values of any type `T`, and `Bag` holds bytes (`u8`) and keeps them in ascending
order. The trait, the two types and their impls are already in the editor.

```rust
pub trait Container {
    type Item;
    fn put(&mut self, x: Self::Item);              // adds x
    fn take_last(&mut self) -> Option<Self::Item>; // removes and returns the last value
    fn items(&self) -> &[Self::Item];              // all the values, in order
}

pub struct Stack<T> { pub elems: Vec<T> }  // impl<T> Container for Stack<T>: Item = T
pub struct Bag { /* private */ }           // impl Container for Bag: Item = u8
impl Bag { pub fn new() -> Self }          // an empty bag
```

The body of the function below is already written, but its signature accepts only particular
concrete container types, so the test harness (the `tests.rs` tab) does not compile. Change **only
the signature** so that the harness compiles and passes. Do not change the body, the trait, the two
types or their impls.

```rust
pub fn move_all(from: &mut Stack<u8>, to: &mut Bag) -> usize
```

- `move_all(from, to)` takes the values out of `from` one by one with `take_last`, puts each of them
  into `to` with `put`, and returns how many values it moved. The values already in `to` stay. The
  two containers may be of different types, as long as they hold values of the same type. The
  harness moves values from `Stack<u8>` to `Bag`, from `Bag` to `Stack<u8>`, from `Bag` to `Bag`,
  between two `Stack<String>`s, between two `Stack<i64>`s, and from `Shelf<Book>` to `Stack<Book>`
  and back.

`Shelf<T>` is a container type of the harness (its `Item` is `T`), and `Book` is a value type of the
harness that is not `Copy` or `Clone`.

### Examples

```rust
let mut s = Stack { elems: vec![3, 1, 2] };
let mut b = Bag::new();
move_all(&mut s, &mut b)  // 3: s.elems is now empty, and b.items() is [1, 2, 3]

let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
let mut y = Stack { elems: vec![String::from("c")] };
move_all(&mut x, &mut y)  // 2: y.elems is now ["c", "b", "a"]
```
