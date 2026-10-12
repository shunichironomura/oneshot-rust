次に `1` を言う人が先頭に来るように、輪を `VecDeque` で表します。`rotate_left(m)` は
先頭の `m` 個の要素を、順番を保ったまま後ろに回します。`k - 1` 人が数えて後ろに回ると、`k` を言う人が
先頭に来るので、`pop_front` で取り除きます。すると次の人が先頭に来て、また `1` から数え始められます。

落とし穴は `rotate_left` のドキュメントにあります。`m` が `len()` より大きいと**パニック**します。
数えるときは輪を何周もすることがあるので、意味があるのは `k - 1` 歩を `len` で割った余りの
`(k - 1) % len` 歩だけです。例の入力でも、余りを取らずに `rotate_left(k - 1)` と書くと、
最後に 4 番だけが残ったところで `k - 1` が `2` になってパニックします。ほかにも次のような失敗があります。

- `k % len - 1` は、`k` が輪の人数の倍数のときにアンダーフローします。
- 1 人ずつ後ろに回す方法（`pop_front` して `push_back` するのを `k - 1` 回）は正しいのですが、
  1 人抜けるごとに `k` 歩かかるので、`k = usize::MAX` では終わりません。

### 別解

`Vec` の添字を使っても書けます。`remove(pos)` の後は、次の人が `pos` に来ます
（`pos` が最後の添字だった場合、次の人は `0` にいますが、次の回の `%` がそれを処理します）。
`k - 1` は足す前に余りを取ってください。`pos + k - 1` は `k = usize::MAX` のときにオーバーフローします。

```rust
pub fn josephus(n: usize, k: usize) -> Vec<usize> {
    let mut circle: Vec<usize> = (1..=n).collect();
    let mut order = Vec::new();
    let mut pos = 0; // the index of the person who says 1
    while !circle.is_empty() {
        pos = (pos + (k - 1) % circle.len()) % circle.len();
        order.push(circle.remove(pos));
    }
    order
}
```
