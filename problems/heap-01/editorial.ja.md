`BinaryHeap` は**最大ヒープ**です。`pop` と `peek` は、`Ord` で比べて最も大きい要素を返します。
タプルは左の要素から順に比べられるので、キーを重要な順に並べたタプルを使えます。
`(severity, Reverse(arrival), name)` では、まず重症度が高いほうが勝ちます。重症度が同じなら、
`Reverse` によって到着番号が**小さい**ほうが大きいとみなされ、先に来た患者が先に出てきます。
到着番号はすべて異なるので、名前が実際に比べられることはありません。
名前はキーと一緒に運ばれるだけです。

落とし穴は同点の扱いです。最初のフィールド `(u8, String)` のままだと、重症度が同じときは
名前で比べることになり、辞書順で最後の名前が先に出てきます。`Reverse(name)` で例が通るのは、
`bob` が `cid` より先に来ていて、しかも辞書順でも前だからにすぎません。
到着順は記録しておく必要があり、増える一方のカウンタを持たせます。
`self.heap.len()` では代わりになりません。`next` の後に減るので、到着番号が重複してしまいます。

`peek` は先頭の要素から借用した `&str` を返します。`self.heap.peek()` は
`Option<&(u8, Reverse<u64>, String)>` を返し、`name.as_str()` はそこから
`&self` と同じ期間だけ借用します。

`k_smallest` では、各値を `Reverse` で包むと最大ヒープが最小ヒープになります。
代わりに値の符号を反転する（`-x`）と、`i32::MIN` でオーバーフローします（デバッグビルドではパニック）。
値を順番どおりに取り出せるのは `pop` と `into_sorted_vec` だけです。`iter`、`into_iter`、`into_vec` は
ヒープ内部の配列をそのまま走査し、その配列はソートされていないので、先頭の `k` 個を取ると
誤った答えになります。ヒープが空になったら止めれば `k > v.len()` にも対応できます。
一方、`Vec::with_capacity(k)` は `k = usize::MAX` のときにパニックします。

ヒープを使わずに `Vec` を走査する場合、`max_by_key` は同じ最大値が複数あると**最後の**要素を
返すことに注意してください。重症度が同じ患者のうち、最後に来た患者を選んでしまいます。

### 別解

`BTreeMap` はキーをソートされた順に保つので、最小のキー `(Reverse(severity), arrival)` が
次に診察する患者のものです。`pop_first` と `first_key_value` で、取り除いたり見たりできます。

```rust
use std::collections::BTreeMap;

pub struct Triage {
    waiting: BTreeMap<(Reverse<u8>, u64), String>,
    arrivals: u64,
}

impl Triage {
    pub fn new() -> Self {
        Triage { waiting: BTreeMap::new(), arrivals: 0 }
    }

    pub fn arrive(&mut self, name: &str, severity: u8) {
        self.waiting.insert((Reverse(severity), self.arrivals), name.to_string());
        self.arrivals += 1;
    }

    pub fn next(&mut self) -> Option<String> {
        self.waiting.pop_first().map(|(_, name)| name)
    }

    pub fn peek(&self) -> Option<&str> {
        self.waiting.first_key_value().map(|(_, name)| name.as_str())
    }

    pub fn len(&self) -> usize {
        self.waiting.len()
    }
}
```

`k_smallest` は、最大ヒープに `k` 個の値だけを持たせる方法でも書けます。それまでに見た値のうち小さいほうから
`k` 個を持ち、その中で最大の値が先頭に来ます。それより小さい値が来たら入れ替えます。
`into_sorted_vec` は値を昇順に並べて返します。

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    // それまでに見た値のうち小さいほうから k 個を持つ（その中で最大の値が先頭）
    let mut heap = BinaryHeap::new();
    for &x in v {
        if heap.len() < k {
            heap.push(x);
        } else if heap.peek().is_some_and(|&top| x < top) {
            heap.pop();
            heap.push(x);
        }
    }
    heap.into_sorted_vec()
}
```

1 回だけ求めるなら、コピーをソートするのがいちばん簡単です。

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    let mut sorted = v.to_vec();
    sorted.sort();
    sorted.truncate(k);
    sorted
}
```
