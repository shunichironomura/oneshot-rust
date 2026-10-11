`v` は借用したスライス（`&[i32]`）なので、`v.sort()` はコンパイルが通りません。ソートには `&mut` が必要です。
`to_vec()` で値を自分の `Vec` にコピーしてから、そちらをソートします。整数は等しい値どうしを区別できないので、
`sort_unstable` で十分です。

要素数が偶数のとき、真ん中の 2 つの値を `i32` のまま足してはいけません。`i32::MAX + i32::MAX` はオーバーフローし、
デバッグビルドではパニックします。そもそも整数の割り算では `.5` が切り捨てられます。先にそれぞれを `f64` に
変換しましょう。`f64` はどの `i32` も正確に表せ、`i32` 2 つの和（絶対値は 2^32 以下で、整数を正確に表せる上限の 2^53
よりずっと小さい）も正確に表せます。2 で割るのも正確です。`f64::from` は情報を失わない変換です。ここでは
`as f64` でも同じ結果になります。

`mode` で回数を数えるには、`*counts.entry(x).or_insert(0) += 1` がよく使われます。間違えやすいのは、
同じ回数の値があるときの選び方です。

- `HashMap` を回す順番は決まっていないので、「最初に見つかったもの」には意味がありません。
- `max_by_key` は、最大のものが複数あるとき**最後の**ものを返します（`min_by_key` は最初のもの）。

そのため `max_by_key(|&(_, count)| count)` は、同じ回数の値のうちどれかを返し、どれになるかはハッシュの順番
次第です。ルール全体をキーに入れましょう。タプルは要素ごとに順に比べられるので、`(count, Reverse(value))` は
まず回数を比べ、回数が同じなら値が小さいほど `Reverse` が大きくなります。`Reverse(value)` の代わりに `-value`
と書いても同じに見えますが、`i32::MIN` の符号を反転するとオーバーフローします。

### 別解

`HashMap` を使わない `mode` です。ソートしたコピーでは等しい値が隣り合うので、`chunk_by` で等しい値の連なりを
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

`median` では、真ん中の値を `i64` で足す方法もあります。`i32` の値 2 つの和は必ず `i64` に収まり、その和を
`f64` に変換するのも正確です。`f64::from` は `i64` を受け付けない（`i64` のすべての値を `f64` で正確に表せる
わけではない）ので、この変換には `as` が必要です。変換する前に 2 で割ってはいけません。整数の割り算では
`-1 / 2` が `0` になります。

```rust
pub fn median(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(f64::from(sorted[mid]))
    } else {
        let sum = i64::from(sorted[mid - 1]) + i64::from(sorted[mid]);
        Some(sum as f64 / 2.0)
    }
}
```
