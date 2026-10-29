`Container` は**関連型** `Item` を持つトレイトです。`Item` はコンテナが保持する値の型です。
`Stack<T>` は任意の型 `T` の値を保持し、`Bag` はバイト（`u8`）を保持します。
トレイト、2 つの型とその実装はエディタに入っています。

```rust
pub trait Container {
    type Item;
    fn items(&self) -> &[Self::Item];  // すべての値（順番どおり）
}

pub struct Stack<T> { pub elems: Vec<T> }  // impl<T> Container for Stack<T>：Item = T
pub struct Bag { pub bytes: Vec<u8> }      // impl Container for Bag：Item = u8
```

次の関数の本体はすでに書かれていますが、コードはコンパイルが通らず、シグネチャも `Bag` しか
受け取りません。**シグネチャだけ**を書き換えて、ハーネス（`tests.rs` タブ）がコンパイルでき、
テストに合格するようにしてください。本体、トレイト、2 つの型とその実装は変更しないでください。

```rust
pub fn total(c: &Bag) -> i64
```

- `total(c)` はすべての値を `i64` に変換して、その合計を返します（空のコンテナなら `0` です。
  `true` は 1、`false` は 0 として数えます）。ハーネスは `Bag`、`i32`・`u16`・`u32`・`i64`・
  `bool` のスタック、ハーネス自身のコンテナ型 `Shelf<i8>`（`Item` は `i8`）に対して呼び出します。

### 例

```rust
total(&Bag { bytes: vec![3, 1, 2] })              // 6
total(&Stack { elems: vec![-5i32, 2] })           // -3
total(&Stack { elems: vec![40_000u16, 30_000] })  // 70000
```
