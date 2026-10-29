このコードは、1 つのスライスへの `&mut` を同時に 2 つ持っているので、エラー E0499 になります。
借用チェッカーから見ると、`&mut balances[from]` はどの要素かを区別せずに「スライスのどれかの要素」を
借用します。借用チェッカーには `from != to` であることが証明できず、もし等しければ 2 つの参照が同じ値を
指してしまいます。

`<[T]>::get_disjoint_mut([from, to])`（Rust 1.86 で安定化）は、これを実行時に確かめて複数の `&mut` を
一度に返します。`Result` を返し、添字が重なる（または範囲外の）ときは `Err` になります。
ここでは `from == to` がちょうど「何もしない」場合なので、`if let Ok(..)` で済みます。

1 行ずつ書き換えてもコンパイルは通りますが、`from == to` の場合に注意が必要です。
`balances[to] += balances[from]; balances[from] = 0;` は、残高を 2 倍にしてから 0 にしてしまいます。
借用チェッカーが防いでいた「2 つの参照が同じ要素を指す」場合こそが、壊れる場合なのです。

### 別解

`std::mem::take` は 0 を残して元の残高を返すので、`from == to` のときは残高がそのまま戻されます。

```rust
pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let amount = std::mem::take(&mut balances[from]);
    balances[to] += amount;
}
```

コンパイラが提案する `split_at_mut` でもスライスへの `&mut` を 2 つ得られますが、
`from < to`、`from > to`、`from == to` を別々に扱う必要があります。
