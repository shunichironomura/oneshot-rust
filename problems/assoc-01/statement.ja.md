`Container` は**関連型** `Item` を持つトレイトです。`Item` はコンテナが保持する値の型です。
`Stack<T>` は任意の型 `T` の値を保持し、`Bag` はバイト（`u8`）を昇順に並べて保持します。
トレイト、2 つの型とその実装はエディタに入っています。

```rust
pub trait Container {
    type Item;
    fn put(&mut self, x: Self::Item);              // x を追加する
    fn take_last(&mut self) -> Option<Self::Item>; // 最後の値を取り除いて返す
    fn items(&self) -> &[Self::Item];              // すべての値（順番どおり）
}

pub struct Stack<T> { pub elems: Vec<T> }  // impl<T> Container for Stack<T>：Item = T
pub struct Bag { /* 非公開 */ }            // impl Container for Bag：Item = u8
impl Bag { pub fn new() -> Self }          // 空の Bag
```

次の 4 つの関数の本体はすでに書かれていますが、シグネチャが特定の具体的なコンテナ型しか
受け取らないので、コードはコンパイルが通りません。**シグネチャだけ**を書き換えて、
ハーネス（`tests.rs` タブ）がコンパイルでき、テストに合格するようにしてください。
本体、トレイト、2 つの型とその実装は変更しないでください。`use` 宣言は追加して構いません。

```rust
pub fn move_all(from: &mut Stack<u8>, to: &mut Bag) -> usize
pub fn total(c: &Bag) -> i64
pub fn largest(c: &Stack<String>) -> Option<&String>
pub fn describe_all(c: &Stack<char>) -> Vec<String>
```

- `move_all(from, to)` は `from` から `take_last` で値を 1 つずつ取り出し、それぞれを `put` で
  `to` に入れて、移した値の個数を返します。`to` にもともと入っていた値はそのまま残ります。
  2 つのコンテナは、保持する値の型が同じであれば、違う型でも構いません。ハーネスが値を
  移すのは、`Stack<u8>` から `Bag`、`Bag` から `Stack<u8>`、`Bag` から `Bag`、2 つの
  `Stack<String>` の間、2 つの `Stack<i64>` の間、`Shelf<Book>` から `Stack<Book>` とその
  逆向きです。
- `total(c)` はすべての値を `i64` に変換して、その合計を返します（空のコンテナなら `0` です。
  `true` は 1、`false` は 0 として数えます）。ハーネスは `Bag`、`i32`・`u16`・`u32`・`i64`・
  `bool` のスタック、`Shelf<i8>` に対して呼び出します。
- `largest(c)` は、`c` の中で（`Ord` で比べて）最大の値への参照を返します。`c` が空なら `None` を
  返します。最大の値が複数あるときは、そのうち最後のものを指します。ハーネスは `Bag`、
  `i32`・`&str`・`String` のスタック、`Shelf<Book>` に対して呼び出します。
- `describe_all(c)` はすべての値を `Display` で（`{}` と同じように）文字列にし、順番どおりに
  返します。ハーネスは `Bag`、`char`・`f64`・`&str` のスタック、`Shelf<i8>`、`Shelf<Book>` に
  対して呼び出します。

`Shelf<T>` はハーネス自身のコンテナ型（`Item` は `T`）で、`Book` はハーネス自身の値の型です。
`Book` が実装しているのは `PartialEq`、`Eq`、`PartialOrd`、`Ord`、`Display` だけで、
`Copy`、`Clone`、`Debug` ではありません。

### 例

```rust
let mut s = Stack { elems: vec![3, 1, 2] };
let mut b = Bag::new();
move_all(&mut s, &mut b)  // 3：s.elems は空になり、b.items() は [1, 2, 3] になる
total(&b)                 // 6
largest(&b)               // Some(&3)

let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
let mut y = Stack { elems: vec![String::from("c")] };
move_all(&mut x, &mut y)  // 2：y.elems は ["c", "b", "a"] になる

total(&Stack { elems: vec![-5i32, 2] })           // -3
total(&Stack { elems: vec![40_000u16, 30_000] })  // 70000
largest(&Stack { elems: vec!["a", "b"] })         // Some(&"b")
describe_all(&Stack { elems: vec!['h', 'i'] })    // ["h", "i"]
```
