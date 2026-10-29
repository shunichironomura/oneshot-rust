2024 より前は、`Box<[T]>` に対して `.into_iter()` を呼んでも、後方互換性のためにボックスを借用するだけで、
参照 `&T` を順に返していました（Rust 2021 では `boxed_slice_into_iter` リントの警告も出ます）。
2024 では `IntoIterator for Box<[T]>` が使われ、ボックスを消費して値そのものを順に返します。

- そのため `p` は `u32` になり、`*p` はコンパイルできません（E0614）。
- `*` を消すだけでは足りません。`into_iter()` が `points` をムーブするので、ループの後の `points.len()` は
  ムーブ済みの値を使うことになります（E0382）。

`points.iter()` なら以前と同じ意味になり、`cargo fix --edition` もこう書き換えます。

### 別解

先に長さを読んでから、ボックスを消費する方法です。`for p in points` は値を順に取り出します。

```rust
pub fn score(points: Box<[u32]>) -> u32 {
    let bonus = points.len() as u32;
    let mut total = 0;
    for p in points {
        total += p;
    }
    total + bonus
}
```

イテレータを使えば、要素ごとに得点と 1 を足せます。

```rust
pub fn score(points: Box<[u32]>) -> u32 {
    points.iter().map(|p| p + 1).sum()
}
```
