`sort_by` には `Ordering` を返す関数を渡します。複数のキーは `then_with` でつなぎます。
`then_with` は、それまでのキーが `Equal` のときだけ次のキーを比べます。キーを**降順**にするには、比べる順を入れ替えます（`b.score.cmp(&a.score)`）。

`dedup_by` は**連続した**重複を取り除き、それぞれの最初の要素を残します。そこで、まず名前で並べ、
同じプレイヤーの中ではいちばんよい記録が先頭に来るようにしておけば、残る要素がちょうどいちばんよい記録になります。
その後でもう一度、最終的な順に並べ替えます。「よい記録」の比較を関数として一度だけ書いておくと、2 回のソートで食い違いが起きません。

### 別解

プレイヤーごとのいちばんよい記録を `HashMap` に持ち、`sort_by_key` で並べる方法です。
`std::cmp::Reverse` を使うと、タプルの中の 1 つのキーだけ順序を逆にできます。
キーを返すクロージャは要素の中への参照を返せないので、名前は複製します。

```rust
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn leaderboard(runs: Vec<Run>) -> Vec<Run> {
    let mut best: HashMap<String, Run> = HashMap::new();
    for run in runs {
        let better = match best.get(&run.name) {
            Some(b) => (run.score, Reverse(run.time)) > (b.score, Reverse(b.time)),
            None => true,
        };
        if better {
            best.insert(run.name.clone(), run);
        }
    }
    let mut result: Vec<Run> = best.into_values().collect();
    result.sort_by_key(|r| (Reverse(r.score), r.time, r.name.clone()));
    result
}
```
