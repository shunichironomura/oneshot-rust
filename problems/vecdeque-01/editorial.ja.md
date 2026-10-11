`VecDeque` はリングバッファで、**両端**での要素の追加と削除が O(1) でできます。そのため、上限つきの履歴に
うってつけです。最新の操作は `push_back` で後ろに入れ、`undo` は `pop_back` で取り出します。履歴が上限を
超えたら、最も古い操作を `pop_front` で前から捨てます。`Vec` でも `remove(0)` で同じことはできますが、
残りの要素がすべて 1 つずつ左にずれます。

`record` では、先に追加してから長さを確かめます。逆の順序で「履歴がいっぱいなら最も古い操作を捨ててから
追加する」と書くと、上限が `0` のときに失敗します。空の両端キューに対する `pop_front` は何もしないので、
追加した操作が 1 つ残ってしまいます。

`recent` は最新の操作からさかのぼります：`iter().rev().take(n)`。アダプタの順序が大事で、
`take(n).rev()` だと**最も古い** `n` 個を逆順にしたものになります。`take` は両端キューの終わりで止まるだけ
なので、`n` が `len()` より大きくても、`usize::MAX` でも、特別扱いは要りません。一方、
`Vec::with_capacity(n)` は `n = usize::MAX` でパニックし、`len() - n` は `n` が `len()` より大きいと
アンダーフローします。

両端キューのイテレータは `&String` を返すので、`map(|s| s.as_str())` で 1 つずつ `&str` に変換します
（`map(String::as_str)` でも構いません）。参照外し型強制が働くのは 1 つの参照に対してだけで、イテレータや
`Vec` の要素には働かないので、変換は明示的に書く必要があります。ライフタイムの省略規則により、返す `&str` は
`&self` から借用します。`recent` と `oldest` は文字列を 1 つもコピーしません。

`josephus` では、次に `1` を言う人が先頭に来るように、輪を `VecDeque` で表します。`rotate_left(m)` は
先頭の `m` 個の要素を、順番を保ったまま後ろに回します。`k - 1` 人が数えて後ろに回ると、`k` を言う人が
先頭に来るので、`pop_front` で取り除きます。すると次の人が先頭に来て、また `1` から数え始められます。

落とし穴は `rotate_left` のドキュメントにあります。`m` が `len()` より大きいと**パニック**します。
数えるときは輪を何周もするので、意味があるのは `k - 1` 歩を `len` で割った余りの `(k - 1) % len` 歩だけです。
例の入力でも、余りを取らずに `rotate_left(k - 1)` と書くと、最後に 4 番だけが残ったところで `k - 1` が
`2` になってパニックします。ほかにも次のような失敗があります。

- `k % len - 1` は、`k` が輪の人数の倍数のときにアンダーフローします。
- 1 人ずつ後ろに回す方法（`pop_front` して `push_back` するのを `k - 1` 回）は正しいのですが、
  1 人抜けるごとに `k` 歩かかるので、`k = usize::MAX` では終わりません。

### 別解

`recent` は、`range` で適切な位置から始めてさかのぼる方法でも書けます。`saturating_sub` は、`n` が
`len()` より大きいときにアンダーフローせず `0` で止まります。

```rust
pub fn recent(&self, n: usize) -> Vec<&str> {
    let start = self.actions.len().saturating_sub(n);
    self.actions.range(start..).rev().map(|s| s.as_str()).collect()
}
```

`josephus` は、`Vec` の添字を使っても書けます。`remove(pos)` の後は、次の人が `pos` に来ます
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
