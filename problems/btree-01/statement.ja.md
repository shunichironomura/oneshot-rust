`Timeline` は出来事を記録する年表です。各出来事は時刻 `t`（`u32`）と短い説明を持ち、
同じ時刻の出来事は 1 つまでです。構造体はすでにエディタに入っています。出来事は
`BTreeMap`（キーを順序どおりに並べて保持するマップ）に保存されています。`new` と `len` は
実装済みです。残りのメソッドを実装してください。

```rust
use std::collections::BTreeMap;

pub struct Timeline {
    events: BTreeMap<u32, String>,
}

impl Timeline {
    pub fn new() -> Self;
    pub fn add(&mut self, t: u32, what: &str) -> Option<String>;
    pub fn len(&self) -> usize;
    pub fn between(&self, from: u32, to: u32) -> Vec<(u32, &str)>;
    pub fn last_before(&self, t: u32) -> Option<(u32, &str)>;
    pub fn split_from(&mut self, t: u32) -> Timeline;
}
```

- `new` は空の年表を、`len` は出来事の数を返します（どちらも実装済み）。
- `add` は時刻 `t` に出来事 `what` を追加して `None` を返します。`t` にすでに出来事があるときは
  それを置き換え、古い説明を `Some(old)` で返します。
- `between` は時刻が `from <= time <= to`（両端を含む）の出来事をすべて `(時刻, 説明)` の組にして、
  時刻の昇順に並べて返します。`from > to` のときの結果は空です。
- `last_before` は、時刻が `t` **より小さい**（`t` ちょうどは含まない）出来事のうち、
  最も遅いものを返します。ない場合は `None` です。
- `split_from` は、時刻が `t` 以上の出来事をすべて `self` から新しい年表に移して、それを返します。
  `self` には時刻が `t` より小さい出来事が残ります。
- `0` や `u32::MAX` を含め、どの `u32` の値も出来事の時刻や引数になりえます。

### 例

どの例も、時刻 `5` の "wake"、`10` の "work"、`20` の "sleep" の 3 つの出来事がある年表から始めます。

- `len()` は `3`、`between(6, 20)` は `[(10, "work"), (20, "sleep")]` です。
- `last_before(10)` は `Some((5, "wake"))`、`last_before(100)` は `Some((20, "sleep"))` です。
- `add(10, "lunch")` は `Some("work".to_string())` を返し、`len()` は `3` のままです。
- `split_from(10)` を呼ぶと、元の年表には時刻 `5` の出来事だけが残り、
  時刻 `10` と `20` の出来事を持つ年表が返ります。
