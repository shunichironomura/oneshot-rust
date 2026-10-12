`HashMap` で回数を数えるには、`*counts.entry(x).or_insert(0) += 1` がよく使われます。間違えやすいのは、
同じ回数の値があるときの選び方です。

- `HashMap` を回す順番は決まっていないので、「最初に見つかったもの」には意味がありません。
- `max_by_key` は、最大のものが複数あるとき**最後の**ものを返します（`min_by_key` は最初のもの）。

そのため `max_by_key(|&(_, count)| count)` は、同じ回数の値のうちどれかを返し、どれになるかはハッシュの順番
次第です。ルール全体をキーに入れましょう。タプルは要素ごとに順に比べられるので、`(count, Reverse(value))` は
まず回数を比べ、回数が同じなら値が小さいほど `Reverse` が大きくなります。`Reverse(value)` の代わりに `-value`
と書いても同じに見えますが、`i32::MIN` の符号を反転するとオーバーフローします。

### 別解

`HashMap` を使わない方法です。ソートしたコピーでは等しい値が隣り合うので、`chunk_by` で等しい値の連なりを
1 つずつ取り出せます。連なりは昇順に並んでいて、`min_by_key` は最小のものが複数あるとき最初のものを返すので、
キーを `Reverse(run.len())` にすれば、最も多く現れる値のうち最小のものが選ばれます。

```rust
use std::cmp::Reverse;

pub fn mode(v: &[i32]) -> Option<i32> {
    let mut sorted = v.to_vec();
    sorted.sort_unstable();
    sorted
        .chunk_by(|a, b| a == b)
        .min_by_key(|run| Reverse(run.len()))
        .map(|run| run[0])
}
```
