`entry` API を使うと、挿入と更新を 1 回の検索で書けます：`*counts.entry(key).or_insert(0) += 1`。
`usize::default()` は `0` なので `or_default()` でも構いません。

並べ替えは、件数を**逆順**に比べ（`nb.cmp(na)`）、同数のときは `then_with` で単語を比べます。

### 別解

`BTreeMap` で数えると、取り出した時点で単語がアルファベット順に並んでいます。`sort_by` は安定ソートなので、
件数だけで並べ替えても同数の単語はアルファベット順のまま残ります。

```rust
use std::collections::BTreeMap;

pub fn word_count(text: &str) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_lowercase()).or_default() += 1;
    }
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}
```

`sort_by_key(|(w, n)| (std::cmp::Reverse(*n), w.clone()))` でも書けますが、比較のたびに `String` を複製します。
