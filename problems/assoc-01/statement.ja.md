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

次の関数の本体はすでに書かれていますが、シグネチャが特定の具体的なコンテナ型しか
受け取らないので、ハーネス（`tests.rs` タブ）がコンパイルできません。**シグネチャだけ**を
書き換えて、ハーネスがコンパイルでき、テストに合格するようにしてください。
本体、トレイト、2 つの型とその実装は変更しないでください。

```rust
pub fn move_all(from: &mut Stack<u8>, to: &mut Bag) -> usize
```

- `move_all(from, to)` は `from` から `take_last` で値を 1 つずつ取り出し、それぞれを `put` で
  `to` に入れて、移した値の個数を返します。`to` にもともと入っていた値はそのまま残ります。
  2 つのコンテナは、保持する値の型が同じであれば、違う型でも構いません。ハーネスが値を
  移すのは、`Stack<u8>` から `Bag`、`Bag` から `Stack<u8>`、`Bag` から `Bag`、2 つの
  `Stack<String>` の間、2 つの `Stack<i64>` の間、`Shelf<Book>` から `Stack<Book>` とその
  逆向きです。

`Shelf<T>` はハーネス自身のコンテナ型（`Item` は `T`）で、`Book` はハーネス自身の値の型です。
`Book` は `Copy` でも `Clone` でもありません。

### 例

```rust
let mut s = Stack { elems: vec![3, 1, 2] };
let mut b = Bag::new();
move_all(&mut s, &mut b)  // 3：s.elems は空になり、b.items() は [1, 2, 3] になる

let mut x = Stack { elems: vec![String::from("a"), String::from("b")] };
let mut y = Stack { elems: vec![String::from("c")] };
move_all(&mut x, &mut y)  // 2：y.elems は ["c", "b", "a"] になる
```
