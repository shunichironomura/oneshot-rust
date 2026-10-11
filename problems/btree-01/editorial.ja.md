`BTreeMap` はキーを順序どおりに保持しているので、`range` を使うと、ある範囲に入るキーだけを昇順にたどれます。
そのイテレータは両端から取り出せます。`next()` は範囲の中で最小のキーを、`next_back()` は最大のキーを返します。
3 つの検索は、どれも `range` を 1 回呼ぶだけで書けます。

- `between`：`range(from..=to)`。
- `last_before`：`range(..t).next_back()`。`t` より小さいキーのうち最大のもの。
- `next_at_or_after`：`range(t..).next()`。`t` 以上のキーのうち最小のもの。

落とし穴は `range` のドキュメントに書かれています。始まりが終わりより大きいと**パニック**します
（`range start is greater than range end in BTreeMap`）。そのため `between(9, 3)` は `range` を呼ぶ前に
処理しなければなりません。作ったばかりの空のマップはまだ内部の木を持っていないのでパニックしません。
そのため、新しい空の年表で試しただけでは気づきにくいです。

範囲の端で計算をするのも避けてください。`range(from..to + 1)` は `to` が `u32::MAX` のときにオーバーフローし、
`range(..=t - 1)` は `t` が `0` のときにアンダーフローします。どちらもデバッグビルド（ジャッジもそうです）では
パニックします。飽和演算にすればパニックは消えますが、答えが間違います。`..=t.saturating_sub(1)` は
`t` が `0` のとき時刻 `0` を含んでしまいます。範囲の書き方 `..=to` や `..t` を使えば、計算なしで同じ端を
表せます。

`split_off(&t)` は `t` 以上のキーを新しいマップに移し、小さいキーを元の場所に残します。これはそのまま
`split_from` です。`prune_before` で残したいのは反対側なので、`split_off` が返したものを
`std::mem::replace` で入れ替えます。`replace` は取り除いた側を返すので、その数を数えられます。

イテレータが返すのは `(&u32, &String)` の組です。参照外し型強制はタプルの中までは働かないので、
`(time, what)` に変換しても `(u32, &String)` のままで、`Vec<(u32, &str)>` に `collect` するとコンパイルエラーに
なります。`what.as_str()` で明示的に変換します。
ライフタイムの省略規則により、返す `&str` は `&self` から借用したものになります。

### 別解

`between` は全部の出来事をフィルタしても書けます。パニックはしませんが、指定された範囲だけでなく
マップ全体をたどることになります。

```rust
pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)> {
    self.events
        .iter()
        .filter(|&(&time, _)| from <= time && time <= to)
        .map(|(&time, what)| (time, what.as_str()))
        .collect()
}
```

`prune_before` は `retain` を使い、前後の数の差を返す方法もあります。

```rust
pub fn prune_before(&mut self, t: u32) -> usize {
    let before = self.events.len();
    self.events.retain(|&time, _| time >= t);
    before - self.events.len()
}
```
