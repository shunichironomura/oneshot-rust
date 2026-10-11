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

The bodies of the four functions below are already written, but their signatures accept only
particular concrete container types, and the code does not compile. Change **only the signatures**
so that the test harness (the `tests.rs` tab) compiles and passes. Do not change the bodies, the
trait, the two types or their impls. You may add `use` declarations.

```rust
pub fn move_all(from: &mut Stack<u8>, to: &mut Bag) -> usize
pub fn total(c: &Bag) -> i64
pub fn largest(c: &Stack<String>) -> Option<&String>
pub fn describe_all(c: &Stack<char>) -> Vec<String>
```

- `move_all(from, to)` takes the values out of `from` one by one with `take_last`, puts each of them
  into `to` with `put`, and returns how many values it moved. The values already in `to` stay. The
  two containers may be of different types, as long as they hold values of the same type. The
  harness moves values from `Stack<u8>` to `Bag`, from `Bag` to `Stack<u8>`, from `Bag` to `Bag`,
  between two `Stack<String>`s, between two `Stack<i64>`s, and from `Shelf<Book>` to `Stack<Book>`
  and back.
- `total(c)` converts every value to an `i64` and returns their sum (`0` for an empty container;
  `true` counts as 1 and `false` as 0). The harness calls it on `Bag`, on stacks of `i32`, `u16`,
  `u32`, `i64` and `bool`, and on `Shelf<i8>`.
- `largest(c)` returns a reference to the largest value in `c`, compared with `Ord`, or `None` if `c`
  is empty. If several values are the largest, it refers to the last of them. The harness calls it
  on `Bag`, on stacks of `i32`, `&str` and `String`, and on `Shelf<Book>`.
- `describe_all(c)` formats every value with `Display` (as `{}` does) and returns the strings in
  order. The harness calls it on `Bag`, on stacks of `char`, `f64` and `&str`, and on `Shelf<i8>`
  and `Shelf<Book>`.

`Shelf<T>` is a container type of the harness (its `Item` is `T`), and `Book` is a value type of the
harness that implements only `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Display`. It is not
`Copy`, `Clone` or `Debug`.

### Examples

```rust
let mut s = Stack { elems: vec![3, 1, 2] };
let mut b = Bag::new();
move_all(&mut s, &mut b)  // 3: s.elems is now empty, and b.items() is [1, 2, 3]
total(&b)                 // 6
largest(&b)               // Some(&3)

let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
let mut y = Stack { elems: vec![String::from("c")] };
move_all(&mut x, &mut y)  // 2: y.elems is now ["c", "b", "a"]

total(&Stack { elems: vec![-5i32, 2] })           // -3
total(&Stack { elems: vec![40_000u16, 30_000] })  // 70000
largest(&Stack { elems: vec!["a", "b"] })         // Some(&"b")
describe_all(&Stack { elems: vec!['h', 'i'] })    // ["h", "i"]
```
