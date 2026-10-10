`push` は `v` のバッファを確保し直して要素を移動することがあり、そのとき生ポインタ `first` はダングリングになります。
確保し直さない場合でも、`&mut` 経由で `v` を変更すると、それ以前に作ったポインタは無効になり、Miri の
Stacked Borrows がそれを検出します。

`i32` は `Copy` なので、`push` の**前に**値を読んでおけば解決します。`unsafe` は要りません。

### 別解

先に結果を計算してから `push` すれば、変更をまたいで何も保持せずに済みます。

```rust
pub fn push_and_sum(v: &mut Vec<i32>, extra: i32) -> i32 {
    let result = v.iter().sum::<i32>() + extra + v[0];
    v.push(extra);
    result
}
```
