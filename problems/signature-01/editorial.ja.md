本体が要素に使っているのは `>` だけなので、`T: PartialOrd` を満たす任意の型 `T` に対するジェネリック関数にします。
浮動小数点数（`f64`）は `PartialOrd` しか実装していない（`NaN` はどの値とも比較できない）ので、`Ord` では足りません。

返す参照は `items` の中を指します。参照の引数は `items` だけなので、ライフタイムは省略できます。

### 別解

境界は `where` 節にも書けます。

```rust
pub fn largest<T>(items: &[T]) -> &T
where
    T: PartialOrd,
```
