まず調べて、本当に何かが変わるときだけコピーします。

`[i32]` の所有版は（`ToOwned` によって）`Vec<i32>` なので、`Cow<'_, [i32]>` は `&[i32]` か `Vec<i32>` を
持ちます。`to_mut` は `&mut Vec<i32>` を返します。`Borrowed` のときは、まずスライスを新しい `Vec` にコピーして
`Owned` に切り替えます。`Owned` のときは、持っている `Vec` をそのまま渡すだけです。無条件に呼ぶと、`max` を
超える要素がなくてもコピーしてしまうので、先に `iter().any(...)` で調べます。`for x in v.iter()` のループの
中で `to_mut` を呼ぶと、ループが `v` を借用したままなのでコンパイルが通りません。

### 別解

バリアントで `match` しても書けます。`Owned` の `Vec` はその場で書き換え、`Borrowed` の
スライスは必要なときだけ新しい `Vec` に置き換えます。`Borrowed` のアームの `s` は `v` を借用していますが、
最後に使うのは右辺なので、`*v` への代入は受け入れられます。

```rust
pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    match v {
        Cow::Owned(vec) => {
            for x in vec.iter_mut() {
                *x = (*x).min(max);
            }
        }
        Cow::Borrowed(s) => {
            if s.iter().any(|&x| x > max) {
                *v = Cow::Owned(s.iter().map(|&x| x.min(max)).collect());
            }
        }
    }
}
```
